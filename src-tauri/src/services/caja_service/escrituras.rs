use super::{
    configuracion,
    consultas::{abierto, resumen, turno},
    es_administrador, invalido, modo_cierre, CajaService, LIMITE_CONTEO, LIMITE_FONDO,
};
use crate::{
    dto::caja_dto::{AbrirCajaDto, AclararCajaDto, CierreCajaDto, ManualCajaDto, RevertirCajaDto},
    entities::caja_chica::CajaChicaEntity,
    errors::AppError,
    services::inventario_service::idempotencia::IdempotenciaService,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::Serialize;
use serde_json::{json, Value};

fn escritura<T: Serialize>(
    conn: &mut Connection,
    actor: i64,
    id: &str,
    accion: &str,
    req: &T,
    aplicar: impl FnOnce(&Transaction<'_>, bool) -> Result<Value, AppError>,
) -> Result<Value, AppError> {
    if id != id.trim() || !(16..=100).contains(&id.len()) {
        return Err(invalido("Identificador de operación inválido."));
    }
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let admin = es_administrador(&tx, actor)?;
    if accion == "CAJA_ACLARAR" && !admin {
        return Err(AppError::PermisoDenegado);
    }
    if let Some(resultado) =
        IdempotenciaService::evaluar_y_registrar(&tx, Some(id), accion, req, actor)?
    {
        return Ok(resultado);
    }
    let resultado = aplicar(&tx, admin)?;
    IdempotenciaService::finalizar_solicitud(&tx, Some(id), accion, req, actor, &resultado)?;
    tx.commit()?;
    Ok(resultado)
}

fn auditar(
    tx: &Transaction<'_>,
    actor: i64,
    operacion: &str,
    accion: &str,
    entidad: (&str, i64),
    datos: (Option<Value>, Value),
    motivo: Option<&str>,
) -> Result<(), AppError> {
    let n = tx.execute(
        "INSERT INTO auditoria(id_operacion,id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado)
         SELECT ?1,id_usuario,'USUARIO',username,rol,'CAJA',?2,?3,?4,?5,?6,?7,'OK' FROM usuarios WHERE id_usuario=?8",
        params![operacion,accion,entidad.0,entidad.1.to_string(),datos.0.map(|v|v.to_string()),datos.1.to_string(),motivo,actor],
    )?;
    if n != 1 {
        return Err(AppError::PermisoDenegado);
    }
    Ok(())
}

fn exigir_operativa(conn: &Connection) -> Result<(), AppError> {
    if configuracion(conn)?.1 {
        return Err(invalido(
            "Hay una recuperación pendiente. Solo se puede consultar o cerrar el turno recuperado.",
        ));
    }
    Ok(())
}

fn caja_manual(conn: &Connection, id: i64, actor: i64) -> Result<CajaChicaEntity, AppError> {
    exigir_operativa(conn)?;
    let c = turno(conn, id)?;
    if c.id_usuario != actor {
        return Err(AppError::PermisoDenegado);
    }
    if c.estado != "ABIERTA" || c.por_conciliar {
        return Err(invalido(
            "Los movimientos manuales requieren tu turno abierto y operativo.",
        ));
    }
    Ok(c)
}

fn motivo_requerido(texto: &str) -> Result<&str, AppError> {
    let texto = texto.trim();
    if !(5..=250).contains(&texto.chars().count()) {
        return Err(invalido("Escribe un motivo de 5 a 250 caracteres."));
    }
    Ok(texto)
}

fn validar_disponible(conn: &Connection, c: &CajaChicaEntity, monto: i64) -> Result<(), AppError> {
    let disponible = resumen(conn, c)?.efectivo_esperado_cent;
    if monto > disponible {
        return Err(AppError::Validation(format!(
            "Efectivo insuficiente. Disponible: S/ {}.{:02}.",
            disponible / 100,
            disponible % 100
        )));
    }
    Ok(())
}

impl CajaService {
    pub fn abrir_caja(
        conn: &mut Connection,
        actor: i64,
        req: AbrirCajaDto,
    ) -> Result<Value, AppError> {
        escritura(
            conn,
            actor,
            &req.id_operacion,
            "CAJA_ABRIR",
            &req,
            |tx, _| {
                exigir_operativa(tx)?;
                if !(0..=LIMITE_FONDO).contains(&req.monto_apertura_cent) {
                    return Err(invalido(
                        "El fondo inicial debe estar entre S/ 0,00 y S/ 10 000,00.",
                    ));
                }
                if abierto(tx)?.is_some() {
                    return Err(invalido("Caja ocupada. Ya existe un turno abierto."));
                }
                let (ambiente, _) = configuracion(tx)?;
                let codigo = format!(
                    "CAJA-{}",
                    uuid::Uuid::new_v4().simple().to_string().to_uppercase()
                );
                tx.execute("INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent) VALUES(?1,?2,?3,?4,?5)",
                params![req.id_operacion,codigo,actor,ambiente,req.monto_apertura_cent])?;
                let id = tx.last_insert_rowid();
                let r = json!({"id_caja":id,"codigo_caja":codigo,"monto_apertura_cent":req.monto_apertura_cent});
                auditar(
                    tx,
                    actor,
                    &req.id_operacion,
                    "ABRIR",
                    ("caja_chica", id),
                    (None, r.clone()),
                    None,
                )?;
                Ok(r)
            },
        )
    }

    pub fn manual(
        conn: &mut Connection,
        actor: i64,
        req: ManualCajaDto,
    ) -> Result<Value, AppError> {
        escritura(
            conn,
            actor,
            &req.id_operacion,
            "CAJA_MANUAL",
            &req,
            |tx, _| {
                let c = caja_manual(tx, req.id_caja, actor)?;
                let signo = match (req.tipo.as_str(), req.categoria.as_str()) {
                    ("INGRESO_MANUAL", "APORTE_FONDO" | "OTRO") => 1,
                    ("EGRESO_MANUAL", "GASTO_MENOR" | "RETIRO_PARCIAL" | "OTRO") => -1,
                    _ => return Err(invalido("La categoría no corresponde al movimiento.")),
                };
                let concepto = req.concepto.trim();
                if !(3..=120).contains(&concepto.chars().count())
                    || !(1..=LIMITE_FONDO).contains(&req.monto_cent)
                {
                    return Err(invalido("Usa un concepto de 3 a 120 caracteres y un importe mayor que cero, hasta S/ 10 000,00."));
                }
                if signo < 0 {
                    validar_disponible(tx, &c, req.monto_cent)?;
                }
                tx.execute("INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto)
             VALUES(?1,?2,?3,?4,'EFECTIVO',?5,?6,?7,?8)",params![req.id_operacion,c.id_caja,actor,req.tipo,req.monto_cent,signo,req.categoria,concepto])?;
                let id = tx.last_insert_rowid();
                let r = json!({"id_movimiento":id,"id_caja":c.id_caja,"tipo":req.tipo,"monto_cent":req.monto_cent,"signo":signo,"categoria":req.categoria,"concepto":concepto});
                auditar(
                    tx,
                    actor,
                    &req.id_operacion,
                    "MOVIMIENTO_MANUAL",
                    ("movimientos_caja", id),
                    (None, r.clone()),
                    None,
                )?;
                Ok(r)
            },
        )
    }

    pub fn revertir(
        conn: &mut Connection,
        actor: i64,
        req: RevertirCajaDto,
    ) -> Result<Value, AppError> {
        escritura(
            conn,
            actor,
            &req.id_operacion,
            "CAJA_REVERSO",
            &req,
            |tx, _| {
                exigir_operativa(tx)?;
                let original = tx.query_row(
                "SELECT id_caja, tipo, monto_cent, signo FROM movimientos_caja WHERE id_movimiento=?1",
                [req.id_movimiento], |r| Ok((r.get::<_,i64>(0)?,r.get::<_,String>(1)?,r.get::<_,i64>(2)?,r.get::<_,i64>(3)?)),
            ).optional()?.ok_or_else(|| AppError::NotFound("Movimiento no encontrado.".into()))?;
                let (id_caja, tipo, monto, signo) = original;
                let c = caja_manual(tx, id_caja, actor)?;
                if !["INGRESO_MANUAL", "EGRESO_MANUAL"].contains(&tipo.as_str()) {
                    return Err(invalido(
                        "Solo se pueden revertir movimientos manuales originales.",
                    ));
                }
                let motivo = motivo_requerido(&req.motivo)?;
                let existe: bool = tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM movimientos_caja WHERE id_movimiento_original=?1)",
                    [req.id_movimiento],
                    |r| r.get(0),
                )?;
                if existe {
                    return Err(invalido("Este movimiento ya tiene una reversión."));
                }
                if signo > 0 {
                    validar_disponible(tx, &c, monto)?;
                }
                tx.execute("INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,id_movimiento_original,concepto,motivo)
             VALUES(?1,?2,?3,'REVERSO_MANUAL','EFECTIVO',?4,?5,?6,?7,?8)",
                params![req.id_operacion,id_caja,actor,monto,-signo,req.id_movimiento,format!("Reversión de movimiento #{}",req.id_movimiento),motivo])?;
                let id = tx.last_insert_rowid();
                let r = json!({"id_movimiento":id,"id_caja":id_caja,"id_movimiento_original":req.id_movimiento,"monto_cent":monto,"signo":-signo});
                auditar(
                    tx,
                    actor,
                    &req.id_operacion,
                    "REVERSO_MANUAL",
                    ("movimientos_caja", id),
                    (None, r.clone()),
                    Some(motivo),
                )?;
                Ok(r)
            },
        )
    }

    pub fn cerrar_caja(
        conn: &mut Connection,
        actor: i64,
        req: CierreCajaDto,
    ) -> Result<Value, AppError> {
        escritura(
            conn,
            actor,
            &req.id_operacion,
            "CAJA_CERRAR",
            &req,
            |tx, admin| {
                let c = turno(tx, req.id_caja)?;
                let recuperacion = configuracion(tx)?.1;
                if !admin && c.id_usuario != actor {
                    return Err(AppError::PermisoDenegado);
                }
                if c.estado != "ABIERTA" {
                    return Err(invalido("El turno ya está cerrado."));
                }
                let modo =
                    modo_cierre(&c, actor, admin, recuperacion).ok_or(AppError::PermisoDenegado)?;
                if c.version != req.version_esperada {
                    return Err(invalido("RESUMEN_DESACTUALIZADO: Se registraron movimientos durante el conteo. Revisa el resumen actualizado."));
                }
                if !(0..=LIMITE_CONTEO).contains(&req.efectivo_contado_cent)
                    || !(0..=req.efectivo_contado_cent.min(LIMITE_FONDO))
                        .contains(&req.fondo_siguiente_cent)
                {
                    return Err(invalido("Revisa el efectivo contado y el fondo siguiente. El fondo no puede superar el contado ni S/ 10 000,00."));
                }
                let r = resumen(tx, &c)?;
                let diferencia = req.efectivo_contado_cent - r.efectivo_esperado_cent;
                let texto = req.motivo.as_deref().unwrap_or("").trim();
                let motivo = if modo != "NORMAL" || diferencia != 0 || !texto.is_empty() {
                    Some(motivo_requerido(texto)?)
                } else {
                    None
                };
                let retiro = req.efectivo_contado_cent - req.fondo_siguiente_cent;
                let n=tx.execute("UPDATE caja_chica SET estado='CERRADA',fecha_cierre=datetime('now','-5 hours'),
              id_operacion_cierre=?1,id_usuario_cierre=?2,tipo_cierre=?3,motivo_cierre=?4,
              efectivo_esperado_cent=?5,efectivo_contado_cent=?6,tarjeta_neto_cent=?7,yape_neto_cent=?8,
              plin_neto_cent=?9,transferencia_neto_cent=?10,fondo_siguiente_cent=?11,retiro_cierre_cent=?12,version=version+1
              WHERE id_caja=?13 AND estado='ABIERTA' AND version=?14",
              params![req.id_operacion,actor,modo,motivo,r.efectivo_esperado_cent,req.efectivo_contado_cent,
                      r.tarjeta.neto_cent,r.yape.neto_cent,r.plin.neto_cent,r.transferencia.neto_cent,req.fondo_siguiente_cent,retiro,c.id_caja,req.version_esperada])?;
                if n != 1 {
                    return Err(invalido(
                        "RESUMEN_DESACTUALIZADO: Revisa el resumen actualizado.",
                    ));
                }
                let resultado = json!({"turno":turno(tx,c.id_caja)?,"resumen":r,"puede_mover":false,"modo_cierre":null,"puede_aclarar":admin&&!recuperacion});
                auditar(
                    tx,
                    actor,
                    &req.id_operacion,
                    "CERRAR",
                    ("caja_chica", c.id_caja),
                    (None, resultado.clone()),
                    motivo,
                )?;
                Ok(resultado)
            },
        )
    }

    pub fn aclarar(
        conn: &mut Connection,
        actor: i64,
        req: AclararCajaDto,
    ) -> Result<Value, AppError> {
        escritura(
            conn,
            actor,
            &req.id_operacion,
            "CAJA_ACLARAR",
            &req,
            |tx, _| {
                exigir_operativa(tx)?;
                let c = turno(tx, req.id_caja)?;
                if c.estado != "CERRADA" {
                    return Err(invalido("Solo se puede aclarar un cierre."));
                }
                let texto = req.aclaracion.trim();
                if texto.chars().count() > 500 {
                    return Err(invalido("La aclaración admite hasta 500 caracteres."));
                }
                let nuevo = (!texto.is_empty()).then_some(texto);
                if c.aclaracion.as_deref() != nuevo {
                    tx.execute("UPDATE caja_chica SET aclaracion=?1,id_usuario_aclaracion=?2,fecha_aclaracion=datetime('now','-5 hours') WHERE id_caja=?3",
                    params![nuevo,actor,c.id_caja])?;
                    auditar(
                        tx,
                        actor,
                        &req.id_operacion,
                        "ACLARAR",
                        ("caja_chica", c.id_caja),
                        (
                            Some(
                                json!({"aclaracion":c.aclaracion,"id_usuario_aclaracion":c.id_usuario_aclaracion,"fecha_aclaracion":c.fecha_aclaracion}),
                            ),
                            json!({"aclaracion":nuevo,"id_usuario_aclaracion":actor}),
                        ),
                        None,
                    )?;
                }
                CajaService::detalle(tx, actor, c.id_caja)
            },
        )
    }
}

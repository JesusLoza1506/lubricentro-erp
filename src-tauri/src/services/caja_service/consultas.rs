use super::{configuracion, es_administrador, invalido, modo_cierre, CajaService};
use crate::{
    dto::caja_dto::{FiltroHistorialCaja, FiltroMovimientosCaja, ResumenCaja, ResumenMedioCaja},
    entities::caja_chica::CajaChicaEntity,
    errors::AppError,
    services::pos_service::consultas::filas,
};
use chrono::NaiveDate;
use rusqlite::{named_params, params, Connection, OptionalExtension};
use serde_json::{json, Value};

const TURNO_SQL: &str = "SELECT c.*, u.nombre_completo AS titular, uc.nombre_completo AS quien_cerro,
 ua.nombre_completo AS autor_aclaracion FROM caja_chica c JOIN usuarios u ON u.id_usuario=c.id_usuario
 LEFT JOIN usuarios uc ON uc.id_usuario=c.id_usuario_cierre
 LEFT JOIN usuarios ua ON ua.id_usuario=c.id_usuario_aclaracion";
const PAGINA: i64 = 20;

pub(super) fn abierto(conn: &Connection) -> Result<Option<CajaChicaEntity>, AppError> {
    Ok(conn
        .query_row(
            &format!("{TURNO_SQL} WHERE c.estado='ABIERTA'"),
            [],
            CajaChicaEntity::desde_fila,
        )
        .optional()?)
}

pub(super) fn turno(conn: &Connection, id: i64) -> Result<CajaChicaEntity, AppError> {
    conn.query_row(
        &format!("{TURNO_SQL} WHERE c.id_caja=?1"),
        [id],
        CajaChicaEntity::desde_fila,
    )
    .optional()?
    .ok_or_else(|| AppError::NotFound("Turno no encontrado.".into()))
}

fn autorizado(
    c: &CajaChicaEntity,
    actor: i64,
    admin: bool,
    recuperacion: bool,
) -> Result<(), AppError> {
    if (!admin && (c.id_usuario != actor || c.por_conciliar || recuperacion))
        || (recuperacion && (!c.por_conciliar || c.estado != "ABIERTA"))
    {
        return Err(AppError::PermisoDenegado);
    }
    Ok(())
}

pub(super) fn resumen(conn: &Connection, c: &CajaChicaEntity) -> Result<ResumenCaja, AppError> {
    let mut r = ResumenCaja::default();
    let mut stmt = conn.prepare(
        "SELECT medio_pago,
            COALESCE(SUM(CASE WHEN tipo='COBRO' THEN monto_cent ELSE 0 END),0),
            COALESCE(SUM(CASE WHEN tipo='REINTEGRO' THEN monto_cent ELSE 0 END),0),
            COALESCE(SUM(CASE WHEN tipo IN ('INGRESO_MANUAL','EGRESO_MANUAL','REVERSO_MANUAL') AND signo=1 THEN monto_cent ELSE 0 END),0),
            COALESCE(SUM(CASE WHEN tipo IN ('INGRESO_MANUAL','EGRESO_MANUAL','REVERSO_MANUAL') AND signo=-1 THEN monto_cent ELSE 0 END),0)
         FROM movimientos_caja WHERE id_caja=?1 GROUP BY medio_pago",
    )?;
    let mut filas = stmt.query([c.id_caja])?;
    while let Some(f) = filas.next()? {
        let medio: String = f.get(0)?;
        let cobros_cent: i64 = f.get(1)?;
        let reintegros_cent: i64 = f.get(2)?;
        let (destino, guardado) = match medio.as_str() {
            "EFECTIVO" => {
                r.ingresos_manuales_cent = f.get(3)?;
                r.egresos_manuales_cent = f.get(4)?;
                (&mut r.efectivo, None)
            }
            "TARJETA" => (&mut r.tarjeta, c.tarjeta_neto_cent),
            "YAPE" => (&mut r.yape, c.yape_neto_cent),
            "PLIN" => (&mut r.plin, c.plin_neto_cent),
            "TRANSFERENCIA" => (&mut r.transferencia, c.transferencia_neto_cent),
            _ => continue,
        };
        *destino = ResumenMedioCaja {
            cobros_cent,
            reintegros_cent,
            neto_cent: guardado.unwrap_or(cobros_cent - reintegros_cent),
        };
    }
    r.efectivo_esperado_cent = c.efectivo_esperado_cent.unwrap_or(
        c.monto_apertura_cent + r.efectivo.neto_cent + r.ingresos_manuales_cent
            - r.egresos_manuales_cent,
    );
    Ok(r)
}

fn detalle_turno(
    conn: &Connection,
    c: &CajaChicaEntity,
    actor: i64,
    admin: bool,
    recuperacion: bool,
) -> Result<Value, AppError> {
    Ok(json!({
        "turno": c,
        "resumen": resumen(conn,c)?,
        "puede_mover": c.estado == "ABIERTA" && c.id_usuario == actor && !c.por_conciliar && !recuperacion,
        "modo_cierre": modo_cierre(c,actor,admin,recuperacion),
        "puede_aclarar": c.estado == "CERRADA" && admin && !recuperacion,
    }))
}

impl CajaService {
    pub fn inicio(conn: &Connection, actor: i64) -> Result<Value, AppError> {
        let admin = es_administrador(conn, actor)?;
        let (ambiente, recuperacion) = configuracion(conn)?;
        let hoy: String = conn.query_row("SELECT date('now','-5 hours')", [], |r| r.get(0))?;
        let activo = abierto(conn)?;
        let ocupada = activo.is_some();
        let puede_abrir = !ocupada && !recuperacion;
        let fondo_sugerido = if puede_abrir {
            conn.query_row(
                "SELECT fondo_siguiente_cent FROM caja_chica WHERE estado='CERRADA' AND ambiente=?1 ORDER BY fecha_cierre DESC,id_caja DESC LIMIT 1",
                [&ambiente], |r| r.get::<_,i64>(0),
            ).optional()?.unwrap_or(0)
        } else {
            0
        };
        let detalle = activo
            .as_ref()
            .filter(|c| autorizado(c, actor, admin, recuperacion).is_ok())
            .map(|c| detalle_turno(conn, c, actor, admin, recuperacion))
            .transpose()?;
        let responsables = if admin && !recuperacion {
            filas(conn, "SELECT json_object('id',id_usuario,'nombre',nombre_completo) FROM usuarios WHERE rol IN ('CAJERO','ADMINISTRADOR') OR EXISTS(SELECT 1 FROM caja_chica c WHERE c.id_usuario=usuarios.id_usuario) ORDER BY nombre_completo,id_usuario", &[])?
        } else {
            Vec::new()
        };
        Ok(
            json!({"ambiente":ambiente,"hoy":hoy,"ocupada":ocupada,"puede_abrir":puede_abrir,
          "recuperacion":recuperacion,"fondo_sugerido_cent":fondo_sugerido,"detalle":detalle,"responsables":responsables}),
        )
    }

    pub fn detalle(conn: &Connection, actor: i64, id: i64) -> Result<Value, AppError> {
        let admin = es_administrador(conn, actor)?;
        let (_, recuperacion) = configuracion(conn)?;
        let c = turno(conn, id)?;
        autorizado(&c, actor, admin, recuperacion)?;
        detalle_turno(conn, &c, actor, admin, recuperacion)
    }

    pub fn movimientos(
        conn: &Connection,
        actor: i64,
        f: FiltroMovimientosCaja,
    ) -> Result<Value, AppError> {
        let admin = es_administrador(conn, actor)?;
        let (_, recuperacion) = configuracion(conn)?;
        let c = turno(conn, f.id_caja)?;
        autorizado(&c, actor, admin, recuperacion)?;
        let origen = f.origen.as_deref().unwrap_or("");
        let direccion = f.direccion.as_deref().unwrap_or("");
        if !["", "POS", "MANUAL"].contains(&origen)
            || !["", "INGRESO", "EGRESO"].contains(&direccion)
        {
            return Err(invalido("Filtros de movimientos inválidos."));
        }
        let buscar = f.buscar.as_deref().unwrap_or("").trim();
        let (pagina, offset) = paginar(f.pagina);
        let cond = " FROM movimientos_caja m
            LEFT JOIN devoluciones d ON d.id_devolucion=m.id_devolucion
            LEFT JOIN ventas v ON v.id_venta=COALESCE(m.id_venta,d.id_venta)
            WHERE m.id_caja=?1
            AND (?2='' OR (?2='POS')=(m.tipo IN ('COBRO','REINTEGRO')))
            AND (?3='' OR (?3='INGRESO')=(m.signo=1))
            AND (?4='' OR '#'||m.id_movimiento=?4 OR instr(lower(m.concepto),lower(?4))>0 OR instr(lower(coalesce(m.referencia,'')),lower(?4))>0
                 OR instr(lower(CASE WHEN m.id_devolucion IS NOT NULL THEN 'D-'||m.id_devolucion WHEN v.id_venta IS NOT NULL THEN 'V-'||v.id_venta ELSE '' END),lower(?4))>0)";
        let total: i64 = conn.query_row(
            &format!("SELECT count(*) {cond}"),
            params![f.id_caja, origen, direccion, buscar],
            |r| r.get(0),
        )?;
        let sql = format!("SELECT json_object('id_movimiento',m.id_movimiento,'tipo',m.tipo,'medio_pago',m.medio_pago,
            'monto_cent',m.monto_cent,'signo',m.signo,'fecha',m.fecha_operacion,'concepto',m.concepto,
            'categoria',m.categoria,'referencia',m.referencia,'motivo',m.motivo,'id_venta',v.id_venta,
            'id_devolucion',m.id_devolucion,'id_movimiento_original',m.id_movimiento_original,
            'id_reverso',(SELECT x.id_movimiento FROM movimientos_caja x WHERE x.id_movimiento_original=m.id_movimiento),
            'ejecutor',(SELECT nombre_completo FROM usuarios WHERE id_usuario=m.id_usuario),
            'autorizante',(SELECT nombre_completo FROM usuarios WHERE id_usuario=CASE WHEN m.tipo='REINTEGRO' THEN d.id_autorizante ELSE v.id_autorizante END))
            {cond} ORDER BY m.fecha_operacion DESC,m.id_movimiento DESC LIMIT {PAGINA} OFFSET ?5");
        let datos = filas(
            conn,
            &sql,
            &[&f.id_caja, &origen, &direccion, &buscar, &offset],
        )?;
        Ok(json!({"datos":datos,"total":total,"pagina":pagina,"por_pagina":PAGINA}))
    }

    pub fn historial(
        conn: &Connection,
        actor: i64,
        f: FiltroHistorialCaja,
    ) -> Result<Value, AppError> {
        let admin = es_administrador(conn, actor)?;
        let (_, recuperacion) = configuracion(conn)?;
        if recuperacion {
            return Err(AppError::PermisoDenegado);
        }
        let hoy: String = conn.query_row("SELECT date('now','-5 hours')", [], |r| r.get(0))?;
        let desde = f.desde.as_deref().unwrap_or(&hoy);
        let hasta = f.hasta.as_deref().unwrap_or(&hoy);
        let fecha = |s: &str| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .ok()
                .filter(|d| d.format("%Y-%m-%d").to_string() == s)
        };
        let dias = fecha(hasta)
            .zip(fecha(desde))
            .map(|(h, d)| (h - d).num_days());
        if !dias.is_some_and(|n| (0..=365).contains(&n)) {
            return Err(invalido(
                "El rango debe ser válido y abarcar hasta 366 días.",
            ));
        }
        let estado = f.estado.as_deref().unwrap_or("CERRADA");
        if !["ABIERTA", "CERRADA", "TODAS"].contains(&estado) {
            return Err(invalido("Estado de turno inválido."));
        }
        if !admin && (estado != "CERRADA" || f.id_usuario.is_some_and(|id| id != actor)) {
            return Err(AppError::PermisoDenegado);
        }
        let usuario = if admin { f.id_usuario } else { Some(actor) };
        let codigo = f.codigo.as_deref().unwrap_or("").trim();
        let diferencia = f.con_diferencia.unwrap_or(false);
        let (pagina, offset) = paginar(f.pagina);
        let where_sql = " WHERE (:usuario IS NULL OR c.id_usuario=:usuario) AND (:estado='TODAS' OR c.estado=:estado)
            AND ((c.estado='CERRADA' AND c.fecha_cierre>=:desde AND c.fecha_cierre<date(:hasta,'+1 day'))
             OR (c.estado='ABIERTA' AND c.fecha_apertura>=:desde AND c.fecha_apertura<date(:hasta,'+1 day')))
            AND (:codigo='' OR instr(upper(c.codigo_caja),upper(:codigo))>0)
            AND (NOT :diferencia OR c.diferencia_efectivo_cent<>0)";
        let total: i64 = conn.query_row(&format!("SELECT count(*) FROM caja_chica c {where_sql}"),
            named_params!{":usuario":usuario,":estado":estado,":desde":desde,":hasta":hasta,":codigo":codigo,":diferencia":diferencia}, |r|r.get(0))?;
        let mut stmt = conn.prepare(&format!("{TURNO_SQL} {where_sql} ORDER BY COALESCE(c.fecha_cierre,c.fecha_apertura) DESC,c.id_caja DESC LIMIT {PAGINA} OFFSET :offset"))?;
        let datos = stmt.query_map(named_params!{":usuario":usuario,":estado":estado,":desde":desde,":hasta":hasta,":codigo":codigo,":diferencia":diferencia,":offset":offset}, CajaChicaEntity::desde_fila)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(json!({"datos":datos,"total":total,"pagina":pagina,"por_pagina":PAGINA}))
    }
}

fn paginar(p: Option<i64>) -> (i64, i64) {
    let p = p.unwrap_or(1).clamp(1, 1_000_000);
    (p, (p - 1) * PAGINA)
}

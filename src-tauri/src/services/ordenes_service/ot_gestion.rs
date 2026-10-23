use super::helpers::{
    generar_codigo_ot, recalcular_kilometraje_vehiculo, tiene_cambio_aceite,
    validar_mecanico_activo, DatosVehiculoCliente,
};
use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{
    CancelarOtDto, CrearOrdenTrabajoDto, FinalizarOtDto, IniciarTrabajoOtDto,
    OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};
use uuid::Uuid;

pub fn crear_orden_trabajo(
    conn: &mut Connection,
    req: CrearOrdenTrabajoDto,
    id_creador: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    if req.servicios_iniciales.is_empty() {
        return Err(AppError::Validation(
            "La Orden de Trabajo debe incluir al menos un servicio inicial.".to_string(),
        ));
    }

    let id_mecanico_final = if let Some(id_m) = req.id_mecanico {
        if id_m > 0 {
            validar_mecanico_activo(conn, id_m)?;
            Some(id_m)
        } else {
            None
        }
    } else {
        None
    };

    let datos_vc = conn
        .query_row(
            "SELECT v.id_vehiculo, v.id_cliente, COALESCE(v.kilometraje_actual, 0),
                    c.nombre_razon_social, c.numero_documento, c.telefono, c.direccion, c.correo, c.tipo_documento, v.placa
             FROM vehiculos v
             JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.id_vehiculo = ?1 AND v.activo = 1",
            [req.id_vehiculo],
            |row| {
                Ok((
                    DatosVehiculoCliente {
                        id_vehiculo: row.get(0)?,
                        id_cliente_atendido: row.get(1)?,
                        km_actual_ficha: row.get(2)?,
                        nombre_cliente: row.get(3)?,
                        doc_cliente: row.get(4)?,
                        tel_cliente: row.get(5)?,
                        dir_cliente: row.get(6)?,
                        mail_cliente: row.get(7)?,
                        tipo_doc_cliente: row.get(8)?,
                    },
                    row.get::<_, String>(9)?,
                ))
            },
        )
        .map_err(|_| AppError::NotFound("El vehículo especificado no existe o está inactivo.".to_string()))?;

    let (datos, placa) = datos_vc;

    if req.kilometraje_ingreso < datos.km_actual_ficha {
        return Err(AppError::Validation(format!(
            "El kilometraje de ingreso ({} km) no puede ser menor al registrado en la Ficha ({} km).",
            req.kilometraje_ingreso, datos.km_actual_ficha
        )));
    }

    let ot_abierta: i32 = conn.query_row(
        "SELECT COUNT(*) FROM ordenes_trabajo WHERE id_vehiculo = ?1 AND estado IN ('EN_ESPERA', 'EN_PROCESO')",
        [req.id_vehiculo],
        |row| row.get(0),
    )?;

    if ot_abierta > 0 {
        return Err(AppError::Validation(
            "El vehículo ya cuenta con una Orden de Trabajo activa.".to_string(),
        ));
    }

    let codigo_ot = generar_codigo_ot(conn)?;
    let id_operacion = Uuid::new_v4().to_string();

    let cliente_snapshot = serde_json::json!({
        "nombre": datos.nombre_cliente,
        "documento": datos.doc_cliente,
        "tipo_documento": datos.tipo_doc_cliente,
        "telefono": datos.tel_cliente,
        "direccion": datos.dir_cliente,
        "correo": datos.mail_cliente
    })
    .to_string();

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tx.execute(
        "INSERT INTO ordenes_trabajo (
            id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot,
            id_creador, id_mecanico, estado, kilometraje_ingreso, pedido_cliente, fecha_ingreso
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'EN_ESPERA', ?9, ?10, datetime('now', '-5 hours'))",
        params![
            id_operacion,
            codigo_ot,
            datos.id_vehiculo,
            datos.id_cliente_atendido,
            placa,
            cliente_snapshot,
            id_creador,
            id_mecanico_final,
            req.kilometraje_ingreso,
            req.pedido_cliente
        ],
    )?;

    let id_ot = tx.last_insert_rowid();

    for serv in req.servicios_iniciales {
        let (precio_cent, nombre, codigo, es_cambio): (i64, String, String, i32) = tx
            .query_row(
                "SELECT precio_venta_cent, nombre, codigo, es_cambio_aceite FROM servicios WHERE id_servicio = ?1 AND activo = 1",
                [serv.id_servicio],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .map_err(|_| AppError::NotFound(format!("Servicio ID {} no encontrado.", serv.id_servicio)))?;

        let id_op_serv = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO detalle_ot_servicios (
                id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot, cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id_op_serv,
                id_ot,
                serv.id_servicio,
                codigo,
                nombre,
                serv.cantidad,
                precio_cent,
                es_cambio,
                id_creador
            ],
        )?;
    }

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, id_ot, id_creador, rol)
}

pub fn iniciar_trabajo_ot(
    conn: &mut Connection,
    req: IniciarTrabajoOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;

    if ot.estado != "EN_ESPERA" {
        return Err(AppError::Validation(
            "Solo se pueden iniciar trabajos en órdenes que estén 'EN_ESPERA'.".to_string(),
        ));
    }

    if !(1..=6).contains(&req.id_zanja_ejecucion) {
        return Err(AppError::Validation(
            "La zanja debe ser un número entre 1 y 6.".to_string(),
        ));
    }

    let ocupada: i32 = conn.query_row(
        "SELECT COUNT(*) FROM ordenes_trabajo WHERE id_zanja_ejecucion = ?1 AND estado = 'EN_PROCESO'",
        [req.id_zanja_ejecucion],
        |row| row.get(0),
    )?;

    if ocupada > 0 {
        return Err(AppError::Validation(format!(
            "La zanja {} ya se encuentra ocupada por otra OT en proceso.",
            req.id_zanja_ejecucion
        )));
    }

    let id_mecanico_final = if let Some(id_m) = ot.id_mecanico {
        if rol == "MECANICO" && id_m != id_usuario {
            return Err(AppError::Validation(
                "No tienes permiso para iniciar una OT asignada a otro mecánico.".to_string(),
            ));
        }
        id_m
    } else {
        id_usuario
    };

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'EN_PROCESO', id_zanja_ejecucion = ?1, id_mecanico = ?2, fecha_inicio = COALESCE(fecha_inicio, datetime('now', '-5 hours'))
         WHERE id_ot = ?3",
        params![req.id_zanja_ejecucion, id_mecanico_final, req.id_ot],
    )?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn reasignar_mecanico_ot(
    conn: &mut Connection,
    req: ReasignarMecanicoOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    if rol.to_uppercase() != "ADMINISTRADOR" {
        return Err(AppError::Validation(
            "Solo un administrador puede reasignar mecánicos a una orden de trabajo.".to_string(),
        ));
    }

    let motivo_clean = req.motivo.trim();
    if motivo_clean.len() < 5 || motivo_clean.len() > 250 {
        return Err(AppError::Validation(
            "El motivo de reasignación debe tener entre 5 y 250 caracteres.".to_string(),
        ));
    }

    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se puede reasignar mecánico en una OT finalizada o cancelada.".to_string(),
        ));
    }

    validar_mecanico_activo(conn, req.nuevo_id_mecanico)?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tx.execute(
        "UPDATE ordenes_trabajo SET id_mecanico = ?1 WHERE id_ot = ?2",
        params![req.nuevo_id_mecanico, req.id_ot],
    )?;

    let (username, user_rol): (String, String) = tx.query_row(
        "SELECT username, rol FROM usuarios WHERE id_usuario = ?1",
        [id_usuario],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;

    AuditoriaService::registrar_v5(
        &tx,
        Some(id_usuario),
        "USUARIO",
        Some(&username),
        Some(&user_rol),
        None,
        "OT",
        "REASIGNAR_MECANICO",
        Some("ordenes_trabajo"),
        Some(&req.id_ot.to_string()),
        None,
        None,
        Some(motivo_clean),
        "OK",
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn finalizar_ot(
    conn: &mut Connection,
    req: FinalizarOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;

    if ot.estado != "EN_PROCESO" {
        return Err(AppError::Validation(
            "Solo se pueden finalizar órdenes de trabajo que estén 'EN_PROCESO'.".to_string(),
        ));
    }

    if req.kilometraje_salida < ot.kilometraje_ingreso {
        return Err(AppError::Validation(format!(
            "El kilometraje de salida ({} km) no puede ser menor al de ingreso ({} km).",
            req.kilometraje_salida, ot.kilometraje_ingreso
        )));
    }

    let requiere_aceite = tiene_cambio_aceite(conn, req.id_ot);
    let mut tipo_aceite_final: Option<String> = None;
    let mut intervalo_km: Option<i32> = None;

    if requiere_aceite {
        let t_aceite = req
            .tipo_aceite
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_uppercase();

        if t_aceite != "MINERAL" && t_aceite != "SINTETICO" {
            return Err(AppError::Validation(
                "La OT incluye cambio de aceite. Debe especificar el tipo ('MINERAL' o 'SINTETICO').".to_string(),
            ));
        }

        intervalo_km = Some(if t_aceite == "SINTETICO" { 10000 } else { 5000 });
        tipo_aceite_final = Some(t_aceite);
    }

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Sin modificar id_zanja_ejecucion ni fecha_inicio para respetar trg_ot_transicion del DDL V7.0
    tx.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'FINALIZADO', fecha_fin = datetime('now', '-5 hours'),
             kilometraje_salida = ?1, id_mecanico_terminal = COALESCE(id_mecanico, ?2),
             resumen_publico = ?3, tipo_aceite = ?4, intervalo_aceite_km = ?5
         WHERE id_ot = ?6",
        params![
            req.kilometraje_salida,
            id_usuario,
            req.resumen_publico.trim(),
            tipo_aceite_final,
            intervalo_km,
            req.id_ot
        ],
    )?;

    tx.execute(
        "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE id_vehiculo = ?2",
        params![req.kilometraje_salida, ot.id_vehiculo],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn cancelar_ot(
    conn: &mut Connection,
    req: CancelarOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let motivo_clean = req.motivo.trim();
    if motivo_clean.len() < 5 || motivo_clean.len() > 250 {
        return Err(AppError::Validation(
            "El motivo de cancelación debe tener entre 5 y 250 caracteres.".to_string(),
        ));
    }

    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se puede cancelar una orden ya finalizada o cancelada.".to_string(),
        ));
    }

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Mantiene id_zanja_ejecucion para evitar el fallo en trg_ot_transicion de DDL V7.0
    tx.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'CANCELADO', fecha_cancelacion = datetime('now', '-5 hours'),
             id_usuario_cancelacion = ?1, motivo_cancelacion = ?2
         WHERE id_ot = ?3",
        params![id_usuario, motivo_clean, req.id_ot],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    recalcular_kilometraje_vehiculo(conn, &ot.placa)?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

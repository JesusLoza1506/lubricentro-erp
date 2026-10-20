use super::helpers::validar_permiso_operacion_ot;
use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{
    AgregarServicioOtDto, OrdenTrabajoHistorialDto, ServicioResumenDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection};
use uuid::Uuid;

pub fn agregar_servicio_ot(
    conn: &Connection,
    req: AgregarServicioOtDto,
    id_usuario_ejecutor: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    validar_permiso_operacion_ot(conn, req.id_ot, id_usuario_ejecutor)?;

    let ot = obtener_ot_por_id(conn, req.id_ot)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se pueden agregar servicios a una orden finalizada o cancelada.".to_string(),
        ));
    }

    let (precio_venta_cent, nombre_serv, codigo_serv, es_cambio): (i64, String, String, i32) = conn
        .query_row(
            "SELECT precio_venta_cent, nombre, codigo, es_cambio_aceite FROM servicios WHERE id_servicio = ?1 AND activo = 1",
            [req.id_servicio],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| AppError::NotFound("Servicio no encontrado o inactivo.".to_string()))?;

    let id_op_serv = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO detalle_ot_servicios (
            id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot, cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario
        ) VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7, ?8)",
        params![
            id_op_serv,
            req.id_ot,
            req.id_servicio,
            codigo_serv,
            nombre_serv,
            precio_venta_cent,
            es_cambio,
            id_usuario_ejecutor
        ],
    )?;

    obtener_ot_por_id(conn, req.id_ot)
}

pub fn eliminar_servicio_ot(
    conn: &Connection,
    id_detalle: i64,
    id_ot: i64,
    id_usuario_ejecutor: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    validar_permiso_operacion_ot(conn, id_ot, id_usuario_ejecutor)?;

    let ot = obtener_ot_por_id(conn, id_ot)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se pueden eliminar servicios de una orden finalizada o cancelada.".to_string(),
        ));
    }

    let cantidad_servicios: i64 = conn.query_row(
        "SELECT COUNT(*) FROM detalle_ot_servicios WHERE id_ot = ?1",
        [id_ot],
        |row| row.get(0),
    )?;

    if cantidad_servicios <= 1 {
        return Err(AppError::Validation(
            "No se puede eliminar el servicio. Una Orden de Trabajo debe conservar al menos un servicio.".to_string(),
        ));
    }

    conn.execute(
        "DELETE FROM detalle_ot_servicios WHERE id_detalle = ?1 AND id_ot = ?2",
        params![id_detalle, id_ot],
    )?;

    obtener_ot_por_id(conn, id_ot)
}

pub fn listar_servicios(conn: &Connection) -> Result<Vec<ServicioResumenDto>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id_servicio, nombre, (precio_venta_cent / 100.0) AS precio_base
         FROM servicios
         WHERE activo = 1
         ORDER BY nombre ASC",
    )?;

    let iter = stmt.query_map([], |row| {
        Ok(ServicioResumenDto {
            id_servicio: row.get(0)?,
            descripcion: row.get(1)?,
            precio_base: row.get(2)?,
        })
    })?;

    let mut lista = Vec::new();
    for item in iter {
        lista.push(item?);
    }

    Ok(lista)
}

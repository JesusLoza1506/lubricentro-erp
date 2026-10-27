use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{
    AgregarServicioOtDto, ModificarServicioOtDto, OrdenTrabajoHistorialDto, ServicioResumenDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

/// Helper interno para validar la propiedad de la OT si el actor es MECANICO
fn validar_propiedad_mecanico(
    ot: &OrdenTrabajoHistorialDto,
    id_usuario: i64,
    rol: &str,
    accion: &str,
) -> Result<(), AppError> {
    if rol.eq_ignore_ascii_case("MECANICO") {
        match ot.id_mecanico {
            Some(id_mec) if id_mec == id_usuario => Ok(()),
            _ => Err(AppError::Validation(format!(
                "No tienes permiso para {} en una orden de trabajo libre o asignada a otro mecánico.",
                accion
            ))),
        }
    } else {
        Ok(())
    }
}

pub fn agregar_servicio_ot(
    conn: &mut Connection,
    req: AgregarServicioOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;

    if ot.estado != "EN_ESPERA" && ot.estado != "EN_PROCESO" {
        return Err(AppError::Validation(
            "Solo se pueden agregar servicios a órdenes en espera o en proceso.".to_string(),
        ));
    }

    // Un mecánico solo puede agregar servicios si la OT le pertenece efectivamente
    validar_propiedad_mecanico(&ot, id_usuario, rol, "agregar servicios")?;

    if req.cantidad <= 0 {
        return Err(AppError::Validation(
            "La cantidad debe ser mayor a 0.".to_string(),
        ));
    }

    // Verificar si el servicio ya existe en la OT para evitar duplicados
    let servicio_existente: Option<(i64, i32)> = conn
        .query_row(
            "SELECT id_detalle, cantidad FROM detalle_ot_servicios WHERE id_ot = ?1 AND id_servicio = ?2",
            params![req.id_ot, req.id_servicio],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;

    if let Some((id_detalle, cant_actual)) = servicio_existente {
        // En lugar de fallar, se actualiza la cantidad acumulándola sobre el precio histórico
        let nueva_cant = cant_actual + req.cantidad;
        conn.execute(
            "UPDATE detalle_ot_servicios SET cantidad = ?1, id_usuario = ?2 WHERE id_detalle = ?3 AND id_ot = ?4",
            params![nueva_cant, id_usuario, id_detalle, req.id_ot],
        )?;
    } else {
        let (precio_cent, nombre, codigo, es_cambio): (i64, String, String, i32) = conn
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
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id_op_serv,
                req.id_ot,
                req.id_servicio,
                codigo,
                nombre,
                req.cantidad,
                precio_cent,
                es_cambio,
                id_usuario
            ],
        )?;
    }

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn modificar_servicio_ot(
    conn: &mut Connection,
    req: ModificarServicioOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;

    if ot.estado != "EN_ESPERA" && ot.estado != "EN_PROCESO" {
        return Err(AppError::Validation(
            "Solo se pueden modificar servicios en órdenes en espera o en proceso.".to_string(),
        ));
    }

    // Un mecánico solo puede modificar servicios si la OT le pertenece efectivamente
    validar_propiedad_mecanico(&ot, id_usuario, rol, "modificar servicios")?;

    if req.nueva_cantidad <= 0 {
        return Err(AppError::Validation(
            "La cantidad debe ser mayor a 0.".to_string(),
        ));
    }

    // Actualiza cantidad e id_usuario para cumplir validaciones del trigger trg_servicio_ot_editar
    conn.execute(
        "UPDATE detalle_ot_servicios SET cantidad = ?1, id_usuario = ?2 WHERE id_detalle = ?3 AND id_ot = ?4",
        params![req.nueva_cantidad, id_usuario, req.id_detalle_servicio, req.id_ot],
    )?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn eliminar_servicio_ot(
    conn: &mut Connection,
    id_detalle: i64,
    id_ot: i64,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, id_ot, id_usuario, rol)?;

    if ot.estado != "EN_ESPERA" && ot.estado != "EN_PROCESO" {
        return Err(AppError::Validation(
            "Solo se pueden eliminar servicios en órdenes en espera o en proceso.".to_string(),
        ));
    }

    // Validación estricta de propiedad de OT para rol MECANICO (Corregido: exige propiedad efectiva, rechaza libres y ajenas)
    validar_propiedad_mecanico(&ot, id_usuario, rol, "eliminar servicios")?;

    let total_servicios: i32 = conn.query_row(
        "SELECT COUNT(*) FROM detalle_ot_servicios WHERE id_ot = ?1",
        [id_ot],
        |row| row.get(0),
    )?;

    if total_servicios <= 1 {
        return Err(AppError::Validation(
            "No se puede eliminar el servicio. Una Orden debe mantener al menos un servicio."
                .to_string(),
        ));
    }

    conn.execute(
        "DELETE FROM detalle_ot_servicios WHERE id_detalle = ?1 AND id_ot = ?2",
        params![id_detalle, id_ot],
    )?;

    obtener_ot_por_id(conn, id_ot, id_usuario, rol)
}

pub fn listar_servicios(conn: &Connection) -> Result<Vec<ServicioResumenDto>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id_servicio, codigo, nombre, precio_venta_cent, (precio_venta_cent / 100.0) as precio_soles, es_cambio_aceite
         FROM servicios WHERE activo = 1 ORDER BY nombre ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(ServicioResumenDto {
            id_servicio: row.get(0)?,
            codigo: row.get(1)?,
            nombre: row.get(2)?,
            precio_venta_cent: row.get(3)?,
            precio_venta_soles: row.get(4)?,
            es_cambio_aceite: row.get(5)?,
        })
    })?;

    let mut lista = Vec::new();
    for r in rows {
        lista.push(r?);
    }
    Ok(lista)
}

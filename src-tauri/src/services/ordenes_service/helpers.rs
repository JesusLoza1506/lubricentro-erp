use crate::dto::orden_trabajo_dto::OrdenTrabajoHistorialDto;
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;

pub(crate) struct DatosVehiculoCliente {
    pub id_vehiculo: i64,
    pub id_cliente_atendido: i64,
    pub km_actual_ficha: i64,
    pub nombre_cliente: String,
    pub doc_cliente: Option<String>,
    pub tel_cliente: Option<String>,
    pub dir_cliente: Option<String>,
    pub mail_cliente: Option<String>,
    pub tipo_doc_cliente: Option<String>,
}

pub fn generar_codigo_ot(conn: &Connection) -> Result<String, AppError> {
    let max_id: i64 = conn.query_row(
        "SELECT COALESCE(MAX(id_ot), 0) FROM ordenes_trabajo",
        [],
        |row| row.get(0),
    )?;

    // Zona Horaria America/Lima (UTC-05:00)
    let hora_lima = chrono::Utc::now() + chrono::Duration::hours(-5);
    let anio_actual = hora_lima.format("%Y").to_string();

    Ok(format!("OT-{}-{:04}", anio_actual, max_id + 1))
}

pub fn tiene_cambio_aceite(conn: &Connection, id_ot: i64) -> bool {
    let serv_count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM detalle_ot_servicios WHERE id_ot = ?1 AND es_cambio_aceite = 1",
            [id_ot],
            |row| row.get(0),
        )
        .unwrap_or(0);

    serv_count > 0
}

pub fn extraer_identidad_cliente_snapshot(
    cliente_snapshot_raw: Option<String>,
    nombre_relacional: Option<String>,
    doc_relacional: Option<String>,
    tipo_doc_relacional: Option<String>,
) -> (Option<String>, Option<String>, Option<String>) {
    if let Some(snapshot_str) = cliente_snapshot_raw {
        if let Ok(v) = serde_json::from_str::<Value>(&snapshot_str) {
            let nombre = v
                .get("nombre")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());
            let documento = v
                .get("documento")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());
            let tipo_doc = v
                .get("tipo_documento")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());

            return (nombre, documento, tipo_doc);
        }
    }

    (nombre_relacional, doc_relacional, tipo_doc_relacional)
}

/// Garantiza que el odómetro del vehículo solo incremente progresivamente y no se reduzca al cancelar
pub fn recalcular_kilometraje_vehiculo(conn: &Connection, placa: &str) -> Result<(), AppError> {
    let max_km_ot: Option<i64> = conn
        .query_row(
            "SELECT MAX(kilometraje_salida) FROM ordenes_trabajo WHERE placa_snapshot = ?1 AND estado = 'FINALIZADO'",
            [placa],
            |row| row.get(0),
        )
        .optional()?
        .flatten();

    if let Some(km_ot) = max_km_ot {
        let km_actual_vehiculo: i64 = conn
            .query_row(
                "SELECT COALESCE(kilometraje_actual, 0) FROM vehiculos WHERE placa = ?1",
                [placa],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if km_ot > km_actual_vehiculo {
            conn.execute(
                "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
                params![km_ot, placa],
            )?;
        }
    }
    Ok(())
}

pub fn validar_mecanico_activo(conn: &Connection, id_mecanico: i64) -> Result<(), AppError> {
    if id_mecanico == 0 {
        return Ok(());
    }

    let es_mecanico_valido: Option<bool> = conn
        .query_row(
            "SELECT (UPPER(rol) = 'MECANICO' AND activo = 1) FROM usuarios WHERE id_usuario = ?1",
            [id_mecanico],
            |row| row.get(0),
        )
        .optional()?;

    match es_mecanico_valido {
        Some(true) => Ok(()),
        Some(false) => Err(AppError::Validation(format!(
            "El usuario asignado con ID {} no es un mecánico activo.",
            id_mecanico
        ))),
        None => Err(AppError::NotFound(format!(
            "No se encontró el mecánico con ID {}.",
            id_mecanico
        ))),
    }
}

/// Sanitiza la información sensible del DTO de Orden de Trabajo según el rol y el ID del usuario consultante.
pub fn sanitizar_ot_por_rol(dto: OrdenTrabajoHistorialDto, rol: &str) -> OrdenTrabajoHistorialDto {
    sanitizar_ot_por_rol_usuario(dto, rol, 0)
}

/// Sanitiza la información considerando id_usuario para verificar la propiedad efectiva de OTs por mecánicos.
pub fn sanitizar_ot_por_rol_usuario(
    mut dto: OrdenTrabajoHistorialDto,
    rol: &str,
    id_usuario: i64,
) -> OrdenTrabajoHistorialDto {
    let rol_upper = rol.to_uppercase();

    // 1. Privacidad de nota interna para CAJERO
    if rol_upper == "CAJERO" {
        dto.nota_interna = None;
    }

    // 2. Privacidad de nota interna para MECANICO:
    // Un mecánico solo puede ver la nota interna si es el mecánico asignado responsable de dicha OT.
    if rol_upper == "MECANICO" || rol_upper == "TECNICO" {
        let es_mecanico_responsable = id_usuario > 0 && dto.id_mecanico == Some(id_usuario);
        if !es_mecanico_responsable {
            dto.nota_interna = None;
        }
    }

    // 3. Privacidad de costos internos (solo ADMINISTRADOR los ve)
    if rol_upper != "ADMINISTRADOR" {
        for insumo in &mut dto.insumos {
            insumo.costo_unitario_micro = None;
        }
    }

    dto
}

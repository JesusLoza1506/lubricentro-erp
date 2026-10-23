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

    let anio_actual = chrono::Local::now().format("%Y").to_string();
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

pub fn recalcular_kilometraje_vehiculo(conn: &Connection, placa: &str) -> Result<(), AppError> {
    let max_km: Option<i64> = conn
        .query_row(
            "SELECT MAX(COALESCE(kilometraje_salida, kilometraje_ingreso)) FROM ordenes_trabajo WHERE placa_snapshot = ?1 AND estado != 'CANCELADO'",
            [placa],
            |row| row.get(0),
        )
        .optional()?
        .flatten();

    if let Some(km) = max_km {
        conn.execute(
            "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
            params![km, placa],
        )?;
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

pub fn sanitizar_ot_por_rol(
    mut dto: OrdenTrabajoHistorialDto,
    rol: &str,
) -> OrdenTrabajoHistorialDto {
    let rol_upper = rol.to_uppercase();

    // Privacidad de nota interna para CAJERO
    if rol_upper == "CAJERO" {
        dto.nota_interna = None;
    }

    // Privacidad de costos internos (solo ADMINISTRADOR los ve)
    if rol_upper != "ADMINISTRADOR" {
        for insumo in &mut dto.insumos {
            insumo.costo_unitario_micro = None;
        }
    }

    dto
}

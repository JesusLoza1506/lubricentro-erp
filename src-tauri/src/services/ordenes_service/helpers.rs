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

pub fn obtener_tipo_aceite_ot(conn: &Connection, id_ot: i64) -> Result<Option<String>, AppError> {
    let sql = "SELECT tipo_aceite FROM ordenes_trabajo WHERE id_ot = ?1";
    if let Ok(Some(Some(t))) = conn
        .query_row(sql, [id_ot], |row| row.get::<_, Option<String>>(0))
        .optional()
    {
        let t_clean = t.trim();
        if !t_clean.is_empty() {
            return Ok(Some(t_clean.to_string()));
        }
    }

    Ok(None)
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

pub fn calcular_proximo_kilometraje(
    conn: &Connection,
    id_ot: i64,
    km_base: i64,
    tipo_aceite: Option<&str>,
) -> Option<i64> {
    if !tiene_cambio_aceite(conn, id_ot) {
        return None;
    }

    let t_aceite = tipo_aceite?;
    let t_clean = t_aceite.trim();
    if t_clean.is_empty() {
        return None;
    }

    let intervalo = match t_clean.to_uppercase().as_str() {
        "SINTETICO" => 10000,
        "MINERAL" => 5000,
        _ => 5000,
    };

    Some(km_base + intervalo)
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

pub fn validar_transicion_estado(estado_actual: &str, nuevo_estado: &str) -> Result<(), AppError> {
    let actual = estado_actual.trim().to_uppercase();
    let nuevo = nuevo_estado.trim().to_uppercase();

    if actual == nuevo {
        return Ok(());
    }

    let es_valida = matches!(
        (actual.as_str(), nuevo.as_str()),
        ("EN_ESPERA", "EN_PROCESO")
            | ("EN_ESPERA", "CANCELADO")
            | ("EN_PROCESO", "FINALIZADO")
            | ("EN_PROCESO", "CANCELADO")
    );

    if !es_valida {
        return Err(AppError::Validation(format!(
            "Transición de estado no permitida: No se puede cambiar de '{}' a '{}'.",
            actual, nuevo
        )));
    }

    Ok(())
}

pub fn validar_permiso_operacion_ot(
    conn: &Connection,
    id_ot: i64,
    id_usuario: i64,
) -> Result<(), AppError> {
    let (rol_usuario, activo): (String, i32) = conn
        .query_row(
            "SELECT UPPER(rol), activo FROM usuarios WHERE id_usuario = ?1",
            [id_usuario],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| AppError::Validation("Usuario ejecutor no encontrado.".to_string()))?;

    if activo != 1 {
        return Err(AppError::Validation("Usuario inactivo.".to_string()));
    }

    if rol_usuario == "ADMINISTRADOR" {
        return Ok(());
    }

    if rol_usuario == "MECANICO" {
        let id_mecanico_ot: Option<i64> = conn
            .query_row(
                "SELECT id_mecanico FROM ordenes_trabajo WHERE id_ot = ?1",
                [id_ot],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound(format!("No se encontró la OT con ID {}.", id_ot)))?;

        if id_mecanico_ot != Some(id_usuario) && id_mecanico_ot != Some(0) {
            return Err(AppError::Validation(
                "Acceso denegado: No tienes permiso para modificar la OT de otro mecánico."
                    .to_string(),
            ));
        }
        return Ok(());
    }

    Err(AppError::Validation(
        "Acceso denegado: Tu rol no permite modificar detalles de Órdenes de Trabajo.".to_string(),
    ))
}

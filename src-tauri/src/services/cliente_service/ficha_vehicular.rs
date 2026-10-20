use super::validaciones::validar_motivo;
use super::vehiculo_crud::obtener_vehiculo_por_placa;
use crate::dto::vehiculo_dto::{
    CorregirHistorialOtPayloadDto, FichaVehicularCompletaDto, ProximoCambioAceiteDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::{params, Connection};

pub fn corregir_historial_ot(
    conn: &Connection,
    payload: CorregirHistorialOtPayloadDto,
    id_usuario: Option<i64>,
    username: Option<&str>,
    rol: Option<&str>,
) -> Result<(), AppError> {
    let motivo = validar_motivo(&payload.motivo)?;
    let resumen = payload.resumen_publico.trim();

    if resumen.is_empty() || resumen.len() > 1000 {
        return Err(AppError::Validation(
            "El resumen público debe tener entre 1 y 1000 caracteres.".to_string(),
        ));
    }

    let (resumen_old, km_old, tipo_aceite_old): (String, i64, Option<String>) = conn
        .query_row(
            "SELECT resumen_publico, kilometraje_salida, tipo_aceite FROM ordenes_trabajo WHERE id_ot = ?1 AND estado = 'FINALIZADO'",
            [payload.id_ot],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_| AppError::NotFound("OT finalizada no encontrada.".to_string()))?;

    let intervalo_km = match payload.tipo_aceite.as_deref() {
        Some("MINERAL") => Some(5000),
        Some("SINTETICO") => Some(10000),
        _ => None,
    };

    conn.execute(
        "UPDATE ordenes_trabajo
         SET resumen_publico = ?1,
             kilometraje_salida = ?2,
             tipo_aceite = ?3,
             intervalo_aceite_km = ?4,
             corregido = 1
         WHERE id_ot = ?5 AND estado = 'FINALIZADO'",
        params![
            resumen,
            payload.kilometraje_salida,
            payload.tipo_aceite,
            intervalo_km,
            payload.id_ot
        ],
    )?;

    let antes_json = serde_json::json!({
        "resumen_publico": resumen_old,
        "kilometraje_salida": km_old,
        "tipo_aceite": tipo_aceite_old,
    })
    .to_string();

    let despues_json = serde_json::json!({
        "resumen_publico": resumen,
        "kilometraje_salida": payload.kilometraje_salida,
        "tipo_aceite": payload.tipo_aceite,
    })
    .to_string();

    AuditoriaService::registrar_v5(
        conn,
        id_usuario,
        if id_usuario.is_some() {
            "USUARIO"
        } else {
            "NO_IDENTIFICADO"
        },
        username,
        rol,
        None,
        "FICHA_VEHICULAR",
        "CORREGIR_HISTORIAL_OT",
        Some("ordenes_trabajo"),
        Some(&payload.id_ot.to_string()),
        Some(&antes_json),
        Some(&despues_json),
        Some(&motivo),
        "OK",
    )?;

    Ok(())
}

pub fn obtener_ficha_vehicular_completa(
    conn: &Connection,
    placa: &str,
    rol_ejecutor: Option<&str>,
) -> Result<FichaVehicularCompletaDto, AppError> {
    let vehiculo = obtener_vehiculo_por_placa(conn, placa, rol_ejecutor)?;
    let historial = OrdenesService::obtener_historial_por_id_vehiculo(conn, vehiculo.id_vehiculo)?;

    Ok(FichaVehicularCompletaDto {
        vehiculo,
        historial,
    })
}

pub fn obtener_proximos_cambios_aceite(
    conn: &Connection,
    rol_ejecutor: Option<&str>,
) -> Result<Vec<ProximoCambioAceiteDto>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id_vehiculo, placa, kilometraje_actual, id_cliente, nombre_razon_social, telefono,
                id_ot, fecha_servicio, tipo_aceite, intervalo_aceite_km, km_proximo, causa
         FROM v_proximos_cambios
         ORDER BY fecha_servicio DESC",
    )?;

    let es_mecanico = rol_ejecutor
        .map(|r| r.to_uppercase() == "MECANICO")
        .unwrap_or(false);

    let rows = stmt.query_map([], |row| {
        let mut telefono: Option<String> = row.get(5)?;
        if es_mecanico {
            telefono = None;
        }

        Ok(ProximoCambioAceiteDto {
            id_vehiculo: row.get(0)?,
            placa: row.get(1)?,
            kilometraje_actual: row.get(2)?,
            id_cliente: row.get(3)?,
            nombre_razon_social: row.get(4)?,
            telefono,
            id_ot: row.get(6)?,
            fecha_servicio: row.get(7)?,
            tipo_aceite: row.get(8)?,
            intervalo_aceite_km: row.get(9)?,
            km_proximo: row.get(10)?,
            causa: row.get(11)?,
        })
    })?;

    let mut lista = Vec::new();
    for r in rows {
        lista.push(r?);
    }

    Ok(lista)
}

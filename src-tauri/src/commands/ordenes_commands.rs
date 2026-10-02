use crate::dto::orden_trabajo_dto::{CambiarEstadoOtDto, CrearOrdenTrabajoDto, OrdenTrabajoDto};
use crate::errors::AppError;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn crear_orden_trabajo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearOrdenTrabajoDto,
) -> Result<OrdenTrabajoDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    OrdenesService::crear_orden_trabajo(&conn, payload)
}

#[tauri::command]
pub fn obtener_ot_por_codigo_cmd(
    state: State<'_, Mutex<Connection>>,
    codigo: String,
) -> Result<OrdenTrabajoDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    OrdenesService::obtener_ot_por_codigo(&conn, &codigo)
}

#[tauri::command]
pub fn cambiar_estado_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CambiarEstadoOtDto,
) -> Result<OrdenTrabajoDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    OrdenesService::cambiar_estado_ot(&conn, payload)
}

#[tauri::command]
pub fn listar_ordenes_trabajo_cmd(
    state: State<'_, Mutex<Connection>>,
    filtro_estado: Option<String>,
) -> Result<Vec<OrdenTrabajoDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    OrdenesService::listar_ordenes_trabajo(&conn, filtro_estado)
}

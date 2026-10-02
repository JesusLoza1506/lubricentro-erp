use crate::dto::caja_dto::{AbrirCajaDto, CajaChicaDto, CierreCajaDto};
use crate::errors::AppError;
use crate::services::caja_service::CajaService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn abrir_caja_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: AbrirCajaDto,
) -> Result<i64, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    CajaService::abrir_caja(&conn, payload)
}

#[tauri::command]
pub fn cerrar_caja_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CierreCajaDto,
) -> Result<CajaChicaDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    CajaService::cerrar_caja(&conn, payload)
}

use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::errors::AppError;
use crate::services::comprobante_service::ComprobanteService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn emitir_comprobante_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: EmitirComprobanteDto,
) -> Result<ComprobanteDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    ComprobanteService::emitir_comprobante(&conn, payload)
}

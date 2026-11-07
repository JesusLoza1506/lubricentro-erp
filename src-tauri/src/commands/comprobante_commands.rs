use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::errors::AppError;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn emitir_comprobante_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: EmitirComprobanteDto,
) -> Result<ComprobanteDto, AppError> {
    let _ = (db_state, session_state, payload);
    Err(AppError::Validation("Utiliza la confirmación de venta de Punto de Venta POS. La emisión aislada anterior fue sustituida.".into()))
}

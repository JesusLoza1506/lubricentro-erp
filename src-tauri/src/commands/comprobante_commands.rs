use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::errors::AppError;
use crate::services::comprobante_service::ComprobanteService;
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
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _id_usuario = session.validar_sesion_operativa(&conn)?;

    ComprobanteService::emitir_comprobante(&conn, payload)
}

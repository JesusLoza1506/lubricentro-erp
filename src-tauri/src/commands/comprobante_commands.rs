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
    mut payload: EmitirComprobanteDto,
) -> Result<ComprobanteDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    // Impera la sesión: Extracción estricta del ID del cajero/usuario real
    let id_usuario_ejecutor = session.validar_sesion_operativa(&conn)?;
    payload.id_cajero = id_usuario_ejecutor;

    ComprobanteService::emitir_comprobante(&conn, payload)
}

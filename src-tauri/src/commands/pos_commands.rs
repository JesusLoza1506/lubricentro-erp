use crate::{
    dto::pos_dto::CredencialesPos,
    errors::AppError,
    services::{pos_service::operaciones, session_manager::SessionState},
};
use rusqlite::Connection;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;
#[tauri::command]
pub fn pos_operacion_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    accion: String,
    payload: Value,
    credenciales: Option<CredencialesPos>,
) -> Result<Value, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    operaciones::ejecutar(&mut conn, &mut session, &accion, payload, credenciales)
}

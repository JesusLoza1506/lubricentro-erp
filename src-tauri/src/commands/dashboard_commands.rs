use crate::dto::dashboard_dto::DashboardDto;
use crate::errors::AppError;
use crate::services::dashboard_service::DashboardService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn obtener_dashboard_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    session_id: String,
) -> Result<DashboardDto, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    DashboardService::obtener_para_sesion(&conn, &mut session, &session_id)
}

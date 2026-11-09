use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
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
) -> Result<DashboardDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let id_usuario = session.validar_sesion_operativa(&conn)?;

    DashboardService::obtener_dashboard(&conn, id_usuario)
}

#[tauri::command]
pub fn listar_alertas_fidelizacion_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<AlertaFidelizacionDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _ = session.validar_sesion_operativa(&conn)?;

    DashboardService::listar_alertas_fidelizacion(&conn)
}

use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
use crate::errors::AppError;
use crate::services::dashboard_service::DashboardService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

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

#[tauri::command]
pub fn generar_backup_cmd(
    app_handle: tauri::AppHandle,
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    destino_path: String,
) -> Result<String, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _ = session.validar_sesion_operativa(&conn)?;

    let rol = session.rol.as_deref().unwrap_or("").to_uppercase();
    if rol != "ADMINISTRADOR" {
        return Err(AppError::Validation(
            "Acceso denegado: Se requiere rol ADMINISTRADOR para generar backups.".to_string(),
        ));
    }

    let app_data_dir = app_handle.path().app_data_dir().map_err(|_| {
        AppError::Validation("No se pudo obtener el directorio de la app".to_string())
    })?;

    let db_path = app_data_dir.join("lubricentro.db");
    let destino_dir = PathBuf::from(destino_path);

    DashboardService::generar_backup_db(&db_path, &destino_dir)
}

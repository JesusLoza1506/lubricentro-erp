use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
use crate::errors::AppError;
use crate::services::dashboard_service::DashboardService;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

#[tauri::command]
pub fn obtener_dashboard_cmd(
    state: State<'_, Mutex<Connection>>,
    id_usuario: i64,
) -> Result<DashboardDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    DashboardService::obtener_dashboard(&conn, id_usuario)
}

#[tauri::command]
pub fn listar_alertas_fidelizacion_cmd(
    state: State<'_, Mutex<Connection>>,
) -> Result<Vec<AlertaFidelizacionDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    DashboardService::listar_alertas_fidelizacion(&conn)
}

#[tauri::command]
pub fn generar_backup_cmd(
    app_handle: tauri::AppHandle,
    destino_path: String,
) -> Result<String, AppError> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|_| {
        AppError::Validation("No se pudo obtener el directorio de la app".to_string())
    })?;

    let db_path = app_data_dir.join("lubricentro.db");
    let destino_dir = PathBuf::from(destino_path);

    DashboardService::generar_backup_db(&db_path, &destino_dir)
}

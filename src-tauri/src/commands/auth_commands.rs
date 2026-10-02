use crate::dto::usuario_dto::{CrearUsuarioRequestDto, LoginRequestDto, LoginResponseDto};
use crate::errors::AppError;
use crate::services::auth_service::AuthService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn login_cmd(
    state: State<'_, Mutex<Connection>>,
    req: LoginRequestDto,
) -> Result<LoginResponseDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::login(&conn, req)
}

#[tauri::command]
pub fn obtener_usuario_sesion_cmd(
    state: State<'_, Mutex<Connection>>,
    id_usuario: i64,
) -> Result<LoginResponseDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::obtener_usuario_sesion(&conn, id_usuario)
}

#[tauri::command]
pub fn crear_usuario_cmd(
    state: State<'_, Mutex<Connection>>,
    req: CrearUsuarioRequestDto,
) -> Result<i64, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::crear_usuario(&conn, req)
}

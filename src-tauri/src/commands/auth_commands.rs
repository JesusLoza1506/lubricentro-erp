use crate::dto::usuario_dto::UsuarioSesionDTO;
use crate::services::auth_service::AuthService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn obtener_usuario_sesion_cmd(
    usuario: String,
    password_hash: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<UsuarioSesionDTO, String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Error al acceder a la BD: {}", e))?;

    // Delegamos la consulta SQL y la validación directamente a AuthService
    AuthService::autenticar_usuario(&conn, &usuario, &password_hash)
}

// NUEVO COMANDO: Para crear usuarios con contraseña encriptada de forma segura
#[tauri::command]
pub async fn crear_usuario_cmd(
    nombre_completo: String,
    usuario: String,
    password_plano: String,
    rol: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<String, String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Error al acceder a la BD: {}", e))?;

    AuthService::crear_usuario(&conn, &nombre_completo, &usuario, &password_plano, &rol)?;
    Ok("Usuario creado exitosamente".to_string())
}

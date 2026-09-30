use crate::dto::usuario_dto::UsuarioSesionDTO;

#[tauri::command]
pub async fn obtener_usuario_sesion_cmd() -> Result<UsuarioSesionDTO, String> {
    // Handler delgado de Tauri (Command). En la Fase 5 se conectará directamente al frontend.
    Ok(UsuarioSesionDTO {
        id_usuario: 1,
        nombre_completo: "Administrador General".to_string(),
        usuario: "admin".to_string(),
        rol: "ADMINISTRADOR".to_string(),
        activo: true,
    })
}

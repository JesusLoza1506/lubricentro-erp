use super::permiso_dto::PermisoDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginResponseDto {
    pub id_usuario: i64,
    pub nombre_completo: String,
    pub username: String,
    pub rol: String,
    pub activo: bool,
    pub debe_cambiar_clave: bool,
    pub estado_sesion: String, // "NORMAL" o "CAMBIO_OBLIGATORIO"
    pub permisos: Vec<PermisoDto>,
}

#[derive(Debug, Deserialize)]
pub struct InstalacionInicialRequestDto {
    pub nombre_completo: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct InstalacionInicialResponseDto {
    pub id_usuario: i64,
    pub username: String,
    pub codigo_recuperacion: String, // Se muestra una sola vez al usuario
}

#[derive(Debug, Deserialize)]
pub struct RecuperarAccesoAdminRequestDto {
    pub username: String,
    pub codigo_recuperacion: String,
    pub nueva_password: String,
}

/// DTO para Cambio Obligatorio de Clave (RF-LOG-03): Solo exige nueva contraseña
#[derive(Debug, Deserialize)]
pub struct CambiarClaveObligatoriaRequestDto {
    pub nueva_password: String,
}

/// DTO para Cambio Voluntario de Clave (RF-LOG-04): Exige la clave actual
#[derive(Debug, Deserialize)]
pub struct CambiarClaveVoluntariaRequestDto {
    pub password_actual: String,
    pub nueva_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ReautenticarRequestDto {
    pub password_actual: String,
    pub accion: String,
    pub referencia: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AutorizarDescuentoRequestDto {
    pub username_admin: String,
    pub password_admin: String,
    pub referencia_venta: String,
    pub descuento_bp: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SesionResumenDto {
    pub id_usuario: i64,
    pub username: String,
    pub nombre_completo: String,
    pub rol: String,
    pub estado_sesion: String,
    pub permisos: Vec<PermisoDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MecanicoResumenDto {
    pub id_usuario: i64,
    pub nombre_completo: String,
}

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
    pub permisos: Vec<PermisoDto>,
}

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioRequestDto {
    pub nombre_completo: String,
    pub username: String,
    pub password: String,
    pub rol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MecanicoResumenDto {
    pub id_usuario: i64,
    pub nombre_completo: String,
}

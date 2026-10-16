use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsuarioEntity {
    pub id_usuario: i64,
    pub nombre_completo: String,
    pub username: String,
    pub password_hash: String,
    pub rol: String,
    pub activo: i32,
    pub debe_cambiar_clave: i32,
    pub intentos_fallidos: i32,
    pub bloqueado_hasta: Option<String>,
    pub fecha_creacion: String,
    pub fecha_cambio_clave: Option<String>,
    pub version: i64,
}

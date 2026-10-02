use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsuarioEntity {
    pub id_usuario: i64,
    pub nombre_completo: String,
    pub username: String,
    pub password_hash: String,
    pub rol: String,
    pub activo: i32,
}

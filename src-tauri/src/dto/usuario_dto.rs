use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsuarioSesionDTO {
    pub id_usuario: i32,
    pub nombre_completo: String,
    pub usuario: String,
    pub rol: String,
    pub activo: bool,
}

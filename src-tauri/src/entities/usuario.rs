#[derive(Debug, Clone, PartialEq)]
pub struct UsuarioEntity {
    pub id_usuario: i32,
    pub nombre_completo: String,
    pub usuario: String,
    pub password_hash: String,
    pub rol: String,
    pub activo: bool,
}

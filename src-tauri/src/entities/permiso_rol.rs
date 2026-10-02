use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermisoRolEntity {
    pub id_permiso: i64,
    pub rol: String,
    pub modulo: String,
    pub puede_ver: i32,
    pub puede_crear: i32,
    pub puede_editar: i32,
    pub puede_eliminar: i32,
}

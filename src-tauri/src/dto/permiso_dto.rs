use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermisoDto {
    pub modulo: String,
    pub puede_ver: bool,
    pub puede_crear: bool,
    pub puede_editar: bool,
    pub puede_eliminar: bool,
}

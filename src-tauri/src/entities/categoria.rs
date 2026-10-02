use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoriaEntity {
    pub id_categoria: i64,
    pub nombre: String,
    pub activo: i32,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoriaEntity {
    pub id_categoria: i64,
    #[serde(rename = "nombre_categoria")]
    pub nombre: String,
    pub activo: i32,
}

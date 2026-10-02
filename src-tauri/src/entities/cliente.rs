use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteEntity {
    pub id_cliente: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub activo: i32,
}

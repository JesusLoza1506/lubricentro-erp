use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteDTO {
    pub id_cliente: i32,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

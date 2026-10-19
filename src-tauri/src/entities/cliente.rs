use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteEntity {
    pub id_cliente: i64,
    pub tipo_documento: Option<String>,
    pub numero_documento: Option<String>,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub es_varios: i32,
    pub fecha_creacion: String,
    pub version: i64,
}

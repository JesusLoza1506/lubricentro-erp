use crate::entities::cliente::ClienteEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteDto {
    pub id_cliente: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub activo: i32,
}

impl From<ClienteEntity> for ClienteDto {
    fn from(entity: ClienteEntity) -> Self {
        Self {
            id_cliente: entity.id_cliente,
            tipo_documento: entity.tipo_documento,
            numero_documento: entity.numero_documento,
            nombre_razon_social: entity.nombre_razon_social,
            telefono: entity.telefono,
            direccion: entity.direccion,
            activo: entity.activo,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CrearClienteDto {
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

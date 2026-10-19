use crate::entities::cliente::ClienteEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteDto {
    pub id_cliente: i64,
    pub tipo_documento: Option<String>,
    pub numero_documento: Option<String>,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub es_varios: bool,
    pub fecha_creacion: String,
}

impl From<ClienteEntity> for ClienteDto {
    fn from(entity: ClienteEntity) -> Self {
        Self {
            id_cliente: entity.id_cliente,
            tipo_documento: entity.tipo_documento,
            numero_documento: entity.numero_documento,
            nombre_razon_social: entity.nombre_razon_social,
            telefono: entity.telefono,
            correo: entity.correo,
            direccion: entity.direccion,
            es_varios: entity.es_varios == 1,
            fecha_creacion: entity.fecha_creacion,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CrearClienteDto {
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EditarClienteDto {
    pub id_cliente: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub motivo: Option<String>,
}

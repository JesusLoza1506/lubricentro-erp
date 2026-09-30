use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EstadoColaDTO {
    pub id_cola: i32,
    pub id_comprobante: i32,
    pub estado_envio: String,
    pub intentos: i32,
    pub ultima_error: Option<String>,
}

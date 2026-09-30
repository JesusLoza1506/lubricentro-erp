use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiculoDTO {
    pub placa: String,
    pub id_cliente: i32,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: Option<i32>,
}

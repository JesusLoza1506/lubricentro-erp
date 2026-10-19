use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiculoEntity {
    pub id_vehiculo: i64,
    pub placa: String,
    pub id_cliente: i64,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
    pub activo: i32,
    pub fecha_creacion: String,
    pub version: i64,
}

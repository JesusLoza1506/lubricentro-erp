use crate::entities::vehiculo::VehiculoEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiculoDto {
    pub placa: String,
    pub id_cliente: i64,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
    pub activo: i32,
}

impl From<VehiculoEntity> for VehiculoDto {
    fn from(entity: VehiculoEntity) -> Self {
        Self {
            placa: entity.placa,
            id_cliente: entity.id_cliente,
            marca: entity.marca,
            modelo: entity.modelo,
            anio: entity.anio,
            tipo_motor: entity.tipo_motor,
            kilometraje_actual: entity.kilometraje_actual,
            activo: entity.activo,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CrearVehiculoDto {
    pub placa: String,
    pub id_cliente: i64,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
}

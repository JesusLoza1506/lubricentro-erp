use crate::entities::orden_trabajo::OrdenTrabajoEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdenTrabajoDto {
    pub id_ot: i64,
    pub codigo_ot: String,
    pub placa: String,
    pub id_mecanico: i64,
    pub zanja: Option<i32>,
    pub estado: String,
    pub kilometraje_ingreso: i64,
    pub proximo_kilometraje: i64,
    pub observaciones: Option<String>,
    pub fecha_ingreso: Option<String>,
}

impl From<OrdenTrabajoEntity> for OrdenTrabajoDto {
    fn from(entity: OrdenTrabajoEntity) -> Self {
        Self {
            id_ot: entity.id_ot,
            codigo_ot: entity.codigo_ot,
            placa: entity.placa,
            id_mecanico: entity.id_mecanico,
            zanja: entity.zanja,
            estado: entity.estado,
            kilometraje_ingreso: entity.kilometraje_ingreso,
            proximo_kilometraje: entity.proximo_kilometraje,
            observaciones: entity.observaciones,
            fecha_ingreso: entity.fecha_ingreso,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CrearOrdenTrabajoDto {
    pub placa: String,
    pub id_mecanico: i64,
    pub zanja: Option<i32>,
    pub kilometraje_ingreso: i64,
    pub proximo_kilometraje: i64,
    pub observaciones: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CambiarEstadoOtDto {
    pub id_ot: i64,
    pub nuevo_estado: String,
    pub zanja: Option<i32>,
}

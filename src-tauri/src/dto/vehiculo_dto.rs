use crate::dto::orden_trabajo_dto::OrdenTrabajoHistorialDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteResumenDto {
    pub id_cliente: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiculoCompletoDto {
    pub placa: String,
    pub id_cliente: i64,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
    pub activo: i32,
    pub cliente: Option<ClienteResumenDto>,
    pub nombre_cliente: Option<String>,
    pub tipo_documento_cliente: Option<String>,
    pub documento_cliente: Option<String>,
    pub telefono_cliente: Option<String>,
    pub direccion_cliente: Option<String>,
}

/// DTO Unificado para Ficha Vehicular e Historial en 1 sola consulta atómica
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FichaVehicularCompletaDto {
    pub vehiculo: VehiculoCompletoDto,
    pub historial: Vec<OrdenTrabajoHistorialDto>,
}

#[derive(Debug, Deserialize)]
pub struct CrearVehiculoPayloadDto {
    pub placa: String,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
    #[serde(alias = "idCliente")]
    pub id_cliente: Option<i64>,
    #[serde(alias = "nombreRazonSocial")]
    pub nombre_razon_social: Option<String>,
    #[serde(alias = "tipoDocumento")]
    pub tipo_documento: Option<String>,
    #[serde(alias = "numeroDocumento")]
    pub numero_documento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EditarVehiculoPayloadDto {
    #[serde(alias = "idVehiculo")]
    pub id_vehiculo: Option<i64>,
    pub placa: String,
    pub marca: String,
    pub modelo: String,
    pub anio: Option<i32>,
    pub tipo_motor: Option<String>,
    pub kilometraje_actual: i64,
    #[serde(alias = "idCliente")]
    pub id_cliente: Option<i64>,
    #[serde(alias = "nombreRazonSocial")]
    pub nombre_razon_social: Option<String>,
    #[serde(alias = "tipoDocumento")]
    pub tipo_documento: Option<String>,
    #[serde(alias = "numeroDocumento")]
    pub numero_documento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

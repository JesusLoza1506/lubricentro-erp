use crate::dto::orden_trabajo_dto::OrdenTrabajoHistorialDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClienteResumenDto {
    pub id_cliente: i64,
    pub tipo_documento: Option<String>,
    pub numero_documento: Option<String>,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiculoCompletoDto {
    pub id_vehiculo: i64,
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
    pub correo_cliente: Option<String>,
    pub direccion_cliente: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FichaVehicularCompletaDto {
    pub vehiculo: VehiculoCompletoDto,
    pub historial: Vec<OrdenTrabajoHistorialDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximoCambioAceiteDto {
    pub id_vehiculo: i64,
    pub placa: String,
    pub kilometraje_actual: i64,
    pub id_cliente: i64,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub id_ot: i64,
    pub fecha_servicio: String,
    pub tipo_aceite: Option<String>,
    pub intervalo_aceite_km: Option<i32>,
    pub km_proximo: Option<i64>,
    pub causa: String,
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
    pub correo: Option<String>,
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
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub motivo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CambiarEstadoVehiculoPayloadDto {
    pub placa: String,
    pub activo: i32,
    pub motivo: String,
}

#[derive(Debug, Deserialize)]
pub struct CorregirHistorialOtPayloadDto {
    pub id_ot: i64,
    pub resumen_publico: String,
    pub kilometraje_salida: i64,
    pub tipo_aceite: Option<String>,
    pub motivo: String,
}

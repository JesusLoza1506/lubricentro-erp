use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveedorDto {
    pub id_proveedor: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub contacto_asesor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CrearProveedorDto {
    pub tipo_documento: String, // DNI | RUC | CE
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub contacto_asesor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarProveedorDto {
    pub id_proveedor: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub contacto_asesor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicioMantenimientoDto {
    pub id_servicio: i64,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta_cent: i64,
    pub es_cambio_aceite: bool,
    pub activo: bool,
}

#[derive(Debug, Deserialize)]
pub struct CrearServicioDto {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta_cent: i64,
    pub es_cambio_aceite: bool,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarServicioDto {
    pub id_servicio: i64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta_cent: i64,
    pub es_cambio_aceite: bool,
}

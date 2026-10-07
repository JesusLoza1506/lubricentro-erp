use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetalleItemOtDto {
    pub id_detalle: i64,
    pub descripcion: String,
    pub cantidad: f64,
    pub precio_unitario: f64, // Convertido a soles (/ 100.0)
    pub subtotal: f64,        // Convertido a soles (/ 100.0)
    pub tipo: String,         // "PRODUCTO" | "SERVICIO"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdenTrabajoHistorialDto {
    pub id_ot: i64,
    pub codigo_ot: String,
    pub placa: String,
    pub id_mecanico: i64,
    pub nombre_mecanico: Option<String>,
    pub zanja: Option<i32>,
    pub estado: String,
    pub kilometraje_ingreso: i64,
    pub proximo_kilometraje: i64,
    pub observaciones: Option<String>,
    pub fecha_ingreso: Option<String>,
    pub tipo_aceite: Option<String>,
    pub detalles: Vec<DetalleItemOtDto>,
}

#[derive(Debug, Deserialize)]
pub struct CrearOrdenTrabajoDto {
    pub placa: String,
    pub id_mecanico: i64,
    pub zanja: Option<i32>,
    pub kilometraje_ingreso: i64,
    pub tipo_aceite: String, // "MINERAL" | "SINTETICO" (Usado para auto-calcular próximo KM)
    pub observaciones: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CambiarEstadoOtDto {
    pub id_ot: i64,
    pub nuevo_estado: String,
    pub zanja: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ReasignarMecanicoOtDto {
    pub id_ot: i64,
    pub nuevo_id_mecanico: i64,
}

// --- NUEVOS DTOS PARA PASO 4 (GESTIÓN DE DETALLES) ---

#[derive(Debug, Deserialize)]
pub struct AgregarProductoOtDto {
    pub id_ot: i64,
    pub id_producto: i64,
    pub cantidad: f64,
}

#[derive(Debug, Deserialize)]
pub struct AgregarServicioOtDto {
    pub id_ot: i64,
    pub id_servicio: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicioResumenDto {
    pub id_servicio: i64,
    pub descripcion: String,
    pub precio_base: f64, // En soles
}

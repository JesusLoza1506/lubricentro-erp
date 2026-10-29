use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DetalleIngresoRequestDto {
    pub id_producto: i64,
    pub cantidad: i64,             // En UND o ML
    pub costo_efectivo_micro: i64, // Micro-soles por unidad/ML
}

#[derive(Debug, Deserialize)]
pub struct RegistrarIngresoDto {
    pub origen: String, // "COMPRA" | "INICIAL"
    pub id_proveedor: Option<i64>,
    pub tipo_documento: Option<String>, // "FACTURA" | "BOLETA" | "OTRO" | "SIN_DOCUMENTO"
    pub numero_documento: Option<String>,
    pub fecha_documento: Option<String>, // YYYY-MM-DD
    pub referencia_inicial: Option<String>,
    pub observacion: Option<String>,
    pub detalles: Vec<DetalleIngresoRequestDto>,
}

#[derive(Debug, Deserialize)]
pub struct AnularIngresoDto {
    pub id_ingreso: i64,
    pub motivo: String, // Mínimo 5 caracteres
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngresoStockResponseDto {
    pub id_ingreso: i64,
    pub id_operacion: String,
    pub codigo: String,
    pub origen: String,
    pub id_proveedor: Option<i64>,
    pub tipo_documento: Option<String>,
    pub numero_documento: Option<String>,
    pub fecha_documento: Option<String>,
    pub estado: String,
    pub total_costo_micro: i64,
    pub id_usuario: i64,
    pub observacion: Option<String>,
    pub fecha_registro: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetalleIngresoRequestDto {
    pub id_producto: i64,
    pub cantidad: i64,             // En UND o ML
    pub costo_efectivo_micro: i64, // Micro-soles por UND o litro (captura comercial de dos decimales)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnularIngresoDto {
    pub id_ingreso: i64,
    pub motivo: String, // Mínimo 5 caracteres
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetalleIngresoResponseDto {
    pub id_detalle: i64,
    pub id_ingreso: i64,
    pub id_producto: i64,
    pub codigo_snapshot: String,
    pub nombre_snapshot: String,
    pub unidad_medida: String,
    pub cantidad: i64,
    pub costo_efectivo_micro: i64,
    pub valor_aporte_micro: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngresoStockResponseDto {
    pub id_ingreso: i64,
    pub id_operacion: String,
    pub codigo: String,
    pub origen: String,
    pub id_proveedor: Option<i64>,
    pub proveedor_nombre: Option<String>,
    pub tipo_documento: Option<String>,
    pub numero_documento: Option<String>,
    pub fecha_documento: Option<String>,
    pub estado: String,
    pub total_costo_micro: i64,
    pub id_usuario: i64,
    pub usuario_nombre: Option<String>,
    pub observacion: Option<String>,
    pub fecha_registro: String,
    pub fecha_anulacion: Option<String>,
    pub motivo_anulacion: Option<String>,
    pub detalles: Vec<DetalleIngresoResponseDto>,
}

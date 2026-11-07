use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LineaDirecta {
    pub tipo: String,
    pub id: i64,
    /// UND enteras o mililitros enteros. El precio siempre es por UND o litro.
    pub cantidad: i64,
    pub precio_esperado_cent: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PagoPos {
    pub medio: String,
    pub monto_cent: i64,
    pub recibido_cent: Option<i64>,
    pub referencia: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmarVentaPos {
    pub id_operacion: String,
    pub id_ot: Option<i64>,
    pub id_cliente: i64,
    pub tipo_comprobante: String,
    pub descuento_bp: i64,
    pub motivo: String,
    pub lineas: Vec<LineaDirecta>,
    pub pagos: Vec<PagoPos>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LineaRetorno {
    pub id_detalle: i64,
    pub cantidad: i64,
    pub recuperada: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DevolverVentaPos {
    pub id_operacion: String,
    pub id_venta: i64,
    pub motivo: String,
    pub medio: String,
    pub referencia: Option<String>,
    pub lineas: Vec<LineaRetorno>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredencialesPos {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReemitirPos {
    pub id_operacion: String,
    pub id_documento: i64,
    pub receptor: serde_json::Value,
    pub motivo: String,
}

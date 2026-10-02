use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComprobanteEntity {
    pub id_comprobante: i64,
    pub id_ot: Option<i64>,
    pub id_cliente: i64,
    pub id_cajero: i64,
    pub id_caja: i64,
    pub tipo_comprobante: String,
    pub id_serie: Option<i64>,
    pub correlativo: i64,
    pub monto_subtotal: i64,
    pub monto_igv: i64,
    pub monto_total: i64,
    pub estado_sunat: String,
    pub fecha_emision: Option<String>,
}

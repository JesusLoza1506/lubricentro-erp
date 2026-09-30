#[derive(Debug, Clone, PartialEq)]
pub struct ComprobanteEntity {
    pub id_comprobante: i32,
    pub id_ot: Option<i32>,
    pub id_cliente: i32,
    pub id_cajero: i32,
    pub tipo_comprobante: String,
    pub id_serie: Option<i32>,
    pub correlativo: i32,
    pub monto_subtotal: f64,
    pub monto_igv: f64,
    pub monto_total: f64,
    pub medio_pago: String,
    pub estado_sunat: String,
    pub fecha_emision: Option<String>,
}

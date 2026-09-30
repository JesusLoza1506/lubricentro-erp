#[derive(Debug, Clone, PartialEq)]
pub struct DetalleComprobanteEntity {
    pub id_detalle: i32,
    pub id_comprobante: i32,
    pub id_producto: i32,
    pub cantidad: f64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

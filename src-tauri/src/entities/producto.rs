#[derive(Debug, Clone, PartialEq)]
pub struct ProductoEntity {
    pub id_producto: i32,
    pub id_categoria: i32,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_compra: f64,
    pub precio_venta: f64,
}

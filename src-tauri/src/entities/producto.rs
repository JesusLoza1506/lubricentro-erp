use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoEntity {
    pub id_producto: i64,
    pub id_categoria: i64,
    pub nombre: String,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_compra: i64, // Almacenado en céntimos (ej. S/ 15.00 -> 1500)
    pub precio_venta: i64,  // Almacenado en céntimos (ej. S/ 25.00 -> 2500)
    pub activo: i32,
}

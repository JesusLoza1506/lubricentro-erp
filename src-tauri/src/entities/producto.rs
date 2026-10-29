use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoEntity {
    pub id_producto: i64,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,     // ACEITE | FILTRO | ADITIVO | GRANEL | OTRO
    pub unidad_medida: String, // UND | ML
    pub stock_actual: i64,     // Unidades enteras o mililitros
    pub stock_minimo: i64,
    pub valor_stock_micro: i64,          // Micro-soles
    pub ultimo_costo_micro: Option<i64>, // Micro-soles
    pub precio_venta_cent: i64,          // Céntimos de Sol
    pub activo: i32,
}

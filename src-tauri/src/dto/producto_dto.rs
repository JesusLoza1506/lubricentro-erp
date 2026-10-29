use crate::entities::producto::ProductoEntity;
use serde::{Deserialize, Serialize};

/// DTO Público (Cajeros y Mecánicos): Jamás expone costos ni valores de almacén
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoPublicoDto {
    pub id_producto: i64,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub unidad_medida: String,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub precio_venta_cent: i64,
    pub es_critico: bool,
}

impl ProductoPublicoDto {
    pub fn desde_entity(entity: ProductoEntity) -> Self {
        let es_critico = entity.stock_actual <= entity.stock_minimo;
        Self {
            id_producto: entity.id_producto,
            codigo: entity.codigo,
            nombre: entity.nombre,
            descripcion: entity.descripcion,
            categoria: entity.categoria,
            unidad_medida: entity.unidad_medida,
            stock_actual: entity.stock_actual,
            stock_minimo: entity.stock_minimo,
            precio_venta_cent: entity.precio_venta_cent,
            es_critico,
        }
    }
}

/// DTO Administración: Incluye valoración en micro-soles, costos y márgenes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoAdminDto {
    pub id_producto: i64,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub unidad_medida: String,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub valor_stock_micro: i64,
    pub ultimo_costo_micro: Option<i64>,
    pub precio_venta_cent: i64,
    pub es_critico: bool,
}

impl ProductoAdminDto {
    pub fn desde_entity(entity: ProductoEntity) -> Self {
        let es_critico = entity.stock_actual <= entity.stock_minimo;
        Self {
            id_producto: entity.id_producto,
            codigo: entity.codigo,
            nombre: entity.nombre,
            descripcion: entity.descripcion,
            categoria: entity.categoria,
            unidad_medida: entity.unidad_medida,
            stock_actual: entity.stock_actual,
            stock_minimo: entity.stock_minimo,
            valor_stock_micro: entity.valor_stock_micro,
            ultimo_costo_micro: entity.ultimo_costo_micro,
            precio_venta_cent: entity.precio_venta_cent,
            es_critico,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CrearProductoDto {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,     // ACEITE | FILTRO | ADITIVO | GRANEL | OTRO
    pub unidad_medida: String, // UND | ML
    pub stock_minimo: i64,
    pub precio_venta_cent: i64,
    // Stock e ingreso inicial opcional
    pub cantidad_inicial: Option<i64>,
    pub costo_unitario_inicial_micro: Option<i64>,
    pub referencia_inicial: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarProductoDto {
    pub id_producto: i64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub stock_minimo: i64,
    pub precio_venta_cent: i64,
}

#[derive(Debug, Deserialize)]
pub struct AjustarStockDto {
    pub id_producto: i64,
    pub tipo_ajuste: String,               // "ENTRADA" | "SALIDA"
    pub cantidad: i64,                     // Cantidad positiva a ajustar
    pub costo_unitario_micro: Option<i64>, // Requerido para ENTRADA
    pub motivo: String,                    // Obligatorio de 5 a 250 caracteres
}

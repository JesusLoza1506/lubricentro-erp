use crate::entities::producto::ProductoEntity;
use serde::{Deserialize, Serialize};

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
    pub activo: bool,
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
            activo: entity.activo == 1,
        }
    }
}

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
    pub activo: bool,
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
            activo: entity.activo == 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrearProductoDto {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,     // ACEITE | FILTRO | ADITIVO | GRANEL | OTRO
    pub unidad_medida: String, // UND | ML
    pub stock_minimo: i64,
    pub precio_venta_cent: i64,
    pub cantidad_inicial: Option<f64>,
    pub costo_unitario_inicial_micro: Option<i64>,
    pub referencia_inicial: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarProductoDto {
    pub id_producto: i64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub stock_minimo: i64,
    pub precio_venta_cent: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AjustarStockDto {
    pub id_producto: i64,
    pub tipo_ajuste: String,               // "ENTRADA" | "SALIDA"
    pub cantidad: i64,                     // Cantidad en UND o ML
    pub costo_unitario_micro: Option<i64>, // Requerido para ENTRADA
    pub motivo: String,                    // Obligatorio de 5 a 250 caracteres
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConteoFisicoDto {
    pub id_producto: i64,
    pub stock_fisico_contado: f64, // En unidades o litros según el caso
    pub stock_observado: Option<f64>, // Concurrencia Optimista: Stock en pantalla antes de contar
    pub costo_unitario_micro: Option<i64>, // Solo para sobrantes cuando stock_actual es 0
    pub motivo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimientoKardexDto {
    pub id_movimiento: i64,
    pub id_producto: i64,
    pub tipo_movimiento: String,
    pub unidad_medida: String,
    pub stock_anterior: i64,
    pub diferencia: i64,
    pub stock_resultante: i64,
    pub valor_anterior_micro: i64,
    pub valor_delta_micro: i64,
    pub valor_resultante_micro: i64,
    pub costo_unitario_micro: i64,
    pub motivo: Option<String>,
    pub fecha_movimiento: String,
    pub usuario_nombre: Option<String>,
    pub id_ingreso: Option<i64>,
    pub id_ot: Option<i64>,
    pub id_venta: Option<i64>,
    pub id_devolucion: Option<i64>,
    pub codigo_origen: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LineaOrigenKardexDto {
    pub nombre: String,
    pub unidad_medida: String,
    pub cantidad: i64,
    pub valor_micro: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ComprobanteOrigenKardexDto {
    pub id_comprobante: i64,
    pub numero: String,
    pub tipo_comprobante: String,
    pub estado_envio: String,
    pub total_cent: i64,
}

#[derive(Debug, Serialize)]
pub struct OrigenKardexDto {
    pub tipo: String,
    pub codigo: String,
    pub fecha: String,
    pub estado: String,
    pub documento: Option<String>,
    pub resumen: Option<String>,
    pub lineas: Vec<LineaOrigenKardexDto>,
    pub comprobantes: Vec<ComprobanteOrigenKardexDto>,
}

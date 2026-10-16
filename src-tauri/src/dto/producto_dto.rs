use crate::entities::producto::ProductoEntity;
use serde::{Deserialize, Serialize};

/// DTO Público (POS / Mecánicos / Cajeros): Oculta precio_compra por seguridad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoPublicoDto {
    pub id_producto: i64,
    pub id_categoria: i64,
    pub nombre_categoria: Option<String>,
    pub nombre: String,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_venta: i64, // En céntimos
    pub es_critico: bool,
}

impl ProductoPublicoDto {
    pub fn desde_entity(entity: ProductoEntity, nombre_categoria: Option<String>) -> Self {
        let es_critico = entity.stock_actual <= entity.stock_minimo;
        Self {
            id_producto: entity.id_producto,
            id_categoria: entity.id_categoria,
            nombre_categoria,
            nombre: entity.nombre,
            codigo_barras: entity.codigo_barras,
            unidad_medida: entity.unidad_medida,
            stock_actual: entity.stock_actual,
            stock_minimo: entity.stock_minimo,
            precio_venta: entity.precio_venta,
            es_critico,
        }
    }
}

/// DTO Interno (Administrador / ERP Inventario): Incluye precio_compra y margen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoAdminDto {
    pub id_producto: i64,
    pub id_categoria: i64,
    pub nombre_categoria: Option<String>,
    pub nombre: String,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_compra: i64, // En céntimos
    pub precio_venta: i64,  // En céntimos
    pub margen: i64,        // En céntimos (precio_venta - precio_compra)
    pub es_critico: bool,
}

impl ProductoAdminDto {
    pub fn desde_entity(entity: ProductoEntity, nombre_categoria: Option<String>) -> Self {
        let es_critico = entity.stock_actual <= entity.stock_minimo;
        let margen = entity.precio_venta - entity.precio_compra;
        Self {
            id_producto: entity.id_producto,
            id_categoria: entity.id_categoria,
            nombre_categoria,
            nombre: entity.nombre,
            codigo_barras: entity.codigo_barras,
            unidad_medida: entity.unidad_medida,
            stock_actual: entity.stock_actual,
            stock_minimo: entity.stock_minimo,
            precio_compra: entity.precio_compra,
            precio_venta: entity.precio_venta,
            margen,
            es_critico,
        }
    }
}

/// DTO para la creación de un nuevo producto
#[derive(Debug, Deserialize)]
pub struct CrearProductoDto {
    pub id_categoria: i64,
    pub nombre: String,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_compra: i64, // Recibido en céntimos
    pub precio_venta: i64,  // Recibido en céntimos
}

/// DTO para actualizar un producto existente
#[derive(Debug, Deserialize)]
pub struct ActualizarProductoDto {
    pub id_producto: i64,
    pub id_categoria: i64,
    pub nombre: String,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_minimo: f64,
    pub precio_compra: i64, // Recibido en céntimos
    pub precio_venta: i64,  // Recibido en céntimos
}

/// DTO para despacho/uso en taller o POS a granel
#[derive(Debug, Deserialize)]
pub struct DespacharProductoDto {
    pub id_producto: i64,
    pub cantidad: f64,
    pub motivo: Option<String>,
}

/// DTO para ajuste manual de stock (Exclusivo de Administrador)
#[derive(Debug, Deserialize)]
pub struct AjustarStockDto {
    pub id_producto: i64,
    pub tipo_movimiento: String, // "ENTRADA" | "AJUSTE"
    pub cantidad_nueva: f64,
    pub motivo: String, // Obligatorio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuga_precio_compra_producto() {
        let producto_entity = ProductoEntity {
            id_producto: 1,
            id_categoria: 1,
            nombre: "Aceite Mobil Super 3000".to_string(),
            codigo_barras: Some("7750001001".to_string()),
            unidad_medida: Some("LITRO".to_string()),
            stock_actual: 50.0,
            stock_minimo: 10.0,
            precio_compra: 1550, // S/ 15.50 en céntimos
            precio_venta: 2800,  // S/ 28.00 en céntimos
            activo: 1,
        };

        let dto = ProductoPublicoDto::desde_entity(producto_entity, Some("Aceites".to_string()));
        let json_salida = serde_json::to_string(&dto).expect("Fallo al serializar DTO");

        // VERIFICACIÓN DE SEGURIDAD QA (Continuidad Fase 3.5)
        assert!(!json_salida.contains("precio_compra"));
        assert!(!json_salida.contains("1550"));
        assert!(!json_salida.contains("margen"));
        assert_eq!(dto.precio_venta, 2800);
        assert_eq!(dto.nombre, "Aceite Mobil Super 3000");
        assert_eq!(dto.es_critico, false);
    }
}

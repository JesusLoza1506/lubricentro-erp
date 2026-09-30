use crate::entities::producto::ProductoEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductoPublicoDTO {
    pub id_producto: i32,
    pub id_categoria: i32,
    pub codigo_barras: Option<String>,
    pub unidad_medida: Option<String>,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub precio_venta: f64,
}

impl ProductoPublicoDTO {
    pub fn desde_entity(entity: ProductoEntity) -> Self {
        Self {
            id_producto: entity.id_producto,
            id_categoria: entity.id_categoria,
            codigo_barras: entity.codigo_barras,
            unidad_medida: entity.unidad_medida,
            stock_actual: entity.stock_actual,
            stock_minimo: entity.stock_minimo,
            precio_venta: entity.precio_venta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuga_precio_compra_producto() {
        let producto_entity = ProductoEntity {
            id_producto: 1,
            id_categoria: 1,
            codigo_barras: Some("7750001001".to_string()),
            unidad_medida: Some("Litro".to_string()),
            stock_actual: 50.0,
            stock_minimo: 10.0,
            precio_compra: 15.50, // ⚠️ DATO SENSIBLE / MARGEN INTERNO
            precio_venta: 28.00,
        };

        let dto = ProductoPublicoDTO::desde_entity(producto_entity);
        let json_salida = serde_json::to_string(&dto).expect("Fallo al serializar DTO");

        // VERIFICACIÓN DE SEGURIDAD
        assert!(!json_salida.contains("precio_compra"));
        assert!(!json_salida.contains("15.5"));
        assert_eq!(dto.precio_venta, 28.00);
    }
}

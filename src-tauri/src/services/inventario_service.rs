use crate::dto::producto_dto::ProductoPublicoDTO;
use crate::entities::producto::ProductoEntity;

pub struct InventarioService;

impl InventarioService {
    pub fn obtener_producto_publico(entity: ProductoEntity) -> ProductoPublicoDTO {
        ProductoPublicoDTO::desde_entity(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obtener_producto_publico() {
        let entity = ProductoEntity {
            id_producto: 1,
            id_categoria: 1,
            codigo_barras: Some("7750001001".to_string()),
            unidad_medida: Some("Litro".to_string()),
            stock_actual: 50.0,
            stock_minimo: 10.0,
            precio_compra: 15.5,
            precio_venta: 28.0,
        };

        let dto = InventarioService::obtener_producto_publico(entity);
        assert_eq!(dto.precio_venta, 28.0);
    }
}

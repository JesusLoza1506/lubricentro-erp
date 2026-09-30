use crate::dto::producto_dto::ProductoPublicoDTO;
use crate::entities::producto::ProductoEntity;

pub struct InventarioService;

impl InventarioService {
    pub fn obtener_producto_publico(entity: ProductoEntity) -> ProductoPublicoDTO {
        ProductoPublicoDTO::desde_entity(entity)
    }
}

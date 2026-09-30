use crate::dto::producto_dto::ProductoPublicoDTO;

#[tauri::command]
pub async fn listar_productos_publicos_cmd() -> Result<Vec<ProductoPublicoDTO>, String> {
    Ok(vec![ProductoPublicoDTO {
        id_producto: 1,
        id_categoria: 1,
        codigo_barras: Some("7750001001".to_string()),
        unidad_medida: Some("Litro".to_string()),
        stock_actual: 45.5,
        stock_minimo: 10.0,
        precio_venta: 28.0,
    }])
}

use crate::dto::producto_dto::{ProductoAdminDTO, ProductoPublicoDTO};
use crate::entities::producto::ProductoEntity;
use rusqlite::Connection;

pub struct InventarioService;

impl InventarioService {
    /// Redondeo a 2 decimales para precisión monetaria
    pub fn redondear_moneda(val: f64) -> f64 {
        (val * 100.0).round() / 100.0
    }

    /// Valida las reglas de negocio del producto
    pub fn validar_datos_producto(
        conn: &Connection,
        id_categoria: i32,
        nombre: &str,
        stock_actual: f64,
        stock_minimo: f64,
        precio_compra: f64,
        precio_venta: f64,
    ) -> Result<(), String> {
        let nombre_trim = nombre.trim();
        if nombre_trim.is_empty() {
            return Err("El nombre del producto no puede estar vacío.".to_string());
        }

        if stock_actual.is_nan()
            || stock_actual.is_infinite()
            || stock_minimo.is_nan()
            || stock_minimo.is_infinite()
            || precio_compra.is_nan()
            || precio_compra.is_infinite()
            || precio_venta.is_nan()
            || precio_venta.is_infinite()
        {
            return Err("Los valores numéricos ingresados no son válidos.".to_string());
        }

        if stock_actual < 0.0 {
            return Err("El stock actual no puede ser negativo.".to_string());
        }
        if stock_minimo < 0.0 {
            return Err("El stock mínimo no puede ser negativo.".to_string());
        }
        if precio_compra < 0.0 {
            return Err("El precio de compra no puede ser negativo.".to_string());
        }
        if precio_venta < 0.0 {
            return Err("El precio de venta no puede ser negativo.".to_string());
        }

        if precio_venta < precio_compra {
            return Err(format!(
                "El precio de venta (S/ {:.2}) no puede ser menor al precio de compra (S/ {:.2}).",
                precio_venta, precio_compra
            ));
        }

        let existe_cat: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM categorias WHERE id_categoria = ?1",
                [id_categoria],
                |row| row.get(0),
            )
            .map_err(|e| format!("Error al verificar categoría: {}", e))?;

        if existe_cat == 0 {
            return Err(format!(
                "La categoría seleccionada (ID {}) no existe.",
                id_categoria
            ));
        }

        Ok(())
    }

    pub fn obtener_producto_publico(
        entity: ProductoEntity,
        nombre_categoria: Option<String>,
    ) -> ProductoPublicoDTO {
        ProductoPublicoDTO::desde_entity(entity, nombre_categoria)
    }

    pub fn obtener_producto_admin(
        entity: ProductoEntity,
        nombre_categoria: Option<String>,
    ) -> ProductoAdminDTO {
        ProductoAdminDTO::desde_entity(entity, nombre_categoria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE categorias (id_categoria INTEGER PRIMARY KEY, nombre_categoria TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO categorias (id_categoria, nombre_categoria) VALUES (1, 'Aceites')",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_obtener_producto_publico() {
        let entity = ProductoEntity {
            id_producto: 1,
            id_categoria: 1,
            nombre: "Aceite Mobil Super 3000".to_string(),
            codigo_barras: Some("7750001001".to_string()),
            unidad_medida: Some("Litro".to_string()),
            stock_actual: 50.0,
            stock_minimo: 10.0,
            precio_compra: 15.5,
            precio_venta: 28.0,
        };

        let dto = InventarioService::obtener_producto_publico(entity, Some("Aceites".to_string()));
        assert_eq!(dto.precio_venta, 28.0);
        assert_eq!(dto.nombre, "Aceite Mobil Super 3000");
    }

    #[test]
    fn test_validaciones_inventario_service() {
        let conn = setup_test_db();

        // Validar nombre vacío
        let res = InventarioService::validar_datos_producto(&conn, 1, "   ", 10.0, 2.0, 10.0, 15.0);
        assert!(res.is_err());

        // Validar precio venta menor a compra
        let res =
            InventarioService::validar_datos_producto(&conn, 1, "Mobil", 10.0, 2.0, 20.0, 15.0);
        assert!(res.is_err());

        // Validar categoría inexistente
        let res =
            InventarioService::validar_datos_producto(&conn, 99, "Mobil", 10.0, 2.0, 10.0, 15.0);
        assert!(res.is_err());

        // Validar caso exitoso
        let res =
            InventarioService::validar_datos_producto(&conn, 1, "Mobil", 10.0, 2.0, 10.0, 15.0);
        assert!(res.is_ok());
    }

    #[test]
    fn test_redondear_moneda() {
        assert_eq!(InventarioService::redondear_moneda(15.556), 15.56);
        assert_eq!(InventarioService::redondear_moneda(15.554), 15.55);
    }
}

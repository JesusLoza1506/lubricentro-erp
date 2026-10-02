use crate::dto::producto_dto::{
    ActualizarProductoDto, CrearProductoDto, ProductoAdminDto, ProductoPublicoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::entities::producto::ProductoEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct InventarioService;

impl InventarioService {
    pub fn validar_datos_producto(
        conn: &Connection,
        id_categoria: i64,
        nombre: &str,
        stock_actual: f64,
        stock_minimo: f64,
        precio_compra: i64,
        precio_venta: i64,
    ) -> Result<(), AppError> {
        let nombre_trim = nombre.trim();
        if nombre_trim.is_empty() {
            return Err(AppError::Validation(
                "El nombre del producto no puede estar vacío.".to_string(),
            ));
        }

        if stock_actual < 0.0 {
            return Err(AppError::Validation(
                "El stock actual no puede ser negativo.".to_string(),
            ));
        }
        if stock_minimo < 0.0 {
            return Err(AppError::Validation(
                "El stock mínimo no puede ser negativo.".to_string(),
            ));
        }
        if precio_compra < 0 {
            return Err(AppError::Validation(
                "El precio de compra no puede ser negativo.".to_string(),
            ));
        }
        if precio_venta < 0 {
            return Err(AppError::Validation(
                "El precio de venta no puede ser negativo.".to_string(),
            ));
        }

        if precio_venta < precio_compra {
            return Err(AppError::Validation(format!(
                "El precio de venta (S/ {:.2}) no puede ser menor al precio de compra (S/ {:.2}).",
                precio_venta as f64 / 100.0,
                precio_compra as f64 / 100.0
            )));
        }

        let existe_cat: i32 = conn.query_row(
            "SELECT COUNT(*) FROM categorias WHERE id_categoria = ?1 AND activo = 1",
            [id_categoria],
            |row| row.get(0),
        )?;

        if existe_cat == 0 {
            return Err(AppError::Validation(format!(
                "La categoría seleccionada (ID {}) no existe o está inactiva.",
                id_categoria
            )));
        }

        Ok(())
    }

    pub fn registrar_producto(
        conn: &Connection,
        req: CrearProductoDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        Self::validar_datos_producto(
            conn,
            req.id_categoria,
            &req.nombre,
            req.stock_actual,
            req.stock_minimo,
            req.precio_compra,
            req.precio_venta,
        )?;

        let unidad = req.unidad_medida.unwrap_or_else(|| "UNIDAD".to_string());

        conn.execute(
            "INSERT INTO productos (id_categoria, descripcion, codigo_barras, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1)",
            params![
                req.id_categoria,
                req.nombre.trim(),
                req.codigo_barras,
                unidad,
                req.stock_actual,
                req.stock_minimo,
                req.precio_compra,
                req.precio_venta,
            ],
        )?;

        let id_producto = conn.last_insert_rowid();

        if req.stock_actual > 0.0 {
            conn.execute(
                "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia)
                 VALUES (?1, ?2, 'INGRESO_COMPRA', 0.0, ?3, ?3, 'Inventario Inicial')",
                params![id_producto, id_usuario, req.stock_actual],
            )?;
        }

        Ok(id_producto)
    }

    pub fn actualizar_producto(
        conn: &Connection,
        req: ActualizarProductoDto,
    ) -> Result<(), AppError> {
        let stock_actual: f64 = conn
            .query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        Self::validar_datos_producto(
            conn,
            req.id_categoria,
            &req.nombre,
            stock_actual,
            req.stock_minimo,
            req.precio_compra,
            req.precio_venta,
        )?;

        let unidad = req.unidad_medida.unwrap_or_else(|| "UNIDAD".to_string());

        conn.execute(
            "UPDATE productos 
             SET id_categoria = ?1, descripcion = ?2, codigo_barras = ?3, unidad_medida = ?4, stock_minimo = ?5, precio_compra = ?6, precio_venta = ?7
             WHERE id_producto = ?8 AND activo = 1",
            params![
                req.id_categoria,
                req.nombre.trim(),
                req.codigo_barras,
                unidad,
                req.stock_minimo,
                req.precio_compra,
                req.precio_venta,
                req.id_producto
            ],
        )?;

        Ok(())
    }

    pub fn eliminar_producto(conn: &Connection, id_producto: i64) -> Result<(), AppError> {
        let rows_affected = conn.execute(
            "UPDATE productos SET activo = 0 WHERE id_producto = ?1 AND activo = 1",
            [id_producto],
        )?;

        if rows_affected == 0 {
            return Err(AppError::NotFound(
                "Producto no encontrado o ya eliminado.".to_string(),
            ));
        }

        Ok(())
    }

    pub fn listar_productos_publicos(
        conn: &Connection,
    ) -> Result<Vec<ProductoPublicoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.id_producto, p.id_categoria, c.nombre_categoria, p.descripcion, p.codigo_barras, p.unidad_medida, p.stock_actual, p.stock_minimo, p.precio_compra, p.precio_venta, p.activo
             FROM productos p
             LEFT JOIN categorias c ON p.id_categoria = c.id_categoria
             WHERE p.activo = 1"
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                precio_compra: row.get(8)?,
                precio_venta: row.get(9)?,
                activo: row.get(10)?,
            };
            let cat_nombre: Option<String> = row.get(2)?;
            Ok(ProductoPublicoDto::desde_entity(entity, cat_nombre))
        })?;

        let mut productos = Vec::new();
        for item in iter {
            productos.push(item?);
        }

        Ok(productos)
    }

    pub fn listar_productos_admin(conn: &Connection) -> Result<Vec<ProductoAdminDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.id_producto, p.id_categoria, c.nombre_categoria, p.descripcion, p.codigo_barras, p.unidad_medida, p.stock_actual, p.stock_minimo, p.precio_compra, p.precio_venta, p.activo
             FROM productos p
             LEFT JOIN categorias c ON p.id_categoria = c.id_categoria
             WHERE p.activo = 1"
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                precio_compra: row.get(8)?,
                precio_venta: row.get(9)?,
                activo: row.get(10)?,
            };
            let cat_nombre: Option<String> = row.get(2)?;
            Ok(ProductoAdminDto::desde_entity(entity, cat_nombre))
        })?;

        let mut productos = Vec::new();
        for item in iter {
            productos.push(item?);
        }

        Ok(productos)
    }

    pub fn listar_categorias(conn: &Connection) -> Result<Vec<CategoriaEntity>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_categoria, nombre_categoria, activo FROM categorias WHERE activo = 1",
        )?;

        let iter = stmt.query_map([], |row| {
            Ok(CategoriaEntity {
                id_categoria: row.get(0)?,
                nombre: row.get(1)?,
                activo: row.get(2)?,
            })
        })?;

        let mut categorias = Vec::new();
        for item in iter {
            categorias.push(item?);
        }

        Ok(categorias)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_validaciones_y_centimos_inventario_service() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        // Insertar categoría previa para asegurar ID válido dinámicamente
        conn.execute(
            "INSERT INTO categorias (nombre_categoria, activo) VALUES ('Lubricantes Test', 1)",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        // Validar nombre vacío
        let res =
            InventarioService::validar_datos_producto(&conn, cat_id, "   ", 10.0, 2.0, 1000, 1500);
        assert!(res.is_err());

        // Validar precio venta menor a compra en céntimos
        let res = InventarioService::validar_datos_producto(
            &conn, cat_id, "Mobil", 10.0, 2.0, 2000, 1500,
        );
        assert!(res.is_err());

        // Validar categoría inexistente
        let res =
            InventarioService::validar_datos_producto(&conn, 999, "Mobil", 10.0, 2.0, 1000, 1500);
        assert!(res.is_err());

        // Validar caso exitoso
        let res = InventarioService::validar_datos_producto(
            &conn,
            cat_id,
            "Mobil 20W50",
            10.0,
            2.0,
            1000,
            1500,
        );
        assert!(res.is_ok());
    }
}

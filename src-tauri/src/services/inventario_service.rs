use crate::dto::producto_dto::{
    ActualizarProductoDto, AjustarStockDto, CrearProductoDto, DespacharProductoDto,
    ProductoAdminDto, ProductoPublicoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::entities::producto::ProductoEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct InventarioService;

const UNIDADES_VALIDAS: [&str; 2] = ["UND", "ML"];

impl InventarioService {
    pub fn validar_datos_producto(
        conn: &Connection,
        _id_categoria: i64,
        nombre: &str,
        codigo_barras: Option<&str>,
        unidad_medida: Option<&str>,
        stock_actual: f64,
        stock_minimo: f64,
        precio_compra: i64,
        precio_venta: i64,
        id_producto_excluir: Option<i64>,
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

        if let Some(u) = unidad_medida {
            let u_upper = u.trim().to_uppercase();
            if !u_upper.is_empty() && !UNIDADES_VALIDAS.contains(&u_upper.as_str()) {
                return Err(AppError::Validation(format!(
                    "Unidad de medida inválida ('{}'). Solo se permiten: UND, ML.",
                    u
                )));
            }
        }

        if let Some(cb) = codigo_barras {
            let cb_trim = cb.trim();
            if !cb_trim.is_empty() {
                let mut query =
                    "SELECT COUNT(*) FROM productos WHERE codigo = ?1 AND activo = 1".to_string();
                let mut params_vec: Vec<rusqlite::types::Value> = vec![cb_trim.to_string().into()];

                if let Some(id_excluir) = id_producto_excluir {
                    query.push_str(" AND id_producto != ?2");
                    params_vec.push(id_excluir.into());
                }

                let count: i32 =
                    conn.query_row(&query, rusqlite::params_from_iter(params_vec), |row| {
                        row.get(0)
                    })?;
                if count > 0 {
                    return Err(AppError::Validation(format!(
                        "El código de barras '{}' ya está registrado en otro producto.",
                        cb_trim
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn registrar_producto(
        conn: &Connection,
        req: CrearProductoDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        let unidad = req
            .unidad_medida
            .as_deref()
            .map(|u| u.trim().to_uppercase())
            .filter(|u| !u.is_empty())
            .unwrap_or_else(|| "UND".to_string());

        Self::validar_datos_producto(
            conn,
            req.id_categoria,
            &req.nombre,
            req.codigo_barras.as_deref(),
            Some(&unidad),
            req.stock_actual,
            req.stock_minimo,
            req.precio_compra,
            req.precio_venta,
            None,
        )?;

        let categoria_db = if unidad == "ML" { "GRANEL" } else { "ACEITE" };

        let cb = req
            .codigo_barras
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "PROD-{}",
                    uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
                )
            });

        conn.execute(
            "INSERT INTO productos (codigo, nombre, categoria, unidad_medida, stock_actual, stock_minimo, precio_venta_cent, activo)
             VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6, 1)",
            params![
                cb,
                req.nombre.trim(),
                categoria_db,
                unidad,
                req.stock_minimo as i64,
                req.precio_venta,
            ],
        )?;

        let id_producto = conn.last_insert_rowid();

        if req.stock_actual > 0.0 {
            Self::ajustar_stock(
                conn,
                AjustarStockDto {
                    id_producto,
                    tipo_movimiento: "AJUSTE".to_string(),
                    cantidad_nueva: req.stock_actual,
                    motivo: "Inventario Inicial".to_string(),
                },
                id_usuario,
            )?;
        }

        Ok(id_producto)
    }

    pub fn actualizar_producto(
        conn: &Connection,
        req: ActualizarProductoDto,
    ) -> Result<(), AppError> {
        let stock_actual: i64 = conn
            .query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let unidad = req
            .unidad_medida
            .as_deref()
            .map(|u| u.trim().to_uppercase())
            .filter(|u| !u.is_empty())
            .unwrap_or_else(|| "UND".to_string());

        Self::validar_datos_producto(
            conn,
            req.id_categoria,
            &req.nombre,
            req.codigo_barras.as_deref(),
            Some(&unidad),
            stock_actual as f64,
            req.stock_minimo,
            req.precio_compra,
            req.precio_venta,
            Some(req.id_producto),
        )?;

        let cb = req
            .codigo_barras
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        conn.execute(
            "UPDATE productos 
             SET nombre = ?1, codigo = COALESCE(?2, codigo), unidad_medida = ?3, stock_minimo = ?4, precio_venta_cent = ?5
             WHERE id_producto = ?6 AND activo = 1",
            params![
                req.nombre.trim(),
                cb,
                unidad,
                req.stock_minimo as i64,
                req.precio_venta,
                req.id_producto
            ],
        )?;

        Ok(())
    }

    pub fn eliminar_producto(conn: &Connection, id_producto: i64) -> Result<(), AppError> {
        let existe: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE id_producto = ?1 AND activo = 1",
                [id_producto],
                |r| r.get(0),
            )
            .unwrap_or(0);

        if existe == 0 {
            return Err(AppError::NotFound(
                "Producto no encontrado o ya eliminado.".to_string(),
            ));
        }

        let movs_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM movimientos_inventario WHERE id_producto = ?1",
                [id_producto],
                |r| r.get(0),
            )
            .unwrap_or(0);

        if movs_count > 0 {
            return Err(AppError::Validation(
                "No se puede eliminar el producto porque tiene movimientos de inventario asociados."
                    .to_string(),
            ));
        }

        conn.execute(
            "UPDATE productos SET activo = 0 WHERE id_producto = ?1",
            [id_producto],
        )?;

        Ok(())
    }

    pub fn despachar_producto_granel(
        conn: &Connection,
        req: DespacharProductoDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        if req.cantidad <= 0.0 {
            return Err(AppError::Validation(
                "La cantidad a despachar debe ser mayor a 0.".to_string(),
            ));
        }

        let (stock_actual, desc, unidad_medida): (i64, String, String) = conn
            .query_row(
                "SELECT stock_actual, nombre, unidad_medida FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let req_cant_i64 = req.cantidad as i64;

        if req_cant_i64 > stock_actual {
            return Err(AppError::Validation(format!(
                "Stock insuficiente para '{}'. Disponible: {}, Solicitado: {}",
                desc, stock_actual, req_cant_i64
            )));
        }

        let id_op_kardex = uuid::Uuid::new_v4().to_string();
        let motivo_despacho = req
            .motivo
            .as_deref()
            .map(|m| m.trim())
            .filter(|m| !m.is_empty())
            .unwrap_or("Despacho a Granel");

        conn.execute(
            "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro, motivo)
             VALUES (?1, ?2, ?3, 'AJUSTE', ?4, ?5, ?6, ?5 + ?6, 0, 0, 0, 0, ?7)",
            params![
                id_op_kardex,
                req.id_producto,
                id_usuario,
                unidad_medida,
                stock_actual,
                -req_cant_i64,
                motivo_despacho
            ],
        )?;

        Ok(())
    }

    pub fn ajustar_stock(
        conn: &Connection,
        req: AjustarStockDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let motivo_trim = req.motivo.trim();
        if motivo_trim.is_empty() {
            return Err(AppError::Validation(
                "El motivo del ajuste manual de stock es obligatorio.".to_string(),
            ));
        }

        if req.cantidad_nueva < 0.0 {
            return Err(AppError::Validation(
                "La nueva cantidad de stock no puede ser negativa.".to_string(),
            ));
        }

        let (stock_anterior, unidad_medida): (i64, String) = conn
            .query_row(
                "SELECT stock_actual, unidad_medida FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let cant_nueva_i64 = req.cantidad_nueva as i64;
        let diferencia = cant_nueva_i64 - stock_anterior;

        let id_op_kardex = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro, motivo)
             VALUES (?1, ?2, ?3, 'AJUSTE', ?4, ?5, ?6, ?7, 0, 0, 0, 0, ?8)",
            params![
                id_op_kardex,
                req.id_producto,
                id_usuario,
                unidad_medida,
                stock_anterior,
                diferencia,
                cant_nueva_i64,
                motivo_trim
            ],
        )?;

        Ok(())
    }

    pub fn listar_productos_publicos(
        conn: &Connection,
    ) -> Result<Vec<ProductoPublicoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.id_producto, 1 AS id_categoria, p.categoria AS nombre_categoria, p.nombre, p.codigo, p.unidad_medida, p.stock_actual, p.stock_minimo, 0 AS precio_compra, p.precio_venta_cent, p.activo
             FROM productos p
             WHERE p.activo = 1
             ORDER BY p.nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get::<_, i64>(6)? as f64,
                stock_minimo: row.get::<_, i64>(7)? as f64,
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
            "SELECT p.id_producto, 1 AS id_categoria, p.categoria AS nombre_categoria, p.nombre, p.codigo, p.unidad_medida, p.stock_actual, p.stock_minimo, 0 AS precio_compra, p.precio_venta_cent, p.activo
             FROM productos p
             WHERE p.activo = 1
             ORDER BY p.nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get::<_, i64>(6)? as f64,
                stock_minimo: row.get::<_, i64>(7)? as f64,
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

    pub fn listar_productos_criticos_publicos(
        conn: &Connection,
    ) -> Result<Vec<ProductoPublicoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.id_producto, 1 AS id_categoria, p.categoria AS nombre_categoria, p.nombre, p.codigo, p.unidad_medida, p.stock_actual, p.stock_minimo, 0 AS precio_compra, p.precio_venta_cent, p.activo
             FROM productos p
             WHERE p.activo = 1 AND p.stock_actual <= p.stock_minimo
             ORDER BY (p.stock_actual - p.stock_minimo) ASC, p.nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get::<_, i64>(6)? as f64,
                stock_minimo: row.get::<_, i64>(7)? as f64,
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

    pub fn listar_productos_criticos_admin(
        conn: &Connection,
    ) -> Result<Vec<ProductoAdminDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.id_producto, 1 AS id_categoria, p.categoria AS nombre_categoria, p.nombre, p.codigo, p.unidad_medida, p.stock_actual, p.stock_minimo, 0 AS precio_compra, p.precio_venta_cent, p.activo
             FROM productos p
             WHERE p.activo = 1 AND p.stock_actual <= p.stock_minimo
             ORDER BY (p.stock_actual - p.stock_minimo) ASC, p.nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre: row.get(3)?,
                codigo_barras: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get::<_, i64>(6)? as f64,
                stock_minimo: row.get::<_, i64>(7)? as f64,
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

    pub fn listar_categorias(_conn: &Connection) -> Result<Vec<CategoriaEntity>, AppError> {
        Ok(vec![
            CategoriaEntity {
                id_categoria: 1,
                nombre: "ACEITE".to_string(),
                activo: 1,
            },
            CategoriaEntity {
                id_categoria: 2,
                nombre: "GRANEL".to_string(),
                activo: 1,
            },
        ])
    }
}

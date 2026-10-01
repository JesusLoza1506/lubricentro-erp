use crate::dto::producto_dto::ProductoAdminDTO;
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;

pub type DbState = Mutex<Connection>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearProductoPayload {
    #[serde(alias = "id_categoria")]
    pub id_categoria: i32,
    pub nombre: String,
    #[serde(alias = "codigo_barras")]
    pub codigo_barras: Option<String>,
    #[serde(alias = "unidad_medida")]
    pub unidad_medida: Option<String>,
    #[serde(alias = "stock_actual")]
    pub stock_actual: f64,
    #[serde(alias = "stock_minimo")]
    pub stock_minimo: f64,
    #[serde(alias = "precio_compra")]
    pub precio_compra: f64,
    #[serde(alias = "precio_venta")]
    pub precio_venta: f64,
    #[serde(alias = "id_usuario")]
    pub id_usuario: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditarProductoPayload {
    #[serde(alias = "id_producto")]
    pub id_producto: i32,
    #[serde(alias = "id_categoria")]
    pub id_categoria: i32,
    pub nombre: String,
    #[serde(alias = "codigo_barras")]
    pub codigo_barras: Option<String>,
    #[serde(alias = "unidad_medida")]
    pub unidad_medida: Option<String>,
    #[serde(alias = "stock_actual")]
    pub stock_actual: f64,
    #[serde(alias = "stock_minimo")]
    pub stock_minimo: f64,
    #[serde(alias = "precio_compra")]
    pub precio_compra: f64,
    #[serde(alias = "precio_venta")]
    pub precio_venta: f64,
    #[serde(alias = "id_usuario")]
    pub id_usuario: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriaDTO {
    pub id_categoria: i32,
    pub nombre_categoria: String,
}

fn redondear_moneda(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

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

#[tauri::command]
pub async fn listar_productos_cmd(db: State<'_, DbState>) -> Result<Vec<ProductoAdminDTO>, String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT 
                p.id_producto, 
                p.id_categoria, 
                COALESCE(c.nombre_categoria, 'Sin Categoría') as nombre_categoria,
                p.nombre, 
                p.codigo_barras, 
                p.unidad_medida, 
                p.stock_actual, 
                p.stock_minimo, 
                p.precio_compra, 
                p.precio_venta 
             FROM productos p 
             LEFT JOIN categorias c ON p.id_categoria = c.id_categoria
             ORDER BY p.nombre ASC",
        )
        .map_err(|e| format!("Error preparando consulta de productos: {}", e))?;

    let productos = stmt
        .query_map([], |row| {
            Ok(ProductoAdminDTO {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre_categoria: row.get(2)?,
                nombre: row.get(3)?,
                codigo_barras: row.get::<_, Option<String>>(4)?,
                unidad_medida: row.get::<_, Option<String>>(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                precio_compra: row.get(8)?,
                precio_venta: row.get(9)?,
            })
        })
        .map_err(|e| format!("Error mapeando productos: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error recopilando productos: {}", e))?;

    Ok(productos)
}

#[tauri::command]
pub async fn listar_productos_criticos_cmd(
    db: State<'_, DbState>,
) -> Result<Vec<ProductoAdminDTO>, String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT 
                p.id_producto, 
                p.id_categoria, 
                COALESCE(c.nombre_categoria, 'Sin Categoría') as nombre_categoria,
                p.nombre, 
                p.codigo_barras, 
                p.unidad_medida, 
                p.stock_actual, 
                p.stock_minimo, 
                p.precio_compra, 
                p.precio_venta 
             FROM productos p 
             LEFT JOIN categorias c ON p.id_categoria = c.id_categoria
             WHERE p.stock_actual <= p.stock_minimo
             ORDER BY p.stock_actual ASC",
        )
        .map_err(|e| format!("Error preparando consulta de productos críticos: {}", e))?;

    let productos = stmt
        .query_map([], |row| {
            Ok(ProductoAdminDTO {
                id_producto: row.get(0)?,
                id_categoria: row.get(1)?,
                nombre_categoria: row.get(2)?,
                nombre: row.get(3)?,
                codigo_barras: row.get::<_, Option<String>>(4)?,
                unidad_medida: row.get::<_, Option<String>>(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                precio_compra: row.get(8)?,
                precio_venta: row.get(9)?,
            })
        })
        .map_err(|e| format!("Error mapeando productos críticos: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error recopilando productos críticos: {}", e))?;

    Ok(productos)
}

#[tauri::command]
pub async fn registrar_producto_cmd(
    db: State<'_, DbState>,
    payload: CrearProductoPayload,
) -> Result<(), String> {
    tracing::info!("📝 [REGISTRAR] Creando producto: {:?}", payload.nombre);
    let mut conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let p_compra = redondear_moneda(payload.precio_compra);
    let p_venta = redondear_moneda(payload.precio_venta);

    validar_datos_producto(
        &conn,
        payload.id_categoria,
        &payload.nombre,
        payload.stock_actual,
        payload.stock_minimo,
        p_compra,
        p_venta,
    )?;

    let nombre_trim = payload.nombre.trim();

    let count_nombre: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM productos WHERE LOWER(TRIM(nombre)) = LOWER(TRIM(?1))",
            [nombre_trim],
            |row| row.get(0),
        )
        .map_err(|e| format!("Error comprobando duplicado de nombre: {}", e))?;

    if count_nombre > 0 {
        return Err(format!(
            "Ya existe un producto registrado con el nombre '{}'",
            nombre_trim
        ));
    }

    if let Some(ref codigo) = payload.codigo_barras {
        let cod_clean = codigo.trim();
        if !cod_clean.is_empty() {
            let count_codigo: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM productos WHERE codigo_barras = ?1",
                    [cod_clean],
                    |row| row.get(0),
                )
                .map_err(|e| format!("Error comprobando duplicado de código de barras: {}", e))?;

            if count_codigo > 0 {
                return Err(format!(
                    "El código de barras '{}' ya pertenece a otro producto",
                    cod_clean
                ));
            }
        }
    }

    let tx = conn
        .transaction()
        .map_err(|e| format!("Error al iniciar transacción: {}", e))?;

    tx.execute(
        "INSERT INTO productos (id_categoria, nombre, codigo_barras, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            payload.id_categoria,
            nombre_trim,
            payload.codigo_barras.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            payload.unidad_medida.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            payload.stock_actual,
            payload.stock_minimo,
            p_compra,
            p_venta,
        ],
    )
    .map_err(|e| format!("Error SQLite al insertar producto: {}", e))?;

    let new_id = tx.last_insert_rowid();

    // 1. Registro en Movimientos de Inventario
    tx.execute(
        "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, cantidad_anterior, cantidad_nueva, diferencia, motivo)
         VALUES (?1, ?2, 'INICIAL', 0.0, ?3, ?3, 'Alta de producto en catálogo')",
        rusqlite::params![new_id, payload.id_usuario, payload.stock_actual],
    )
    .map_err(|e| format!("Error al registrar movimiento de inventario: {}", e))?;

    // 2. Registro de Auditoría
    let detalle = format!(
        "ALTA_PRODUCTO | ID: {} | Nombre: '{}' | Stock Inicial: {} | P.Compra: S/{:.2} | P.Venta: S/{:.2}",
        new_id, nombre_trim, payload.stock_actual, p_compra, p_venta
    );

    tx.execute(
        "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![payload.id_usuario, "productos", "CREAR", detalle],
    )
    .map_err(|e| format!("Error al registrar auditoría: {}", e))?;

    tx.commit()
        .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn actualizar_producto_cmd(
    db: State<'_, DbState>,
    payload: EditarProductoPayload,
) -> Result<(), String> {
    tracing::info!(
        "✏️ [ACTUALIZAR] Editando id_producto: {}",
        payload.id_producto
    );
    let mut conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let p_compra = redondear_moneda(payload.precio_compra);
    let p_venta = redondear_moneda(payload.precio_venta);

    validar_datos_producto(
        &conn,
        payload.id_categoria,
        &payload.nombre,
        payload.stock_actual,
        payload.stock_minimo,
        p_compra,
        p_venta,
    )?;

    let (stock_anterior, p_venta_anterior): (f64, f64) = conn
        .query_row(
            "SELECT stock_actual, precio_venta FROM productos WHERE id_producto = ?1",
            [payload.id_producto],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| {
            format!(
                "No se encontró ningún producto con el ID {}",
                payload.id_producto
            )
        })?;

    let nombre_trim = payload.nombre.trim();

    let count_nombre: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM productos WHERE LOWER(TRIM(nombre)) = LOWER(TRIM(?1)) AND id_producto != ?2",
            rusqlite::params![nombre_trim, payload.id_producto],
            |row| row.get(0),
        )
        .map_err(|e| format!("Error comprobando duplicado de nombre: {}", e))?;

    if count_nombre > 0 {
        return Err(format!(
            "Ya existe otro producto registrado con el nombre '{}'",
            nombre_trim
        ));
    }

    if let Some(ref codigo) = payload.codigo_barras {
        let cod_clean = codigo.trim();
        if !cod_clean.is_empty() {
            let count_codigo: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM productos WHERE codigo_barras = ?1 AND id_producto != ?2",
                    rusqlite::params![cod_clean, payload.id_producto],
                    |row| row.get(0),
                )
                .map_err(|e| format!("Error comprobando duplicado de código de barras: {}", e))?;

            if count_codigo > 0 {
                return Err(format!(
                    "El código de barras '{}' ya pertenece a otro producto",
                    cod_clean
                ));
            }
        }
    }

    let tx = conn
        .transaction()
        .map_err(|e| format!("Error al iniciar transacción: {}", e))?;

    let filas = tx
        .execute(
            "UPDATE productos 
         SET id_categoria = ?1, nombre = ?2, codigo_barras = ?3, unidad_medida = ?4, 
             stock_actual = ?5, stock_minimo = ?6, precio_compra = ?7, precio_venta = ?8
         WHERE id_producto = ?9",
            rusqlite::params![
                payload.id_categoria,
                nombre_trim,
                payload
                    .codigo_barras
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty()),
                payload
                    .unidad_medida
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty()),
                payload.stock_actual,
                payload.stock_minimo,
                p_compra,
                p_venta,
                payload.id_producto,
            ],
        )
        .map_err(|e| format!("Error SQLite al actualizar producto: {}", e))?;

    if filas == 0 {
        return Err(format!(
            "No se encontró ningún producto con el ID {}",
            payload.id_producto
        ));
    }

    let diff_stock = payload.stock_actual - stock_anterior;

    // 1. Registro en Movimientos de Inventario si cambió el stock
    if diff_stock.abs() > 0.0001 {
        let tipo = if diff_stock > 0.0 {
            "ENTRADA"
        } else {
            "SALIDA"
        };
        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, cantidad_anterior, cantidad_nueva, diferencia, motivo)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'Ajuste manual de stock desde edición de producto')",
            rusqlite::params![payload.id_producto, payload.id_usuario, tipo, stock_anterior, payload.stock_actual, diff_stock],
        )
        .map_err(|e| format!("Error al registrar movimiento de inventario: {}", e))?;
    }

    // 2. Auditoría
    let detalle = format!(
        "MODIFICACION_PRODUCTO | ID: {} | Nombre: '{}' | Stock: {} -> {} | P.Venta: S/{:.2} -> S/{:.2}",
        payload.id_producto, nombre_trim, stock_anterior, payload.stock_actual, p_venta_anterior, p_venta
    );

    tx.execute(
        "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![payload.id_usuario, "productos", "EDITAR", detalle],
    )
    .map_err(|e| format!("Error al registrar auditoría: {}", e))?;

    tx.commit()
        .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn eliminar_producto_cmd(
    db: State<'_, DbState>,
    id_producto: i32,
    id_usuario: Option<i32>,
) -> Result<(), String> {
    tracing::info!(
        "🗑 [ELIMINAR] Solicitada eliminación para id_producto: {}",
        id_producto
    );
    let mut conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let (nombre_producto, stock_ultimo): (String, f64) = conn
        .query_row(
            "SELECT nombre, stock_actual FROM productos WHERE id_producto = ?1",
            [id_producto],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or_else(|_| ("Producto Desconocido".to_string(), 0.0));

    let tx = conn
        .transaction()
        .map_err(|e| format!("Error al iniciar transacción: {}", e))?;

    // 1. Registro de Salida en Movimientos antes de eliminar
    tx.execute(
        "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, cantidad_anterior, cantidad_nueva, diferencia, motivo)
         VALUES (?1, ?2, 'ELIMINACION', ?3, 0.0, -?3, 'Eliminación de producto del catálogo')",
        rusqlite::params![id_producto, id_usuario, stock_ultimo],
    )
    .map_err(|e| format!("Error al registrar movimiento de inventario: {}", e))?;

    let filas = tx
        .execute(
            "DELETE FROM productos WHERE id_producto = ?1",
            rusqlite::params![id_producto],
        )
        .map_err(|e| {
            format!("No se pudo eliminar el producto (puede estar asociado a registros de ventas u órdenes): {}", e)
        })?;

    if filas == 0 {
        return Err(format!("No se encontró el producto con ID {}", id_producto));
    }

    let detalle = format!(
        "BAJA_PRODUCTO | ID: {} | Nombre: '{}'",
        id_producto, nombre_producto
    );

    // 2. Registro en Auditoría con id_usuario
    tx.execute(
        "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id_usuario, "productos", "ELIMINAR", detalle],
    )
    .map_err(|e| format!("Error al registrar auditoría: {}", e))?;

    tx.commit()
        .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

    tracing::info!(
        "✅ [ELIMINAR] Producto ID {} eliminado correctamente.",
        id_producto
    );
    Ok(())
}

#[tauri::command]
pub async fn listar_categorias_cmd(db: State<'_, DbState>) -> Result<Vec<CategoriaDTO>, String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Error al bloquear DB: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT id_categoria, nombre_categoria FROM categorias ORDER BY nombre_categoria ASC",
        )
        .map_err(|e| format!("Error preparando consulta de categorías: {}", e))?;

    let categorias = stmt
        .query_map([], |row| {
            Ok(CategoriaDTO {
                id_categoria: row.get(0)?,
                nombre_categoria: row.get(1)?,
            })
        })
        .map_err(|e| format!("Error mapeando categorías: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error recopilando categorías: {}", e))?;

    Ok(categorias)
}

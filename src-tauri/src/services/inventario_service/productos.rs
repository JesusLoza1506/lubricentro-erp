use super::{InventarioService, ProductosPaginadosDto, CATEGORIAS_VALIDAS};
use crate::dto::producto_dto::{
    ActualizarProductoDto, CrearProductoDto, ProductoAdminDto, ProductoPublicoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::entities::producto::ProductoEntity;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};

impl InventarioService {
    pub fn registrar_producto(
        conn: &mut Connection,
        req: CrearProductoDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        let cat_upper = req.categoria.trim().to_uppercase();
        let un_upper = req.unidad_medida.trim().to_uppercase();

        if !CATEGORIAS_VALIDAS.contains(&cat_upper.as_str()) {
            return Err(AppError::Validation(format!(
                "Categoría inválida: '{}'. Permitidas: ACEITE, FILTRO, ADITIVO, GRANEL, OTRO.",
                cat_upper
            )));
        }

        if cat_upper == "GRANEL" && un_upper != "ML" {
            return Err(AppError::Validation(
                "La categoría GRANEL debe usar strictly la unidad ML.".to_string(),
            ));
        }
        if cat_upper != "GRANEL" && un_upper != "UND" {
            return Err(AppError::Validation(
                "Las categorías distintas de GRANEL deben usar la unidad UND.".to_string(),
            ));
        }

        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO productos (codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, precio_venta_cent, activo)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, 0, ?7, 1)",
            params![
                req.codigo.trim().to_uppercase(),
                req.nombre.trim(),
                req.descripcion.as_deref().map(|d| d.trim()),
                cat_upper,
                un_upper,
                req.stock_minimo,
                req.precio_venta_cent,
            ],
        )?;

        let id_producto = tx.last_insert_rowid();

        let despues_prod_json = serde_json::json!({
            "id_producto": id_producto,
            "codigo": req.codigo.trim().to_uppercase(),
            "nombre": req.nombre.trim(),
            "categoria": cat_upper,
            "unidad_medida": un_upper,
            "stock_minimo": req.stock_minimo,
            "precio_venta_cent": req.precio_venta_cent,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "CREAR_PRODUCTO",
            Some("productos"),
            Some(&id_producto.to_string()),
            None,
            Some(&despues_prod_json),
            Some("Alta de nuevo producto en el catálogo"),
            "OK",
        )?;

        if let Some(cant_init_f) = req.cantidad_inicial {
            if cant_init_f > 0.0 {
                if un_upper == "UND" && (cant_init_f - cant_init_f.round()).abs() > 0.0001 {
                    return Err(AppError::Validation(
                        "No se permiten cantidades fraccionadas para productos en unidades (UND)."
                            .to_string(),
                    ));
                }

                let costo_micro_ingresado = req.costo_unitario_inicial_micro.ok_or_else(|| {
                    AppError::Validation(
                        "Costo unitario inicial obligatorio si hay stock inicial.".to_string(),
                    )
                })?;
                let ref_init = req.referencia_inicial.ok_or_else(|| {
                    AppError::Validation(
                        "Referencia inicial obligatoria para stock inicial.".to_string(),
                    )
                })?;

                let (cant_init, costo_micro, total_aporte) = if un_upper == "ML" {
                    let cant_ml = (cant_init_f * 1000.0).round() as i64;
                    let costo_ml_micro_raw = (costo_micro_ingresado as f64 / 1000.0).round() as i64;
                    let costo_ml_validado = (costo_ml_micro_raw / 10_000) * 10_000;
                    let costo_ml_final = costo_ml_validado.max(10_000);
                    let total = cant_ml * costo_ml_final;
                    (cant_ml, costo_ml_final, total)
                } else {
                    let cant_und = cant_init_f.round() as i64;
                    let costo_validado = (costo_micro_ingresado / 10_000) * 10_000;
                    let costo_final = costo_validado.max(10_000);
                    let total = cant_und * costo_final;
                    (cant_und, costo_final, total)
                };

                let id_op_ingreso = uuid::Uuid::new_v4().to_string();
                let cod_ingreso = format!("ING-{}", id_op_ingreso[..8].to_uppercase());

                tx.execute(
                    "INSERT INTO ingresos_stock (id_operacion, codigo, origen, referencia_inicial, estado, total_costo_micro, id_usuario, observacion)
                     VALUES (?1, ?2, 'INICIAL', ?3, 'PREPARANDO', ?4, ?5, 'Alta con Stock Inicial')",
                    params![id_op_ingreso, cod_ingreso, ref_init.trim(), total_aporte, id_usuario],
                )?;
                let id_ingreso = tx.last_insert_rowid();

                tx.execute(
                    "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_efectivo_micro, valor_aporte_micro)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        id_ingreso,
                        id_producto,
                        req.codigo.trim().to_uppercase(),
                        req.nombre.trim(),
                        un_upper,
                        cant_init,
                        costo_micro,
                        total_aporte,
                    ],
                )?;
                let id_det_ingreso = tx.last_insert_rowid();

                tx.execute(
                    "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
                     VALUES (?1, ?2, ?3, 'STOCK_INICIAL', ?4, ?5, 0, ?6, ?6, 0, ?7, ?7, ?8)",
                    params![
                        uuid::Uuid::new_v4().to_string(),
                        id_producto,
                        id_usuario,
                        id_det_ingreso,
                        un_upper,
                        cant_init,
                        total_aporte,
                        costo_micro,
                    ],
                )?;

                tx.execute(
                    "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2, ultimo_costo_micro = ?3 WHERE id_producto = ?4",
                    params![cant_init, total_aporte, costo_micro, id_producto],
                )?;

                tx.execute(
                    "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
                    params![id_ingreso],
                )?;

                let despues_stock_json = serde_json::json!({
                    "id_ingreso": id_ingreso,
                    "referencia_inicial": ref_init.trim(),
                    "cantidad_ingresada": cant_init,
                    "costo_efectivo_micro": costo_micro,
                    "total_aporte_micro": total_aporte,
                })
                .to_string();

                AuditoriaService::registrar_transaccion_v5(
                    &tx,
                    Some(id_usuario),
                    "USUARIO",
                    None,
                    None,
                    None,
                    "INVENTARIO",
                    "STOCK_INICIAL",
                    Some("ingresos_stock"),
                    Some(&id_ingreso.to_string()),
                    None,
                    Some(&despues_stock_json),
                    Some("Registro de stock inicial al crear producto"),
                    "OK",
                )?;
            }
        }

        tx.commit()?;
        Ok(id_producto)
    }

    pub fn actualizar_producto(
        conn: &mut Connection,
        req: ActualizarProductoDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let producto_ant: Option<(String, String, Option<String>, i64, i64)> = conn
            .query_row(
                "SELECT codigo, nombre, descripcion, stock_minimo, precio_venta_cent FROM productos WHERE id_producto = ?1",
                [req.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .ok();

        let (cod_prod, nom_ant, desc_ant, min_ant, precio_ant) = match producto_ant {
            Some(p) => p,
            None => return Err(AppError::NotFound("Producto no encontrado.".to_string())),
        };

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE productos SET nombre = ?1, descripcion = ?2, stock_minimo = ?3, precio_venta_cent = ?4 WHERE id_producto = ?5",
            params![
                req.nombre.trim(),
                req.descripcion.as_deref().map(|d| d.trim()),
                req.stock_minimo,
                req.precio_venta_cent,
                req.id_producto
            ],
        )?;

        let antes_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": nom_ant,
            "descripcion": desc_ant,
            "stock_minimo": min_ant,
            "precio_venta_cent": precio_ant,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": req.nombre.trim(),
            "descripcion": req.descripcion.as_deref().map(|d| d.trim()),
            "stock_minimo": req.stock_minimo,
            "precio_venta_cent": req.precio_venta_cent,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "ACTUALIZAR_PRODUCTO",
            Some("productos"),
            Some(&req.id_producto.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some("Modificación de datos básicos del producto"),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn eliminar_producto(
        conn: &mut Connection,
        id_producto: i64,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let consumo_neto_abierto: i64 = conn.query_row(
            "SELECT COALESCE(SUM(p.cantidad - COALESCE((
                SELECT SUM(m.diferencia) 
                FROM movimientos_inventario m 
                WHERE m.id_consumo_ot = p.id_detalle AND m.tipo_movimiento = 'REPOSICION_OT'
             ), 0)), 0)
             FROM detalle_ot_productos p 
             JOIN ordenes_trabajo o ON o.id_ot = p.id_ot 
             WHERE p.id_producto = ?1 AND o.estado IN ('EN_ESPERA', 'EN_PROCESO')",
            [id_producto],
            |r| r.get(0),
        )?;

        if consumo_neto_abierto > 0 {
            return Err(AppError::Validation(
                "No se puede desactivar el producto porque tiene consumos netos vigentes en Órdenes de Trabajo abiertas."
                    .to_string(),
            ));
        }

        let (cod_prod, nom_prod): (String, String) = conn
            .query_row(
                "SELECT codigo, nombre FROM productos WHERE id_producto = ?1",
                [id_producto],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado.".to_string()))?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE productos SET activo = 0 WHERE id_producto = ?1",
            [id_producto],
        )?;

        let antes_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": nom_prod,
            "activo": 1,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": nom_prod,
            "activo": 0,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "DESACTIVAR_PRODUCTO",
            Some("productos"),
            Some(&id_producto.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some("Desactivación lógica de producto del catálogo"),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn reactivar_producto(
        conn: &mut Connection,
        id_producto: i64,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let (cod_prod, nom_prod): (String, String) = conn
            .query_row(
                "SELECT codigo, nombre FROM productos WHERE id_producto = ?1",
                [id_producto],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado.".to_string()))?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE productos SET activo = 1 WHERE id_producto = ?1",
            [id_producto],
        )?;

        let antes_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": nom_prod,
            "activo": 0,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "codigo": cod_prod,
            "nombre": nom_prod,
            "activo": 1,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "REACTIVAR_PRODUCTO",
            Some("productos"),
            Some(&id_producto.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some("Reactivación de producto en catálogo"),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn listar_productos_publicos(
        conn: &Connection,
    ) -> Result<Vec<ProductoPublicoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo
             FROM productos WHERE activo = 1 ORDER BY nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                codigo: row.get(1)?,
                nombre: row.get(2)?,
                descripcion: row.get(3)?,
                categoria: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                valor_stock_micro: row.get(8)?,
                ultimo_costo_micro: row.get(9)?,
                precio_venta_cent: row.get(10)?,
                activo: row.get(11)?,
            };
            Ok(ProductoPublicoDto::desde_entity(entity))
        })?;

        let mut list = Vec::new();
        for i in iter {
            list.push(i?);
        }
        Ok(list)
    }

    pub fn listar_productos_admin(
        conn: &Connection,
        incluir_inactivos: bool,
    ) -> Result<Vec<ProductoAdminDto>, AppError> {
        let sql = if incluir_inactivos {
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo FROM productos ORDER BY nombre ASC"
        } else {
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo FROM productos WHERE activo = 1 ORDER BY nombre ASC"
        };

        let mut stmt = conn.prepare(sql)?;
        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                codigo: row.get(1)?,
                nombre: row.get(2)?,
                descripcion: row.get(3)?,
                categoria: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                valor_stock_micro: row.get(8)?,
                ultimo_costo_micro: row.get(9)?,
                precio_venta_cent: row.get(10)?,
                activo: row.get(11)?,
            };
            Ok(ProductoAdminDto::desde_entity(entity))
        })?;

        let mut list = Vec::new();
        for i in iter {
            list.push(i?);
        }
        Ok(list)
    }

    pub fn listar_productos_admin_paginado(
        conn: &Connection,
        busqueda: Option<String>,
        categoria: Option<String>,
        incluir_inactivos: Option<bool>,
        limite: i64,
        offset: i64,
    ) -> Result<ProductosPaginadosDto, AppError> {
        let mut where_clause = String::from(" WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if !incluir_inactivos.unwrap_or(false) {
            where_clause.push_str(" AND activo = 1");
        }

        if let Some(b) = busqueda {
            let b_trim = b.trim();
            if !b_trim.is_empty() {
                params_vec.push(Box::new(format!("%{}%", b_trim.to_uppercase())));
                let p_idx = params_vec.len();
                where_clause.push_str(&format!(
                    " AND (codigo LIKE ?{0} OR upper(nombre) LIKE ?{0})",
                    p_idx
                ));
            }
        }

        if let Some(cat) = categoria {
            let cat_trim = cat.trim().to_uppercase();
            if !cat_trim.is_empty() && cat_trim != "TODOS" {
                params_vec.push(Box::new(cat_trim));
                where_clause.push_str(&format!(" AND categoria = ?{}", params_vec.len()));
            }
        }

        let count_sql = format!("SELECT COUNT(*) FROM productos{}", where_clause);
        let mut stmt_count = conn.prepare(&count_sql)?;
        let count_params: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        let total_registros: i64 = stmt_count.query_row(count_params.as_slice(), |r| r.get(0))?;

        let mut data_sql = format!(
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo FROM productos{}",
            where_clause
        );
        params_vec.push(Box::new(limite));
        let limit_idx = params_vec.len();
        params_vec.push(Box::new(offset));
        let offset_idx = params_vec.len();

        data_sql.push_str(&format!(
            " ORDER BY nombre ASC LIMIT ?{} OFFSET ?{}",
            limit_idx, offset_idx
        ));

        let mut stmt_data = conn.prepare(&data_sql)?;
        let data_params: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let iter = stmt_data.query_map(data_params.as_slice(), |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                codigo: row.get(1)?,
                nombre: row.get(2)?,
                descripcion: row.get(3)?,
                categoria: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                valor_stock_micro: row.get(8)?,
                ultimo_costo_micro: row.get(9)?,
                precio_venta_cent: row.get(10)?,
                activo: row.get(11)?,
            };
            Ok(ProductoAdminDto::desde_entity(entity))
        })?;

        let mut productos = Vec::new();
        for i in iter {
            productos.push(i?);
        }

        Ok(ProductosPaginadosDto {
            productos,
            total_registros,
        })
    }

    pub fn listar_productos_criticos_publicos(
        conn: &Connection,
    ) -> Result<Vec<ProductoPublicoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo
             FROM productos WHERE activo = 1 AND stock_actual <= stock_minimo ORDER BY (stock_actual - stock_minimo) ASC, nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                codigo: row.get(1)?,
                nombre: row.get(2)?,
                descripcion: row.get(3)?,
                categoria: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                valor_stock_micro: row.get(8)?,
                ultimo_costo_micro: row.get(9)?,
                precio_venta_cent: row.get(10)?,
                activo: row.get(11)?,
            };
            Ok(ProductoPublicoDto::desde_entity(entity))
        })?;

        let mut list = Vec::new();
        for i in iter {
            list.push(i?);
        }
        Ok(list)
    }

    pub fn listar_productos_criticos_admin(
        conn: &Connection,
    ) -> Result<Vec<ProductoAdminDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_producto, codigo, nombre, descripcion, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo
             FROM productos WHERE activo = 1 AND stock_actual <= stock_minimo ORDER BY (stock_actual - stock_minimo) ASC, nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            let entity = ProductoEntity {
                id_producto: row.get(0)?,
                codigo: row.get(1)?,
                nombre: row.get(2)?,
                descripcion: row.get(3)?,
                categoria: row.get(4)?,
                unidad_medida: row.get(5)?,
                stock_actual: row.get(6)?,
                stock_minimo: row.get(7)?,
                valor_stock_micro: row.get(8)?,
                ultimo_costo_micro: row.get(9)?,
                precio_venta_cent: row.get(10)?,
                activo: row.get(11)?,
            };
            Ok(ProductoAdminDto::desde_entity(entity))
        })?;

        let mut list = Vec::new();
        for i in iter {
            list.push(i?);
        }
        Ok(list)
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
                nombre: "FILTRO".to_string(),
                activo: 1,
            },
            CategoriaEntity {
                id_categoria: 3,
                nombre: "ADITIVO".to_string(),
                activo: 1,
            },
            CategoriaEntity {
                id_categoria: 4,
                nombre: "GRANEL".to_string(),
                activo: 1,
            },
            CategoriaEntity {
                id_categoria: 5,
                nombre: "OTRO".to_string(),
                activo: 1,
            },
        ])
    }
}

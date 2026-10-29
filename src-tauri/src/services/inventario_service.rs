use crate::dto::ingreso_dto::RegistrarIngresoDto;
use crate::dto::producto_dto::{
    ActualizarProductoDto, AjustarStockDto, CrearProductoDto, ProductoAdminDto, ProductoPublicoDto,
};
use crate::dto::proveedor_dto::{CrearProveedorDto, ProveedorDto};
use crate::entities::categoria::CategoriaEntity;
use crate::entities::producto::ProductoEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct InventarioService;

const CATEGORIAS_VALIDAS: [&str; 5] = ["ACEITE", "FILTRO", "ADITIVO", "GRANEL", "OTRO"];

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
                "La categoría GRANEL debe usar estrictamente la unidad ML.".to_string(),
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

        if let Some(cant_init) = req.cantidad_inicial {
            if cant_init > 0 {
                let costo_micro = req.costo_unitario_inicial_micro.ok_or_else(|| {
                    AppError::Validation(
                        "Costo unitario inicial obligatorio si hay stock inicial.".to_string(),
                    )
                })?;
                let ref_init = req.referencia_inicial.ok_or_else(|| {
                    AppError::Validation(
                        "Referencia inicial obligatoria para stock inicial.".to_string(),
                    )
                })?;

                let id_op_ingreso = uuid::Uuid::new_v4().to_string();
                let cod_ingreso = format!("ING-{}", id_op_ingreso[..8].to_uppercase());
                let total_aporte = cant_init * costo_micro;

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
            }
        }

        tx.commit()?;
        Ok(id_producto)
    }

    pub fn actualizar_producto(
        conn: &Connection,
        req: ActualizarProductoDto,
    ) -> Result<(), AppError> {
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM productos WHERE id_producto = ?1 AND activo = 1",
            [req.id_producto],
            |r| r.get(0),
        )?;

        if count == 0 {
            return Err(AppError::NotFound(
                "Producto no encontrado o inactivo.".to_string(),
            ));
        }

        conn.execute(
            "UPDATE productos SET nombre = ?1, descripcion = ?2, stock_minimo = ?3, precio_venta_cent = ?4 WHERE id_producto = ?5 AND activo = 1",
            params![
                req.nombre.trim(),
                req.descripcion.as_deref().map(|d| d.trim()),
                req.stock_minimo,
                req.precio_venta_cent,
                req.id_producto
            ],
        )?;

        Ok(())
    }

    pub fn eliminar_producto(conn: &Connection, id_producto: i64) -> Result<(), AppError> {
        let movs: i32 = conn.query_row(
            "SELECT COUNT(*) FROM movimientos_inventario WHERE id_producto = ?1",
            [id_producto],
            |r| r.get(0),
        )?;

        if movs > 0 {
            return Err(AppError::Validation(
                "No se puede dar de baja un producto que tiene movimientos en el Kardex."
                    .to_string(),
            ));
        }

        conn.execute(
            "UPDATE productos SET activo = 0 WHERE id_producto = ?1",
            [id_producto],
        )?;

        Ok(())
    }

    pub fn registrar_ingreso_compra(
        conn: &mut Connection,
        req: RegistrarIngresoDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        if req.detalles.is_empty() || req.detalles.len() > 100 {
            return Err(AppError::Validation(
                "El ingreso debe contener entre 1 y 100 productos.".to_string(),
            ));
        }

        let id_prov = req
            .id_proveedor
            .ok_or_else(|| AppError::Validation("Proveedor requerido para compras.".to_string()))?;
        let tipo_doc = req
            .tipo_documento
            .ok_or_else(|| AppError::Validation("Tipo de documento requerido.".to_string()))?;
        let num_doc = req
            .numero_documento
            .ok_or_else(|| AppError::Validation("Número de documento requerido.".to_string()))?;
        let fec_doc = req
            .fecha_documento
            .ok_or_else(|| AppError::Validation("Fecha de documento requerida.".to_string()))?;

        let tx = conn.transaction()?;

        let prov_snapshot: String = tx.query_row(
            "SELECT json_object('id_proveedor', id_proveedor, 'tipo_documento', tipo_documento, 'numero_documento', numero_documento, 'nombre_razon_social', nombre_razon_social)
             FROM proveedores WHERE id_proveedor = ?1",
            [id_prov],
            |r| r.get(0),
        ).map_err(|_| AppError::NotFound("Proveedor no encontrado.".to_string()))?;

        let mut total_costo_micro = 0i64;
        for d in &req.detalles {
            total_costo_micro += d.cantidad * d.costo_efectivo_micro;
        }

        let id_op = uuid::Uuid::new_v4().to_string();
        let cod_ingreso = format!("ING-{}", id_op[..8].to_uppercase());

        tx.execute(
            "INSERT INTO ingresos_stock (id_operacion, codigo, origen, id_proveedor, proveedor_snapshot, tipo_documento, numero_documento, fecha_documento, estado, total_costo_micro, id_usuario, observacion)
             VALUES (?1, ?2, 'COMPRA', ?3, ?4, ?5, ?6, ?7, 'PREPARANDO', ?8, ?9, ?10)",
            params![id_op, cod_ingreso, id_prov, prov_snapshot, tipo_doc, num_doc, fec_doc, total_costo_micro, id_usuario, req.observacion],
        )?;
        let id_ingreso = tx.last_insert_rowid();

        for d in &req.detalles {
            let (cod_prod, nom_prod, un_prod, stock_ant, val_ant_micro): (String, String, String, i64, i64) = tx.query_row(
                "SELECT codigo, nombre, unidad_medida, stock_actual, valor_stock_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
                [d.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            ).map_err(|_| AppError::NotFound(format!("Producto ID {} no encontrado.", d.id_producto)))?;

            let aporte_micro = d.cantidad * d.costo_efectivo_micro;

            tx.execute(
                "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_efectivo_micro, valor_aporte_micro)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id_ingreso, d.id_producto, cod_prod, nom_prod, un_prod, d.cantidad, d.costo_efectivo_micro, aporte_micro],
            )?;
            let id_det = tx.last_insert_rowid();

            let stock_res = stock_ant + d.cantidad;
            let val_res_micro = val_ant_micro + aporte_micro;

            tx.execute(
                "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
                 VALUES (?1, ?2, ?3, 'COMPRA', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![uuid::Uuid::new_v4().to_string(), d.id_producto, id_usuario, id_det, un_prod, stock_ant, d.cantidad, stock_res, val_ant_micro, aporte_micro, val_res_micro, d.costo_efectivo_micro],
            )?;

            tx.execute(
                "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2, ultimo_costo_micro = ?3 WHERE id_producto = ?4",
                params![stock_res, val_res_micro, d.costo_efectivo_micro, d.id_producto],
            )?;
        }

        tx.execute(
            "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
            params![id_ingreso],
        )?;

        tx.commit()?;
        Ok(id_ingreso)
    }

    pub fn ajustar_stock(
        conn: &mut Connection,
        req: AjustarStockDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let motivo_trim = req.motivo.trim();
        if motivo_trim.len() < 5 || motivo_trim.len() > 250 {
            return Err(AppError::Validation(
                "El motivo del ajuste debe tener entre 5 y 250 caracteres.".to_string(),
            ));
        }

        let tipo_mov = match req.tipo_ajuste.to_uppercase().as_str() {
            "ENTRADA" => "AJUSTE_ENTRADA",
            "SALIDA" => "AJUSTE_SALIDA",
            _ => {
                return Err(AppError::Validation(
                    "Tipo de ajuste inválido. Use ENTRADA o SALIDA.".to_string(),
                ))
            }
        };

        let tx = conn.transaction()?;

        let (stock_ant, val_ant, un_med, ultimo_costo): (i64, i64, String, Option<i64>) = tx.query_row(
            "SELECT stock_actual, valor_stock_micro, unidad_medida, ultimo_costo_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
            [req.id_producto],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        ).map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let (diferencia, val_delta_micro, costo_unit_micro) = if tipo_mov == "AJUSTE_ENTRADA" {
            let costo = req.costo_unitario_micro.ok_or_else(|| {
                AppError::Validation("Costo unitario requerido para ajuste de entrada.".to_string())
            })?;
            let delta_val = req.cantidad * costo;
            (req.cantidad, delta_val, costo)
        } else {
            if req.cantidad > stock_ant {
                return Err(AppError::Validation(
                    "La cantidad a descontar supera el stock actual.".to_string(),
                ));
            }
            let costo = if stock_ant > 0 {
                val_ant / stock_ant
            } else {
                ultimo_costo.unwrap_or(0)
            };
            let delta_val = -(req.cantidad * costo);
            (-req.cantidad, delta_val, costo)
        };

        let stock_res = stock_ant + diferencia;
        let val_res = val_ant + val_delta_micro;

        tx.execute(
            "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro, motivo)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                uuid::Uuid::new_v4().to_string(),
                req.id_producto,
                id_usuario,
                tipo_mov,
                un_med,
                stock_ant,
                diferencia,
                stock_res,
                val_ant,
                val_delta_micro,
                val_res,
                costo_unit_micro,
                motivo_trim
            ],
        )?;

        tx.execute(
            "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2 WHERE id_producto = ?3",
            params![stock_res, val_res, req.id_producto],
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

    pub fn listar_productos_admin(conn: &Connection) -> Result<Vec<ProductoAdminDto>, AppError> {
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
            Ok(ProductoAdminDto::desde_entity(entity))
        })?;

        let mut list = Vec::new();
        for i in iter {
            list.push(i?);
        }
        Ok(list)
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

    pub fn listar_proveedores(conn: &Connection) -> Result<Vec<ProveedorDto>, AppError> {
        let mut stmt = conn.prepare("SELECT id_proveedor, tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor FROM proveedores ORDER BY nombre_razon_social ASC")?;
        let iter = stmt.query_map([], |r| {
            Ok(ProveedorDto {
                id_proveedor: r.get(0)?,
                tipo_documento: r.get(1)?,
                numero_documento: r.get(2)?,
                nombre_razon_social: r.get(3)?,
                telefono: r.get(4)?,
                correo: r.get(5)?,
                direccion: r.get(6)?,
                contacto_asesor: r.get(7)?,
            })
        })?;

        let mut list = Vec::new();
        for p in iter {
            list.push(p?);
        }
        Ok(list)
    }

    pub fn crear_proveedor(conn: &Connection, req: CrearProveedorDto) -> Result<i64, AppError> {
        conn.execute(
            "INSERT INTO proveedores (tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![req.tipo_documento.trim(), req.numero_documento.trim(), req.nombre_razon_social.trim(), req.telefono, req.correo, req.direccion, req.contacto_asesor],
        )?;
        Ok(conn.last_insert_rowid())
    }
}

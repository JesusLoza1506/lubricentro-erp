use super::{idempotencia::IdempotenciaService, obtener_fecha_lima, InventarioService};
use crate::dto::ingreso_dto::{
    AnularIngresoDto, DetalleIngresoResponseDto, IngresoStockResponseDto, RegistrarIngresoDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};
use serde_json::Value;

impl InventarioService {
    pub fn registrar_ingreso_compra(
        conn: &mut Connection,
        req: RegistrarIngresoDto,
        id_operacion: Option<&str>,
        id_usuario: i64,
    ) -> Result<Value, AppError> {
        if req.detalles.is_empty() || req.detalles.len() > 100 {
            return Err(AppError::Validation(
                "El ingreso debe contener entre 1 y 100 productos.".to_string(),
            ));
        }

        let id_prov = req
            .id_proveedor
            .ok_or_else(|| AppError::Validation("Proveedor requerido para compras.".to_string()))?;

        let tipo_doc_raw = req.tipo_documento.as_deref().unwrap_or("SIN_DOCUMENTO");
        let tipo_doc = tipo_doc_raw.trim().to_uppercase();

        let now_lima = obtener_fecha_lima();

        let num_doc = if tipo_doc == "SIN_DOCUMENTO" {
            format!("SD-{}", uuid::Uuid::new_v4().simple())
        } else {
            req.numero_documento
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| AppError::Validation("Número de documento requerido.".to_string()))?
                .to_string()
        };

        let fec_doc = if tipo_doc == "SIN_DOCUMENTO" {
            now_lima.format("%Y-%m-%d").to_string()
        } else {
            req.fecha_documento
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| AppError::Validation("Fecha de documento requerida.".to_string()))?
                .to_string()
        };

        // INICIO DE LA TRANSACCIÓN ÚNICA Y ATÓMICA
        let tx = conn.transaction()?;

        // Idempotencia Atómica
        if let Some(resultado_previo) = IdempotenciaService::evaluar_y_registrar(
            &tx,
            id_operacion,
            "REGISTRAR_COMPRA",
            &req,
            id_usuario,
        )? {
            tx.commit()?;
            return Ok(resultado_previo);
        }

        // Verificación de duplicado en negocio por documento
        if tipo_doc != "SIN_DOCUMENTO" {
            let existe_compra: i64 = tx.query_row(
                "SELECT COUNT(*) FROM ingresos_stock WHERE id_proveedor = ?1 AND tipo_documento = ?2 AND numero_documento = ?3",
                params![id_prov, tipo_doc, num_doc.trim()],
                |r| r.get(0),
            )?;

            if existe_compra > 0 {
                return Err(AppError::Validation(format!(
                    "Ya existe un ingreso registrado con el documento {} - {} para este proveedor.",
                    tipo_doc, num_doc
                )));
            }
        }

        let prov_snapshot: String = tx.query_row(
            "SELECT json_object('id_proveedor', id_proveedor, 'tipo_documento', tipo_documento, 'numero_documento', numero_documento, 'nombre_razon_social', nombre_razon_social)
             FROM proveedores WHERE id_proveedor = ?1",
            [id_prov],
            |r| r.get(0),
        ).map_err(|_| AppError::NotFound("Proveedor no encontrado.".to_string()))?;

        // Precalculamos aportes considerando UND vs ML y validaciones estrictas
        let mut detalles_calculados = Vec::new();
        let mut total_costo_micro = 0i64;

        for d in &req.detalles {
            if d.cantidad <= 0 {
                return Err(AppError::Validation(format!(
                    "La cantidad del producto ID {} debe ser mayor a cero.",
                    d.id_producto
                )));
            }

            if d.costo_efectivo_micro <= 0 {
                return Err(AppError::Validation(format!(
                    "El costo unitario del producto ID {} debe ser mayor a cero (costo cero no permitido).",
                    d.id_producto
                )));
            }

            let (cod_prod, nom_prod, un_prod, stock_ant, val_ant_micro): (String, String, String, i64, i64) = tx.query_row(
                "SELECT codigo, nombre, unidad_medida, stock_actual, valor_stock_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
                [d.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            ).map_err(|_| AppError::NotFound(format!("Producto ID {} no encontrado.", d.id_producto)))?;

            super::validaciones::cantidad_base(d.cantidad, &un_prod)?;
            super::validaciones::cantidad_base(stock_ant + d.cantidad, &un_prod)?;
            let costo_efectivo_micro =
                super::validaciones::costo_capturado(d.costo_efectivo_micro)?;
            if costo_efectivo_micro <= 0 {
                return Err(AppError::Validation(format!(
                    "El costo unitario redondeado del producto ID {} no es válido.",
                    d.id_producto
                )));
            }

            let aporte_micro =
                super::validaciones::aporte_micro(d.cantidad, costo_efectivo_micro, &un_prod)?;
            super::validaciones::valor_stock(val_ant_micro as i128 + aporte_micro as i128)?;
            let costo_unitario_kardex = if un_prod == "ML" {
                (aporte_micro / d.cantidad).max(1)
            } else {
                costo_efectivo_micro
            };
            total_costo_micro =
                super::validaciones::valor_stock(total_costo_micro as i128 + aporte_micro as i128)?;
            detalles_calculados.push((
                d.id_producto,
                cod_prod,
                nom_prod,
                un_prod,
                d.cantidad,
                costo_efectivo_micro,
                aporte_micro,
                costo_unitario_kardex,
                stock_ant,
                val_ant_micro,
            ));
        }

        let id_op = uuid::Uuid::new_v4().to_string();
        let cod_ingreso = format!("ING-{}", id_op[..8].to_uppercase());

        tx.execute(
            "INSERT INTO ingresos_stock (id_operacion, codigo, origen, id_proveedor, proveedor_snapshot, tipo_documento, numero_documento, fecha_documento, estado, total_costo_micro, id_usuario, observacion)
             VALUES (?1, ?2, 'COMPRA', ?3, ?4, ?5, ?6, ?7, 'PREPARANDO', ?8, ?9, ?10)",
            params![id_op, cod_ingreso, id_prov, prov_snapshot, tipo_doc, num_doc, fec_doc, total_costo_micro, id_usuario, req.observacion],
        )?;
        let id_ingreso = tx.last_insert_rowid();

        for (
            id_prod,
            cod_prod,
            nom_prod,
            un_prod,
            cant,
            costo_efectivo_micro,
            aporte_micro,
            costo_kardex,
            stock_ant,
            val_ant_micro,
        ) in detalles_calculados
        {
            tx.execute(
                "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_efectivo_micro, valor_aporte_micro)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id_ingreso, id_prod, cod_prod, nom_prod, un_prod, cant, costo_efectivo_micro, aporte_micro],
            )?;
            let id_det = tx.last_insert_rowid();

            let stock_res = stock_ant + cant;
            let val_res_micro = val_ant_micro + aporte_micro;

            tx.execute(
                "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
                 VALUES (?1, ?2, ?3, 'COMPRA', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![uuid::Uuid::new_v4().to_string(), id_prod, id_usuario, id_det, un_prod, stock_ant, cant, stock_res, val_ant_micro, aporte_micro, val_res_micro, costo_kardex],
            )?;

            tx.execute(
                "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2, ultimo_costo_micro = ?3 WHERE id_producto = ?4",
                params![stock_res, val_res_micro, costo_kardex, id_prod],
            )?;
        }

        tx.execute(
            "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
            params![id_ingreso],
        )?;

        let despues_compra_json = serde_json::json!({
            "id_ingreso": id_ingreso,
            "codigo": cod_ingreso,
            "id_proveedor": id_prov,
            "tipo_documento": tipo_doc,
            "numero_documento": num_doc,
            "fecha_documento": fec_doc,
            "total_costo_micro": total_costo_micro,
            "items_count": req.detalles.len(),
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
            "REGISTRAR_COMPRA",
            Some("ingresos_stock"),
            Some(&id_ingreso.to_string()),
            None,
            Some(&despues_compra_json),
            req.observacion
                .as_deref()
                .or(Some("Ingreso de mercadería por compra a proveedor")),
            "OK",
        )?;

        let response_payload = serde_json::json!({
            "id_ingreso": id_ingreso,
            "codigo": cod_ingreso,
            "total_costo_micro": total_costo_micro
        });

        // Finalizar Idempotencia en la transacción
        IdempotenciaService::finalizar_solicitud(
            &tx,
            id_operacion,
            "REGISTRAR_COMPRA",
            &req,
            id_usuario,
            &response_payload,
        )?;

        tx.commit()?;
        Ok(response_payload)
    }

    pub fn listar_ingresos_stock(
        conn: &Connection,
        limite: i64,
        offset: i64,
        estado: Option<String>,
        origen: Option<String>,
    ) -> Result<Vec<IngresoStockResponseDto>, AppError> {
        Self::listar_ingresos_stock_filtrado(conn, limite, offset, estado, origen, None, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn listar_ingresos_stock_filtrado(
        conn: &Connection,
        limite: i64,
        offset: i64,
        estado: Option<String>,
        origen: Option<String>,
        fecha_inicio: Option<String>,
        fecha_fin: Option<String>,
    ) -> Result<Vec<IngresoStockResponseDto>, AppError> {
        if !(1..=100).contains(&limite) || offset < 0 {
            return Err(AppError::Validation("Paginación inválida.".into()));
        }
        let fechas = match (fecha_inicio, fecha_fin) {
            (None, None) => None,
            (Some(inicio), Some(fin)) => {
                let parsear = |s: &str| {
                    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map_err(|_| AppError::Validation("Fecha inválida.".into()))
                };
                let desde = parsear(&inicio)?;
                let hasta = parsear(&fin)?;
                if desde > hasta || (hasta - desde).num_days() > 366 {
                    return Err(AppError::Validation(
                        "El rango debe estar ordenado y no superar 366 días.".into(),
                    ));
                }
                Some((inicio, fin))
            }
            _ => {
                return Err(AppError::Validation(
                    "Complete ambas fechas del rango.".into(),
                ))
            }
        };
        let mut sql = String::from(
            "SELECT i.id_ingreso, i.id_operacion, i.codigo, i.origen, i.id_proveedor, 
                    p.nombre_razon_social, i.tipo_documento, i.numero_documento, i.fecha_documento, 
                    i.estado, i.total_costo_micro, i.id_usuario, u.nombre_completo, 
                    i.observacion, i.fecha_registro, i.fecha_anulacion, i.motivo_anulacion,
                    i.proveedor_snapshot
             FROM ingresos_stock i
             LEFT JOIN proveedores p ON p.id_proveedor = i.id_proveedor
             LEFT JOIN usuarios u ON u.id_usuario = i.id_usuario
             WHERE 1=1",
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(est) = estado {
            if !est.trim().is_empty() && est.trim().to_uppercase() != "TODOS" {
                params_vec.push(Box::new(est.trim().to_uppercase()));
                sql.push_str(&format!(" AND i.estado = ?{}", params_vec.len()));
            }
        }

        if let Some(orig) = origen {
            if !orig.trim().is_empty() && orig.trim().to_uppercase() != "TODOS" {
                params_vec.push(Box::new(orig.trim().to_uppercase()));
                sql.push_str(&format!(" AND i.origen = ?{}", params_vec.len()));
            }
        }

        if let Some((inicio, fin)) = fechas {
            params_vec.push(Box::new(format!("{inicio} 00:00:00")));
            sql.push_str(&format!(" AND i.fecha_registro >= ?{}", params_vec.len()));
            params_vec.push(Box::new(format!("{fin} 23:59:59")));
            sql.push_str(&format!(" AND i.fecha_registro <= ?{}", params_vec.len()));
        }
        params_vec.push(Box::new(limite));
        let limit_idx = params_vec.len();
        params_vec.push(Box::new(offset));
        let offset_idx = params_vec.len();

        sql.push_str(&format!(
            " ORDER BY i.id_ingreso DESC LIMIT ?{} OFFSET ?{}",
            limit_idx, offset_idx
        ));

        let mut stmt = conn.prepare(&sql)?;
        let slice: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

        let iter = stmt.query_map(slice.as_slice(), |r| {
            let id_ingreso: i64 = r.get(0)?;
            let id_operacion: String = r.get(1)?;
            let codigo: String = r.get(2)?;
            let origen_str: String = r.get(3)?;
            let id_proveedor: Option<i64> = r.get(4)?;
            let proveedor_nombre_join: Option<String> = r.get(5)?;
            let tipo_documento: Option<String> = r.get(6)?;
            let numero_documento: Option<String> = r.get(7)?;
            let fecha_documento: Option<String> = r.get(8)?;
            let estado_str: String = r.get(9)?;
            let total_costo_micro: i64 = r.get(10)?;
            let id_usuario: i64 = r.get(11)?;
            let usuario_nombre: Option<String> = r.get(12)?;
            let observacion: Option<String> = r.get(13)?;
            let fecha_registro: String = r.get(14)?;
            let fecha_anulacion: Option<String> = r.get(15)?;
            let motivo_anulacion: Option<String> = r.get(16)?;
            let proveedor_snapshot_raw: Option<String> = r.get(17)?;

            let proveedor_nombre = if let Some(ref raw_json) = proveedor_snapshot_raw {
                if let Ok(v) = serde_json::from_str::<Value>(raw_json) {
                    v.get("nombre_razon_social")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                        .or(proveedor_nombre_join)
                } else {
                    proveedor_nombre_join
                }
            } else {
                proveedor_nombre_join
            };

            Ok(IngresoStockResponseDto {
                id_ingreso,
                id_operacion,
                codigo,
                origen: origen_str,
                id_proveedor,
                proveedor_nombre,
                tipo_documento,
                numero_documento,
                fecha_documento,
                estado: estado_str,
                total_costo_micro,
                id_usuario,
                usuario_nombre,
                observacion,
                fecha_registro,
                fecha_anulacion,
                motivo_anulacion,
                detalles: Vec::new(),
            })
        })?;

        let mut lista = Vec::new();
        for item in iter {
            let mut ing = item?;
            let mut stmt_det = conn.prepare(
                "SELECT id_detalle, id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_efectivo_micro, valor_aporte_micro
                 FROM detalle_ingreso WHERE id_ingreso = ?1 ORDER BY id_detalle ASC"
            )?;
            let det_iter = stmt_det.query_map([ing.id_ingreso], |dr| {
                Ok(DetalleIngresoResponseDto {
                    id_detalle: dr.get(0)?,
                    id_ingreso: dr.get(1)?,
                    id_producto: dr.get(2)?,
                    codigo_snapshot: dr.get(3)?,
                    nombre_snapshot: dr.get(4)?,
                    unidad_medida: dr.get(5)?,
                    cantidad: dr.get(6)?,
                    costo_efectivo_micro: dr.get(7)?,
                    valor_aporte_micro: dr.get(8)?,
                })
            })?;
            for d in det_iter {
                ing.detalles.push(d?);
            }
            lista.push(ing);
        }

        Ok(lista)
    }

    pub fn anular_ingreso_compra(
        conn: &mut Connection,
        req: AnularIngresoDto,
        id_operacion: Option<&str>,
        id_usuario: i64,
    ) -> Result<Value, AppError> {
        let motivo_trim = req.motivo.trim();
        if motivo_trim.len() < 5 || motivo_trim.len() > 250 {
            return Err(AppError::Validation(
                "El motivo de anulación debe tener entre 5 y 250 caracteres.".to_string(),
            ));
        }

        let tx = conn.transaction()?;

        // Idempotencia Atómica
        if let Some(resultado_previo) = IdempotenciaService::evaluar_y_registrar(
            &tx,
            id_operacion,
            "ANULAR_COMPRA",
            &req,
            id_usuario,
        )? {
            tx.commit()?;
            return Ok(resultado_previo);
        }

        let (estado_actual, origen_ing, cod_ingreso, total_costo): (String, String, String, i64) = tx
            .query_row(
                "SELECT estado, origen, codigo, total_costo_micro FROM ingresos_stock WHERE id_ingreso = ?1",
                [req.id_ingreso],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .map_err(|_| AppError::NotFound("Ingreso no encontrado.".to_string()))?;

        if estado_actual != "REGISTRADO" {
            return Err(AppError::Validation(
                "Solo se pueden anular ingresos en estado REGISTRADO.".to_string(),
            ));
        }

        let detalles: Vec<(i64, i64, String, i64, i64, i64)> = {
            let mut stmt = tx.prepare(
                "SELECT id_detalle, id_producto, unidad_medida, cantidad, valor_aporte_micro, costo_efectivo_micro 
                 FROM detalle_ingreso WHERE id_ingreso = ?1",
            )?;
            let iter = stmt.query_map([req.id_ingreso], |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            })?;
            iter.collect::<Result<_, _>>()?
        };

        for (id_det, id_prod, un_med, cant, val_aporte, costo_unit) in detalles {
            let (stock_ant, val_ant): (i64, i64) = tx.query_row(
                "SELECT stock_actual, valor_stock_micro FROM productos WHERE id_producto = ?1",
                [id_prod],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )?;

            if cant > stock_ant {
                return Err(AppError::Validation(
                    "No se puede anular el ingreso porque el stock actual disponible es menor a la cantidad ingresada.".to_string(),
                ));
            }

            let stock_res = stock_ant - cant;

            let val_res = if stock_res == 0 {
                0
            } else {
                val_ant - val_aporte
            };
            let val_delta = if stock_res == 0 {
                -val_ant
            } else {
                -val_aporte
            };

            tx.execute(
                "INSERT INTO movimientos_inventario (
                    id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, 
                    unidad_medida, stock_anterior, diferencia, stock_resultante, 
                    valor_anterior_micro, valor_delta_micro, valor_resultante_micro, 
                    costo_unitario_micro, motivo
                 ) VALUES (?1, ?2, ?3, 'ANULACION_INGRESO', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    uuid::Uuid::new_v4().to_string(),
                    id_prod,
                    id_usuario,
                    id_det,
                    un_med,
                    stock_ant,
                    -cant,
                    stock_res,
                    val_ant,
                    val_delta,
                    val_res,
                    costo_unit,
                    motivo_trim
                ],
            ).map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("no anulable") || err_msg.contains("movimientos posteriores") {
                    AppError::Validation(
                        "No se puede anular esta entrada porque existen movimientos posteriores en el Kárdex para uno o más de sus productos."
                            .to_string(),
                    )
                } else {
                    AppError::Database(e)
                }
            })?;

            tx.execute(
                "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2 WHERE id_producto = ?3",
                params![stock_res, val_res, id_prod],
            )?;
        }

        let now_lima = obtener_fecha_lima();
        let now_str = now_lima.format("%Y-%m-%d %H:%M:%S").to_string();

        tx.execute(
            "UPDATE ingresos_stock SET estado = 'ANULADO', fecha_anulacion = ?1, id_usuario_anulacion = ?2, motivo_anulacion = ?3 WHERE id_ingreso = ?4",
            params![now_str, id_usuario, motivo_trim, req.id_ingreso],
        )?;

        let antes_anul_json = serde_json::json!({
            "id_ingreso": req.id_ingreso,
            "codigo": cod_ingreso,
            "origen": origen_ing,
            "estado": "REGISTRADO",
            "total_costo_micro": total_costo,
        })
        .to_string();

        let despues_anul_json = serde_json::json!({
            "id_ingreso": req.id_ingreso,
            "codigo": cod_ingreso,
            "origen": origen_ing,
            "estado": "ANULADO",
            "fecha_anulacion": now_str,
            "total_costo_micro": total_costo,
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
            "ANULAR_COMPRA",
            Some("ingresos_stock"),
            Some(&req.id_ingreso.to_string()),
            Some(&antes_anul_json),
            Some(&despues_anul_json),
            Some(motivo_trim),
            "OK",
        )?;

        let response_payload = serde_json::json!({
            "status": "OK",
            "message": "Ingreso de compra anulado exitosamente",
            "id_ingreso": req.id_ingreso
        });

        IdempotenciaService::finalizar_solicitud(
            &tx,
            id_operacion,
            "ANULAR_COMPRA",
            &req,
            id_usuario,
            &response_payload,
        )?;

        tx.commit()?;
        Ok(response_payload)
    }
}

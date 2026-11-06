use super::{idempotencia::IdempotenciaService, InventarioService};
use crate::dto::producto_dto::{AjustarStockDto, ConteoFisicoDto};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection, Transaction};
use serde_json::Value;

impl InventarioService {
    pub fn ajustar_stock(
        conn: &mut Connection,
        req: AjustarStockDto,
        id_operacion: Option<&str>,
        id_usuario: i64,
    ) -> Result<Value, AppError> {
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        if let Some(previo) = IdempotenciaService::evaluar_y_registrar(
            &tx,
            id_operacion,
            "AJUSTAR_STOCK",
            &req,
            id_usuario,
        )? {
            tx.commit()?;
            return Ok(previo);
        }
        let resultado = Self::aplicar_ajuste(&tx, &req, id_usuario, None)?;
        IdempotenciaService::finalizar_solicitud(
            &tx,
            id_operacion,
            "AJUSTAR_STOCK",
            &req,
            id_usuario,
            &resultado,
        )?;
        tx.commit()?;
        Ok(resultado)
    }

    fn aplicar_ajuste(
        tx: &Transaction<'_>,
        req: &AjustarStockDto,
        id_usuario: i64,
        aporte_conteo: Option<i64>,
    ) -> Result<Value, AppError> {
        let motivo_trim = req.motivo.trim();
        if !(5..=250).contains(&motivo_trim.chars().count()) {
            return Err(AppError::Validation(
                "El motivo debe contener entre 5 y 250 caracteres.".into(),
            ));
        }
        if req.cantidad <= 0 {
            return Err(AppError::Validation(
                "La cantidad debe ser mayor a cero.".into(),
            ));
        }
        let tipo_mov = match req.tipo_ajuste.trim().to_uppercase().as_str() {
            "ENTRADA" => "AJUSTE_ENTRADA",
            "SALIDA" => "AJUSTE_SALIDA",
            _ => return Err(AppError::Validation("Tipo de ajuste inválido.".into())),
        };
        let (stock_ant, val_ant, un_med, ultimo_costo): (i64, i64, String, Option<i64>) = tx
            .query_row(
                "SELECT stock_actual, valor_stock_micro, unidad_medida, ultimo_costo_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let un_upper = un_med.to_uppercase();
        super::validaciones::cantidad_base(req.cantidad, &un_upper)?;

        let (diferencia, val_delta_micro, costo_unit_micro) = if tipo_mov == "AJUSTE_ENTRADA" {
            let costo_ingresado = req.costo_unitario_micro.ok_or_else(|| {
                AppError::Validation(
                    "Costo unitario obligatorio para ajustes de ENTRADA.".to_string(),
                )
            })?;

            if costo_ingresado <= 0 {
                return Err(AppError::Validation(
                    "El costo unitario ingresado debe ser mayor a cero (costo cero no permitido)."
                        .to_string(),
                ));
            }

            let costo_validado = if aporte_conteo.is_some() {
                costo_ingresado
            } else {
                super::validaciones::costo_capturado(costo_ingresado)?
            };
            if costo_validado <= 0 {
                return Err(AppError::Validation(
                    "El costo unitario ingresado redondeado debe ser mayor a cero.".to_string(),
                ));
            }

            let total_delta = if let Some(valor) = aporte_conteo {
                super::validaciones::valor_stock(valor as i128)?
            } else {
                super::validaciones::aporte_micro(req.cantidad, costo_validado, &un_upper)?
            };
            let c_kardex = ((total_delta as i128 + req.cantidad as i128 / 2) / req.cantidad as i128)
                .max(1) as i64;

            (req.cantidad, total_delta, c_kardex)
        } else {
            if req.cantidad > stock_ant {
                return Err(AppError::Validation(format!(
                    "Stock insuficiente para realizar el ajuste de salida. Stock actual: {}",
                    stock_ant
                )));
            }

            let stock_res_temp = stock_ant - req.cantidad;
            let (delta_val, costo_kardex) = if stock_res_temp == 0 {
                let costo_aprox = if stock_ant > 0 {
                    (val_ant / stock_ant.max(1)).max(1)
                } else {
                    ultimo_costo.unwrap_or(1).max(1)
                };
                (-val_ant, costo_aprox)
            } else {
                let costo_prom =
                    ((val_ant as i128 + stock_ant as i128 / 2) / stock_ant as i128).max(1) as i64;
                // valor/stock ya está expresado por UND o por ML. No dividir otra vez entre 1000.
                let delta = ((req.cantidad as i128 * val_ant as i128 + stock_ant as i128 / 2)
                    / stock_ant as i128) as i64;
                (-delta, costo_prom)
            };

            (-req.cantidad, delta_val, costo_kardex)
        };

        let stock_res = stock_ant + diferencia;
        super::validaciones::cantidad_base(stock_res, &un_upper)?;
        let val_res = super::validaciones::valor_stock(val_ant as i128 + val_delta_micro as i128)?;

        tx.execute(
            "INSERT INTO movimientos_inventario (
                id_operacion, id_producto, id_usuario, tipo_movimiento, unidad_medida, 
                stock_anterior, diferencia, stock_resultante, valor_anterior_micro, 
                valor_delta_micro, valor_resultante_micro, costo_unitario_micro, motivo
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
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

        if tipo_mov == "AJUSTE_ENTRADA" {
            tx.execute(
                "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2, ultimo_costo_micro = ?3 WHERE id_producto = ?4",
                params![stock_res, val_res, costo_unit_micro, req.id_producto],
            )?;
        } else {
            tx.execute(
                "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2 WHERE id_producto = ?3",
                params![stock_res, val_res, req.id_producto],
            )?;
        }

        let antes_ajuste_json = serde_json::json!({
            "id_producto": req.id_producto,
            "stock_anterior": stock_ant,
            "valor_anterior_micro": val_ant,
        })
        .to_string();

        let despues_ajuste_json = serde_json::json!({
            "id_producto": req.id_producto,
            "tipo_ajuste": req.tipo_ajuste.to_uppercase(),
            "diferencia": diferencia,
            "stock_resultante": stock_res,
            "valor_resultante_micro": val_res,
            "costo_unitario_micro": costo_unit_micro,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            tipo_mov,
            Some("productos"),
            Some(&req.id_producto.to_string()),
            Some(&antes_ajuste_json),
            Some(&despues_ajuste_json),
            Some(motivo_trim),
            "OK",
        )?;

        let response_payload = serde_json::json!({
            "status": "OK",
            "message": "Ajuste de stock procesado con éxito",
            "id_producto": req.id_producto,
            "stock_resultante": stock_res,
            "valor_resultante_micro": val_res
        });

        Ok(response_payload)
    }

    pub fn procesar_conteo_fisico(
        conn: &mut Connection,
        req: ConteoFisicoDto,
        id_operacion: Option<&str>,
        id_usuario: i64,
    ) -> Result<Value, AppError> {
        // El comprobante de solicitud pertenece al conteo ORIGINAL, nunca al ajuste derivado.
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        if let Some(previo) = IdempotenciaService::evaluar_y_registrar(
            &tx,
            id_operacion,
            "CONTEO_FISICO",
            &req,
            id_usuario,
        )? {
            tx.commit()?;
            return Ok(previo);
        }
        let (stock, valor, unidad, ultimo): (i64, i64, String, Option<i64>) = tx.query_row(
            "SELECT stock_actual, valor_stock_micro, unidad_medida, ultimo_costo_micro FROM productos WHERE id_producto=?1 AND activo=1",
            [req.id_producto], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)))
            .map_err(|e| if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                AppError::NotFound("Producto no encontrado o inactivo.".into())
            } else { e.into() })?;
        let contado = super::validaciones::cantidad_comercial(req.stock_fisico_contado, &unidad)?;
        if let Some(observado) = req.stock_observado {
            if super::validaciones::cantidad_comercial(observado, &unidad)? != stock {
                return Err(AppError::Validation("El stock del producto fue modificado por otro usuario durante el conteo. Recargue los datos antes de continuar.".into()));
            }
        }
        let diferencia = contado - stock;
        let resultado = if diferencia == 0 {
            serde_json::json!({"status":"OK", "sin_diferencias":true,
                "message":"Conteo verificado: sin diferencias de stock.",
                "id_producto":req.id_producto, "stock_resultante":stock,
                "valor_resultante_micro":valor})
        } else {
            let aporte = if diferencia > 0 {
                let total = if stock > 0 {
                    ((diferencia as i128 * valor as i128 + stock as i128 / 2) / stock as i128)
                        as i64
                } else if let Some(capturado) = req.costo_unitario_micro {
                    let costo = super::validaciones::costo_capturado(capturado)?;
                    if unidad == "ML" {
                        ((diferencia as i128 * costo as i128 + 500) / 1000) as i64
                    } else {
                        diferencia
                            .checked_mul(costo)
                            .ok_or_else(|| AppError::Validation("Valor fuera de rango.".into()))?
                    }
                } else {
                    let costo = ultimo.filter(|v| *v > 0).ok_or_else(||
                        AppError::Validation("Ingrese el costo por unidad o litro para el sobrante sin costo previo.".into()))?;
                    diferencia
                        .checked_mul(costo)
                        .ok_or_else(|| AppError::Validation("Valor fuera de rango.".into()))?
                };
                Some(total)
            } else {
                None
            };
            Self::aplicar_ajuste(
                &tx,
                &AjustarStockDto {
                    id_producto: req.id_producto,
                    tipo_ajuste: if diferencia > 0 { "ENTRADA" } else { "SALIDA" }.into(),
                    cantidad: diferencia.abs(),
                    costo_unitario_micro: aporte.map(|_| 1),
                    motivo: req.motivo.clone(),
                },
                id_usuario,
                aporte,
            )?
        };
        IdempotenciaService::finalizar_solicitud(
            &tx,
            id_operacion,
            "CONTEO_FISICO",
            &req,
            id_usuario,
            &resultado,
        )?;
        tx.commit()?;
        Ok(resultado)
    }
}

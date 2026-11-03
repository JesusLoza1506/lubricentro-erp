use super::InventarioService;
use crate::dto::producto_dto::{AjustarStockDto, ConteoFisicoDto};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};

impl InventarioService {
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
            &tx,
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

        tx.commit()?;
        Ok(())
    }

    pub fn procesar_conteo_fisico(
        conn: &mut Connection,
        req: ConteoFisicoDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let (stock_ant, val_ant, un_med, ultimo_costo): (i64, i64, String, Option<i64>) = conn.query_row(
            "SELECT stock_actual, valor_stock_micro, unidad_medida, ultimo_costo_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
            [req.id_producto],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        ).map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let un_upper = un_med.to_uppercase();

        if let Some(obs) = req.stock_observado {
            let stock_observado_base = if un_upper == "ML" {
                (obs * 1000.0).round() as i64
            } else {
                obs.round() as i64
            };

            if stock_ant != stock_observado_base {
                return Err(AppError::Validation(
                    "El stock del producto fue modificado por otro usuario mientras realizaba el conteo. Por favor, recargue los datos e intente nuevamente."
                        .to_string(),
                ));
            }
        }

        if un_upper == "UND"
            && (req.stock_fisico_contado - req.stock_fisico_contado.round()).abs() > 0.0001
        {
            return Err(AppError::Validation(
                "No se permiten números decimales en el conteo de productos medidos en unidades (UND).".to_string(),
            ));
        }

        let stock_fisico_base = if un_upper == "ML" {
            (req.stock_fisico_contado * 1000.0).round() as i64
        } else {
            req.stock_fisico_contado.round() as i64
        };

        let diferencia = stock_fisico_base - stock_ant;
        if diferencia == 0 {
            return Ok(());
        }

        let motivo_trim = req.motivo.trim().to_string();
        if motivo_trim.len() < 5 || motivo_trim.len() > 250 {
            return Err(AppError::Validation(
                "El motivo del conteo físico debe tener entre 5 y 250 caracteres.".to_string(),
            ));
        }

        if diferencia > 0 {
            let costo = if stock_ant > 0 {
                val_ant / stock_ant
            } else {
                req.costo_unitario_micro.or(ultimo_costo).ok_or_else(|| {
                    AppError::Validation(
                        "Costo unitario requerido para registrar un sobrante inicial.".to_string(),
                    )
                })?
            };

            Self::ajustar_stock(
                conn,
                AjustarStockDto {
                    id_producto: req.id_producto,
                    tipo_ajuste: "ENTRADA".to_string(),
                    cantidad: diferencia,
                    costo_unitario_micro: Some(costo),
                    motivo: motivo_trim,
                },
                id_usuario,
            )
        } else {
            Self::ajustar_stock(
                conn,
                AjustarStockDto {
                    id_producto: req.id_producto,
                    tipo_ajuste: "SALIDA".to_string(),
                    cantidad: diferencia.abs(),
                    costo_unitario_micro: None,
                    motivo: motivo_trim,
                },
                id_usuario,
            )
        }
    }
}

use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::entities::comprobante::ComprobanteEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};

pub struct ComprobanteService;

impl ComprobanteService {
    /// Emite un comprobante (Boleta, Factura o Proforma), actualiza la serie/correlativo y registra el pago
    pub fn emitir_comprobante(
        conn: &Connection,
        req: EmitirComprobanteDto,
    ) -> Result<ComprobanteDto, AppError> {
        // Validar montos en céntimos
        if req.monto_total != req.monto_subtotal + req.monto_igv {
            return Err(AppError::Validation(
                "El monto total debe ser exactamente la suma del subtotal y el IGV.".to_string(),
            ));
        }

        let tipo = req.tipo_comprobante.trim().to_uppercase();
        if tipo != "BOLETA" && tipo != "FACTURA" && tipo != "PROFORMA" {
            return Err(AppError::Validation(
                "Tipo de comprobante inválido. Debe ser BOLETA, FACTURA o PROFORMA.".to_string(),
            ));
        }

        let mut correlativo_val = 0;
        let serie_id_val = req.id_serie;

        // Si no es proforma, gestionamos la serie y correlativo atómicamente
        if tipo != "PROFORMA" {
            if let Some(id_s) = serie_id_val {
                // Incrementar correlativo actual de la serie
                conn.execute(
                    "UPDATE series_comprobante SET correlativo_actual = correlativo_actual + 1 WHERE id_serie = ?1 AND activa = 1",
                    [id_s],
                )?;

                correlativo_val = conn.query_row(
                    "SELECT correlativo_actual FROM series_comprobante WHERE id_serie = ?1",
                    [id_s],
                    |row| row.get(0),
                )?;
            } else {
                return Err(AppError::Validation(
                    "Se requiere especificar una serie activa para Boletas y Facturas.".to_string(),
                ));
            }
        }

        // Insertar comprobante
        conn.execute(
            "INSERT INTO comprobantes (id_ot, id_cliente, id_cajero, id_caja, tipo_comprobante, id_serie, correlativo, monto_subtotal, monto_igv, monto_total, estado_sunat, fecha_emision)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'PENDIENTE', datetime('now', 'localtime'))",
            params![
                req.id_ot,
                req.id_cliente,
                req.id_cajero,
                req.id_caja,
                tipo,
                serie_id_val,
                correlativo_val,
                req.monto_subtotal,
                req.monto_igv,
                req.monto_total,
            ],
        )?;

        let id_comprobante = conn.last_insert_rowid();

        // Registrar pago asociado
        conn.execute(
            "INSERT INTO pagos_comprobante (id_comprobante, medio_pago, monto)
             VALUES (?1, ?2, ?3)",
            params![
                id_comprobante,
                req.medio_pago.trim().to_uppercase(),
                req.monto_pago
            ],
        )?;

        // Registrar movimiento en caja si corresponde a venta efectiva
        if tipo != "PROFORMA" {
            conn.execute(
                "INSERT INTO movimientos_caja (id_caja, id_comprobante, tipo_movimiento, monto, descripcion, fecha_movimiento)
                 VALUES (?1, ?2, 'INGRESO_VENTA', ?3, ?4, datetime('now', 'localtime'))",
                params![
                    req.id_caja,
                    id_comprobante,
                    req.monto_total,
                    format!("Venta de comprobante ID {}", id_comprobante)
                ],
            )?;

            // GESTIÓN INTELIGENTE DE STOCK / KÁRDEX SEGÚN LA ORIGEN DE LA OT
            if let Some(id_ot_val) = req.id_ot {
                let mut stmt_prods = conn.prepare(
                    "SELECT id_detalle, id_producto, cantidad, precio_aplicado FROM detalle_ot_productos WHERE id_ot = ?1",
                )?;

                let prods_iter = stmt_prods.query_map([id_ot_val], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, f64>(2)?,
                        row.get::<_, i64>(3)?,
                    ))
                })?;

                for prod_res in prods_iter {
                    let (id_det_ot, id_prod, cant, p_unit) = prod_res?;

                    // Insertar en detalle_comprobante
                    conn.execute(
                        "INSERT INTO detalle_comprobante (id_comprobante, id_producto, cantidad, precio_unitario, subtotal)
                         VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![
                            id_comprobante,
                            id_prod,
                            cant,
                            p_unit,
                            (cant * p_unit as f64) as i64
                        ],
                    )?;

                    let id_det_comp = conn.last_insert_rowid();

                    // Verificar si el producto ya descontó stock cuando estuvo en el Taller
                    let mov_ot: Option<i64> = conn
                        .query_row(
                            "SELECT id_movimiento FROM movimientos_inventario WHERE id_detalle_ot = ?1",
                            [id_det_ot],
                            |row| row.get(0),
                        )
                        .optional()?;

                    if let Some(id_mov) = mov_ot {
                        // OT NUEVA: Ya descontó stock en el taller -> Vincular el movimiento existente al comprobante
                        conn.execute(
                            "UPDATE movimientos_inventario SET id_detalle_comprobante = ?1 WHERE id_movimiento = ?2",
                            params![id_det_comp, id_mov],
                        )?;
                    } else {
                        // OT ANTIGUA: No había descontado en taller -> Descontar stock ahora en POS
                        let stock_actual: f64 = conn.query_row(
                            "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                            [id_prod],
                            |row| row.get(0),
                        )?;

                        let stock_resultante = stock_actual - cant;

                        conn.execute(
                            "UPDATE productos SET stock_actual = ?1 WHERE id_producto = ?2",
                            params![stock_resultante, id_prod],
                        )?;

                        conn.execute(
                            "INSERT INTO movimientos_inventario (id_producto, id_usuario, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento)
                             VALUES (?1, ?2, ?3, 'EGRESO_VENTA', ?4, ?5, ?6, ?7, datetime('now', 'localtime'))",
                            params![
                                id_prod,
                                req.id_cajero,
                                id_det_comp,
                                stock_actual,
                                -cant,
                                stock_resultante,
                                format!("Venta desde OT Antigua {}", id_ot_val)
                            ],
                        )?;
                    }
                }
            }
        }

        Self::obtener_comprobante_por_id(conn, id_comprobante)
    }

    pub fn obtener_comprobante_por_id(
        conn: &Connection,
        id_comprobante: i64,
    ) -> Result<ComprobanteDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT c.id_comprobante, c.tipo_comprobante, s.serie, c.correlativo, c.monto_subtotal, c.monto_igv, c.monto_total, c.estado_sunat, c.fecha_emision
             FROM comprobantes c
             LEFT JOIN series_comprobante s ON c.id_serie = s.id_serie
             WHERE c.id_comprobante = ?1",
        )?;

        let entity = stmt
            .query_row([id_comprobante], |row| {
                Ok((
                    ComprobanteEntity {
                        id_comprobante: row.get(0)?,
                        tipo_comprobante: row.get(1)?,
                        id_serie: None,
                        correlativo: row.get(3)?,
                        monto_subtotal: row.get(4)?,
                        monto_igv: row.get(5)?,
                        monto_total: row.get(6)?,
                        estado_sunat: row.get(7)?,
                        fecha_emision: row.get(8)?,
                        id_ot: None,
                        id_cliente: 0,
                        id_cajero: 0,
                        id_caja: 0,
                    },
                    row.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(|_| {
                AppError::NotFound(format!("Comprobante ID {} no encontrado.", id_comprobante))
            })?;

        Ok(ComprobanteDto {
            id_comprobante: entity.0.id_comprobante,
            tipo_comprobante: entity.0.tipo_comprobante,
            serie: entity.1,
            correlativo: entity.0.correlativo,
            monto_subtotal: entity.0.monto_subtotal,
            monto_igv: entity.0.monto_igv,
            monto_total: entity.0.monto_total,
            estado_sunat: entity.0.estado_sunat,
            fecha_emision: entity.0.fecha_emision,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_emitir_comprobante_centimos_e_igv() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (1, 'Cajero Test', 'cajero1', 'hash', 'CAJERO', 1)",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, activo)
             VALUES ('DNI', '12345678', 'Cliente Comprobante', 1)",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO caja_chica (id_caja, id_usuario, monto_apertura, estado)
             VALUES (1, 1, 5000, 'ABIERTA')",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa)
             VALUES (1, 'BOLETA', 'B001', 0, 1)",
            [],
        )
        .unwrap();

        let req = EmitirComprobanteDto {
            id_ot: None,
            id_cliente: 1,
            id_cajero: 1,
            id_caja: 1,
            tipo_comprobante: "BOLETA".to_string(),
            id_serie: Some(1),
            monto_subtotal: 10000,
            monto_igv: 1800,
            monto_total: 11800,
            medio_pago: "EFECTIVO".to_string(),
            monto_pago: 11800,
        };

        let comp = ComprobanteService::emitir_comprobante(&conn, req).unwrap();
        assert_eq!(comp.correlativo, 1);
        assert_eq!(comp.monto_total, 11800);
        assert_eq!(comp.serie.unwrap(), "B001");
    }
}

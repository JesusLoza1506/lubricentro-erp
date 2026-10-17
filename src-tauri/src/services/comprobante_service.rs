use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::entities::comprobante::ComprobanteEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};
use uuid::Uuid;

pub struct ComprobanteService;

impl ComprobanteService {
    /// Emite un comprobante, actualiza la serie/correlativo y registra el pago
    pub fn emitir_comprobante(
        conn: &Connection,
        req: EmitirComprobanteDto,
    ) -> Result<ComprobanteDto, AppError> {
        if req.monto_total != req.monto_subtotal + req.monto_igv {
            return Err(AppError::Validation(
                "El monto total debe ser exactamente la suma del subtotal y el IGV.".to_string(),
            ));
        }

        let id_venta_real = req.id_ot.ok_or_else(|| {
            AppError::Validation("Se requiere el ID de la orden o venta asociada.".to_string())
        })?;

        let tipo = req.tipo_comprobante.trim().to_uppercase();
        if tipo != "BOLETA" && tipo != "FACTURA" {
            return Err(AppError::Validation(
                "Tipo de comprobante inválido. Debe ser BOLETA o FACTURA.".to_string(),
            ));
        }

        let correlativo_val: i64;
        let serie_id_val = req.id_serie;

        if let Some(id_s) = serie_id_val {
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

        let id_op_mov = Uuid::new_v4().to_string();
        let medio_pago = req.medio_pago.trim().to_uppercase();

        let (recibido_cent, vuelto_cent) = if medio_pago == "EFECTIVO" {
            (Some(req.monto_pago), Some(req.monto_pago - req.monto_total))
        } else {
            (None, None)
        };

        // 1. Registrar movimiento de cobro en caja usando el ID real de la orden/venta
        conn.execute(
            "INSERT INTO movimientos_caja (
                id_operacion, id_caja, id_usuario, tipo, medio_pago, monto_cent, signo,
                id_venta, recibido_cent, vuelto_cent, concepto, fecha_operacion
            ) VALUES (?1, ?2, ?3, 'COBRO', ?4, ?5, 1, ?6, ?7, ?8, ?9, datetime('now', '-5 hours'))",
            params![
                id_op_mov,
                req.id_caja,
                req.id_cajero,
                medio_pago,
                req.monto_total,
                id_venta_real,
                recibido_cent,
                vuelto_cent,
                format!("Cobro de venta ID {}", id_venta_real),
            ],
        )?;

        // 2. Confirmar la venta
        conn.execute(
            "UPDATE ventas SET estado_registro = 'CONFIRMADA' WHERE id_venta = ?1",
            [id_venta_real],
        )?;

        // 3. Insertar el comprobante
        let id_operacion = Uuid::new_v4().to_string();
        let payload_sha256 = "0000000000000000000000000000000000000000000000000000000000000000";

        conn.execute(
            "INSERT INTO comprobantes (
                id_operacion, id_venta, relacion, tipo_comprobante, ambiente,
                id_serie, correlativo, empresa_snapshot, receptor_snapshot,
                payload_json, payload_sha256, base_cent, igv_cent, total_cent, fecha_emision
            ) VALUES (?1, ?2, 'ORIGINAL', ?3, 'PRODUCCION', ?4, ?5, '{}', '{}', '{}', ?6, ?7, ?8, ?9, datetime('now', '-5 hours'))",
            params![
                id_operacion,
                id_venta_real,
                tipo,
                serie_id_val,
                correlativo_val,
                payload_sha256,
                req.monto_subtotal,
                req.monto_igv,
                req.monto_total,
            ],
        )?;

        let id_comprobante = conn.last_insert_rowid();

        Self::obtener_comprobante_por_id(conn, id_comprobante)
    }

    pub fn obtener_comprobante_por_id(
        conn: &Connection,
        id_comprobante: i64,
    ) -> Result<ComprobanteDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT c.id_comprobante, c.tipo_comprobante, s.serie, c.correlativo, c.base_cent, c.igv_cent, c.total_cent, c.fecha_emision
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
                        estado_sunat: "EMITIDO".to_string(),
                        fecha_emision: row.get(7)?,
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

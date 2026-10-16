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

        // 1. Registrar movimiento de cobro en caja
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
                req.id_ot.unwrap_or(1),
                recibido_cent,
                vuelto_cent,
                format!("Cobro de venta ID {}", req.id_ot.unwrap_or(1)),
            ],
        )?;

        // 2. Confirmar la venta
        if let Some(id_venta_val) = req.id_ot {
            conn.execute(
                "UPDATE ventas SET estado_registro = 'CONFIRMADA' WHERE id_venta = ?1",
                [id_venta_val],
            )?;
        }

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
                req.id_ot.unwrap_or(1),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use uuid::Uuid;

    const INITIAL_SCHEMA_V7: &str = include_str!("../../migrations/0001_initial_schema_v7.sql");
    const DUMMY_ARGON2_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

    fn setup_inmemory_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
        conn.execute_batch(INITIAL_SCHEMA_V7).unwrap();

        let _ = conn.execute(
            "UPDATE configuracion SET ambiente_actual = 'PRODUCCION' WHERE id_config = 1",
            [],
        );

        conn
    }

    #[test]
    fn test_emitir_comprobante_centimos_e_igv() {
        let conn = setup_inmemory_db();

        conn.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo)
             VALUES ('Admin Test', 'admin_comp_unique', ?1, 'ADMINISTRADOR', 1)",
            [DUMMY_ARGON2_HASH],
        )
        .unwrap();
        let id_admin_test = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social)
             VALUES ('DNI', '12345678', 'Cliente Comprobante')",
            [],
        )
        .unwrap();

        let id_operacion_caja = Uuid::new_v4().to_string();
        let codigo_caja = format!("CAJA-{}", Uuid::new_v4().to_string()[..8].to_uppercase());
        conn.execute(
            "INSERT INTO caja_chica (id_operacion, codigo_caja, id_usuario, ambiente, monto_apertura_cent, estado)
             VALUES (?1, ?2, ?3, 'PRODUCCION', 5000, 'ABIERTA')",
            params![id_operacion_caja, codigo_caja, id_admin_test],
        )
        .unwrap();
        let id_caja_test = conn.last_insert_rowid();

        let id_op_venta = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO ventas (
                id_operacion, id_cliente, receptor_snapshot, id_emisor, id_caja,
                ambiente, tipo_original, estado_registro, descuento_bp,
                bruto_cent, descuento_cent, base_cent, igv_cent, total_cent
            ) VALUES (?1, 1, '{\"nombre\":\"Cliente Comprobante\"}', ?2, ?3, 'PRODUCCION', 'BOLETA', 'PREPARANDO', 0, 11800, 0, 10000, 1800, 11800)",
            params![id_op_venta, id_admin_test, id_caja_test],
        )
        .unwrap();
        let id_venta_test = conn.last_insert_rowid();

        let id_op_ingreso = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO ingresos_stock (
                id_operacion, codigo, origen, referencia_inicial, estado, total_costo_micro, id_usuario
            ) VALUES (?1, 'ING-001', 'INICIAL', 'Carga inicial', 'PREPARANDO', 100000000, ?2)",
            params![id_op_ingreso, id_admin_test],
        )
        .unwrap();
        let id_ingreso_test = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO productos (codigo, nombre, categoria, unidad_medida, precio_venta_cent)
             VALUES ('PROD-01', 'Aceite Test', 'ACEITE', 'UND', 11800)",
            [],
        )
        .unwrap();
        let id_prod_test = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO detalle_ingreso (
                id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad,
                costo_efectivo_micro, valor_aporte_micro
            ) VALUES (?1, ?2, 'PROD-01', 'Aceite Test', 'UND', 10, 10000000, 100000000)",
            params![id_ingreso_test, id_prod_test],
        )
        .unwrap();
        let id_det_ingreso = conn.last_insert_rowid();

        let id_op_kardex_ingreso = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO movimientos_inventario (
                id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida,
                stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
                valor_resultante_micro, costo_unitario_micro
            ) VALUES (?1, ?2, ?3, 'STOCK_INICIAL', ?4, 'UND', 0, 10, 10, 0, 100000000, 100000000, 10000000)",
            params![id_op_kardex_ingreso, id_prod_test, id_admin_test, id_det_ingreso],
        )
        .unwrap();

        conn.execute(
            "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
            [id_ingreso_test],
        )
        .unwrap();

        // 6. Detalle Venta
        conn.execute(
            "INSERT INTO detalle_venta (
                id_venta, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida,
                cantidad, precio_unitario_cent, bruto_cent, descuento_cent, base_cent, igv_cent, igv_bp, total_cent, costo_unitario_micro, costo_total_micro
            ) VALUES (?1, ?2, 'PROD-01', 'Aceite Test', 'UND', 1, 11800, 11800, 0, 10000, 1800, 1800, 11800, 10000000, 10000000)",
            params![id_venta_test, id_prod_test],
        )
        .unwrap();
        let id_det_venta = conn.last_insert_rowid();

        // Sincronizar el stock del producto en la tabla antes del kárdex para satisfacer el trigger
        conn.execute(
            "UPDATE productos SET stock_actual = 10, valor_stock_micro = 100000000 WHERE id_producto = ?1",
            [id_prod_test],
        ).unwrap();

        // 7. Salida Kárdex
        let id_op_kardex_venta = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO movimientos_inventario (
                id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_venta, unidad_medida,
                stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
                valor_resultante_micro, costo_unitario_micro
            ) VALUES (?1, ?2, ?3, 'VENTA_DIRECTA', ?4, 'UND', 10, -1, 9, 100000000, -10000000, 90000000, 10000000)",
            params![id_op_kardex_venta, id_prod_test, id_admin_test, id_det_venta],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO series_comprobante (id_serie, ambiente, tipo_comprobante, serie, correlativo_actual, activa)
             VALUES (1, 'PRODUCCION', 'BOLETA', 'B001', 0, 1)",
            [],
        )
        .unwrap();

        let req = EmitirComprobanteDto {
            id_ot: Some(id_venta_test),
            id_cliente: 1,
            id_cajero: id_admin_test,
            id_caja: id_caja_test,
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

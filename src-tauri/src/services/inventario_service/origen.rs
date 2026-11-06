use super::InventarioService;
use crate::dto::producto_dto::{ComprobanteOrigenKardexDto, LineaOrigenKardexDto, OrigenKardexDto};
use crate::errors::AppError;
use rusqlite::{Connection, OptionalExtension};

type Vinculos = (Option<i64>, Option<i64>, Option<i64>, Option<i64>);
type Cabecera = (String, String, String, Option<String>, Option<String>);

impl InventarioService {
    /// Consulta de solo lectura. El comando exige sesión administrativa.
    /// Resuelve el origen desde la FK del movimiento, nunca desde un ID enviado por React.
    pub fn obtener_origen_kardex(
        conn: &Connection,
        id_movimiento: i64,
    ) -> Result<OrigenKardexDto, AppError> {
        let vinculos: Vinculos = conn
            .query_row(
                "SELECT di.id_ingreso, dop.id_ot, dv.id_venta, dd.id_devolucion
             FROM movimientos_inventario m
             LEFT JOIN detalle_ingreso di ON di.id_detalle = m.id_detalle_ingreso
             LEFT JOIN detalle_ot_productos dop ON dop.id_detalle = m.id_consumo_ot
             LEFT JOIN detalle_venta dv ON dv.id_detalle = m.id_detalle_venta
             LEFT JOIN detalle_devolucion dd ON dd.id_detalle = m.id_detalle_devolucion
             WHERE m.id_movimiento = ?1",
                [id_movimiento],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .optional()?
            .ok_or_else(|| AppError::NotFound("Movimiento de inventario no encontrado.".into()))?;
        let (tipo, id, cabecera_sql, lineas_sql) = if let Some(id) = vinculos.0 {
            ("INGRESO", id,
             "SELECT codigo, fecha_registro, estado, COALESCE(numero_documento, referencia_inicial), COALESCE(motivo_anulacion, observacion) FROM ingresos_stock WHERE id_ingreso = ?1",
             "SELECT nombre_snapshot, unidad_medida, cantidad, valor_aporte_micro FROM detalle_ingreso WHERE id_ingreso = ?1 ORDER BY id_detalle")
        } else if let Some(id) = vinculos.1 {
            ("OT", id,
             "SELECT codigo_ot, fecha_ingreso, estado, placa_snapshot, COALESCE(resumen_publico, pedido_cliente) FROM ordenes_trabajo WHERE id_ot = ?1",
             "SELECT nombre_snapshot, unidad_medida, cantidad, valor_consumido_micro FROM detalle_ot_productos WHERE id_ot = ?1 ORDER BY id_detalle")
        } else if let Some(id) = vinculos.3 {
            ("DEVOLUCION", id,
             "SELECT 'Devolución #' || id_devolucion, fecha_operacion, estado_registro, 'Venta #' || id_venta, motivo FROM devoluciones WHERE id_devolucion = ?1",
             "SELECT dv.nombre_snapshot, dv.unidad_medida, dd.cantidad_recuperada, dd.costo_revertido_micro FROM detalle_devolucion dd JOIN detalle_venta dv ON dv.id_detalle = dd.id_detalle_venta WHERE dd.id_devolucion = ?1 ORDER BY dd.id_detalle")
        } else if let Some(id) = vinculos.2 {
            ("VENTA", id,
             "SELECT 'Venta #' || id_venta, fecha_venta, estado_registro, tipo_original, motivo_descuento FROM ventas WHERE id_venta = ?1",
             "SELECT nombre_snapshot, unidad_medida, cantidad, costo_total_micro FROM detalle_venta WHERE id_venta = ?1 ORDER BY id_detalle")
        } else {
            return Err(AppError::Validation("El ajuste manual no tiene un documento de origen; consulte su motivo y usuario en el Kardex.".into()));
        };
        let (codigo, fecha, estado, documento, resumen): Cabecera =
            conn.query_row(cabecera_sql, [id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
            })?;
        let mut stmt = conn.prepare(lineas_sql)?;
        let lineas = stmt
            .query_map([id], |r| {
                Ok(LineaOrigenKardexDto {
                    nombre: r.get(0)?,
                    unidad_medida: r.get(1)?,
                    cantidad: r.get(2)?,
                    valor_micro: r.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let mut comprobantes = Vec::new();
        if tipo == "VENTA" || tipo == "DEVOLUCION" {
            let mut stmt = conn.prepare(
                "SELECT c.id_comprobante, s.serie || '-' || printf('%08d', c.correlativo),
                        c.tipo_comprobante, c.estado_envio, c.total_cent
                 FROM comprobantes c JOIN series_comprobante s ON s.id_serie = c.id_serie
                 WHERE (?1 = 'VENTA' AND c.id_venta = ?2)
                    OR (?1 = 'DEVOLUCION' AND c.id_devolucion = ?2)
                 ORDER BY c.id_comprobante",
            )?;
            comprobantes = stmt
                .query_map(rusqlite::params![tipo, id], |r| {
                    Ok(ComprobanteOrigenKardexDto {
                        id_comprobante: r.get(0)?,
                        numero: r.get(1)?,
                        tipo_comprobante: r.get(2)?,
                        estado_envio: r.get(3)?,
                        total_cent: r.get(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(OrigenKardexDto {
            tipo: tipo.into(),
            codigo,
            fecha,
            estado,
            documento,
            resumen,
            lineas,
            comprobantes,
        })
    }
}

use super::{InventarioService, ResumenSaldosKardexDto};
use crate::dto::producto_dto::MovimientoKardexDto;
use crate::errors::AppError;
use rusqlite::{Connection, OptionalExtension};

impl InventarioService {
    pub fn listar_kardex_producto(
        conn: &Connection,
        id_producto: i64,
        limite: i64,
        offset: i64,
        fecha_inicio: Option<String>,
        fecha_fin: Option<String>,
        tipo_movimiento: Option<String>,
    ) -> Result<Vec<MovimientoKardexDto>, AppError> {
        validar_fechas_kardex(fecha_inicio.as_deref(), fecha_fin.as_deref())?;
        if !(1..=100).contains(&limite) || offset < 0 {
            return Err(AppError::Validation(
                "Paginación de Kardex inválida.".into(),
            ));
        }

        let mut query = String::from(
            "SELECT m.id_movimiento, m.id_producto, m.tipo_movimiento, m.unidad_medida,
                    m.stock_anterior, m.diferencia, m.stock_resultante,
                    m.valor_anterior_micro, m.valor_delta_micro, m.valor_resultante_micro,
                    m.costo_unitario_micro, 
                    CASE 
                        WHEN ing.codigo IS NOT NULL THEN '[' || COALESCE(ing.origen, 'COMPRA') || ': ' || ing.codigo || '] ' || COALESCE(m.motivo, '')
                        ELSE m.motivo 
                    END as motivo_completo,
                    m.fecha_movimiento, u.nombre_completo,
                    ing.id_ingreso, ot.id_ot, COALESCE(v.id_venta, vd.id_venta), dev.id_devolucion,
                    COALESCE(ing.codigo, ot.codigo_ot,
                        CASE WHEN dev.id_devolucion IS NOT NULL THEN 'Devolución #' || dev.id_devolucion END,
                        CASE WHEN v.id_venta IS NOT NULL THEN 'Venta #' || v.id_venta END)
             FROM movimientos_inventario m
             LEFT JOIN usuarios u ON u.id_usuario = m.id_usuario
             LEFT JOIN detalle_ingreso det ON det.id_detalle = m.id_detalle_ingreso
             LEFT JOIN ingresos_stock ing ON ing.id_ingreso = det.id_ingreso
             LEFT JOIN detalle_ot_productos consumo ON consumo.id_detalle = m.id_consumo_ot
             LEFT JOIN ordenes_trabajo ot ON ot.id_ot = consumo.id_ot
             LEFT JOIN detalle_venta dv ON dv.id_detalle = m.id_detalle_venta
             LEFT JOIN ventas v ON v.id_venta = dv.id_venta
             LEFT JOIN detalle_devolucion dd ON dd.id_detalle = m.id_detalle_devolucion
             LEFT JOIN devoluciones dev ON dev.id_devolucion = dd.id_devolucion
             LEFT JOIN ventas vd ON vd.id_venta = dev.id_venta
             WHERE m.id_producto = ?1",
        );

        let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(id_producto)];

        if let Some(ref fi) = fecha_inicio {
            if !fi.trim().is_empty() {
                query_params.push(Box::new(format!("{} 00:00:00", fi.trim())));
                query.push_str(&format!(
                    " AND m.fecha_movimiento >= ?{}",
                    query_params.len()
                ));
            }
        }

        if let Some(ref ff) = fecha_fin {
            if !ff.trim().is_empty() {
                query_params.push(Box::new(format!("{} 23:59:59", ff.trim())));
                query.push_str(&format!(
                    " AND m.fecha_movimiento <= ?{}",
                    query_params.len()
                ));
            }
        }

        if let Some(ref tm) = tipo_movimiento {
            if !tm.trim().is_empty() && tm.trim().to_uppercase() != "TODOS" {
                query_params.push(Box::new(tm.trim().to_uppercase()));
                query.push_str(&format!(" AND m.tipo_movimiento = ?{}", query_params.len()));
            }
        }

        query_params.push(Box::new(limite));
        let limit_idx = query_params.len();
        query_params.push(Box::new(offset));
        let offset_idx = query_params.len();

        query.push_str(&format!(
            " ORDER BY m.fecha_movimiento DESC, m.id_movimiento DESC LIMIT ?{} OFFSET ?{}",
            limit_idx, offset_idx
        ));

        let mut stmt = conn.prepare(&query)?;
        let params_slice: Vec<&dyn rusqlite::ToSql> =
            query_params.iter().map(|p| p.as_ref()).collect();

        let iter = stmt.query_map(params_slice.as_slice(), |r| {
            Ok(MovimientoKardexDto {
                id_movimiento: r.get(0)?,
                id_producto: r.get(1)?,
                tipo_movimiento: r.get(2)?,
                unidad_medida: r.get(3)?,
                stock_anterior: r.get(4)?,
                diferencia: r.get(5)?,
                stock_resultante: r.get(6)?,
                valor_anterior_micro: r.get(7)?,
                valor_delta_micro: r.get(8)?,
                valor_resultante_micro: r.get(9)?,
                costo_unitario_micro: r.get(10)?,
                motivo: r.get(11)?,
                fecha_movimiento: r.get(12)?,
                usuario_nombre: r.get(13)?,
                id_ingreso: r.get(14)?,
                id_ot: r.get(15)?,
                id_venta: r.get(16)?,
                id_devolucion: r.get(17)?,
                codigo_origen: r.get(18)?,
            })
        })?;

        let mut lista = Vec::new();
        for item in iter {
            lista.push(item?);
        }
        Ok(lista)
    }

    pub fn obtener_resumen_saldos_kardex(
        conn: &Connection,
        id_producto: i64,
        fecha_inicio: Option<String>,
        fecha_fin: Option<String>,
        _tipo_movimiento: Option<String>,
    ) -> Result<ResumenSaldosKardexDto, AppError> {
        validar_fechas_kardex(fecha_inicio.as_deref(), fecha_fin.as_deref())?;
        // Los saldos son físicos del período completo, no sumas parciales por tipo.
        let lectura = conn.unchecked_transaction()?;
        let conn = &lectura;
        let saldo_actual: i64 = conn.query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = ?1",
            [id_producto],
            |r| r.get(0),
        )?;
        let inicio = fecha_inicio
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(|v| format!("{} 00:00:00", v.trim()));
        let fin = fecha_fin
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(|v| format!("{} 23:59:59", v.trim()));
        let saldo_inicial = if let Some(inicio) = inicio {
            conn.query_row(
                "SELECT stock_resultante FROM movimientos_inventario
                 WHERE id_producto = ?1 AND fecha_movimiento < ?2
                 ORDER BY fecha_movimiento DESC, id_movimiento DESC LIMIT 1",
                rusqlite::params![id_producto, inicio],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or(0)
        } else {
            conn.query_row(
                "SELECT stock_anterior FROM movimientos_inventario WHERE id_producto = ?1
                 ORDER BY fecha_movimiento ASC, id_movimiento ASC LIMIT 1",
                [id_producto],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or(0)
        };
        let saldo_final = conn
            .query_row(
                "SELECT stock_resultante FROM movimientos_inventario
             WHERE id_producto = ?1 AND (?2 IS NULL OR fecha_movimiento <= ?2)
             ORDER BY fecha_movimiento DESC, id_movimiento DESC LIMIT 1",
                rusqlite::params![id_producto, fin],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or(0);
        Ok(ResumenSaldosKardexDto {
            saldo_inicial,
            saldo_final,
            saldo_actual,
        })
    }
}

fn validar_fechas_kardex(inicio: Option<&str>, fin: Option<&str>) -> Result<(), AppError> {
    let parsear = |valor: Option<&str>| -> Result<Option<chrono::NaiveDate>, AppError> {
        valor
            .filter(|v| !v.trim().is_empty())
            .map(|v| {
                chrono::NaiveDate::parse_from_str(v.trim(), "%Y-%m-%d").map_err(|_| {
                    AppError::Validation("Fecha de Kardex inválida. Use AAAA-MM-DD.".into())
                })
            })
            .transpose()
    };
    if let (Some(a), Some(b)) = (parsear(inicio)?, parsear(fin)?) {
        if b < a || (b - a).num_days() > 366 {
            return Err(AppError::Validation("El rango de consulta del Kárdex debe estar ordenado y no puede ser superior a 366 días.".into()));
        }
    }
    Ok(())
}

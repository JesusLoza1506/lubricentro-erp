use super::{InventarioService, ResumenSaldosKardexDto};
use crate::dto::producto_dto::MovimientoKardexDto;
use crate::errors::AppError;
use rusqlite::Connection;

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
        if let (Some(fi), Some(ff)) = (&fecha_inicio, &fecha_fin) {
            if !fi.trim().is_empty() && !ff.trim().is_empty() {
                if let (Ok(d_start), Ok(d_end)) = (
                    chrono::NaiveDate::parse_from_str(fi.trim(), "%Y-%m-%d"),
                    chrono::NaiveDate::parse_from_str(ff.trim(), "%Y-%m-%d"),
                ) {
                    if (d_end - d_start).num_days() > 366 {
                        return Err(AppError::Validation(
                            "El rango de consulta del Kárdex no puede ser superior a 366 días."
                                .to_string(),
                        ));
                    }
                }
            }
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
                    m.fecha_movimiento, u.nombre_completo
             FROM movimientos_inventario m
             LEFT JOIN usuarios u ON u.id_usuario = m.id_usuario
             LEFT JOIN detalle_ingreso det ON det.id_detalle = m.id_detalle_ingreso
             LEFT JOIN ingresos_stock ing ON ing.id_ingreso = det.id_ingreso
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
            " ORDER BY m.id_movimiento DESC LIMIT ?{} OFFSET ?{}",
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
        tipo_movimiento: Option<String>,
    ) -> Result<ResumenSaldosKardexDto, AppError> {
        let saldo_actual: i64 = conn
            .query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                [id_producto],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let fecha_primer_mov: Option<String> = conn
            .query_row(
                "SELECT fecha_movimiento FROM movimientos_inventario WHERE id_producto = ?1 ORDER BY id_movimiento ASC LIMIT 1",
                [id_producto],
                |r| r.get(0),
            )
            .ok();

        if let (Some(ref ff), Some(ref f_primer)) = (&fecha_fin, &fecha_primer_mov) {
            let ff_str = format!("{} 23:59:59", ff.trim());
            if !ff.trim().is_empty() && ff_str < *f_primer {
                return Ok(ResumenSaldosKardexDto {
                    saldo_inicial: 0,
                    saldo_final: 0,
                    saldo_actual,
                });
            }
        }

        let mut query_base = String::from(
            "SELECT stock_anterior, stock_resultante FROM movimientos_inventario WHERE id_producto = ?1"
        );
        let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(id_producto)];

        if let Some(ref fi) = fecha_inicio {
            if !fi.trim().is_empty() {
                query_params.push(Box::new(format!("{} 00:00:00", fi.trim())));
                query_base.push_str(&format!(" AND fecha_movimiento >= ?{}", query_params.len()));
            }
        }

        if let Some(ref ff) = fecha_fin {
            if !ff.trim().is_empty() {
                query_params.push(Box::new(format!("{} 23:59:59", ff.trim())));
                query_base.push_str(&format!(" AND fecha_movimiento <= ?{}", query_params.len()));
            }
        }

        if let Some(ref tm) = tipo_movimiento {
            if !tm.trim().is_empty() && tm.trim().to_uppercase() != "TODOS" {
                query_params.push(Box::new(tm.trim().to_uppercase()));
                query_base.push_str(&format!(" AND tipo_movimiento = ?{}", query_params.len()));
            }
        }

        let params_slice: Vec<&dyn rusqlite::ToSql> =
            query_params.iter().map(|p| p.as_ref()).collect();

        let query_first = format!("{} ORDER BY id_movimiento ASC LIMIT 1", query_base);
        let primer_mov: Option<(i64, i64)> = conn
            .query_row(&query_first, params_slice.as_slice(), |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .ok();

        let query_last = format!("{} ORDER BY id_movimiento DESC LIMIT 1", query_base);
        let ultimo_mov: Option<(i64, i64)> = conn
            .query_row(&query_last, params_slice.as_slice(), |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .ok();

        let (saldo_inicial, saldo_final) = match (primer_mov, ultimo_mov) {
            (Some(f), Some(l)) => (f.0, l.1),
            _ => {
                let mut last_before_query = String::from(
                    "SELECT stock_resultante FROM movimientos_inventario WHERE id_producto = ?1",
                );
                let mut params_before: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(id_producto)];

                if let Some(ref fi) = fecha_inicio {
                    if !fi.trim().is_empty() {
                        params_before.push(Box::new(format!("{} 00:00:00", fi.trim())));
                        last_before_query
                            .push_str(&format!(" AND fecha_movimiento < ?{}", params_before.len()));
                    }
                }
                last_before_query.push_str(" ORDER BY id_movimiento DESC LIMIT 1");

                let params_before_slice: Vec<&dyn rusqlite::ToSql> =
                    params_before.iter().map(|p| p.as_ref()).collect();

                let stock_arrastrado: i64 = conn
                    .query_row(&last_before_query, params_before_slice.as_slice(), |r| {
                        r.get(0)
                    })
                    .unwrap_or(0);

                (stock_arrastrado, stock_arrastrado)
            }
        };

        Ok(ResumenSaldosKardexDto {
            saldo_inicial,
            saldo_final,
            saldo_actual,
        })
    }
}

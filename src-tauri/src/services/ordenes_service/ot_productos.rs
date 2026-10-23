use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{
    DevolverInsumoOtDto, OrdenTrabajoHistorialDto, RegistrarInsumoOtDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection};
use uuid::Uuid;

pub fn registrar_insumo_ot(
    conn: &mut Connection,
    req: RegistrarInsumoOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;

    if ot.estado != "EN_PROCESO" {
        return Err(AppError::Validation(
            "Solo se pueden registrar consumos de insumos en órdenes 'EN_PROCESO'.".to_string(),
        ));
    }

    if req.cantidad <= 0.0 {
        return Err(AppError::Validation(
            "La cantidad de consumo debe ser mayor a 0.".to_string(),
        ));
    }

    let (precio_cent, stock_actual, nombre, codigo, unidad_medida, valor_stock_micro, ultimo_costo_micro): (
        i64,
        i64,
        String,
        String,
        String,
        i64,
        Option<i64>,
    ) = conn
        .query_row(
            "SELECT precio_venta_cent, stock_actual, nombre, codigo, unidad_medida, valor_stock_micro, ultimo_costo_micro FROM productos WHERE id_producto = ?1 AND activo = 1",
            [req.id_producto],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?)),
        )
        .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

    let cant_base: i64 = if unidad_medida == "ML" {
        (req.cantidad * 1000.0).round() as i64
    } else {
        req.cantidad.round() as i64
    };

    if stock_actual < cant_base {
        return Err(AppError::Validation(format!(
            "Stock insuficiente para '{}'. Disponible: {}, Solicitado: {}",
            nombre, stock_actual, cant_base
        )));
    }

    let costo_unitario: i64 = match ultimo_costo_micro {
        Some(c) if c >= 1 => c,
        _ => {
            if stock_actual > 0 && valor_stock_micro > 0 {
                valor_stock_micro / stock_actual
            } else {
                50000000
            }
        }
    }
    .max(1);

    let valor_consumido = costo_unitario * cant_base;
    let id_op_det = Uuid::new_v4().to_string();

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tx.execute(
        "INSERT INTO detalle_ot_productos (
            id_operacion, id_ot, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida,
            cantidad, precio_unitario_cent, costo_unitario_micro, valor_consumido_micro, id_usuario
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            id_op_det,
            req.id_ot,
            req.id_producto,
            codigo,
            nombre,
            unidad_medida,
            cant_base,
            precio_cent,
            costo_unitario,
            valor_consumido,
            id_usuario
        ],
    )?;

    let id_detalle_ot = tx.last_insert_rowid();
    let id_op_kardex = Uuid::new_v4().to_string();

    let nuevo_stock = stock_actual - cant_base;
    let nuevo_valor_stock = (valor_stock_micro - valor_consumido).max(0);

    // 1. Primero se inserta el Kardex
    tx.execute(
        "INSERT INTO movimientos_inventario (
            id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
            stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
            valor_resultante_micro, costo_unitario_micro
        ) VALUES (?1, ?2, ?3, 'CONSUMO_OT', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            id_op_kardex,
            req.id_producto,
            id_usuario,
            id_detalle_ot,
            unidad_medida,
            stock_actual,
            -cant_base,
            nuevo_stock,
            valor_stock_micro,
            -valor_consumido,
            nuevo_valor_stock,
            costo_unitario
        ],
    )?;

    // 2. Luego se actualiza la tabla productos (satisface trg_stock_directo_no_bypass)
    tx.execute(
        "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2 WHERE id_producto = ?3",
        params![nuevo_stock, nuevo_valor_stock, req.id_producto],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

pub fn devolver_insumo_ot(
    conn: &mut Connection,
    req: DevolverInsumoOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let motivo_clean = req.motivo.trim();
    if motivo_clean.len() < 5 || motivo_clean.len() > 250 {
        return Err(AppError::Validation(
            "El motivo de devolución debe tener entre 5 y 250 caracteres.".to_string(),
        ));
    }

    let ot = obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se puede devolver insumos en órdenes finalizadas o canceladas.".to_string(),
        ));
    }

    let (id_producto, unidad_medida, cant_consumida, costo_unitario): (i64, String, i64, i64) = conn
        .query_row(
            "SELECT id_producto, unidad_medida, cantidad, costo_unitario_micro FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
            params![req.id_detalle_producto, req.id_ot],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| AppError::NotFound("No se encontró el registro de consumo de producto.".to_string()))?;

    let cant_dev_base: i64 = if unidad_medida == "ML" {
        (req.cantidad_a_devolver * 1000.0).round() as i64
    } else {
        req.cantidad_a_devolver.round() as i64
    };

    let devuelto_previo: i64 = conn.query_row(
        "SELECT COALESCE(SUM(diferencia), 0) FROM movimientos_inventario WHERE id_consumo_ot = ?1 AND tipo_movimiento = 'REPOSICION_OT'",
        [req.id_detalle_producto],
        |row| row.get(0),
    )?;

    let neto_actual = cant_consumida - devuelto_previo;
    if cant_dev_base <= 0 || cant_dev_base > neto_actual {
        return Err(AppError::Validation(format!(
            "La cantidad a devolver debe ser mayor a 0 y no superar el neto disponible ({})",
            neto_actual
        )));
    }

    let (stock_actual, valor_stock_micro): (i64, i64) = conn.query_row(
        "SELECT stock_actual, valor_stock_micro FROM productos WHERE id_producto = ?1",
        [id_producto],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    let valor_reposicion = costo_unitario * cant_dev_base;
    let nuevo_stock = stock_actual + cant_dev_base;
    let nuevo_valor_stock = valor_stock_micro + valor_reposicion;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let id_op_kardex = Uuid::new_v4().to_string();

    // 1. Insertar Kardex primero
    tx.execute(
        "INSERT INTO movimientos_inventario (
            id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
            stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
            valor_resultante_micro, costo_unitario_micro, motivo
        ) VALUES (?1, ?2, ?3, 'REPOSICION_OT', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            id_op_kardex,
            id_producto,
            id_usuario,
            req.id_detalle_producto,
            unidad_medida,
            stock_actual,
            cant_dev_base,
            nuevo_stock,
            valor_stock_micro,
            valor_reposicion,
            nuevo_valor_stock,
            costo_unitario,
            motivo_clean
        ],
    )?;

    // 2. Actualizar productos después (cumple trg_stock_directo_no_bypass)
    tx.execute(
        "UPDATE productos SET stock_actual = ?1, valor_stock_micro = ?2 WHERE id_producto = ?3",
        params![nuevo_stock, nuevo_valor_stock, id_producto],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, req.id_ot, id_usuario, rol)
}

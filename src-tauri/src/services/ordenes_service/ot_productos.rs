use super::helpers::validar_permiso_operacion_ot;
use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{AgregarProductoOtDto, OrdenTrabajoHistorialDto};
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

pub fn agregar_producto_ot(
    conn: &mut Connection,
    req: AgregarProductoOtDto,
    id_usuario_ejecutor: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    validar_permiso_operacion_ot(conn, req.id_ot, id_usuario_ejecutor)?;

    let ot = obtener_ot_por_id(conn, req.id_ot)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se pueden agregar productos a una orden finalizada o cancelada.".to_string(),
        ));
    }

    if req.cantidad <= 0.0 {
        return Err(AppError::Validation(
            "La cantidad debe ser mayor a 0.".to_string(),
        ));
    }

    let (precio_venta_cent, stock_actual, nombre_prod, codigo_prod, unidad_medida): (i64, i64, String, String, String) = conn
        .query_row(
            "SELECT precio_venta_cent, stock_actual, nombre, codigo, unidad_medida FROM productos WHERE id_producto = ?1 AND activo = 1",
            [req.id_producto],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

    let cant_entera = req.cantidad as i64;
    if stock_actual < cant_entera {
        return Err(AppError::Validation(format!(
            "Stock insuficiente para '{}'. Disponible: {}, solicitado: {}",
            nombre_prod, stock_actual, cant_entera
        )));
    }

    let id_op_det = Uuid::new_v4().to_string();

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tx.execute(
        "INSERT INTO detalle_ot_productos (
            id_operacion, id_ot, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida,
            cantidad, precio_unitario_cent, costo_unitario_micro, valor_consumido_micro, id_usuario
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, 1, ?9)",
        params![
            id_op_det,
            req.id_ot,
            req.id_producto,
            codigo_prod,
            nombre_prod,
            unidad_medida,
            cant_entera,
            precio_venta_cent,
            id_usuario_ejecutor
        ],
    )?;

    let id_detalle_ot = tx.last_insert_rowid();
    let id_op_kardex = Uuid::new_v4().to_string();

    tx.execute(
        "INSERT INTO movimientos_inventario (
            id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
            stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
            valor_resultante_micro, costo_unitario_micro
        ) VALUES (?1, ?2, ?3, 'CONSUMO_OT', ?4, ?5, ?6, ?7, ?8, 1, -1, 0, 1)",
        params![
            id_op_kardex,
            req.id_producto,
            id_usuario_ejecutor,
            id_detalle_ot,
            unidad_medida,
            stock_actual,
            -cant_entera,
            stock_actual - cant_entera
        ],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, req.id_ot)
}

pub fn eliminar_producto_ot(
    conn: &mut Connection,
    id_detalle: i64,
    id_ot: i64,
    id_usuario_ejecutor: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    validar_permiso_operacion_ot(conn, id_ot, id_usuario_ejecutor)?;

    let ot = obtener_ot_por_id(conn, id_ot)?;
    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se pueden eliminar productos de una orden finalizada o cancelada.".to_string(),
        ));
    }

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let detalle_opt: Option<(i64, i64, String)> = tx
        .query_row(
            "SELECT id_producto, cantidad, unidad_medida FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
            params![id_detalle, id_ot],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()?;

    if let Some((id_prod, cant, unidad_medida)) = detalle_opt {
        let stock_actual: i64 = tx.query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = ?1",
            [id_prod],
            |row| row.get(0),
        )?;

        let stock_resultante = stock_actual + cant;
        let id_op_kardex = Uuid::new_v4().to_string();

        tx.execute(
            "INSERT INTO movimientos_inventario (
                id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
                stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
                valor_resultante_micro, costo_unitario_micro, motivo
            ) VALUES (?1, ?2, ?3, 'REPOSICION_OT', ?4, ?5, ?6, ?7, ?8, 0, 1, 1, 1, 'Devolución de producto por eliminación de detalle en OT')",
            params![
                id_op_kardex,
                id_prod,
                id_usuario_ejecutor,
                id_detalle,
                unidad_medida,
                stock_actual,
                cant,
                stock_resultante
            ],
        )?;

        tx.execute(
            "DELETE FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
            params![id_detalle, id_ot],
        )?;
    }

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, id_ot)
}

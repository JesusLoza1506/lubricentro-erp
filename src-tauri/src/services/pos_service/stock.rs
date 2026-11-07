use super::{calculos::redondear, error};
use crate::errors::AppError;
use rusqlite::{params, Connection};
pub fn valor_salida(q: i64, v: i64, cantidad: i64) -> Result<(i64, i64), AppError> {
    if cantidad <= 0 || cantidad > q || v <= 0 {
        return Err(error("Stock insuficiente o valoración inválida."));
    }
    let valor = if q == cantidad {
        v
    } else {
        redondear(v as i128 * cantidad as i128, q as i128)
    };
    if valor <= 0 || (cantidad < q && valor >= v) {
        return Err(error(
            "La cantidad produce una valoración no representable.",
        ));
    }
    Ok((redondear(v as i128, q as i128).max(1), valor))
}
#[allow(clippy::too_many_arguments)]
pub fn mover(
    conn: &Connection,
    actor: i64,
    op: &str,
    producto: i64,
    tipo: &str,
    linea: i64,
    delta: i64,
    valor: i64,
    costo: i64,
) -> Result<(), AppError> {
    let (q, v, u): (i64, i64, String) = conn.query_row(
        "SELECT stock_actual,valor_stock_micro,unidad_medida FROM productos WHERE id_producto=?1",
        [producto],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )?;
    let nq = q
        .checked_add(delta)
        .ok_or_else(|| error("Cantidad fuera de rango."))?;
    let nv = v
        .checked_add(valor)
        .ok_or_else(|| error("Valor fuera de rango."))?;
    if nq < 0 || nv < 0 || (nq == 0) != (nv == 0) {
        return Err(error("El movimiento no conserva un saldo válido."));
    }
    conn.execute("INSERT INTO movimientos_inventario(id_operacion,id_producto,id_usuario,tipo_movimiento,id_detalle_venta,id_detalle_devolucion,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",params![op,producto,actor,tipo,if tipo=="VENTA_DIRECTA"{Some(linea)}else{None},if tipo=="DEVOLUCION_VENTA"{Some(linea)}else{None},u,q,delta,nq,v,valor,nv,costo])?;
    conn.execute("UPDATE productos SET stock_actual=?1,valor_stock_micro=?2,version=version+1 WHERE id_producto=?3",params![nq,nv,producto])?;
    Ok(())
}

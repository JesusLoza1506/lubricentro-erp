use super::{
    auditar, caja_propia,
    calculos::{calcular, publicas, LineaCalculada},
    configuracion, consultas, error, motivo, stock, validar_config, validar_rol,
};
use crate::{
    dto::pos_dto::ConfirmarVentaPos, errors::AppError,
    services::inventario_service::idempotencia::IdempotenciaService,
};
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::collections::HashSet;

pub fn preparar(
    conn: &Connection,
    req: &ConfirmarVentaPos,
) -> Result<(Vec<LineaCalculada>, Value), AppError> {
    if req.lineas.len() > 100 {
        return Err(error("Máximo 100 líneas directas."));
    }
    if !["BOLETA", "FACTURA"].contains(&req.tipo_comprobante.as_str()) {
        return Err(error("Selecciona Boleta o Factura."));
    }
    let mut l = Vec::new();
    if let Some(ot) = req.id_ot {
        let existe:i64=conn.query_row("SELECT count(*) FROM ordenes_trabajo o WHERE id_ot=?1 AND estado='FINALIZADO' AND NOT EXISTS(SELECT 1 FROM ventas WHERE id_ot=o.id_ot)",[ot],|r|r.get(0))?;
        if existe != 1 {
            return Err(error("La OT no está finalizada o ya tiene una venta."));
        }
        l = lineas_ot(conn, ot)?;
    }
    let mut keys = HashSet::new();
    for d in &req.lineas {
        if !keys.insert((d.tipo.clone(), d.id)) {
            return Err(error(
                "Combina las cantidades de productos o servicios directos repetidos.",
            ));
        }
        if d.tipo == "PRODUCTO" {
            let(c,n,u,p,q,v):(String,String,String,i64,i64,i64)=conn.query_row("SELECT codigo,nombre,unidad_medida,precio_venta_cent,stock_actual,valor_stock_micro FROM productos WHERE id_producto=?1 AND activo=1",[d.id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?)))?;
            validar_cantidad_directa(&u, d.cantidad)?;
            if p != d.precio_esperado_cent {
                return Err(error(
                    "El precio de un producto cambió. Retira esa línea y vuelve a agregarla desde el catálogo actualizado para revisar el precio.",
                ));
            }
            let (cu, cv) = stock::valor_salida(q, v, d.cantidad)?;
            l.push(linea(
                Some(d.id),
                None,
                None,
                None,
                c,
                n,
                u,
                d.cantidad,
                p,
                Some(cu),
                Some(cv),
            ));
        } else if d.tipo == "SERVICIO" {
            validar_cantidad_directa("UND", d.cantidad)?;
            let(c,n,p):(String,String,i64)=conn.query_row("SELECT codigo,nombre,precio_venta_cent FROM servicios WHERE id_servicio=?1 AND activo=1",[d.id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?)))?;
            if p != d.precio_esperado_cent {
                return Err(error(
                    "El precio de un servicio cambió. Retira esa línea y vuelve a agregarla desde el catálogo actualizado para revisar el precio.",
                ));
            }
            l.push(linea(
                None,
                Some(d.id),
                None,
                None,
                c,
                n,
                "UND".into(),
                d.cantidad,
                p,
                None,
                None,
            ));
        } else {
            return Err(error("Tipo de línea inválido."));
        }
    }
    let t = calcular(&mut l, req.descuento_bp)?;
    Ok((l, t))
}
pub(super) fn lineas_ot(conn: &Connection, ot: i64) -> Result<Vec<LineaCalculada>, AppError> {
    let mut l = Vec::new();
    let mut s=conn.prepare("SELECT id_detalle,id_servicio,codigo_snapshot,nombre_snapshot,cantidad,precio_unitario_cent FROM detalle_ot_servicios WHERE id_ot=?1 ORDER BY id_detalle")?;
    for r in s.query_map([ot], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, i64>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, i64>(4)?,
            r.get::<_, i64>(5)?,
        ))
    })? {
        let (id, s, c, n, q, p) = r?;
        l.push(linea(
            None,
            Some(s),
            None,
            Some(id),
            c,
            n,
            "UND".into(),
            q,
            p,
            None,
            None,
        ));
    }
    let mut s=conn.prepare("SELECT p.id_detalle,p.id_producto,p.codigo_snapshot,p.nombre_snapshot,p.unidad_medida,p.cantidad-coalesce((SELECT sum(diferencia) FROM movimientos_inventario WHERE id_consumo_ot=p.id_detalle AND tipo_movimiento='REPOSICION_OT'),0),p.precio_unitario_cent,p.costo_unitario_micro,p.valor_consumido_micro-coalesce((SELECT sum(valor_delta_micro) FROM movimientos_inventario WHERE id_consumo_ot=p.id_detalle AND tipo_movimiento='REPOSICION_OT'),0) FROM detalle_ot_productos p WHERE id_ot=?1 ORDER BY id_detalle")?;
    for r in s.query_map([ot], |r| {
        Ok(linea(
            Some(r.get(1)?),
            None,
            Some(r.get(0)?),
            None,
            r.get(2)?,
            r.get(3)?,
            r.get(4)?,
            r.get(5)?,
            r.get(6)?,
            Some(r.get(7)?),
            Some(r.get(8)?),
        ))
    })? {
        let r = r?;
        if r.cantidad > 0 {
            l.push(r);
        }
    }
    Ok(l)
}

fn validar_cantidad_directa(unidad: &str, cantidad: i64) -> Result<(), AppError> {
    if unidad == "UND" && !(1..=99_999).contains(&cantidad) {
        return Err(error("Ingresa de 1 a 99 999 unidades por línea directa."));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn linea(
    pr: Option<i64>,
    se: Option<i64>,
    co: Option<i64>,
    so: Option<i64>,
    codigo: String,
    nombre: String,
    unidad: String,
    cantidad: i64,
    precio_cent: i64,
    cu: Option<i64>,
    ct: Option<i64>,
) -> LineaCalculada {
    LineaCalculada {
        id_producto: pr,
        id_servicio: se,
        id_consumo_ot: co,
        id_servicio_ot: so,
        codigo,
        nombre,
        unidad,
        cantidad,
        precio_cent,
        bruto_cent: 0,
        descuento_cent: 0,
        base_cent: 0,
        igv_cent: 0,
        total_cent: 0,
        costo_unitario_micro: cu,
        costo_total_micro: ct,
    }
}
pub fn cotizar(conn: &Connection, actor: i64, req: &ConfirmarVentaPos) -> Result<Value, AppError> {
    validar_rol(conn, actor)?;
    let (l, mut t) = preparar(conn, req)?;
    if let Some(id) = req.id_ot {
        let total = l
            .iter()
            .filter(|linea| linea.id_consumo_ot.is_some() || linea.id_servicio_ot.is_some())
            .map(|linea| linea.bruto_cent)
            .sum();
        t["ot"] = consultas::contexto_ot(conn, id, total)?;
    }
    t["lineas"] = publicas(&l)?;
    Ok(t)
}
pub fn receptor(conn: &Connection, id: i64, tipo: &str, total: i64) -> Result<Value, AppError> {
    let c=consultas::filas(conn,"SELECT json_object('id',id_cliente,'tipo',tipo_documento,'documento',numero_documento,'nombre',nombre_razon_social,'direccion',direccion,'correo',correo,'varios',es_varios) FROM clientes WHERE id_cliente=?1",&[&id])?.into_iter().next().ok_or_else(||error("Selecciona un cliente existente."))?;
    if tipo == "FACTURA"
        && (c["tipo"] != "RUC" || c["direccion"].as_str().unwrap_or("").trim().is_empty())
    {
        return Err(error(
            "Factura requiere RUC, razón social y dirección del receptor.",
        ));
    }
    if total > 70000 && c["varios"] == 1 {
        return Err(error(
            "Para boleta superior a S/ 700 identifica al receptor.",
        ));
    }
    Ok(c)
}
pub fn confirmar(
    conn: &mut Connection,
    actor: i64,
    admin: Option<i64>,
    req: &ConfirmarVentaPos,
) -> Result<Value, AppError> {
    let rol = validar_rol(conn, actor)?;
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    if let Some(v) = IdempotenciaService::evaluar_y_registrar(
        &tx,
        Some(&req.id_operacion),
        "POS_VENTA",
        req,
        actor,
    )? {
        return Ok(v);
    }
    let caja = caja_propia(&tx, actor)?;
    let cfg = configuracion(&tx)?;
    validar_config(&cfg)?;
    let (l, t) = preparar(&tx, req)?;
    if req.descuento_bp > 500 {
        motivo(&req.motivo)?;
        if rol == "CAJERO" && admin.is_none() {
            return Err(AppError::PermisoDenegado);
        }
    }
    let receptor = receptor(
        &tx,
        req.id_cliente,
        &req.tipo_comprobante,
        t["total_cent"].as_i64().unwrap_or(0),
    )?;
    if !(1..=2).contains(&req.pagos.len()) {
        return Err(error("Utiliza uno o dos medios de pago."));
    }
    let mut medios = HashSet::new();
    let mut suma = 0i64;
    for p in &req.pagos {
        if !["EFECTIVO", "TARJETA", "YAPE", "PLIN", "TRANSFERENCIA"].contains(&p.medio.as_str())
            || !medios.insert(&p.medio)
            || p.monto_cent <= 0
            || p.monto_cent > 9_999_999_999
            || p.referencia
                .as_ref()
                .is_some_and(|s| s.chars().count() > 100)
        {
            return Err(error(
                "Revisa los medios, importes y referencias del cobro.",
            ));
        }
        if p.medio == "EFECTIVO" && p.recibido_cent.unwrap_or(0) < p.monto_cent {
            return Err(error("El efectivo recibido no cubre el importe aplicado."));
        }
        if p.medio != "EFECTIVO" && p.recibido_cent.is_some() {
            return Err(error("Los pagos digitales no generan vuelto."));
        }
        suma += p.monto_cent;
    }
    if suma != t["total_cent"].as_i64().unwrap_or(0) {
        return Err(error(
            "Los importes aplicados deben sumar exactamente el total.",
        ));
    }
    tx.execute("INSERT INTO ventas(id_operacion,id_ot,id_cliente,receptor_snapshot,id_emisor,id_autorizante,id_caja,ambiente,tipo_original,descuento_bp,motivo_descuento,bruto_cent,descuento_cent,base_cent,igv_cent,total_cent) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",params![req.id_operacion,req.id_ot,req.id_cliente,receptor.to_string(),actor,admin,caja,cfg["ambiente"].as_str(),req.tipo_comprobante,req.descuento_bp,if req.descuento_bp>500{Some(req.motivo.trim())}else{None},t["bruto_cent"].as_i64(),t["descuento_cent"].as_i64(),t["base_cent"].as_i64(),t["igv_cent"].as_i64(),t["total_cent"].as_i64()])?;
    let id = tx.last_insert_rowid();
    for (i, l) in l.iter().enumerate() {
        tx.execute("INSERT INTO detalle_venta(id_venta,id_producto,id_servicio,id_consumo_ot,id_servicio_ot,codigo_snapshot,nombre_snapshot,unidad_medida,cantidad,precio_unitario_cent,bruto_cent,descuento_cent,base_cent,igv_cent,igv_bp,total_cent,costo_unitario_micro,costo_total_micro) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,1800,?15,?16,?17)",params![id,l.id_producto,l.id_servicio,l.id_consumo_ot,l.id_servicio_ot,l.codigo,l.nombre,l.unidad,l.cantidad,l.precio_cent,l.bruto_cent,l.descuento_cent,l.base_cent,l.igv_cent,l.total_cent,l.costo_unitario_micro,l.costo_total_micro])?;
        let detalle = tx.last_insert_rowid();
        if let Some(producto) = l.id_producto.filter(|_| l.id_consumo_ot.is_none()) {
            stock::mover(
                &tx,
                actor,
                &format!("{}-stock-{i}", req.id_operacion),
                producto,
                "VENTA_DIRECTA",
                detalle,
                -l.cantidad,
                -l.costo_total_micro.unwrap_or(0),
                l.costo_unitario_micro.unwrap_or(1),
            )?;
        }
    }
    for (i, p) in req.pagos.iter().enumerate() {
        let recibido = if p.medio == "EFECTIVO" {
            p.recibido_cent
        } else {
            None
        };
        let vuelto = recibido.map(|r| r - p.monto_cent);
        tx.execute("INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,id_venta,recibido_cent,vuelto_cent,concepto,referencia,fecha_operacion) VALUES(?1,?2,?3,'COBRO',?4,?5,1,?6,?7,?8,?9,?10,(SELECT fecha_venta FROM ventas WHERE id_venta=?6))",params![format!("{}-pago-{i}",req.id_operacion),caja,actor,p.medio,p.monto_cent,id,recibido,vuelto,format!("Cobro venta {id}"),p.referencia])?;
    }
    tx.execute(
        "UPDATE ventas SET estado_registro='CONFIRMADA' WHERE id_venta=?1",
        [id],
    )?;
    let doc = super::documentos::crear(
        &tx,
        actor,
        &req.id_operacion,
        id,
        None,
        None,
        None,
        &req.tipo_comprobante,
        "ORIGINAL",
        &cfg,
        &receptor,
        &t,
    )?;
    auditar(
        &tx,
        actor,
        &req.id_operacion,
        "VENTA_CONFIRMADA",
        "ventas",
        id,
        &json!({"total_cent":t["total_cent"],"id_ot":req.id_ot,"documento":doc}),
        &req.motivo,
        admin,
    )?;
    let result = json!({"id_venta":id,"id_documento":doc,"total_cent":t["total_cent"]});
    IdempotenciaService::finalizar_solicitud(
        &tx,
        Some(&req.id_operacion),
        "POS_VENTA",
        req,
        actor,
        &result,
    )?;
    tx.commit()?;
    Ok(result)
}

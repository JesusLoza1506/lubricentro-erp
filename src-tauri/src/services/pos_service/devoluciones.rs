use super::{
    auditar, caja_propia, calculos::redondear, configuracion, consultas, error, motivo, stock,
    validar_config, validar_rol,
};
use crate::{
    dto::pos_dto::DevolverVentaPos, errors::AppError,
    services::inventario_service::idempotencia::IdempotenciaService,
};
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::collections::HashSet;

pub fn devolver(
    conn: &mut Connection,
    actor: i64,
    admin: Option<i64>,
    req: &DevolverVentaPos,
) -> Result<Value, AppError> {
    let rol = validar_rol(conn, actor)?;
    consultas::visible(conn, actor, req.id_venta)?;
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    if let Some(v) = IdempotenciaService::evaluar_y_registrar(
        &tx,
        Some(&req.id_operacion),
        "POS_DEVOLVER",
        req,
        actor,
    )? {
        return Ok(v);
    }
    motivo(&req.motivo)?;
    let caja = caja_propia(&tx, actor)?;
    if rol == "CAJERO" && admin.is_none() {
        return Err(AppError::PermisoDenegado);
    }
    if !["EFECTIVO", "TARJETA", "YAPE", "PLIN", "TRANSFERENCIA"].contains(&req.medio.as_str())
        || req
            .referencia
            .as_ref()
            .is_some_and(|s| s.chars().count() > 100)
    {
        return Err(error("Revisa el medio y la referencia del reintegro."));
    }
    if req.lineas.is_empty() {
        return Err(error("Selecciona las cantidades que devolverás."));
    }
    let cfg = configuracion(&tx)?;
    validar_config(&cfg)?;
    let(doc,tipo,receptor):(i64,String,String)=tx.query_row("SELECT id_comprobante,tipo_comprobante,receptor_snapshot FROM comprobantes c WHERE id_venta=?1 AND relacion IN ('ORIGINAL','REEMISION') AND estado_envio='ACEPTADO' AND NOT EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante)",[req.id_venta],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).map_err(|_|error("Resuelve el comprobante original antes de devolver."))?;
    let (detalles, base, total) = preparar_lineas(&tx, req)?;
    if req.medio == "EFECTIVO" {
        let saldo:i64=tx.query_row("SELECT monto_apertura_cent+coalesce((SELECT sum(signo*monto_cent) FROM movimientos_caja WHERE id_caja=c.id_caja AND medio_pago='EFECTIVO'),0) FROM caja_chica c WHERE id_caja=?1",[caja],|r|r.get(0))?;
        if total > saldo {
            return Err(error(
                "El efectivo disponible no alcanza para este reintegro.",
            ));
        }
    }
    tx.execute("INSERT INTO devoluciones(id_operacion,id_venta,id_usuario,id_autorizante,id_caja,id_documento_original,motivo,base_cent,igv_cent,total_cent) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",params![req.id_operacion,req.id_venta,actor,admin,caja,doc,req.motivo.trim(),base,total-base,total])?;
    let id = tx.last_insert_rowid();
    for (i, (l, producto, cu, rc, rb, rt)) in detalles.iter().enumerate() {
        tx.execute("INSERT INTO detalle_devolucion(id_devolucion,id_detalle_venta,cantidad,cantidad_recuperada,base_cent,igv_cent,total_cent,costo_revertido_micro) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",params![id,l.id_detalle,l.cantidad,l.recuperada,rb,rt-rb,rt,rc])?;
        let detalle = tx.last_insert_rowid();
        if let Some(pr) = producto.filter(|_| l.recuperada > 0) {
            stock::mover(
                &tx,
                actor,
                &format!("{}-retorno-{i}", req.id_operacion),
                pr,
                "DEVOLUCION_VENTA",
                detalle,
                l.recuperada,
                *rc,
                *cu,
            )?;
        }
    }
    tx.execute("INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,id_devolucion,concepto,referencia,motivo,fecha_operacion) VALUES(?1,?2,?3,'REINTEGRO',?4,?5,-1,?6,?7,?8,?9,(SELECT fecha_operacion FROM devoluciones WHERE id_devolucion=?6))",params![format!("{}-reintegro",req.id_operacion),caja,actor,req.medio,total,id,format!("Devolución venta {}",req.id_venta),req.referencia,req.motivo])?;
    tx.execute(
        "UPDATE devoluciones SET estado_registro='CONFIRMADA' WHERE id_devolucion=?1",
        [id],
    )?;
    let receptor: Value = serde_json::from_str(&receptor).map_err(super::json_error)?;
    let nota = super::documentos::crear(
        &tx,
        actor,
        &req.id_operacion,
        req.id_venta,
        Some(id),
        None,
        Some(doc),
        if tipo == "FACTURA" {
            "NC_FACTURA"
        } else {
            "NC_BOLETA"
        },
        "NOTA_CREDITO",
        &cfg,
        &receptor,
        &json!({"base_cent":base,"igv_cent":total-base,"total_cent":total}),
    )?;
    auditar(
        &tx,
        actor,
        &req.id_operacion,
        "DEVOLUCION_CONFIRMADA",
        "devoluciones",
        id,
        &json!({"venta":req.id_venta,"total_cent":total,"nota":nota}),
        &req.motivo,
        admin,
    )?;
    let r =
        json!({"id_venta":req.id_venta,"id_devolucion":id,"id_documento":nota,"total_cent":total});
    IdempotenciaService::finalizar_solicitud(
        &tx,
        Some(&req.id_operacion),
        "POS_DEVOLVER",
        req,
        actor,
        &r,
    )?;
    tx.commit()?;
    Ok(r)
}

#[allow(clippy::type_complexity)]
fn preparar_lineas<'a>(
    tx: &Connection,
    req: &'a DevolverVentaPos,
) -> Result<
    (
        Vec<(
            &'a crate::dto::pos_dto::LineaRetorno,
            Option<i64>,
            i64,
            i64,
            i64,
            i64,
        )>,
        i64,
        i64,
    ),
    AppError,
> {
    let mut detalles = Vec::new();
    let mut ids = HashSet::new();
    let mut base = 0i64;
    let mut total = 0i64;
    for l in &req.lineas {
        if !ids.insert(l.id_detalle)
            || l.cantidad <= 0
            || l.recuperada < 0
            || l.recuperada > l.cantidad
        {
            return Err(error("Cantidades de devolución inválidas o repetidas."));
        }
        let(q,b,t,producto,costo,cu):(i64,i64,i64,Option<i64>,i64,i64)=tx.query_row("SELECT cantidad,base_cent,total_cent,id_producto,coalesce(costo_total_micro,0),coalesce(costo_unitario_micro,1) FROM detalle_venta WHERE id_detalle=?1 AND id_venta=?2",params![l.id_detalle,req.id_venta],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?)))?;
        let(pq,pb,pt,pr,pc):(i64,i64,i64,i64,i64)=tx.query_row("SELECT coalesce(sum(cantidad),0),coalesce(sum(base_cent),0),coalesce(sum(total_cent),0),coalesce(sum(cantidad_recuperada),0),coalesce(sum(costo_revertido_micro),0) FROM detalle_devolucion WHERE id_detalle_venta=?1",[l.id_detalle],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?)))?;
        if l.cantidad > q - pq || producto.is_none() && l.recuperada > 0 {
            return Err(error(
                "La devolución supera lo pendiente o intenta recuperar un servicio.",
            ));
        }
        let rt = redondear(t as i128 * (pq + l.cantidad) as i128, q as i128) - pt;
        let rb = redondear(b as i128 * (pq + l.cantidad) as i128, q as i128) - pb;
        let rc = redondear(costo as i128 * (pr + l.recuperada) as i128, q as i128) - pc;
        if rt <= 0 || rb < 0 || rb > rt {
            return Err(error(
                "La cantidad genera un reintegro no representable en céntimos.",
            ));
        }
        base += rb;
        total += rt;
        detalles.push((l, producto, cu, rc, rb, rt));
    }
    Ok((detalles, base, total))
}
pub fn cotizar(conn: &Connection, actor: i64, req: &DevolverVentaPos) -> Result<Value, AppError> {
    consultas::visible(conn, actor, req.id_venta)?;
    if req.lineas.is_empty() {
        return Err(error("Selecciona cantidades para devolver."));
    }
    let (_, base, total) = preparar_lineas(conn, req)?;
    Ok(json!({"base_cent":base,"igv_cent":total-base,"total_cent":total}))
}

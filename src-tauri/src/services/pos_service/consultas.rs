use super::{caja_propia, configuracion, error, json_error, validar_rol};
use crate::errors::AppError;
use rusqlite::{params, Connection, ToSql};
use serde_json::{json, Value};
pub fn filas(conn: &Connection, sql: &str, p: &[&dyn ToSql]) -> Result<Vec<Value>, AppError> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(p, |r| r.get::<_, String>(0))?;
    let mut res = Vec::new();
    for r in rows {
        res.push(serde_json::from_str(&r?).map_err(json_error)?);
    }
    Ok(res)
}
pub fn visible(conn: &Connection, actor: i64, id: i64) -> Result<String, AppError> {
    let rol = validar_rol(conn, actor)?;
    let owner: i64 = conn.query_row(
        "SELECT id_emisor FROM ventas WHERE id_venta=?1 AND estado_registro='CONFIRMADA'",
        [id],
        |r| r.get(0),
    )?;
    if rol != "ADMINISTRADOR" && owner != actor {
        return Err(AppError::PermisoDenegado);
    }
    Ok(rol)
}
pub fn inicio(conn: &Connection, actor: i64) -> Result<Value, AppError> {
    validar_rol(conn, actor)?;
    let c = configuracion(conn)?;
    let caja=filas(conn,"SELECT json_object('id',id_caja,'codigo',codigo_caja,'titular',u.nombre_completo,'propia',c.id_usuario=?1,'efectivo_cent',monto_apertura_cent+coalesce((SELECT sum(signo*monto_cent) FROM movimientos_caja WHERE id_caja=c.id_caja AND medio_pago='EFECTIVO'),0)) FROM caja_chica c JOIN usuarios u ON u.id_usuario=c.id_usuario WHERE estado='ABIERTA'",&[&actor])?;
    let series=filas(conn,"SELECT json_object('id',id_serie,'tipo',tipo_comprobante,'serie',serie) FROM series_comprobante WHERE activa=1 AND requiere_conciliacion=0 AND ambiente=?1",&[&c["ambiente"].as_str().unwrap_or("")])?;
    let hoy: String = conn.query_row("SELECT date('now','-5 hours')", [], |r| r.get(0))?;
    Ok(
        json!({"ambiente":c["ambiente"],"empresa":c["razon_social"],"configurada":super::validar_config(&c).is_ok(),"caja":caja.first(),"series":series,"hoy":hoy,"caja_propia":caja_propia(conn,actor).ok()}),
    )
}
pub fn catalogo(
    conn: &Connection,
    actor: i64,
    q: &str,
    tipo: &str,
    pagina: i64,
) -> Result<Value, AppError> {
    validar_rol(conn, actor)?;
    let patron = format!("%{}%", q.trim());
    let offset = (pagina.max(1) - 1) * 20;
    let sql="WITH cat AS (SELECT 'PRODUCTO' tipo,id_producto id,codigo,nombre,unidad_medida unidad,precio_venta_cent precio_cent,stock_actual stock FROM productos WHERE activo=1 UNION ALL SELECT 'SERVICIO',id_servicio,codigo,nombre,'UND',precio_venta_cent,NULL FROM servicios WHERE activo=1) ";
    let condicion = " WHERE (codigo LIKE ?1 OR nombre LIKE ?1) AND (?2='TODOS' OR tipo=?2)";
    let total: i64 = conn.query_row(
        &format!("{sql} SELECT count(*) FROM cat {condicion}"),
        params![patron, tipo],
        |r| r.get(0),
    )?;
    let rows=filas(conn,&format!("{sql} SELECT json_object('tipo',tipo,'id',id,'codigo',codigo,'nombre',nombre,'unidad',unidad,'precio_cent',precio_cent,'stock',stock) FROM cat {condicion} ORDER BY nombre,codigo,tipo,id LIMIT 20 OFFSET ?3"),&[&patron,&tipo,&offset])?;
    Ok(json!({"filas":rows,"total":total,"pagina":pagina.max(1)}))
}
const DATOS_OT: &str = "json_object('id',id_ot,'codigo',codigo_ot,'placa',placa_snapshot,'cliente',coalesce(json_extract(cliente_snapshot,'$.nombre'),'—'),'fecha',fecha_fin)";

pub(super) fn contexto_ot(conn: &Connection, id: i64, total: i64) -> Result<Value, AppError> {
    let mut ot = filas(
        conn,
        &format!("SELECT {DATOS_OT} FROM ordenes_trabajo WHERE id_ot=?1"),
        &[&id],
    )?
    .into_iter()
    .next()
    .ok_or_else(|| error("No se encontró la orden de trabajo."))?;
    ot["total_cent"] = json!(total);
    Ok(ot)
}

fn normalizar_advertencias(valor: &Value) -> Vec<&str> {
    match valor {
        Value::String(nota) => vec![nota.as_str()],
        Value::Array(notas) => notas.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    }
    .into_iter()
    .map(str::trim)
    .filter(|nota| !nota.is_empty())
    .collect()
}

pub fn ots(conn: &Connection, actor: i64, q: &str, pagina: i64) -> Result<Value, AppError> {
    validar_rol(conn, actor)?;
    let patron = format!("%{}%", q.trim());
    let offset = (pagina.max(1) - 1) * 20;
    let where_sql=" FROM ordenes_trabajo o WHERE estado='FINALIZADO' AND NOT EXISTS(SELECT 1 FROM ventas v WHERE v.id_ot=o.id_ot) AND (codigo_ot LIKE ?1 OR placa_snapshot LIKE ?1)";
    let total: i64 = conn.query_row(&format!("SELECT count(*) {where_sql}"), [&patron], |r| {
        r.get(0)
    })?;
    let mut rows = filas(
        conn,
        &format!("SELECT {DATOS_OT} {where_sql} ORDER BY fecha_fin,id_ot LIMIT 20 OFFSET ?2"),
        &[&patron, &offset],
    )?;
    for ot in &mut rows {
        let id = ot["id"]
            .as_i64()
            .ok_or_else(|| error("Identificador de OT inválido."))?;
        let lineas = super::ventas::lineas_ot(conn, id)?;
        let total = lineas.iter().try_fold(0i64, |total, linea| {
            total
                .checked_add(super::calculos::importe_bruto(linea)?)
                .ok_or_else(|| error("Importe de OT fuera de rango."))
        })?;
        ot["total_cent"] = json!(total);
    }
    Ok(json!({"filas":rows,"total":total}))
}
pub fn detalle(conn: &Connection, actor: i64, id: i64) -> Result<Value, AppError> {
    let rol = visible(conn, actor, id)?;
    let mut v=filas(conn,"SELECT json_object('id',v.id_venta,'id_ot',v.id_ot,'id_emisor',v.id_emisor,'emisor',u.nombre_completo,'caja',c.codigo_caja,'fecha',fecha_venta,'tipo',tipo_original,'receptor',json(receptor_snapshot),'bruto_cent',bruto_cent,'descuento_bp',descuento_bp,'descuento_cent',descuento_cent,'base_cent',base_cent,'igv_cent',igv_cent,'total_cent',total_cent,'devuelto_cent',coalesce((SELECT sum(total_cent) FROM devoluciones WHERE id_venta=v.id_venta AND estado_registro='CONFIRMADA'),0)) FROM ventas v JOIN usuarios u ON u.id_usuario=v.id_emisor JOIN caja_chica c ON c.id_caja=v.id_caja WHERE v.id_venta=?1",&[&id])?.remove(0);
    v["lineas"]=json!(filas(conn,"SELECT json_object('id_detalle',l.id_detalle,'id_producto',id_producto,'nombre',nombre_snapshot,'codigo',codigo_snapshot,'unidad',unidad_medida,'cantidad',cantidad,'precio_cent',precio_unitario_cent,'total_cent',total_cent,'base_cent',base_cent,'devuelta',coalesce((SELECT sum(cantidad) FROM detalle_devolucion WHERE id_detalle_venta=l.id_detalle),0),'recuperada',coalesce((SELECT sum(cantidad_recuperada) FROM detalle_devolucion WHERE id_detalle_venta=l.id_detalle),0),'origen',CASE WHEN id_consumo_ot IS NOT NULL OR id_servicio_ot IS NOT NULL THEN 'OT' ELSE 'DIRECTO' END) FROM detalle_venta l WHERE id_venta=?1 ORDER BY id_detalle",&[&id])?);
    v["pagos"]=json!(filas(conn,"SELECT json_object('medio',medio_pago,'monto_cent',monto_cent,'recibido_cent',recibido_cent,'vuelto_cent',vuelto_cent,'referencia',referencia) FROM movimientos_caja WHERE id_venta=?1 AND tipo='COBRO' ORDER BY id_movimiento",&[&id])?);
    let mut documentos=filas(conn,"SELECT json_object('id',id_comprobante,'tipo',c.tipo_comprobante,'serie',s.serie,'numero',correlativo,'estado',estado_envio,'relacion',relacion,'incierto',resultado_incierto,'requiere_consulta',requiere_consulta,'intento',intento_iniciado,'mensaje',respuesta_resumida,'advertencias',json(c.advertencias_json),'pdf',ruta_pdf,'xml',ruta_xml,'cdr',ruta_cdr,'fecha',fecha_emision,'reemplazado',EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante)) FROM comprobantes c JOIN series_comprobante s ON s.id_serie=c.id_serie WHERE id_venta=?1 ORDER BY id_comprobante",&[&id])?;
    for documento in &mut documentos {
        documento["advertencias"] = json!(normalizar_advertencias(&documento["advertencias"]));
    }
    v["documentos"] = json!(documentos);
    v["devoluciones"]=json!(filas(conn,"SELECT json_object('id',id_devolucion,'fecha',fecha_operacion,'total_cent',total_cent,'motivo',motivo) FROM devoluciones WHERE id_venta=?1 AND estado_registro='CONFIRMADA' ORDER BY id_devolucion",&[&id])?);
    if rol == "ADMINISTRADOR" {
        v["costos"]=json!(filas(conn,"SELECT json_object('id_detalle',id_detalle,'costo_total_micro',costo_total_micro) FROM detalle_venta WHERE id_venta=?1",&[&id])?);
    }
    Ok(v)
}
pub fn listado(conn: &Connection, actor: i64, p: &Value) -> Result<Value, AppError> {
    let rol = validar_rol(conn, actor)?;
    let hoy: String = conn.query_row("SELECT date('now','-5 hours')", [], |r| r.get(0))?;
    let desde = p["desde"].as_str().unwrap_or(&hoy);
    let hasta = p["hasta"].as_str().unwrap_or(&hoy);
    let dias: Option<f64> = conn.query_row(
        "SELECT julianday(?2)-julianday(?1)",
        params![desde, hasta],
        |r| r.get(0),
    )?;
    if desde.len() != 10 || hasta.len() != 10 || !dias.is_some_and(|n| (0.0..=365.0).contains(&n)) {
        return Err(error(
            "Selecciona un rango de fechas válido de hasta 366 días.",
        ));
    }
    let buscar = format!("%{}%", p["buscar"].as_str().unwrap_or("").trim());
    let estado = p["estado"].as_str().unwrap_or("");
    let problemas = p["problemas"].as_bool().unwrap_or(false) && rol == "ADMINISTRADOR";
    let emisor = if rol == "ADMINISTRADOR" {
        p["emisor"].as_i64().unwrap_or(0)
    } else {
        actor
    };
    let offset = (p["pagina"].as_i64().unwrap_or(1).max(1) - 1) * 20;
    let condition="FROM ventas v WHERE v.estado_registro='CONFIRMADA' AND date(fecha_venta) BETWEEN ?1 AND ?2 AND (?3=0 OR id_emisor=?3) AND (CAST(id_venta AS TEXT) LIKE ?4 OR receptor_snapshot LIKE ?4 OR EXISTS(SELECT 1 FROM comprobantes c JOIN series_comprobante s ON s.id_serie=c.id_serie WHERE c.id_venta=v.id_venta AND s.serie||'-'||c.correlativo LIKE ?4)) AND (?5='' OR EXISTS(SELECT 1 FROM comprobantes c WHERE c.id_venta=v.id_venta AND c.estado_envio=?5 AND NOT EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante))) AND (?6=0 OR EXISTS(SELECT 1 FROM comprobantes c WHERE c.id_venta=v.id_venta AND c.estado_envio IN ('PENDIENTE','ERROR','RECHAZADO','ENVIANDO') AND NOT EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante)))";
    let args: &[&dyn ToSql] = &[&desde, &hasta, &emisor, &buscar, &estado, &problemas];
    let total: i64 = conn.query_row(&format!("SELECT count(*) {condition}"), args, |r| r.get(0))?;
    let rows=filas(conn,&format!("SELECT json_object('id',id_venta,'fecha',fecha_venta,'cliente',json_extract(receptor_snapshot,'$.nombre'),'total_cent',total_cent,'id_ot',id_ot,'emisor',(SELECT nombre_completo FROM usuarios WHERE id_usuario=v.id_emisor),'devuelto_cent',coalesce((SELECT sum(total_cent) FROM devoluciones WHERE id_venta=v.id_venta AND estado_registro='CONFIRMADA'),0),'estado',(SELECT estado_envio FROM comprobantes WHERE id_venta=v.id_venta ORDER BY id_comprobante DESC LIMIT 1)) {condition} ORDER BY fecha_venta DESC,id_venta DESC LIMIT 20 OFFSET ?7"),&[&desde,&hasta,&emisor,&buscar,&estado,&problemas,&offset])?;
    Ok(json!({"filas":rows,"total":total}))
}

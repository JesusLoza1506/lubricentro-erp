use super::{auditar, configuracion, consultas, error, motivo, validar_config};
use crate::{
    dto::pos_dto::ReemitirPos, errors::AppError,
    services::inventario_service::idempotencia::IdempotenciaService,
};
use rusqlite::{params, Connection};
use serde_json::{json, Value};

pub fn tipo_api(tipo: &str) -> i64 {
    match tipo {
        "FACTURA" => 1,
        "BOLETA" => 2,
        _ => 3,
    }
}
#[allow(clippy::too_many_arguments)]
pub fn crear(
    conn: &Connection,
    actor: i64,
    op: &str,
    venta: i64,
    devolucion: Option<i64>,
    anterior: Option<i64>,
    original: Option<i64>,
    tipo: &str,
    relacion: &str,
    cfg: &Value,
    receptor: &Value,
    totales: &Value,
) -> Result<i64, AppError> {
    let (serie_id,serie,numero):(i64,String,i64)=conn.query_row("SELECT id_serie,serie,correlativo_actual+1 FROM series_comprobante WHERE ambiente=?1 AND tipo_comprobante=?2 AND activa=1 AND requiere_conciliacion=0",params![cfg["ambiente"].as_str(),tipo],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).map_err(|_|error("Configura una serie activa y conciliada para este tipo de comprobante."))?;
    conn.execute(
        "UPDATE series_comprobante SET correlativo_actual=?1 WHERE id_serie=?2",
        params![numero, serie_id],
    )?;
    let fecha: String = conn.query_row("SELECT datetime('now','-5 hours')", [], |r| r.get(0))?;
    let mut items = Vec::new();
    let sql = if devolucion.is_some() {
        "SELECT json_object('servicio',v.id_servicio,'codigo',v.codigo_snapshot,'nombre',v.nombre_snapshot,'unidad',v.unidad_medida,'cantidad',d.cantidad,'base',d.base_cent,'igv',d.igv_cent,'total',d.total_cent) FROM detalle_devolucion d JOIN detalle_venta v ON v.id_detalle=d.id_detalle_venta WHERE id_devolucion=?1 ORDER BY d.id_detalle"
    } else {
        "SELECT json_object('servicio',id_servicio,'codigo',codigo_snapshot,'nombre',nombre_snapshot,'unidad',unidad_medida,'cantidad',cantidad,'base',base_cent,'igv',igv_cent,'total',total_cent) FROM detalle_venta WHERE id_venta=?1 ORDER BY id_detalle"
    };
    for l in consultas::filas(conn, sql, &[&devolucion.unwrap_or(venta)])? {
        let q = l["cantidad"].as_i64().unwrap_or(1);
        let escala = if l["unidad"] == "ML" { 1000.0 } else { 1.0 };
        let cant = q as f64 / escala;
        let b = l["base"].as_i64().unwrap_or(0) as f64 / 100.0;
        let total = l["total"].as_i64().unwrap_or(0) as f64 / 100.0;
        items.push(json!({"unidad_de_medida":if !l["servicio"].is_null(){"ZZ"}else if l["unidad"]=="ML"{"LTR"}else{"NIU"},"codigo":l["codigo"],"descripcion":l["nombre"],"cantidad":cant,"valor_unitario":b/cant,"precio_unitario":total/cant,"subtotal":b,"tipo_de_igv":1,"igv":l["igv"].as_i64().unwrap_or(0) as f64/100.0,"total":total,"anticipo_regularizacion":false}));
    }
    let date = format!("{}-{}-{}", &fecha[8..10], &fecha[5..7], &fecha[0..4]);
    let mut payload = json!({"operacion":"generar_comprobante","tipo_de_comprobante":tipo_api(tipo),"serie":serie,"numero":numero,"sunat_transaction":1,"cliente_tipo_de_documento":match receptor["tipo"].as_str(){Some("RUC")=>6,Some("DNI")=>1,Some("CE")=>4,_=>0},"cliente_numero_de_documento":receptor["documento"].as_str().unwrap_or("-"),"cliente_denominacion":receptor["nombre"],"cliente_direccion":receptor["direccion"].as_str().unwrap_or(""),"fecha_de_emision":date,"moneda":1,"porcentaje_de_igv":18,"total_gravada":totales["base_cent"].as_i64().unwrap_or(0) as f64/100.0,"total_igv":totales["igv_cent"].as_i64().unwrap_or(0) as f64/100.0,"total":totales["total_cent"].as_i64().unwrap_or(0) as f64/100.0,"enviar_automaticamente_a_la_sunat":true,"enviar_automaticamente_al_cliente":false,"items":items,"condiciones_de_pago":"CONTADO"});
    if let Some(id) = original {
        let(t,s,n):(String,String,i64)=conn.query_row("SELECT c.tipo_comprobante,s.serie,correlativo FROM comprobantes c JOIN series_comprobante s ON s.id_serie=c.id_serie WHERE id_comprobante=?1",[id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?)))?;
        payload["documento_que_se_modifica_tipo"] = json!(tipo_api(&t));
        payload["documento_que_se_modifica_serie"] = json!(s);
        payload["documento_que_se_modifica_numero"] = json!(n);
        let motivo: String = conn.query_row(
            "SELECT motivo FROM devoluciones WHERE id_devolucion=?1",
            [devolucion],
            |r| r.get(0),
        )?;
        // Devolución por ítem, total o parcial. Las líneas y cantidades quedan explícitas.
        payload["tipo_de_nota_de_credito"] = json!(7);
        payload["observaciones"] = json!(motivo);
    }
    let sha = IdempotenciaService::calcular_sha256(&payload)?;
    let empresa = json!({"ruc":cfg["ruc"],"razon_social":cfg["razon_social"],"direccion":cfg["direccion"],"ubigeo":cfg["ubigeo"]});
    conn.execute("INSERT INTO comprobantes(id_operacion,id_venta,id_devolucion,id_documento_anterior,id_documento_referenciado,relacion,tipo_comprobante,ambiente,id_serie,correlativo,empresa_snapshot,receptor_snapshot,payload_json,payload_sha256,base_cent,igv_cent,total_cent,fecha_emision) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)",params![format!("{op}-documento"),venta,devolucion,anterior,original,relacion,tipo,cfg["ambiente"].as_str(),serie_id,numero,empresa.to_string(),receptor.to_string(),payload.to_string(),sha,totales["base_cent"].as_i64(),totales["igv_cent"].as_i64(),totales["total_cent"].as_i64(),fecha])?;
    let id = conn.last_insert_rowid();
    auditar(
        conn,
        actor,
        op,
        "DOCUMENTO_CREADO",
        "comprobantes",
        id,
        &json!({"tipo":tipo,"serie":serie,"numero":numero,"relacion":relacion}),
        "",
        None,
    )?;
    Ok(id)
}
pub fn reemitir(conn: &mut Connection, actor: i64, req: &ReemitirPos) -> Result<Value, AppError> {
    if super::validar_rol(conn, actor)? != "ADMINISTRADOR" {
        return Err(AppError::PermisoDenegado);
    }
    motivo(&req.motivo)?;
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    if let Some(v) = IdempotenciaService::evaluar_y_registrar(
        &tx,
        Some(&req.id_operacion),
        "POS_REEMITIR",
        req,
        actor,
    )? {
        return Ok(v);
    }
    let docs=consultas::filas(&tx,"SELECT json_object('venta',id_venta,'devolucion',id_devolucion,'original',id_documento_referenciado,'tipo',tipo_comprobante,'base_cent',base_cent,'igv_cent',igv_cent,'total_cent',total_cent,'receptor',json(receptor_snapshot)) FROM comprobantes c WHERE id_comprobante=?1 AND estado_envio='RECHAZADO' AND NOT EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante)",&[&req.id_documento])?;
    let d = docs
        .first()
        .ok_or_else(|| error("Solo puedes reemitir un rechazo oficial vigente."))?;
    let tipo = d["tipo"].as_str().unwrap_or("");
    let receptor = if !d["devolucion"].is_null() {
        if !req.receptor.is_null()
            && ["tipo", "documento", "nombre", "direccion"]
                .iter()
                .any(|k| req.receptor[*k] != d["receptor"][*k])
        {
            return Err(error(
                "La nota de crédito debe conservar el receptor del comprobante aceptado.",
            ));
        }
        d["receptor"].clone()
    } else if req.receptor.is_null() {
        d["receptor"].clone()
    } else {
        validar_receptor_corregido(&req.receptor, tipo, d["total_cent"].as_i64().unwrap_or(0))?
    };
    let cfg = configuracion(&tx)?;
    validar_config(&cfg)?;
    let id = crear(
        &tx,
        actor,
        &req.id_operacion,
        d["venta"].as_i64().unwrap_or(0),
        d["devolucion"].as_i64(),
        Some(req.id_documento),
        d["original"].as_i64(),
        tipo,
        if d["devolucion"].is_null() {
            "REEMISION"
        } else {
            "REEMISION_NC"
        },
        &cfg,
        &receptor,
        d,
    )?;
    auditar(
        &tx,
        actor,
        &req.id_operacion,
        "REEMISION",
        "comprobantes",
        id,
        &json!({"anterior":req.id_documento}),
        &req.motivo,
        None,
    )?;
    let r = json!({"id_documento":id,"id_venta":d["venta"]});
    IdempotenciaService::finalizar_solicitud(
        &tx,
        Some(&req.id_operacion),
        "POS_REEMITIR",
        req,
        actor,
        &r,
    )?;
    tx.commit()?;
    Ok(r)
}
fn validar_receptor_corregido(v: &Value, tipo: &str, total: i64) -> Result<Value, AppError> {
    let t = v["tipo"].as_str().unwrap_or("");
    let n = v["documento"].as_str().unwrap_or("");
    let nombre = v["nombre"].as_str().unwrap_or("").trim();
    let direccion = v["direccion"].as_str().unwrap_or("");
    if !(3..=150).contains(&nombre.chars().count()) || direccion.chars().count() > 200 {
        return Err(error("Revisa nombre y dirección del receptor."));
    }
    if !t.is_empty() {
        crate::services::cliente_service::validaciones::validar_documento(t, n)?;
    } else if tipo.contains("FACTURA") || total > 70000 {
        return Err(error("Identifica al receptor del comprobante."));
    }
    if tipo.contains("FACTURA") && (t != "RUC" || direccion.trim().is_empty()) {
        return Err(error("Factura requiere RUC y dirección."));
    }
    if t == "RUC" {
        let dig: Vec<i64> = n.bytes().map(|b| (b - b'0') as i64).collect();
        let sum: i64 = dig
            .iter()
            .take(10)
            .zip([5, 4, 3, 2, 7, 6, 5, 4, 3, 2])
            .map(|(a, b)| a * b)
            .sum();
        if (11 - sum % 11) % 10 != dig[10] {
            return Err(error("RUC con dígito de control inválido."));
        }
    }
    Ok(
        json!({"tipo":if t.is_empty(){None}else{Some(t)},"documento":if n.is_empty(){None}else{Some(n)},"nombre":nombre,"direccion":direccion,"varios":if t.is_empty(){1}else{0}}),
    )
}
/// Solo una confirmación explícita de SUNAT resuelve el documento.
pub fn clasificar_respuesta(v: &Value) -> (&'static str, bool) {
    if v["aceptada_por_sunat"].as_bool() == Some(true) {
        return ("ACEPTADO", false);
    }
    let code = v["sunat_responsecode"]
        .as_str()
        .unwrap_or("")
        .parse::<i32>()
        .ok();
    if v["aceptada_por_sunat"].as_bool() == Some(false)
        && code.is_some_and(|c| (2000..4000).contains(&c))
    {
        return ("RECHAZADO", false);
    }
    ("ERROR", true)
}
pub fn validar_huella(payload: &Value, sha: &str) -> Result<(), AppError> {
    if IdempotenciaService::calcular_sha256(payload)? != sha {
        return Err(error(
            "El contenido del documento no coincide con su huella. Solicita revisión.",
        ));
    }
    Ok(())
}

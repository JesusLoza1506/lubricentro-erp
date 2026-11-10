use super::*;
use serde_json::{json, Value};

pub(super) const MODULOS: [&str; 10] = [
    "LOGIN",
    "DASHBOARD",
    "FICHA_VEHICULAR",
    "OT",
    "INVENTARIO",
    "POS",
    "CAJA",
    "BACKUP",
    "ADMIN",
    "REPORTES",
];

pub(super) fn texto_seguro(s: &str) -> Option<String> {
    let lower = s.to_lowercase();
    let indicios = [
        "token",
        "contraseña",
        "password",
        "bearer",
        "secret",
        "credencial",
        "argon2",
        "bcrypt",
        "clave",
        "frase",
        "recuperación:",
        "recovery code",
        "authorization",
        "api_key",
        "https://",
        "http://",
        "$2a$",
        "$2b$",
    ];
    if s.contains(['{', '}', '<', '>'])
        || indicios.iter().any(|v| lower.contains(v))
        || s.split(|c: char| !c.is_ascii_alphanumeric())
            .any(|x| x.len() >= 24 && x.bytes().any(|b| b.is_ascii_digit()))
    {
        return Some("Texto reservado".into());
    }
    let texto: String = s
        .trim()
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .take(220)
        .collect();
    (!texto.is_empty()).then_some(texto)
}

fn seg(r: &Row<'_>, i: usize) -> R<Celda> {
    Ok(dato(
        r.get::<_, Option<String>>(i)?
            .and_then(|v| texto_seguro(&v)),
    ))
}
fn cambios(campo: &str, bruto: Option<String>, detalle: &mut Vec<Fila>) {
    const NUMERICOS: [&str; 9] = [
        "id_usuario",
        "id_caja",
        "id_ot",
        "id_venta",
        "id_producto",
        "cantidad",
        "anterior",
        "siguiente",
        "stock_actual",
    ];
    const ESTADOS: [&str; 17] = [
        "ADMINISTRADOR",
        "MECANICO",
        "CAJERO",
        "ACTIVA",
        "INACTIVA",
        "ACTIVO",
        "INACTIVO",
        "ABIERTA",
        "CERRADA",
        "PENDIENTE",
        "ERROR",
        "ACEPTADO",
        "RECHAZADO",
        "EN_PROCESO",
        "FINALIZADO",
        "CANCELADO",
        "EN_ESPERA",
    ];
    let Some(Value::Object(obj)) = bruto.and_then(|s| serde_json::from_str(&s).ok()) else {
        return;
    };
    for (k, v) in obj {
        let valor = match (k.as_str(), v) {
            (k, Value::Number(n)) if NUMERICOS.contains(&k) && n.is_i64() => super::n(n),
            ("activo" | "activa" | "bloqueado", Value::Bool(b)) => t(if b { "Sí" } else { "No" }),
            ("rol" | "estado", Value::String(v)) if ESTADOS.contains(&v.as_str()) => t(v),
            _ => continue,
        };
        detalle.push(Fila::nueva(
            vec![t(format!("{campo}: {}", k.replace('_', " "))), valor],
            [],
        ));
    }
}

pub(super) fn eventos(
    db: &Connection,
    amb: &str,
    desde: &str,
    hasta: &str,
    f: &Filtros,
) -> R<Datos> {
    let mut d = Datos::nuevo(vec![
        "Fecha y hora",
        "Actor",
        "Módulo",
        "Acción",
        "Referencia",
        "Resultado",
        "Motivo",
    ]);
    consulta(db, "WITH durables AS (
        SELECT id_restauracion,fecha_inicio,actor_snapshot,id_usuario,evento_externo_id FROM restauraciones WHERE ambiente=?1
      ), eventos AS (
        SELECT a.fecha_evento fecha,a.actor,a.username_snapshot nombre,a.modulo,a.accion,a.entidad,a.id_entidad ref,
          a.resultado,a.motivo,a.antes_json,a.despues_json,a.id_usuario usuario,a.id_auditoria orden,0 durable
        FROM auditoria a WHERE a.fecha_evento>=?2 AND a.fecha_evento<?3
          AND NOT (a.modulo='BACKUP' AND a.accion='RESTAURAR' AND EXISTS(
            SELECT 1 FROM durables r WHERE r.evento_externo_id=json_extract(a.despues_json,'$.evento')))
        UNION ALL
        SELECT r.fecha_inicio,CASE WHEN r.usuario_externo IS NULL THEN 'SISTEMA' ELSE 'USUARIO' END,
          r.actor_snapshot,'BACKUP','RESTAURAR','restauraciones',cast(r.id_restauracion AS TEXT),'OK',NULL,NULL,NULL,
          r.usuario_externo,r.id_restauracion,1
        FROM (SELECT *,id_usuario usuario_externo FROM durables) r WHERE r.fecha_inicio>=?2 AND r.fecha_inicio<?3
      )
      SELECT fecha,actor,nombre,modulo,accion,entidad,ref,resultado,motivo,antes_json,despues_json,durable
      FROM eventos WHERE (?4 IS NULL OR modulo=?4)
       AND (?5 IS NULL OR (actor IN ('SISTEMA','NO_IDENTIFICADO') AND actor=?5) OR cast(usuario AS TEXT)=?5)
      ORDER BY fecha DESC,durable DESC,orden DESC", &[&amb,&desde,&hasta,&f.modulo,&f.actor], |r| {
        let clase: String = r.get(1)?;
        let actor = if clase == "USUARIO" || r.get::<_, bool>(11)? { seg(r,2)? }
            else { t(if clase == "SISTEMA" {"Sistema"} else {"No identificado"}) };
        let modulo: String = r.get(3)?;
        let entidad: Option<String> = r.get(5)?;
        let referencia: Option<String> = r.get(6)?;
        let destino = match (modulo.as_str(),entidad.as_deref()) {
            ("POS",Some("ventas")) => Some(Destino::Pos),
            ("CAJA",Some("caja_chica")) => Some(Destino::Caja),
            ("OT",Some("ordenes_trabajo")) => Some(Destino::Taller),
            ("INVENTARIO",Some("productos")) => Some(Destino::Inventario),
            ("ADMIN",Some("usuarios")) => Some(Destino::Administracion),
            ("BACKUP",Some("restauraciones")) => Some(Destino::Backup),
            _ => None,
        };
        let origen = destino.zip(referencia.as_deref().and_then(|x| x.parse().ok())).map(|(destino,id)| Origen::nuevo(destino,id));
        let referencia = match (entidad.and_then(|s| texto_seguro(&s)), referencia.and_then(|s| texto_seguro(&s))) {
            (Some(e),Some(i)) => t(format!("{e} · {i}")),
            (e,i) => dato(e.or(i)),
        };
        let motivo = seg(r,8)?;
        let mut detalle = Detalle::pares(vec![("Referencia",referencia.clone()),("Motivo",motivo.clone())]);
        cambios("Antes",r.get(9)?,&mut detalle.filas);
        cambios("Después",r.get(10)?,&mut detalle.filas);
        d.agregar(Fila::nueva(vec![txt(r,0)?,actor,t(modulo),seg(r,4)?,referencia,txt(r,7)?,motivo],origen),detalle);
        Ok(())
    })?;
    d.totales.push(("Eventos", n(d.filas.len())));
    Ok(d)
}

pub fn registrar(db: &Connection, actor: i64, corte: &Corte, formato: &str, ok: bool) -> R<()> {
    let resumen = json!({
        "reporte": corte.reporte.codigo(),
        "formato": formato,
        "desde": corte.rango.as_ref().map(|p| &p.0),
        "hasta": corte.rango.as_ref().map(|p| &p.1),
        "filtros": texto_seguro(&corte.filtros_texto),
        "corte": corte.hora,
        "filas": corte.datos.filas.len(),
    });
    db.execute("INSERT INTO auditoria(id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,id_entidad,despues_json,resultado)
      SELECT id_usuario,'USUARIO',username,rol,'REPORTES','EXPORTAR_REPORTE','reporte',?2,?3,?4
      FROM usuarios WHERE id_usuario=?1",
      (actor,corte.reporte.codigo(),resumen.to_string(),if ok {"OK"} else {"ERROR"}))?;
    Ok(())
}

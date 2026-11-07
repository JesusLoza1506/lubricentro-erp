//! Adaptador POS: sin worker, calendario ni cola paralela. HTTP siempre fuera de SQLite.
use crate::{
    errors::AppError,
    services::{
        pos_service::{self as pos, consultas, documentos},
        session_manager::SessionState,
    },
};
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
    time::Duration,
};
use tauri::State;

#[derive(Debug, PartialEq)]
pub enum ResultadoDocumento {
    Aceptado,
    Rechazado,
    Ausente,
    Incierto,
}
pub fn interpretar(v: &Value, consulta: bool) -> ResultadoDocumento {
    let (estado, _) = documentos::clasificar_respuesta(v);
    if estado == "ACEPTADO" {
        ResultadoDocumento::Aceptado
    } else if estado == "RECHAZADO" {
        ResultadoDocumento::Rechazado
    } else if consulta && (v["codigo"].as_i64() == Some(24) || v["codigo"].as_str() == Some("24")) {
        ResultadoDocumento::Ausente
    } else {
        ResultadoDocumento::Incierto
    }
}

static EN_CURSO: OnceLock<Mutex<HashSet<i64>>> = OnceLock::new();
struct GuardiaDocumento(i64);
impl GuardiaDocumento {
    fn nueva(id: i64) -> Result<Self, AppError> {
        let mut ids = EN_CURSO
            .get_or_init(|| Mutex::new(HashSet::new()))
            .lock()
            .map_err(|_| AppError::DbLock)?;
        if !ids.insert(id) {
            return Err(pos::error("Este documento ya está siendo procesado."));
        }
        Ok(Self(id))
    }
}
impl Drop for GuardiaDocumento {
    fn drop(&mut self) {
        if let Some(m) = EN_CURSO.get() {
            if let Ok(mut ids) = m.lock() {
                ids.remove(&self.0);
            }
        }
    }
}
fn numero(v: &Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
}

#[tauri::command]
pub async fn pos_documento_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_documento: i64,
    consultar: bool,
    automatico: bool,
    token_envio: Option<String>,
) -> Result<Value, AppError> {
    let _guardia = GuardiaDocumento::nueva(id_documento)?;
    // Scope explícito: ningún MutexGuard atraviesa el await.
    let (actor, ruta, token, request, identidad, es_consulta) = {
        let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
        let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
        let actor = session.validar_sesion_operativa(&conn)?;
        let rol = pos::validar_rol(&conn, actor)?;
        let (venta,payload,sha,estado,intento,consulta,version):(i64,String,String,String,i64,i64,i64)=conn.query_row("SELECT id_venta,payload_json,payload_sha256,estado_envio,intento_iniciado,requiere_consulta,version FROM comprobantes WHERE id_comprobante=?1",[id_documento],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?)))?;
        consultas::visible(&conn, actor, venta)?;
        if rol != "ADMINISTRADOR"
            && (!automatico || consultar || estado != "PENDIENTE" || intento != 0 || consulta != 0)
        {
            return Err(AppError::PermisoDenegado);
        }
        if rol != "ADMINISTRADOR" {
            session.consumir_autorizacion(
                token_envio.as_deref().ok_or(AppError::PermisoDenegado)?,
                actor,
                "POS_ENVIO_INICIAL",
                &id_documento.to_string(),
            )?;
        }
        if ["ACEPTADO", "RECHAZADO"].contains(&estado.as_str()) {
            return Err(pos::error("El documento ya tiene resultado definitivo."));
        }
        // Estado ENVIANDO puede ser un envío vivo o un intento interrumpido: solo consultar.
        let es_consulta = consultar || intento != 0 || consulta != 0 || estado == "ENVIANDO";
        let payload: Value = serde_json::from_str(&payload).map_err(pos::json_error)?;
        documentos::validar_huella(&payload, &sha)?;
        let cfg = pos::configuracion(&conn)?;
        pos::validar_config(&cfg)?;
        let ruta = cfg["ruta"].as_str().unwrap_or("").to_string();
        let url = reqwest::Url::parse(&ruta).map_err(|_| pos::error("Ruta NubeFact inválida."))?;
        if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
            return Err(pos::error(
                "La integración requiere una ruta HTTPS sin credenciales en la URL.",
            ));
        }
        let servicio = format!(
            "lubricentro_erp_nubefact_{}",
            cfg["ambiente"].as_str().unwrap_or("")
        );
        let token=keyring::Entry::new(&servicio,cfg["token_ref"].as_str().unwrap_or("")).map_err(|_|pos::error("No se pudo acceder al almacén de credenciales."))?.get_password().map_err(|_|pos::error("No se encontró el token protegido de este ambiente. Configúralo en Administración."))?;
        let request = if es_consulta {
            json!({"operacion":"consultar_comprobante","tipo_de_comprobante":payload["tipo_de_comprobante"],"serie":payload["serie"],"numero":payload["numero"]})
        } else {
            payload.clone()
        };
        let tx = conn.transaction()?;
        let cambios=tx.execute("UPDATE comprobantes SET estado_envio='ENVIANDO',intento_iniciado=1,requiere_consulta=1,fecha_ultimo_intento=datetime('now','-5 hours'),version=version+1 WHERE id_comprobante=?1 AND version=?2",params![id_documento,version])?;
        if cambios != 1 {
            return Err(pos::error("El documento cambió. Actualiza su detalle."));
        }
        pos::auditar(
            &tx,
            actor,
            &uuid::Uuid::new_v4().to_string(),
            if es_consulta {
                "CONSULTA_INICIADA"
            } else {
                "ENVIO_INICIADO"
            },
            "comprobantes",
            id_documento,
            &json!({"serie":payload["serie"],"numero":payload["numero"]}),
            "",
            None,
        )?;
        tx.commit()?;
        (actor, ruta, token, request, payload, es_consulta)
    };
    let respuesta = async {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(25))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| ())?;
        let res = client
            .post(ruta)
            .header("Authorization", format!("Token token={token}"))
            .json(&request)
            .send()
            .await
            .map_err(|_| ())?;
        let codigo = res.status();
        let body = res.json::<Value>().await.map_err(|_| ())?;
        Ok::<_, ()>((codigo, body))
    }
    .await;
    let (resultado, respuesta) = match respuesta {
        Ok((http, v)) => {
            let coincide = v["serie"] == identidad["serie"]
                && numero(&v["numero"]) == numero(&identidad["numero"])
                && numero(&v["tipo_de_comprobante"]) == numero(&identidad["tipo_de_comprobante"]);
            let resultado = interpretar(&v, es_consulta);
            if (!http.is_success() || !coincide) && resultado != ResultadoDocumento::Ausente {
                (ResultadoDocumento::Incierto, v)
            } else {
                (resultado, v)
            }
        }
        Err(_) => (
            ResultadoDocumento::Incierto,
            json!({"sunat_description":"No se obtuvo una respuesta definitiva. Consulta el estado antes de reenviar."}),
        ),
    };
    let estado = match resultado {
        ResultadoDocumento::Aceptado => "ACEPTADO",
        ResultadoDocumento::Rechazado => "RECHAZADO",
        ResultadoDocumento::Ausente => "PENDIENTE",
        ResultadoDocumento::Incierto => "ERROR",
    };
    let incierto = resultado == ResultadoDocumento::Incierto;
    let mensaje = respuesta["sunat_description"]
        .as_str()
        .or(respuesta["errors"].as_str())
        .unwrap_or(if resultado == ResultadoDocumento::Ausente {
            "El proveedor confirmó ausencia. Puedes enviar el mismo documento."
        } else {
            "Respuesta pendiente de validación. Consulta nuevamente."
        });
    let mensaje: String = mensaje.chars().take(500).collect();
    let codigo = respuesta["sunat_responsecode"]
        .as_str()
        .map(str::to_string)
        .or_else(|| respuesta.get("codigo").map(Value::to_string));
    let link = |k: &str| {
        respuesta[k]
            .as_str()
            .filter(|s| s.starts_with("https://"))
            .map(str::to_string)
    };
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    let tx = conn.transaction()?;
    // Un intento posterior no puede degradar una respuesta definitiva ya guardada.
    tx.execute("UPDATE comprobantes SET estado_envio=?1,intento_iniciado=0,requiere_consulta=?2,resultado_incierto=?2,fecha_ultima_consulta=CASE WHEN ?3 THEN datetime('now','-5 hours') ELSE fecha_ultima_consulta END,fecha_confirmacion=CASE WHEN ?1 IN ('ACEPTADO','RECHAZADO') THEN datetime('now','-5 hours') ELSE NULL END,codigo_respuesta=?4,respuesta_resumida=?5,advertencias_json=?6,ruta_pdf=coalesce(?7,ruta_pdf),ruta_xml=coalesce(?8,ruta_xml),ruta_cdr=coalesce(?9,ruta_cdr),version=version+1 WHERE id_comprobante=?10 AND estado_envio NOT IN ('ACEPTADO','RECHAZADO')",params![estado,incierto,es_consulta,codigo,mensaje,serde_json::to_string(&respuesta["sunat_note"]).map_err(pos::json_error)?,link("enlace_del_pdf"),link("enlace_del_xml"),link("enlace_del_cdr"),id_documento])?;
    pos::auditar(
        &tx,
        actor,
        &uuid::Uuid::new_v4().to_string(),
        "RESULTADO_DOCUMENTAL",
        "comprobantes",
        id_documento,
        &json!({"estado":estado,"incierto":incierto}),
        "",
        None,
    )?;
    tx.commit()?;
    Ok(
        json!({"id_documento":id_documento,"estado":estado,"mensaje":mensaje,"requiere_consulta":incierto}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeout_o_duplicado_no_equivale_aceptado() {
        assert_eq!(
            interpretar(
                &json!({"errors":"El comprobante ya existe","codigo":23}),
                false
            ),
            ResultadoDocumento::Incierto
        );
        assert_eq!(
            interpretar(&json!({"aceptada_por_sunat":false}), true),
            ResultadoDocumento::Incierto
        );
    }
    #[test]
    fn ausencia_solo_consulta_confirmada() {
        assert_eq!(
            interpretar(&json!({"codigo":24}), true),
            ResultadoDocumento::Ausente
        );
        assert_eq!(
            interpretar(&json!({"codigo":24}), false),
            ResultadoDocumento::Incierto
        );
    }
    #[test]
    fn aceptacion_y_rechazo_formal() {
        assert_eq!(
            interpretar(&json!({"aceptada_por_sunat":true}), false),
            ResultadoDocumento::Aceptado
        );
        assert_eq!(
            interpretar(
                &json!({"aceptada_por_sunat":false,"sunat_responsecode":"2335"}),
                true
            ),
            ResultadoDocumento::Rechazado
        );
    }
}

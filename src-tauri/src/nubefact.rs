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

fn requiere_consulta(consultar: bool, estado: &str, intento: i64, consulta: i64) -> bool {
    consultar || intento != 0 || consulta != 0 || estado == "ENVIANDO"
}

fn solicitud_documental(payload: &Value, es_consulta: bool) -> Value {
    if es_consulta {
        json!({"operacion":"consultar_comprobante","tipo_de_comprobante":payload["tipo_de_comprobante"],"serie":payload["serie"],"numero":payload["numero"]})
    } else {
        payload.clone()
    }
}

fn registrar_intento(
    conn: &mut Connection,
    actor: i64,
    id_documento: i64,
    version: i64,
    payload: &Value,
    es_consulta: bool,
) -> Result<(), AppError> {
    let tx = conn.transaction()?;
    // Consultar conserva el estado: la protección SQLite solo permite volver a ENVIANDO
    // después de una ausencia confirmada. Ambas acciones dejan una marca durable.
    let cambios=tx.execute("UPDATE comprobantes SET estado_envio=CASE WHEN ?3 THEN estado_envio ELSE 'ENVIANDO' END,intento_iniciado=1,requiere_consulta=1,fecha_ultimo_intento=datetime('now','-5 hours'),version=version+1 WHERE id_comprobante=?1 AND version=?2",params![id_documento,version,es_consulta])?;
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
    Ok(())
}

#[tauri::command]
pub async fn pos_documento_cmd(
    backup_state: tauri::State<'_, crate::services::backup_service::BackupState>,
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_documento: i64,
    consultar: bool,
    automatico: bool,
    token_envio: Option<String>,
) -> Result<Value, AppError> {
    let _backup = backup_state
        .envios
        .try_read()
        .map_err(|_| AppError::Validation("La restauración está en curso.".into()))?;
    let _guardia = GuardiaDocumento::nueva(id_documento)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(25))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| pos::error("No se pudo preparar la conexión con NubeFact."))?;
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
        let es_consulta = requiere_consulta(consultar, &estado, intento, consulta);
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
        let request = solicitud_documental(&payload, es_consulta);
        registrar_intento(
            &mut conn,
            actor,
            id_documento,
            version,
            &payload,
            es_consulta,
        )?;
        (actor, ruta, token, request, payload, es_consulta)
    };
    let respuesta = async {
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
    let (resultado, respuesta) = clasificar_http(respuesta, &identidad, es_consulta);
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    guardar_resultado(
        &mut conn,
        actor,
        id_documento,
        es_consulta,
        resultado,
        &respuesta,
    )
}

fn clasificar_http(
    respuesta: Result<(reqwest::StatusCode, Value), ()>,
    identidad: &Value,
    es_consulta: bool,
) -> (ResultadoDocumento, Value) {
    match respuesta {
        Ok((http, v)) => {
            let coincide = v["serie"] == identidad["serie"]
                && numero(&v["numero"]) == numero(&identidad["numero"])
                && numero(&v["tipo_de_comprobante"]) == numero(&identidad["tipo_de_comprobante"]);
            let resultado = interpretar(&v, es_consulta);
            match resultado {
                ResultadoDocumento::Aceptado | ResultadoDocumento::Rechazado
                    if !http.is_success() || !coincide =>
                {
                    (
                        ResultadoDocumento::Incierto,
                        json!({"sunat_description":"La respuesta no confirma este comprobante. Consulta su estado antes de reenviar."}),
                    )
                }
                _ => (resultado, v),
            }
        }
        Err(_) => (
            ResultadoDocumento::Incierto,
            json!({"sunat_description":"No se obtuvo una respuesta definitiva. Consulta el estado antes de reenviar."}),
        ),
    }
}

fn guardar_resultado(
    conn: &mut Connection,
    actor: i64,
    id_documento: i64,
    es_consulta: bool,
    resultado: ResultadoDocumento,
    respuesta: &Value,
) -> Result<Value, AppError> {
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
        .unwrap_or(match resultado {
            ResultadoDocumento::Aceptado => "Comprobante aceptado.",
            ResultadoDocumento::Rechazado => "Comprobante rechazado.",
            ResultadoDocumento::Ausente => {
                "El proveedor confirmó ausencia. Puedes enviar el mismo documento."
            }
            ResultadoDocumento::Incierto => {
                "Respuesta pendiente de validación. Consulta nuevamente."
            }
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
    fn documento_temporal(conn: &mut Connection) -> (i64, Value) {
        use crate::dto::pos_dto::{ConfirmarVentaPos, LineaDirecta, PagoPos};

        conn.execute_batch(
            "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol)
             VALUES(1,'Admin de prueba','admin','$argon2id$prueba','ADMINISTRADOR');
             UPDATE configuracion SET ruc='20600695771',razon_social='Empresa de prueba',
             direccion='Av. Principal 123',ubigeo='110201',
             nubefact_ruta_api='https://api.nubefact.com/api/v1/prueba',nubefact_token_ref='token_prueba';
             INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,activa)
             VALUES('PRODUCCION','BOLETA','B001',1);
             INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent)
             VALUES('caja-pos-prueba-doc','CAJA-POS',1,'PRODUCCION',10000);
             INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent)
             VALUES(1,'SER-DOC','Servicio de prueba',2000);",
        )
        .unwrap();
        let v = pos::ventas::confirmar(
            conn,
            1,
            None,
            &ConfirmarVentaPos {
                id_operacion: uuid::Uuid::new_v4().to_string(),
                id_ot: None,
                id_cliente: 1,
                tipo_comprobante: "BOLETA".into(),
                descuento_bp: 0,
                motivo: String::new(),
                lineas: vec![LineaDirecta {
                    tipo: "SERVICIO".into(),
                    id: 1,
                    cantidad: 1,
                    precio_esperado_cent: 2000,
                }],
                pagos: vec![PagoPos {
                    medio: "EFECTIVO".into(),
                    monto_cent: 2000,
                    recibido_cent: Some(2000),
                    referencia: None,
                }],
            },
        )
        .unwrap();
        let id = v["id_documento"].as_i64().unwrap();
        let payload: String = conn
            .query_row(
                "SELECT payload_json FROM comprobantes WHERE id_comprobante=?1",
                [id],
                |r| r.get(0),
            )
            .unwrap();
        (id, serde_json::from_str(&payload).unwrap())
    }

    #[test]
    fn recuperacion_consulta_identidad_antes_de_reenviar() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = crate::db::init_db(dir.path().into()).unwrap();
        let (id, payload) = documento_temporal(&mut conn);
        let estado = |conn: &Connection| {
            conn.query_row(
                "SELECT estado_envio,intento_iniciado,requiere_consulta,version
                 FROM comprobantes WHERE id_comprobante=?1",
                [id],
                |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, i64>(1)?,
                        r.get::<_, i64>(2)?,
                        r.get::<_, i64>(3)?,
                    ))
                },
            )
            .unwrap()
        };
        let (_, _, _, version) = estado(&conn);
        // Si falla el registro antes de HTTP, ni siquiera se guarda el intento.
        conn.execute_batch(
            "CREATE TRIGGER prueba_fallo_auditoria BEFORE INSERT ON auditoria
             BEGIN SELECT RAISE(ABORT,'Fallo de auditoría de prueba'); END;",
        )
        .unwrap();
        assert!(registrar_intento(&mut conn, 1, id, version, &payload, false).is_err());
        assert_eq!(estado(&conn), ("PENDIENTE".into(), 0, 0, version));
        conn.execute_batch("DROP TRIGGER prueba_fallo_auditoria;")
            .unwrap();

        let guardia = GuardiaDocumento::nueva(id).unwrap();
        assert!(GuardiaDocumento::nueva(id).is_err());
        registrar_intento(&mut conn, 1, id, version, &payload, false).unwrap();
        assert_eq!(estado(&conn).0, "ENVIANDO");
        // Reinicio después del envío; solo queda la identidad y la marca durable.
        drop(guardia);
        drop(conn);
        let mut conn = crate::db::init_db(dir.path().into()).unwrap();
        let _guardia = GuardiaDocumento::nueva(id).unwrap();
        let (actual, intento, consulta, version) = estado(&conn);
        assert!(requiere_consulta(false, &actual, intento, consulta));
        assert_eq!(
            solicitud_documental(&payload, true),
            json!({
                "operacion": "consultar_comprobante",
                "tipo_de_comprobante": payload["tipo_de_comprobante"],
                "serie": payload["serie"],
                "numero": payload["numero"]
            })
        );
        registrar_intento(&mut conn, 1, id, version, &payload, true).unwrap();
        let (resultado, cuerpo) = clasificar_http(Err(()), &payload, true);
        guardar_resultado(&mut conn, 1, id, true, resultado, &cuerpo).unwrap();

        let (actual, intento, consulta, version) = estado(&conn);
        assert_eq!(actual, "ERROR");
        assert!(requiere_consulta(false, &actual, intento, consulta));
        assert!(registrar_intento(&mut conn, 1, id, version, &payload, false).is_err());
        registrar_intento(&mut conn, 1, id, version, &payload, true).unwrap();
        let (resultado, cuerpo) = clasificar_http(
            Ok((reqwest::StatusCode::BAD_REQUEST, json!({"codigo":24}))),
            &payload,
            true,
        );
        guardar_resultado(&mut conn, 1, id, true, resultado, &cuerpo).unwrap();

        let (actual, intento, consulta, version) = estado(&conn);
        assert_eq!(actual, "PENDIENTE");
        assert!(!requiere_consulta(false, &actual, intento, consulta));
        assert_eq!(solicitud_documental(&payload, false), payload);
        registrar_intento(&mut conn, 1, id, version, &payload, false).unwrap();
        let mut cuerpo = json!({
            "tipo_de_comprobante": payload["tipo_de_comprobante"], "serie": payload["serie"],
            "numero": payload["numero"], "aceptada_por_sunat": true,
            "sunat_note": "Observación de prueba"
        });
        cuerpo["serie"] = json!("OTRA");
        assert_eq!(
            clasificar_http(
                Ok((reqwest::StatusCode::OK, cuerpo.clone())),
                &payload,
                false
            )
            .0,
            ResultadoDocumento::Incierto,
        );
        cuerpo["serie"] = payload["serie"].clone();
        let (resultado, cuerpo) =
            clasificar_http(Ok((reqwest::StatusCode::OK, cuerpo)), &payload, false);
        guardar_resultado(&mut conn, 1, id, false, resultado, &cuerpo).unwrap();
        assert_eq!(estado(&conn).0, "ACEPTADO");
        let nota: String = conn.query_row(
            "SELECT json_extract(advertencias_json,'$') FROM comprobantes WHERE id_comprobante=?1",
            [id], |r| r.get(0),
        ).unwrap();
        assert_eq!(nota, "Observación de prueba");
        for tabla in ["ventas", "comprobantes", "movimientos_caja"] {
            let cantidad: i64 = conn
                .query_row(&format!("SELECT count(*) FROM {tabla}"), [], |r| r.get(0))
                .unwrap();
            assert_eq!(cantidad, 1);
        }
    }

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

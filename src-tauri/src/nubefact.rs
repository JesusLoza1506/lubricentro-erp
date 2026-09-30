use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};

/// Representa los estados posibles de un envío a NubeFact según la respuesta de SUNAT
#[derive(Debug, PartialEq, Clone)]
pub enum EstadoEnvio {
    Pendiente,
    EnCola,
    Aceptado,
    Rechazado,
    Observado,
    Error(String),
}

/// Estructura de ítem según el manual oficial de NubeFact
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPayload {
    pub unidad_de_medida: String,
    pub codigo: String,
    pub codigo_producto_sunat: String,
    pub descripcion: String,
    pub cantidad: f64,
    pub valor_unitario: f64,
    pub precio_unitario: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descuento: Option<f64>,
    pub subtotal: f64,
    pub tipo_de_igv: i32,
    pub igv: f64,
    pub total: f64,
    pub anticipo_regularizacion: bool,
    pub anticipo_documento_serie: String,
    pub anticipo_documento_numero: String,
}

/// Estructura de payload principal para la Operación 1 (Generar Comprobante)
#[derive(Debug, Serialize, Deserialize)]
pub struct ComprobantePayload {
    pub operacion: String,
    pub tipo_de_comprobante: i32,
    pub serie: String,
    pub numero: i32,
    pub sunat_transaction: i32,
    pub cliente_tipo_de_documento: i32,
    pub cliente_numero_de_documento: String,
    pub cliente_denominacion: String,
    pub cliente_direccion: String,
    pub cliente_email: String,
    pub fecha_de_emision: String,
    pub moneda: i32,
    pub porcentaje_de_igv: f64,
    pub total_gravada: f64,
    pub total_igv: f64,
    pub total: f64,
    pub detraccion: bool,
    pub enviar_automaticamente_a_la_sunat: bool,
    pub enviar_automaticamente_al_cliente: bool,
    pub cancelado: bool,
    pub items: Vec<ItemPayload>,
}

/// Estructura para capturar la respuesta JSON oficial de NubeFact
#[derive(Debug, Serialize, Deserialize)]
pub struct NubeFactResponse {
    pub tipo_de_comprobante: Option<i32>,
    pub serie: Option<String>,
    pub numero: Option<i32>,
    pub enlace: Option<String>,
    pub aceptada_por_sunat: Option<bool>,
    pub sunat_description: Option<String>,
    pub sunat_note: Option<String>,
    pub sunat_responsecode: Option<String>,
    pub sunat_soap_error: Option<String>,
    pub codigo_hash: Option<String>,
}

/// Función que realiza la petición HTTP REAL con reqwest a la API de NubeFact
#[cfg(not(tarpaulin_include))]
pub async fn enviar_a_nubefact(
    payload: &ComprobantePayload,
) -> Result<EstadoEnvio, reqwest::Error> {
    let url = std::env::var("NUBEFACT_URL")
        .unwrap_or_else(|_| "https://api.nubefact.com/api/v1/sandbox".to_string());

    let token =
        std::env::var("NUBEFACT_TOKEN").unwrap_or_else(|_| "token_sandbox_default".to_string());

    info!(
        "🚀 Enviando comprobante {} - {} a NubeFact...",
        payload.serie, payload.numero
    );

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status = res.status();
    let body_text = res.text().await.unwrap_or_default();

    if status.is_success() {
        match serde_json::from_str::<NubeFactResponse>(&body_text) {
            Ok(parsed) => {
                let aceptada = parsed.aceptada_por_sunat.unwrap_or(false);
                let desc = parsed.sunat_description.unwrap_or_default();
                let code = parsed.sunat_responsecode.unwrap_or_default();

                if aceptada {
                    info!("✅ NubeFact / SUNAT: ACEPTADO -> {}", desc);
                    Ok(EstadoEnvio::Aceptado)
                } else if code == "0" || desc.to_lowercase().contains("observ") {
                    info!("⚠️ NubeFact / SUNAT: OBSERVADO -> {}", desc);
                    Ok(EstadoEnvio::Observado)
                } else {
                    error!(
                        "❌ NubeFact / SUNAT: RECHAZADO [Code: {}] -> {}",
                        code, desc
                    );
                    Ok(EstadoEnvio::Rechazado)
                }
            }
            Err(e) => {
                error!(
                    "⚠️ Respuesta exitosa pero no se pudo parsear el JSON: {:?}. Body: {}",
                    e, body_text
                );
                Ok(EstadoEnvio::Error(body_text))
            }
        }
    } else {
        error!(
            "❌ Error HTTP en la petición a NubeFact [{}]: {}",
            status, body_text
        );
        if body_text.contains("ya existe") {
            Ok(EstadoEnvio::Aceptado)
        } else {
            Ok(EstadoEnvio::Rechazado)
        }
    }
}

/// Inicia el worker asíncrono que corre en segundo plano.
/// Revisa la base de datos, extrae el comprobante pendiente y lo envía a NubeFact.
#[cfg(not(tarpaulin_include))]
pub async fn iniciar_worker_nubefact(app_data_dir: PathBuf) {
    info!("🔄 Iniciando worker de NubeFact en segundo plano...");
    let mut intentos_fallidos: u32 = 0;

    loop {
        let delay_segundos = if intentos_fallidos > 0 {
            let backoff = 5 * (2_u64.pow(intentos_fallidos - 1));
            std::cmp::min(backoff, 300)
        } else {
            5
        };

        tokio::time::sleep(Duration::from_secs(delay_segundos)).await;
        info!("🔍 Worker revisando la base de datos para envío a NubeFact...");

        let tarea_pendiente = match crate::db::init_db(app_data_dir.clone()) {
            Ok(conn) => {
                let mut stmt = conn.prepare(
                    "SELECT c.id_comprobante, c.tipo_comprobante, s.serie, c.correlativo, c.monto_subtotal, c.monto_igv, c.monto_total, 
                            cl.tipo_documento, cl.numero_documento, cl.nombre_razon_social, cola.id_cola 
                     FROM cola_envio_sunat cola
                     JOIN comprobantes c ON cola.id_comprobante = c.id_comprobante
                     JOIN clientes cl ON c.id_cliente = cl.id_cliente
                     LEFT JOIN series_comprobante s ON c.id_serie = s.id_serie
                     WHERE cola.estado_envio = 'PENDIENTE'
                     LIMIT 1"
                ).ok();

                stmt.as_mut().and_then(|s| {
                    s.query_row([], |row| {
                        Ok((
                            row.get::<_, i32>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, Option<String>>(2)?
                                .unwrap_or_else(|| "F001".to_string()),
                            row.get::<_, i32>(3)?,
                            row.get::<_, f64>(4)?,
                            row.get::<_, f64>(5)?,
                            row.get::<_, f64>(6)?,
                            row.get::<_, String>(7)?,
                            row.get::<_, String>(8)?,
                            row.get::<_, String>(9)?,
                            row.get::<_, i32>(10)?,
                        ))
                    })
                    .ok()
                })
            }
            Err(e) => {
                error!("❌ Error de acceso a BD en worker: {:?}", e);
                None
            }
        };

        match tarea_pendiente {
            Some((
                comp_id,
                _tipo_comp,
                serie,
                correlativo,
                subtotal,
                igv,
                total,
                cli_tipo_doc,
                cli_num_doc,
                cli_razon,
                cola_id,
            )) => {
                info!(
                    "📦 Comprobante encontrado en cola: {} - {}",
                    serie, correlativo
                );

                let payload = ComprobantePayload {
                    operacion: "generar_comprobante".to_string(),
                    tipo_de_comprobante: 1,
                    serie,
                    numero: correlativo,
                    sunat_transaction: 1,
                    cliente_tipo_de_documento: match cli_tipo_doc.as_str() {
                        "RUC" => 6,
                        "DNI" => 1,
                        _ => 1,
                    },
                    cliente_numero_de_documento: cli_num_doc,
                    cliente_denominacion: cli_razon,
                    cliente_direccion: "AV. LIBERTAD 123 - LIMA".to_string(),
                    cliente_email: "tucliente@gmail.com".to_string(),
                    fecha_de_emision: chrono::Local::now().format("%d-%m-%Y").to_string(),
                    moneda: 1,
                    porcentaje_de_igv: 18.0,
                    total_gravada: subtotal,
                    total_igv: igv,
                    total,
                    detraccion: false,
                    enviar_automaticamente_a_la_sunat: true,
                    enviar_automaticamente_al_cliente: false,
                    cancelado: true,
                    items: vec![ItemPayload {
                        unidad_de_medida: "NIU".to_string(),
                        codigo: "001".to_string(),
                        codigo_producto_sunat: "10000000".to_string(),
                        descripcion: "Servicio de Lubricentro y Mantenimiento".to_string(),
                        cantidad: 1.0,
                        valor_unitario: subtotal,
                        precio_unitario: total,
                        descuento: None,
                        subtotal,
                        tipo_de_igv: 1,
                        igv,
                        total,
                        anticipo_regularizacion: false,
                        anticipo_documento_serie: "".to_string(),
                        anticipo_documento_numero: "".to_string(),
                    }],
                };

                match enviar_a_nubefact(&payload).await {
                    Ok(EstadoEnvio::Aceptado) => {
                        if let Ok(conn) = crate::db::init_db(app_data_dir.clone()) {
                            let _ = conn.execute("UPDATE cola_envio_sunat SET estado_envio = 'ENVIADO' WHERE id_cola = ?1", [&cola_id]);
                            let _ = conn.execute("UPDATE comprobantes SET estado_sunat = 'ACEPTADO' WHERE id_comprobante = ?1", [&comp_id]);
                            info!("✅ Base de datos actualizada: Comprobante ACEPTADO por SUNAT.");
                        }
                        intentos_fallidos = 0;
                    }
                    Ok(EstadoEnvio::Observado) => {
                        if let Ok(conn) = crate::db::init_db(app_data_dir.clone()) {
                            let _ = conn.execute("UPDATE cola_envio_sunat SET estado_envio = 'ENVIADO' WHERE id_cola = ?1", [&cola_id]);
                            let _ = conn.execute("UPDATE comprobantes SET estado_sunat = 'OBSERVADO' WHERE id_comprobante = ?1", [&comp_id]);
                            info!("⚠️ Base de datos actualizada: Comprobante OBSERVADO.");
                        }
                        intentos_fallidos = 0;
                    }
                    Ok(EstadoEnvio::Rechazado) => {
                        intentos_fallidos += 1;
                        if let Ok(conn) = crate::db::init_db(app_data_dir.clone()) {
                            let _ = conn.execute("UPDATE cola_envio_sunat SET estado_envio = 'ERROR', intentos = ?1 WHERE id_cola = ?2", (intentos_fallidos, cola_id));
                            let _ = conn.execute("UPDATE comprobantes SET estado_sunat = 'RECHAZADO' WHERE id_comprobante = ?1", [&comp_id]);
                            info!("❌ Base de datos actualizada: Comprobante RECHAZADO.");
                        }
                    }
                    _ => {
                        intentos_fallidos += 1;
                    }
                }
            }
            None => {
                info!("ℹ Cola vacía, sin comprobantes pendientes.");
                intentos_fallidos = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estructura_payload_comprobante() {
        let payload = ComprobantePayload {
            operacion: "generar_comprobante".to_string(),
            tipo_de_comprobante: 1,
            serie: "FFF1".to_string(),
            numero: 1,
            sunat_transaction: 1,
            cliente_tipo_de_documento: 6,
            cliente_numero_de_documento: "20600695771".to_string(),
            cliente_denominacion: "NUBEFACT SA".to_string(),
            cliente_direccion: "AV. LIBERTAD 123".to_string(),
            cliente_email: "test@gmail.com".to_string(),
            fecha_de_emision: "29-09-2026".to_string(),
            moneda: 1,
            porcentaje_de_igv: 18.0,
            total_gravada: 100.0,
            total_igv: 18.0,
            total: 118.0,
            detraccion: false,
            enviar_automaticamente_a_la_sunat: true,
            enviar_automaticamente_al_cliente: false,
            cancelado: true,
            items: vec![],
        };

        assert_eq!(payload.serie, "FFF1");
        assert_eq!(payload.total, 118.0);
    }

    #[test]
    fn test_parseo_respuesta_nubefact() {
        let json_data = r#"{
            "tipo_de_comprobante": 1,
            "serie": "FFF1",
            "numero": 66,
            "aceptada_por_sunat": true,
            "sunat_description": "ACEPTADA"
        }"#;

        let res: Result<NubeFactResponse, _> = serde_json::from_str(json_data);
        assert!(res.is_ok());
        let parsed = res.unwrap();
        assert_eq!(parsed.aceptada_por_sunat, Some(true));
    }
}

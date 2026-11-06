use crate::errors::AppError;
use rusqlite::{params, Transaction};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub struct IdempotenciaService;

pub enum ResultadoIdempotente<T> {
    /// Indica que es una solicitud nueva y se devolvió el resultado recién calculado
    Nuevo(T),
    /// Indica que era un reintento idéntico exitoso y se devolvió el JSON previamente guardado
    Reintento(Value),
}

impl IdempotenciaService {
    /// Calcula el hash SHA-256 en formato hexadecimal minúscula a partir de cualquier estructura serializable
    pub fn calcular_sha256<T: Serialize>(payload: &T) -> Result<String, AppError> {
        let json_bytes = serde_json::to_vec(payload).map_err(|e| {
            AppError::Validation(format!("Error serializando payload para hash: {}", e))
        })?;
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        let result = hasher.finalize();

        let hex_string = result
            .iter()
            .fold(String::with_capacity(64), |mut acc, byte| {
                use std::fmt::Write;
                let _ = write!(acc, "{:02x}", byte);
                acc
            });

        Ok(hex_string)
    }

    /// Evalúa la idempotencia dentro de la transacción.
    /// Si la operación ya fue confirmada previamente:
    /// - Mismo payload: retorna el JSON almacenado.
    /// - Payload distinto: retorna un AppError::Validation cancelando la operación.
    pub fn evaluar_y_registrar<T: Serialize>(
        tx: &Transaction,
        id_operacion: Option<&str>,
        comando: &str,
        payload: &T,
        id_usuario: i64,
    ) -> Result<Option<Value>, AppError> {
        let op_id = match id_operacion {
            Some(id) if !id.trim().is_empty() => id.trim(),
            _ => return Ok(None),
        };

        if op_id.len() < 16 || op_id.len() > 100 {
            return Err(AppError::Validation(
                "El id_operacion debe tener entre 16 y 100 caracteres.".into(),
            ));
        }

        let sha256_hex = Self::calcular_sha256(payload)?;

        // Verificar si la solicitud ya existe
        let reg_previo: Option<(String, String, Option<String>)> = tx
            .query_row(
                "SELECT solicitud_sha256, resultado_json, fecha_confirmacion FROM solicitudes_operacion WHERE id_operacion = ?1",
                params![op_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .ok();

        if let Some((sha_previo, json_previo, fecha_conf)) = reg_previo {
            if sha_previo != sha256_hex {
                return Err(AppError::Validation(format!(
                    "Conflicto de Idempotencia: El id_operacion '{}' ya fue utilizado con un payload diferente.",
                    op_id
                )));
            }

            // Solicitud previamente confirmada
            if fecha_conf.is_some() {
                let val_json: Value =
                    serde_json::from_str(&json_previo).unwrap_or(serde_json::json!({
                        "status": "OK",
                        "message": "Operación completada previamente"
                    }));
                return Ok(Some(val_json));
            }
        }

        // Registrar la solicitud en estado PENDING solo si no existe
        let actor_snapshot = format!("{{\"id_usuario\":{}}}", id_usuario);
        let resultado_inicial = "{\"status\":\"PENDING\"}";

        tx.execute(
            "INSERT OR IGNORE INTO solicitudes_operacion (
                id_operacion, comando, solicitud_sha256, id_usuario, actor_snapshot, resultado_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                op_id,
                comando.trim(),
                sha256_hex,
                id_usuario,
                actor_snapshot,
                resultado_inicial
            ],
        )?;

        Ok(None)
    }

    /// Actualiza el resultado final de la solicitud antes de hacer COMMIT.
    /// Si ya tiene fecha_confirmacion (ej. en reintentos), el WHERE condiciona el UPDATE a 0 filas
    /// y no dispara el trigger trg_solicitudes_operacion_no_update.
    pub fn finalizar_solicitud<T: Serialize>(
        tx: &Transaction,
        id_operacion: Option<&str>,
        resultado: &T,
    ) -> Result<(), AppError> {
        let op_id = match id_operacion {
            Some(id) if !id.trim().is_empty() => id.trim(),
            _ => return Ok(()),
        };

        let res_json = serde_json::to_string(resultado)
            .map_err(|e| AppError::Validation(format!("Error serializando resultado: {}", e)))?;

        let now_lima = super::obtener_fecha_lima()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // IMPORTANTE: El WHERE fecha_confirmacion IS NULL garantiza que NUNCA toque filas confirmadas
        tx.execute(
            "UPDATE solicitudes_operacion 
             SET resultado_json = ?1, fecha_confirmacion = ?2 
             WHERE id_operacion = ?3 AND fecha_confirmacion IS NULL",
            params![res_json, now_lima, op_id],
        )?;

        Ok(())
    }
}

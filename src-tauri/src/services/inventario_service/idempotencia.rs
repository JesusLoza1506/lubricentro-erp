use crate::errors::AppError;
use rusqlite::{params, OptionalExtension, Transaction};
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

    /// Consulta un resultado definitivo dentro de la transacción de negocio.
    /// Conserva el nombre para los consumidores existentes, pero no inserta
    /// reservas PENDING: el esquema v7 solo admite resultados inmutables.
    pub fn evaluar_y_registrar<T: Serialize>(
        tx: &Transaction,
        id_operacion: Option<&str>,
        comando: &str,
        payload: &T,
        id_usuario: i64,
    ) -> Result<Option<Value>, AppError> {
        let Some(op_id) = Self::validar_id(id_operacion)? else {
            return Ok(None);
        };
        let sha256_hex = Self::calcular_sha256(payload)?;
        let previo: Option<(String, String, Option<i64>, String)> = tx
            .query_row(
                "SELECT solicitud_sha256, comando, id_usuario, resultado_json
                 FROM solicitudes_operacion WHERE id_operacion = ?1",
                params![op_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()?;

        let Some((sha_previo, comando_previo, actor_previo, json_previo)) = previo else {
            return Ok(None);
        };
        if sha_previo != sha256_hex
            || comando_previo != comando.trim()
            || actor_previo != Some(id_usuario)
        {
            return Err(AppError::Validation(format!(
                "Conflicto de Idempotencia: El id_operacion '{}' ya fue utilizado con un payload diferente, otra operación u otro usuario.",
                op_id
            )));
        }
        let resultado: Value = serde_json::from_str(&json_previo).map_err(|_| {
            AppError::Validation(
                "No se pudo leer el resultado de la operación anterior. Requiere revisión antes de reintentar.".into(),
            )
        })?;
        if resultado.get("status").and_then(Value::as_str) == Some("PENDING") {
            return Err(AppError::Validation(
                "Esta operación conserva un resultado PENDING de una versión anterior. Requiere revisión antes de reintentar para evitar duplicar movimientos.".into(),
            ));
        }
        Ok(Some(resultado))
    }

    /// Inserta una sola vez el resultado definitivo antes del COMMIT.
    /// Si falla esta escritura, el llamador devuelve el error y toda la
    /// transacción se revierte. No modifica resultados ya confirmados.
    pub fn finalizar_solicitud<P: Serialize, T: Serialize>(
        tx: &Transaction,
        id_operacion: Option<&str>,
        comando: &str,
        payload: &P,
        id_usuario: i64,
        resultado: &T,
    ) -> Result<(), AppError> {
        let Some(op_id) = Self::validar_id(id_operacion)? else {
            return Ok(());
        };
        let res_json = serde_json::to_string(resultado)
            .map_err(|e| AppError::Validation(format!("Error serializando resultado: {}", e)))?;
        let valor: Value = serde_json::from_str(&res_json)
            .map_err(|e| AppError::Validation(format!("Error leyendo resultado: {}", e)))?;
        if valor.get("status").and_then(Value::as_str) == Some("PENDING") {
            return Err(AppError::Validation(
                "No se puede confirmar una operación con resultado PENDING.".into(),
            ));
        }
        let sha256_hex = Self::calcular_sha256(payload)?;
        let actor_snapshot = serde_json::json!({ "id_usuario": id_usuario }).to_string();
        let now_lima = super::obtener_fecha_lima()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let insertadas = tx.execute(
            "INSERT INTO solicitudes_operacion (
                id_operacion, comando, solicitud_sha256, id_usuario,
                actor_snapshot, resultado_json, fecha_confirmacion
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                op_id,
                comando.trim(),
                sha256_hex,
                id_usuario,
                actor_snapshot,
                res_json,
                now_lima
            ],
        )?;
        if insertadas != 1 {
            return Err(AppError::Validation(
                "No se pudo registrar el resultado de la operación. No se guardaron los cambios."
                    .into(),
            ));
        }
        Ok(())
    }

    fn validar_id(id_operacion: Option<&str>) -> Result<Option<&str>, AppError> {
        let op_id = match id_operacion {
            Some(id) if !id.trim().is_empty() => id.trim(),
            _ => return Ok(None),
        };
        if op_id.chars().count() < 16 || op_id.chars().count() > 100 {
            return Err(AppError::Validation(
                "El id_operacion debe tener entre 16 y 100 caracteres.".into(),
            ));
        }
        Ok(Some(op_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use serde_json::json;
    use tempfile::tempdir;

    #[test]
    fn resultado_definitivo_inmutable_y_conflictos_de_identidad() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let tx = conn.transaction().unwrap();
        tx.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol)
             VALUES (1, 'Admin Prueba', 'admin_prueba', '$argon2id$prueba', 'ADMINISTRADOR')",
            [],
        )
        .unwrap();
        let id = Some("operacion-regresion-0001");
        let payload = json!({"cantidad": 5});
        let resultado = json!({"status": "OK", "stock_resultante": 15});
        assert!(
            IdempotenciaService::evaluar_y_registrar(&tx, id, "AJUSTAR_STOCK", &payload, 1)
                .unwrap()
                .is_none()
        );
        let reservas: i64 = tx
            .query_row("SELECT COUNT(*) FROM solicitudes_operacion", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(reservas, 0);
        IdempotenciaService::finalizar_solicitud(&tx, id, "AJUSTAR_STOCK", &payload, 1, &resultado)
            .unwrap();
        assert_eq!(
            IdempotenciaService::evaluar_y_registrar(&tx, id, "AJUSTAR_STOCK", &payload, 1)
                .unwrap(),
            Some(resultado)
        );
        for (comando, actor, datos) in [
            ("OTRO_COMANDO", 1, payload.clone()),
            ("AJUSTAR_STOCK", 2, payload.clone()),
            ("AJUSTAR_STOCK", 1, json!({"cantidad": 6})),
        ] {
            assert!(
                IdempotenciaService::evaluar_y_registrar(&tx, id, comando, &datos, actor).is_err()
            );
        }
        assert!(tx
            .execute("UPDATE solicitudes_operacion SET resultado_json = '{}'", [])
            .is_err());
        assert!(tx.execute("DELETE FROM solicitudes_operacion", []).is_err());
        tx.commit().unwrap();
    }

    #[test]
    fn fallo_al_guardar_resultado_revierte_efectos_y_permite_reintentar() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        conn.execute_batch(
            "CREATE TABLE efecto_prueba (valor INTEGER NOT NULL);
             INSERT INTO efecto_prueba VALUES (10);
             CREATE TRIGGER fallo_resultado BEFORE INSERT ON solicitudes_operacion
             BEGIN SELECT RAISE(ABORT, 'fallo inyectado'); END;",
        )
        .unwrap();
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol)
             VALUES (1, 'Admin Prueba', 'admin_prueba', '$argon2id$prueba', 'ADMINISTRADOR')",
            [],
        )
        .unwrap();
        let id = Some("operacion-regresion-0002");
        let payload = json!({"cantidad": 5});
        let resultado = json!({"status": "OK"});
        {
            let tx = conn.transaction().unwrap();
            tx.execute("UPDATE efecto_prueba SET valor = valor + 5", [])
                .unwrap();
            assert!(IdempotenciaService::finalizar_solicitud(
                &tx,
                id,
                "AJUSTAR_STOCK",
                &payload,
                1,
                &resultado
            )
            .is_err());
            // Al salir sin COMMIT, rusqlite revierte la transacción.
        }
        let valor: i64 = conn
            .query_row("SELECT valor FROM efecto_prueba", [], |r| r.get(0))
            .unwrap();
        assert_eq!(valor, 10);
        conn.execute_batch("DROP TRIGGER fallo_resultado;").unwrap();
        let tx = conn.transaction().unwrap();
        assert!(
            IdempotenciaService::evaluar_y_registrar(&tx, id, "AJUSTAR_STOCK", &payload, 1)
                .unwrap()
                .is_none()
        );
        tx.execute("UPDATE efecto_prueba SET valor = valor + 5", [])
            .unwrap();
        IdempotenciaService::finalizar_solicitud(&tx, id, "AJUSTAR_STOCK", &payload, 1, &resultado)
            .unwrap();
        tx.commit().unwrap();
        let valor: i64 = conn
            .query_row("SELECT valor FROM efecto_prueba", [], |r| r.get(0))
            .unwrap();
        assert_eq!(valor, 15);
    }

    #[test]
    fn pending_anterior_es_rechazado_sin_repetir_efectos() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let tx = conn.transaction().unwrap();
        let payload = json!({"cantidad": 5});
        let hash = IdempotenciaService::calcular_sha256(&payload).unwrap();
        tx.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol)
             VALUES (1, 'Admin Prueba', 'admin_prueba', '$argon2id$prueba', 'ADMINISTRADOR')",
            [],
        )
        .unwrap();
        tx.execute(
            "INSERT INTO solicitudes_operacion
             (id_operacion, comando, solicitud_sha256, id_usuario, actor_snapshot, resultado_json)
             VALUES (?1, 'AJUSTAR_STOCK', ?2, 1, '{}', '{\"status\":\"PENDING\"}')",
            params!["operacion-regresion-0003", hash],
        )
        .unwrap();
        let error = IdempotenciaService::evaluar_y_registrar(
            &tx,
            Some("operacion-regresion-0003"),
            "AJUSTAR_STOCK",
            &payload,
            1,
        )
        .unwrap_err();
        assert!(error.to_string().contains("PENDING"));
    }

    #[test]
    fn errores_sql_y_resultados_invalidos_no_se_ocultan() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        let tx = conn.transaction().unwrap();
        assert!(IdempotenciaService::evaluar_y_registrar(
            &tx,
            Some("operacion-regresion-0004"),
            "AJUSTAR_STOCK",
            &json!({}),
            1
        )
        .is_err());
        assert!(IdempotenciaService::finalizar_solicitud(
            &tx,
            Some("operacion-regresion-0004"),
            "AJUSTAR_STOCK",
            &json!({}),
            1,
            &json!({"status": "PENDING"})
        )
        .is_err());
    }
}

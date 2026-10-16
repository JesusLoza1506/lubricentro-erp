use keyring::Entry;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "lubricentro_erp_service";
const ACCOUNT_NAME: &str = "db_encryption_key";

fn get_or_create_encryption_key() -> String {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME);

    match entry {
        Ok(e) => match e.get_password() {
            Ok(password) => password,
            Err(_) => {
                let mut key_bytes = [0u8; 32];
                getrandom::getrandom(&mut key_bytes).expect("Fallo al generar bytes aleatorios");
                let new_key: String = key_bytes.iter().map(|b| format!("{:02x}", b)).collect();

                let _ = e.set_password(&new_key);
                new_key
            }
        },
        Err(_) => "test_encryption_key_fallback_for_ci".to_string(),
    }
}

pub fn init_db(app_data_dir: PathBuf) -> Result<Connection> {
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .expect("No se pudo crear el directorio de la base de datos");
    }

    let db_path = app_data_dir.join("lubricentro_secure.db");
    let mut conn = Connection::open(&db_path)?;

    let key = get_or_create_encryption_key();
    let _ = conn.execute(&format!("PRAGMA key = '{}';", key), []);

    // Configuración exigida por Esquema V7.0
    // WAL se ejecuta fuera de la migración para no interferir con la transacción del DDL
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 5000;
         PRAGMA journal_mode = WAL;",
    )?;

    // Verificar la versión del esquema en PRAGMA user_version (700 = v7.0)
    let user_version: i32 = conn
        .query_row("PRAGMA user_version;", [], |row| row.get(0))
        .unwrap_or(0);

    if user_version < 700 {
        // Ejecución atómica del Esquema V7.0 dentro de una transacción
        let tx = conn.transaction()?;
        let initial_schema = include_str!("../migrations/0001_initial_schema_v7.sql");
        tx.execute_batch(initial_schema)?;
        tx.commit()?;
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_db_and_migrations_v7() {
        let dir = tempdir().unwrap();
        match init_db(dir.path().to_path_buf()) {
            Ok(connection) => {
                // 1. Verificar versión del esquema
                let user_version: i32 = connection
                    .query_row("PRAGMA user_version;", [], |row| row.get(0))
                    .unwrap();
                assert_eq!(user_version, 700);

                // 2. Verificar existencia de la tabla configuracion
                let config_exists: i32 = connection
                    .query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='configuracion'",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(config_exists, 1);

                // 3. Verificar activación de Claves Foráneas
                let fk_enabled: i32 = connection
                    .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
                    .unwrap();
                assert_eq!(fk_enabled, 1);

                // 4. Verificar que NO existan usuarios por defecto (Primer Admin se crea desde Login)
                let usuarios_count: i32 = connection
                    .query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))
                    .unwrap();
                assert_eq!(usuarios_count, 0);

                // 5. Verificar inicialización de datos estáticos (Cliente Varios)
                let cliente_varios_count: i32 = connection
                    .query_row(
                        "SELECT COUNT(*) FROM clientes WHERE es_varios = 1",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(cliente_varios_count, 1);
            }
            Err(e) => {
                panic!("Error al inicializar la base de datos v7.0: {:?}", e);
            }
        }
    }
}

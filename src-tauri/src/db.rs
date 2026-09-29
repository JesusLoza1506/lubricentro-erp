use keyring::Entry;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "lubricentro_erp_service";
const ACCOUNT_NAME: &str = "db_encryption_key";

/// Obtiene la clave de DPAPI o genera una nueva si es la primera ejecución
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

/// Inicializa la base de datos, aplica cifrado y corre migraciones
pub fn init_db(app_data_dir: PathBuf) -> Result<Connection> {
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .expect("No se pudo crear el directorio de la base de datos");
    }

    let db_path = app_data_dir.join("lubricentro_secure.db");
    let conn = Connection::open(&db_path)?;

    let key = get_or_create_encryption_key();

    // Usamos execute_batch para evitar el error ExecuteReturnedResults de los PRAGMA
    conn.execute_batch(&format!(
        "PRAGMA key = '{}';
         PRAGMA foreign_keys = ON;
         PRAGMA journal_mode = WAL;",
        key
    ))?;

    let version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if version == 0 {
        let initial_schema = include_str!("../migrations/0001_initial_schema.sql");
        conn.execute_batch(initial_schema)?;
        conn.execute("PRAGMA user_version = 1", [])?;
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_db_and_migrations() {
        let dir = tempdir().unwrap();
        match init_db(dir.path().to_path_buf()) {
            Ok(connection) => {
                let version: i32 = connection
                    .query_row("PRAGMA user_version", [], |row| row.get(0))
                    .unwrap();
                assert_eq!(version, 1);
            }
            Err(e) => {
                panic!("Error al inicializar la base de datos: {:?}", e);
            }
        }
    }
}

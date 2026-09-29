use keyring::Entry;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "lubricentro_erp_service";
const ACCOUNT_NAME: &str = "db_encryption_key";

/// Obtiene la clave de DPAPI o genera una nueva si es la primera ejecución
fn get_or_create_encryption_key() -> String {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .expect("Fallo al conectar con Windows Credential Manager");

    match entry.get_password() {
        Ok(password) => password,
        Err(_) => {
            // Generar clave aleatoria segura usando bytes aleatorios
            let mut key_bytes = [0u8; 32];
            getrandom::fill(&mut key_bytes).expect("Fallo al generar bytes aleatorios");

            // Convertir a cadena hexadecimal para SQLCipher
            let new_key: String = key_bytes.iter().map(|b| format!("{:02x}", b)).collect();

            entry
                .set_password(&new_key)
                .expect("Fallo al guardar la clave en DPAPI");
            new_key
        }
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

    // 1. Aplicar clave de cifrado SQLCipher (RNF-03)
    let key = get_or_create_encryption_key();
    conn.execute(&format!("PRAGMA key = '{}';", key), [])?;

    // 2. Optimizaciones de SQLite
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute("PRAGMA journal_mode = WAL;", [])?;

    // 3. Orquestador de Migraciones (Control de versiones interno)
    let version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if version == 0 {
        let initial_schema = include_str!("../migrations/0001_initial_schema.sql");
        conn.execute_batch(initial_schema)?;

        // Marcar la base de datos como versión 1 para no repetir el script
        conn.execute("PRAGMA user_version = 1", [])?;
    }

    Ok(conn)
}

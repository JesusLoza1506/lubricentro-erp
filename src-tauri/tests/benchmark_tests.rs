use lubricentro_erp_lib::services::cliente_service::ClienteService;
use rusqlite::{params, Connection};
use std::time::Instant;
use uuid::Uuid;

const DUMMY_ARGON2_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

#[test]
fn benchmark_exhaustivo_p50_p95_50k_vehiculos() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    // Carga de la migración V7 real del proyecto
    let initial_schema = include_str!("../migrations/0001_initial_schema_v7.sql");
    conn.execute_batch(initial_schema).unwrap();

    // 1. Insertar Usuario Administrador con Hash Argon2id Válido
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
         VALUES (1, 'Admin Senior', 'admin_bench', ?1, 'ADMINISTRADOR', 1)",
        [DUMMY_ARGON2_HASH],
    )
    .unwrap();

    println!("Iniciando inserción masiva de 50,000 registros para benchmark V7...");
    let start_seed = Instant::now();

    let mut placas_guardadas = Vec::with_capacity(50000);
    conn.execute_batch("BEGIN TRANSACTION;").unwrap();
    for i in 1..=50000 {
        let c1 = (b'A' + ((i / (26 * 26 * 1000)) % 26) as u8) as char;
        let c2 = (b'A' + ((i / (26 * 1000)) % 26) as u8) as char;
        let c3 = (b'A' + ((i / 1000) % 26) as u8) as char;
        let num = i % 1000;
        let placa = format!("{}{}{}{:03}", c1, c2, c3, num);
        placas_guardadas.push(placa.clone());

        // CORRECCIÓN: Se omite la columna 'activo' ya que no existe en clientes (V7)
        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social)
             VALUES ('DNI', ?1, ?2)",
            params![
                format!("{:08}", i % 100000000),
                format!("Cliente Empresa {}", i)
            ],
        )
        .unwrap();

        let id_cliente_creado = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, kilometraje_actual, activo)
             VALUES (?1, ?2, 'Toyota', 'Hilux', 2022, 35000, 1)",
            params![placa, id_cliente_creado],
        )
        .unwrap();

        let id_vehiculo_creado = conn.last_insert_rowid();

        let id_op_ot = Uuid::new_v4().to_string();
        let codigo_ot = format!("OT-{:06}", i);
        let cliente_json = format!(r#"{{"nombre":"Cliente Empresa {}"}}"#, i);

        conn.execute(
            "INSERT INTO ordenes_trabajo (
                id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido,
                placa_snapshot, cliente_snapshot, id_creador,
                estado, kilometraje_ingreso
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, 'EN_ESPERA', 35000)",
            params![
                id_op_ot,
                codigo_ot,
                id_vehiculo_creado,
                id_cliente_creado,
                placa,
                cliente_json
            ],
        )
        .unwrap();
    }
    conn.execute_batch("COMMIT;").unwrap();
    println!(
        "Inserción masiva de 50,000 completada en: {:?}",
        start_seed.elapsed()
    );

    let mut tiempos_ms: Vec<f64> = Vec::with_capacity(100);

    for sample in 0..100 {
        let index = (sample * 490) % placas_guardadas.len();
        let placa_a_buscar = &placas_guardadas[index];

        let start_query = Instant::now();
        let resultado = ClienteService::obtener_ficha_vehicular_completa(&conn, placa_a_buscar);
        let duracion = start_query.elapsed();

        if let Err(ref e) = resultado {
            panic!(
                "❌ Error en placa '{}' (index {}): {:?}",
                placa_a_buscar, index, e
            );
        }

        tiempos_ms.push(duracion.as_secs_f64() * 1000.0);
    }

    tiempos_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = tiempos_ms.len();
    let p50 = tiempos_ms[len * 50 / 100];
    let p95 = tiempos_ms[(len as f64 * 0.95) as usize];
    let max_latencia = tiempos_ms[len - 1];

    println!("--- RESULTADOS BENCHMARK 50K REGISTROS ---");
    println!("Latencia p50 (Mediana): {:.4} ms", p50);
    println!("Latencia p95: {:.4} ms", p95);
    println!("Latencia Máxima: {:.4} ms", max_latencia);

    assert!(p50 < 200.0, "El benchmark falló en p50: {:.4} ms", p50);
    assert!(p95 < 200.0, "El benchmark falló en p95: {:.4} ms", p95);
}

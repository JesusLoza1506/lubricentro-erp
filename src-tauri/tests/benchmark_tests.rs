use lubricentro_erp_lib::services::cliente_service::ClienteService;
use rusqlite::Connection;
use std::time::Instant;

#[test]
fn benchmark_exhaustivo_p50_p95_50k_vehiculos() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    let initial_schema = include_str!("../migrations/0001_initial_schema.sql");
    conn.execute_batch(initial_schema).unwrap();

    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol)
         VALUES (1, 'Mecanico Senior', 'meca_pro', 'hash', 'MECANICO')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO categorias (id_categoria, nombre_categoria) VALUES (1, 'ACEITES Y LUBRICANTES')",
        [],
    ).unwrap();

    conn.execute(
        "INSERT INTO productos (id_producto, id_categoria, descripcion, precio_venta, precio_compra, stock_actual, stock_minimo, unidad_medida) 
         VALUES (1, 1, 'Aceite Sintetico 5W-30', 5000, 4000, 10.0, 2.0, 'GALON')",
        [],
    ).unwrap();

    println!("Iniciando inserción masiva de 50,000 registros para benchmark avanzado...");
    let start_seed = Instant::now();

    let mut placas_guardadas = Vec::with_capacity(50000);

    conn.execute_batch("BEGIN TRANSACTION;").unwrap();
    for i in 1..=50000 {
        let placa = format!("Z{:03x}-{}", i % 4096, i).to_uppercase();
        placas_guardadas.push(placa.clone());

        conn.execute(
            "INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social)
             VALUES (?1, 'DNI', ?2, ?3)",
            rusqlite::params![i, format!("{:08}", i), format!("Cliente Empresa {}", i)],
        ).unwrap();

        conn.execute(
            "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, kilometraje_actual)
             VALUES (?1, ?2, 'Toyota', 'Hilux', 2022, 35000)",
            rusqlite::params![placa, i],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, estado, kilometraje_ingreso, proximo_kilometraje)
             VALUES (?1, ?2, 1, 'FINALIZADO', 35000, 40000)",
            rusqlite::params![format!("OT-{:06}", i), placa],
        ).unwrap();
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

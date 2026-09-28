#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use std::thread;
    use std::time::Duration;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        // IMPORTANTE: En SQLite las claves foráneas están apagadas por defecto, hay que encenderlas
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        
        let schema = include_str!("../migrations/0001_initial_schema.sql");
        conn.execute_batch(schema).unwrap();
        
        conn
    }

    #[test]
    fn test_crud_clientes() {
        let conn = setup_test_db();

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, direccion) VALUES (?1, ?2, ?3, ?4, ?5)",
            &["DNI", "12345678", "Juan Perez", "999888777", "Av. Lima 123"],
        ).unwrap();

        let nombre: String = conn.query_row(
            "SELECT nombre_razon_social FROM clientes WHERE numero_documento = '12345678'",
            [],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(nombre, "Juan Perez");
    }

    #[test]
    fn test_check_constraint_roles_usuario() {
        let conn = setup_test_db();

        let resultado = conn.execute(
            "INSERT INTO usuarios (nombre_completo, usuario, password_hash, rol, activo) VALUES ('Test', 'testuser', 'hash', 'ROL_INVALIDO', 1)",
            [],
        );

        assert!(resultado.is_err(), "La restricción CHECK debería haber rechazado el rol inválido");
    }

    #[test]
    fn test_orden_trabajo_estado_en_espera() {
        let conn = setup_test_db();

        conn.execute("INSERT INTO usuarios (nombre_completo, usuario, password_hash, rol, activo) VALUES ('Mecanico 1', 'mec1', 'hash', 'MECANICO', 1)", []).unwrap();
        conn.execute("INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social) VALUES ('DNI', '87654321', 'Empresa S.A.')", []).unwrap();
        conn.execute("INSERT INTO vehiculos (placa, id_cliente, marca) VALUES ('ABC-123', 1, 'Toyota')", []).unwrap();

        let resultado = conn.execute(
            "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, estado, kilometraje_ingreso, proximo_kilometraje) VALUES ('OT-001', 'ABC-123', 1, 'EN_ESPERA', 50000, 55000)",
            [],
        );

        assert!(resultado.is_ok(), "El estado 'EN_ESPERA' debe ser válido según el esquema");
    }

    #[test]
    fn test_integridad_referencial_cliente_vehiculo() {
        let conn = setup_test_db();

        // 1. Insertamos un cliente
        conn.execute("INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social) VALUES ('DNI', '99999999', 'Cliente Prueba')", []).unwrap();
        let id_cliente = conn.last_insert_rowid();

        // 2. Le asignamos un vehículo
        conn.execute("INSERT INTO vehiculos (placa, id_cliente, marca) VALUES ('XYZ-789', ?1, 'Nissan')", &[&id_cliente]).unwrap();

        // 3. Intentamos borrar el cliente
        let resultado = conn.execute("DELETE FROM clientes WHERE id_cliente = ?1", &[&id_cliente]);

        // 4. Verificamos que SQLite rechace el borrado por la regla ON DELETE RESTRICT
        assert!(resultado.is_err(), "La restricción FK RESTRICT debe impedir borrar un cliente con vehículos asociados");
    }

    #[test]
    fn test_concurrencia_correlativo_comprobante() {
        // Usamos un archivo físico temporal para la prueba en lugar de memoria
        let db_path = "test_concurrencia.db";
        let _ = std::fs::remove_file(db_path); // Limpiar si quedó de una prueba fallida anterior

        let conn_setup = rusqlite::Connection::open(db_path).unwrap();
        
        // Habilitar WAL (Write-Ahead Logging) para permitir lecturas/escrituras concurrentes
        conn_setup.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn_setup.execute_batch(include_str!("../migrations/0001_initial_schema.sql")).unwrap();
        
        // Creamos la serie inicial en 0
        conn_setup.execute("INSERT INTO series_comprobante (tipo_comprobante, serie, correlativo_actual) VALUES ('FACTURA', 'F001', 0)", []).unwrap();

        let mut threads = vec![];

        // Simulamos 10 peticiones concurrentes
        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let conn = rusqlite::Connection::open("test_concurrencia.db").unwrap();
                conn.busy_timeout(Duration::from_secs(5)).unwrap();
                
                // Transacción atómica
                conn.execute("BEGIN EXCLUSIVE TRANSACTION", []).unwrap();
                conn.execute("UPDATE series_comprobante SET correlativo_actual = correlativo_actual + 1 WHERE serie = 'F001'", []).unwrap();
                conn.execute("COMMIT", []).unwrap();
            });
            threads.push(handle);
        }

        // Esperamos a que todos los hilos terminen
        for t in threads {
            t.join().unwrap();
        }

        // Leemos el resultado final
        let correlativo_final: i32 = conn_setup.query_row(
            "SELECT correlativo_actual FROM series_comprobante WHERE serie = 'F001'",
            [],
            |row| row.get(0),
        ).unwrap();

        // Limpieza del archivo de prueba (debemos cerrar la conexión primero)
        drop(conn_setup);
        let _ = std::fs::remove_file(db_path);
        let _ = std::fs::remove_file(format!("{}-wal", db_path));
        let _ = std::fs::remove_file(format!("{}-shm", db_path));

        // Si la concurrencia es segura, 10 hilos sumando 1 deben resultar exactamente en 10
        assert_eq!(correlativo_final, 10, "Los incrementos concurrentes generaron una colisión");
    }

    #[test]
    fn test_seguridad_cifrado() {
        let db_path = "test_seguridad.db";
        let _ = std::fs::remove_file(db_path);

        // 1. Creamos la BD y la ciframos con una clave
        let conn = rusqlite::Connection::open(db_path).unwrap();
        conn.pragma_update(None, "key", "clave_super_secreta").unwrap();
        conn.execute("CREATE TABLE datos_sensibles (id INTEGER PRIMARY KEY);", []).unwrap();
        drop(conn); // Cerramos la conexión

        // 2. Intentamos abrir y leer la BD SIN proporcionar la clave
        let conn_sin_clave = rusqlite::Connection::open(db_path).unwrap();
        let resultado = conn_sin_clave.execute("SELECT * FROM datos_sensibles", []);

        // 3. Verificamos que SQLite rechace la operación por no tener la clave
        assert!(resultado.is_err(), "La base de datos cifrada NO debe ser legible sin la clave");
        
        let _ = std::fs::remove_file(db_path);
    }
}
#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(
            "
            CREATE TABLE categorias (
                id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
                nombre_categoria TEXT NOT NULL
            );

            CREATE TABLE productos (
                id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
                id_categoria INTEGER NOT NULL,
                nombre TEXT NOT NULL,
                codigo_barras TEXT,
                unidad_medida TEXT,
                stock_actual REAL NOT NULL DEFAULT 0 CHECK (stock_actual >= 0),
                stock_minimo REAL NOT NULL DEFAULT 0 CHECK (stock_minimo >= 0),
                precio_compra REAL NOT NULL CHECK (precio_compra >= 0),
                precio_venta REAL NOT NULL CHECK (precio_venta >= 0),
                FOREIGN KEY (id_categoria) REFERENCES categorias(id_categoria)
            );

            CREATE TABLE auditoria (
                id_auditoria INTEGER PRIMARY KEY AUTOINCREMENT,
                id_usuario INTEGER,
                tabla_afectada TEXT NOT NULL,
                accion TEXT NOT NULL,
                detalle TEXT,
                fecha DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE movimientos_inventario (
                id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
                id_producto INTEGER NOT NULL,
                id_usuario INTEGER,
                tipo_movimiento TEXT NOT NULL,
                cantidad_anterior REAL NOT NULL,
                cantidad_nueva REAL NOT NULL,
                diferencia REAL NOT NULL,
                motivo TEXT,
                fecha DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO categorias (nombre_categoria) VALUES ('Lubricantes');
            ",
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_alta_producto_con_movimiento_y_auditoria_usuario() {
        let mut conn = setup_test_db();
        let tx = conn.transaction().unwrap();

        // 1. Inserción de producto
        tx.execute(
            "INSERT INTO productos (id_categoria, nombre, codigo_barras, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta)
             VALUES (1, 'Aceite Mobil Super 3000 5W-30', '7750001234', 'LITRO', 10.0, 2.0, 25.50, 38.90)",
            [],
        ).unwrap();

        let prod_id = tx.last_insert_rowid();

        // 2. Registro en movimientos_inventario
        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, cantidad_anterior, cantidad_nueva, diferencia, motivo)
             VALUES (?1, 1, 'INICIAL', 0.0, 10.0, 10.0, 'Alta de producto en catálogo')",
            [prod_id],
        ).unwrap();

        // 3. Registro en auditoria asociando id_usuario
        tx.execute(
            "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle)
             VALUES (1, 'productos', 'CREAR', ?1)",
            [format!(
                "ALTA_PRODUCTO | ID: {} | Nombre: 'Aceite Mobil Super 3000 5W-30'",
                prod_id
            )],
        )
        .unwrap();

        tx.commit().unwrap();

        // Verificaciones
        let count_prod: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE id_producto = ?1",
                [prod_id],
                |r| r.get(0),
            )
            .unwrap();
        let count_aud: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM auditoria WHERE accion = 'CREAR' AND id_usuario = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let count_mov: i32 = conn.query_row("SELECT COUNT(*) FROM movimientos_inventario WHERE id_producto = ?1 AND tipo_movimiento = 'INICIAL'", [prod_id], |r| r.get(0)).unwrap();

        assert_eq!(count_prod, 1);
        assert_eq!(count_aud, 1);
        assert_eq!(count_mov, 1);
    }

    #[test]
    fn test_baja_producto_con_auditoria_y_movimiento_usuario() {
        let mut conn = setup_test_db();

        conn.execute(
            "INSERT INTO productos (id_categoria, nombre, stock_actual, stock_minimo, precio_compra, precio_venta)
             VALUES (1, 'Filtro a Eliminar', 5.0, 1.0, 10.0, 15.0)",
            [],
        ).unwrap();

        let prod_id = conn.last_insert_rowid();

        let tx = conn.transaction().unwrap();

        // Movimiento de eliminación
        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, cantidad_anterior, cantidad_nueva, diferencia, motivo)
             VALUES (?1, 1, 'ELIMINACION', 5.0, 0.0, -5.0, 'Eliminación de producto del catálogo')",
            [prod_id],
        ).unwrap();

        // Borrado
        tx.execute("DELETE FROM productos WHERE id_producto = ?1", [prod_id])
            .unwrap();

        // Auditoría con id_usuario = 1
        tx.execute(
            "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) VALUES (1, 'productos', 'ELIMINAR', ?1)",
            [format!("BAJA_PRODUCTO | ID: {}", prod_id)],
        ).unwrap();

        tx.commit().unwrap();

        let count_prod: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE id_producto = ?1",
                [prod_id],
                |r| r.get(0),
            )
            .unwrap();
        let count_aud: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM auditoria WHERE accion = 'ELIMINAR' AND id_usuario = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(count_prod, 0);
        assert_eq!(count_aud, 1);
    }

    #[test]
    fn test_rollback_transaccional_en_falla() {
        let mut conn = setup_test_db();

        let tx_result = (|| -> Result<(), rusqlite::Error> {
            let tx = conn.transaction()?;

            tx.execute(
                "INSERT INTO productos (id_categoria, nombre, precio_compra, precio_venta) VALUES (1, 'Producto Fallido', 10.0, 15.0)",
                [],
            )?;

            Err(rusqlite::Error::ExecuteReturnedResults)
        })();

        assert!(tx_result.is_err());

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM productos", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_restricciones_check_base_datos() {
        let conn = setup_test_db();

        // Intento de insertar stock negativo debe ser rechazado por SQLite CHECK
        let res = conn.execute(
            "INSERT INTO productos (id_categoria, nombre, stock_actual, stock_minimo, precio_compra, precio_venta)
             VALUES (1, 'Producto Ilegal', -5.0, 1.0, 10.0, 15.0)",
            [],
        );

        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests {
    use lubricentro_erp_lib::db::init_db;
    use tempfile::tempdir;

    fn setup_usuario_inicial(conn: &rusqlite::Connection) {
        conn.execute(
            "INSERT OR IGNORE INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (1, 'Usuario Test', 'testuser', 'hash', 'ADMINISTRADOR', 1)",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_alta_producto_con_movimiento_y_auditoria_usuario() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();

        // Satisfacer FKs: Usuario y Categoria
        setup_usuario_inicial(&conn);

        conn.execute(
            "INSERT INTO categorias (nombre_categoria, activo) VALUES ('Lubricantes', 1)",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        let tx = conn.transaction().unwrap();

        // 1. Inserción de producto
        tx.execute(
            "INSERT INTO productos (id_categoria, descripcion, codigo_barras, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo)
             VALUES (?1, 'Aceite Mobil Super 3000 5W-30', '7750001234', 'LITRO', 10.0, 2.0, 2550, 3890, 1)",
            [cat_id],
        )
        .unwrap();

        let prod_id = tx.last_insert_rowid();

        // 2. Registro en movimientos_inventario (stock_anterior + diferencia = stock_resultante)
        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia)
             VALUES (?1, 1, 'INGRESO_COMPRA', 0.0, 10.0, 10.0, 'Alta de producto')",
            [prod_id],
        )
        .unwrap();

        // 3. Registro en auditoria
        tx.execute(
            "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle)
             VALUES (1, 'productos', 'INSERT', ?1)",
            [format!(
                "ALTA_PRODUCTO | ID: {} | Descripcion: 'Aceite Mobil Super 3000 5W-30'",
                prod_id
            )],
        )
        .unwrap();

        tx.commit().unwrap();

        // Verificaciones
        let count_prod: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE id_producto = ?1 AND activo = 1",
                [prod_id],
                |r| r.get(0),
            )
            .unwrap();
        let count_aud: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM auditoria WHERE accion = 'INSERT' AND id_usuario = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let count_mov: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM movimientos_inventario WHERE id_producto = ?1 AND tipo_movimiento = 'INGRESO_COMPRA'",
                [prod_id],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(count_prod, 1);
        assert_eq!(count_aud, 1);
        assert_eq!(count_mov, 1);
    }

    #[test]
    fn test_baja_producto_con_auditoria_y_movimiento_usuario() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();

        // Satisfacer FKs: Usuario y Categoria
        setup_usuario_inicial(&conn);

        conn.execute(
            "INSERT INTO categorias (nombre_categoria, activo) VALUES ('Filtros', 1)",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO productos (id_categoria, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo)
             VALUES (?1, 'Filtro a Eliminar', 'UNIDAD', 5.0, 1.0, 1000, 1500, 1)",
            [cat_id],
        )
        .unwrap();

        let prod_id = conn.last_insert_rowid();

        let tx = conn.transaction().unwrap();

        // Movimiento de salida
        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia)
             VALUES (?1, 1, 'MERMA', 5.0, -5.0, 0.0, 'Eliminación del producto')",
            [prod_id],
        )
        .unwrap();

        // Borrado lógico
        tx.execute(
            "UPDATE productos SET activo = 0 WHERE id_producto = ?1",
            [prod_id],
        )
        .unwrap();

        // Auditoría
        tx.execute(
            "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) VALUES (1, 'productos', 'DELETE', ?1)",
            [format!("BAJA_PRODUCTO | ID: {}", prod_id)],
        )
        .unwrap();

        tx.commit().unwrap();

        let count_prod: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE id_producto = ?1 AND activo = 1",
                [prod_id],
                |r| r.get(0),
            )
            .unwrap();
        let count_aud: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM auditoria WHERE accion = 'DELETE' AND id_usuario = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(count_prod, 0);
        assert_eq!(count_aud, 1);
    }

    #[test]
    fn test_rollback_transaccional_en_falla() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();

        conn.execute(
            "INSERT INTO categorias (nombre_categoria, activo) VALUES ('Grasas', 1)",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        let count_inicial: i32 = conn
            .query_row("SELECT COUNT(*) FROM productos", [], |r| r.get(0))
            .unwrap();

        let tx_result = (|| -> Result<(), rusqlite::Error> {
            let tx = conn.transaction()?;

            tx.execute(
                "INSERT INTO productos (id_categoria, descripcion, unidad_medida, precio_compra, precio_venta, activo) VALUES (?1, 'Producto Fallido', 'UNIDAD', 1000, 1500, 1)",
                [cat_id],
            )?;

            Err(rusqlite::Error::ExecuteReturnedResults)
        })();

        assert!(tx_result.is_err());

        let count_final: i32 = conn
            .query_row("SELECT COUNT(*) FROM productos", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count_inicial, count_final);
    }

    #[test]
    fn test_restricciones_check_base_datos() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        conn.execute(
            "INSERT INTO categorias (nombre_categoria, activo) VALUES ('Baterías', 1)",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        // Intento de insertar stock negativo debe ser rechazado por el CHECK de SQLite
        let res = conn.execute(
            "INSERT INTO productos (id_categoria, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo)
             VALUES (?1, 'Producto Ilegal', 'UNIDAD', -5.0, 1.0, 1000, 1500, 1)",
            [cat_id],
        );

        assert!(res.is_err());
    }
}

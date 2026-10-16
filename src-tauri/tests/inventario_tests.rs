#[cfg(test)]
mod tests {
    use lubricentro_erp_lib::db::init_db;
    use lubricentro_erp_lib::dto::producto_dto::{
        AjustarStockDto, CrearProductoDto, DespacharProductoDto, ProductoPublicoDto,
    };
    use lubricentro_erp_lib::entities::producto::ProductoEntity;
    use lubricentro_erp_lib::services::inventario_service::InventarioService;
    use tempfile::tempdir;

    const DUMMY_ARGON2_HASH: &str =
        "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

    fn setup_usuario_admin(conn: &rusqlite::Connection) -> i64 {
        conn.execute(
            "INSERT OR IGNORE INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (1, 'Admin Test', 'admin_inv_unique', ?1, 'ADMINISTRADOR', 1)",
            [DUMMY_ARGON2_HASH],
        )
        .unwrap();
        1
    }

    #[test]
    fn test_fuga_precio_compra_seguridad_json() {
        let entity = ProductoEntity {
            id_producto: 1,
            id_categoria: 1,
            nombre: "Aceite Sintetico 5W30".to_string(),
            codigo_barras: Some("7750009999".to_string()),
            unidad_medida: Some("ML".to_string()),
            stock_actual: 20.0,
            stock_minimo: 5.0,
            precio_compra: 1500,
            precio_venta: 2500,
            activo: 1,
        };

        let dto = ProductoPublicoDto::desde_entity(entity, Some("Aceites".to_string()));
        let json_salida = serde_json::to_string(&dto).unwrap();

        assert!(!json_salida.contains("precio_compra"));
        assert!(!json_salida.contains("1500"));
        assert!(!json_salida.contains("margen"));
    }

    #[test]
    fn test_alta_producto_unidad_medida_valida_e_invalida() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req_invalido = CrearProductoDto {
            id_categoria: 1,
            nombre: "Filtro Invalido".to_string(),
            codigo_barras: None,
            unidad_medida: Some("KILO".to_string()),
            stock_actual: 0.0,
            stock_minimo: 2.0,
            precio_compra: 1000,
            precio_venta: 1500,
        };
        let res = InventarioService::registrar_producto(&conn, req_invalido, id_u);
        assert!(res.is_err());

        let req_valido = CrearProductoDto {
            id_categoria: 1,
            nombre: "Aceite Galon".to_string(),
            codigo_barras: None,
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 2.0,
            precio_compra: 4000,
            precio_venta: 6000,
        };
        let res = InventarioService::registrar_producto(&conn, req_valido, id_u);
        assert!(res.is_ok());
    }

    #[test]
    fn test_codigo_barras_duplicado_rechazado() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req1 = CrearProductoDto {
            id_categoria: 1,
            nombre: "Producto 1".to_string(),
            codigo_barras: Some("775111222333".to_string()),
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 1.0,
            precio_compra: 1000,
            precio_venta: 1500,
        };
        InventarioService::registrar_producto(&conn, req1, id_u).unwrap();

        let req2 = CrearProductoDto {
            id_categoria: 1,
            nombre: "Producto 2 Duplicado".to_string(),
            codigo_barras: Some("775111222333".to_string()),
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 1.0,
            precio_compra: 1000,
            precio_venta: 1500,
        };
        let res = InventarioService::registrar_producto(&conn, req2, id_u);
        assert!(res.is_err());
    }

    #[test]
    fn test_promocion_precio_venta_menor_a_precio_compra_permitido() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req = CrearProductoDto {
            id_categoria: 1,
            nombre: "Aceite en Liquidacion".to_string(),
            codigo_barras: None,
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 2.0,
            precio_compra: 1000,
            precio_venta: 1500,
        };

        let res = InventarioService::registrar_producto(&conn, req, id_u);
        assert!(res.is_ok());
    }

    #[test]
    fn test_despachar_producto_granel_exitoso_e_insuficiente() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req = CrearProductoDto {
            id_categoria: 2,
            nombre: "Aceite Granel Barril".to_string(),
            codigo_barras: None,
            unidad_medida: Some("ML".to_string()),
            stock_actual: 50.0,
            stock_minimo: 10.0,
            precio_compra: 50000,
            precio_venta: 80000,
        };
        let id_prod = InventarioService::registrar_producto(&conn, req, id_u).unwrap();

        let desp1 = DespacharProductoDto {
            id_producto: id_prod,
            cantidad: 20.0,
            motivo: Some("Uso en Taller OT-101".to_string()),
        };
        InventarioService::despachar_producto_granel(&conn, desp1, id_u).unwrap();

        let stock_post1: i64 = conn
            .query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                [id_prod],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(stock_post1, 30);

        let desp2 = DespacharProductoDto {
            id_producto: id_prod,
            cantidad: 35.0,
            motivo: Some("Uso excedente".to_string()),
        };
        let res2 = InventarioService::despachar_producto_granel(&conn, desp2, id_u);
        assert!(res2.is_err());
    }

    #[test]
    fn test_ajuste_manual_stock_motivo_obligatorio() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req = CrearProductoDto {
            id_categoria: 1,
            nombre: "Filtro Aceite".to_string(),
            codigo_barras: None,
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 2.0,
            precio_compra: 800,
            precio_venta: 1500,
        };
        let id_prod = InventarioService::registrar_producto(&conn, req, id_u).unwrap();

        let adj_sin_motivo = AjustarStockDto {
            id_producto: id_prod,
            tipo_movimiento: "AJUSTE".to_string(),
            cantidad_nueva: 15.0,
            motivo: "   ".to_string(),
        };
        assert!(InventarioService::ajustar_stock(&conn, adj_sin_motivo, id_u).is_err());

        let adj_con_motivo = AjustarStockDto {
            id_producto: id_prod,
            tipo_movimiento: "AJUSTE".to_string(),
            cantidad_nueva: 15.0,
            motivo: "Conteo fisico de fin de mes".to_string(),
        };
        assert!(InventarioService::ajustar_stock(&conn, adj_con_motivo, id_u).is_ok());
    }

    #[test]
    fn test_eliminar_producto_con_movimientos_rechazado() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req = CrearProductoDto {
            id_categoria: 1,
            nombre: "Producto Sin Historial".to_string(),
            codigo_barras: None,
            unidad_medida: Some("UND".to_string()),
            stock_actual: 0.0,
            stock_minimo: 1.0,
            precio_compra: 1000,
            precio_venta: 1500,
        };
        let id_prod = InventarioService::registrar_producto(&conn, req, id_u).unwrap();

        InventarioService::ajustar_stock(
            &conn,
            AjustarStockDto {
                id_producto: id_prod,
                tipo_movimiento: "AJUSTE".to_string(),
                cantidad_nueva: 10.0,
                motivo: "Ajuste para generar historial".to_string(),
            },
            id_u,
        )
        .unwrap();

        let res = InventarioService::eliminar_producto(&conn, id_prod);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests {
    use lubricentro_erp_lib::db::init_db;
    use lubricentro_erp_lib::dto::ingreso_dto::{DetalleIngresoRequestDto, RegistrarIngresoDto};
    use lubricentro_erp_lib::dto::producto_dto::{
        AjustarStockDto, CrearProductoDto, ProductoPublicoDto,
    };
    use lubricentro_erp_lib::dto::proveedor_dto::CrearProveedorDto;
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
            codigo: "MOBIL-5W30".to_string(),
            nombre: "Aceite Sintetico 5W30".to_string(),
            descripcion: None,
            categoria: "ACEITE".to_string(),
            unidad_medida: "UND".to_string(),
            stock_actual: 20,
            stock_minimo: 5,
            valor_stock_micro: 300_000_000,
            ultimo_costo_micro: Some(15_000_000),
            precio_venta_cent: 2500,
            activo: 1,
        };

        let dto = ProductoPublicoDto::desde_entity(entity);
        let json_salida = serde_json::to_string(&dto).unwrap();

        assert!(!json_salida.contains("valor_stock_micro"));
        assert!(!json_salida.contains("ultimo_costo_micro"));
        assert!(!json_salida.contains("300000000"));
        assert!(!json_salida.contains("15000000"));
        assert_eq!(dto.precio_venta_cent, 2500);
    }

    #[test]
    fn test_alta_producto_categoria_y_unidad_medida_reglas() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req_granel_invalido = CrearProductoDto {
            codigo: "GRANEL-01".to_string(),
            nombre: "Aceite a Granel Invalido".to_string(),
            descripcion: None,
            categoria: "GRANEL".to_string(),
            unidad_medida: "UND".to_string(),
            stock_minimo: 10,
            precio_venta_cent: 2000,
            cantidad_inicial: None,
            costo_unitario_inicial_micro: None,
            referencia_inicial: None,
        };
        assert!(
            InventarioService::registrar_producto(&mut conn, req_granel_invalido, id_u).is_err()
        );

        let req_aceite_invalido = CrearProductoDto {
            codigo: "ACEITE-01".to_string(),
            nombre: "Aceite Botella ML Invalido".to_string(),
            descripcion: None,
            categoria: "ACEITE".to_string(),
            unidad_medida: "ML".to_string(),
            stock_minimo: 2,
            precio_venta_cent: 3500,
            cantidad_inicial: None,
            costo_unitario_inicial_micro: None,
            referencia_inicial: None,
        };
        assert!(
            InventarioService::registrar_producto(&mut conn, req_aceite_invalido, id_u).is_err()
        );

        let req_valido = CrearProductoDto {
            codigo: "FILTRO-M01".to_string(),
            nombre: "Filtro Aceite Shell".to_string(),
            descripcion: Some("Filtro blindado".to_string()),
            categoria: "FILTRO".to_string(),
            unidad_medida: "UND".to_string(),
            stock_minimo: 5,
            precio_venta_cent: 1800,
            cantidad_inicial: None,
            costo_unitario_inicial_micro: None,
            referencia_inicial: None,
        };
        let res = InventarioService::registrar_producto(&mut conn, req_valido, id_u);
        assert!(res.is_ok());
    }

    #[test]
    fn test_alta_producto_con_stock_inicial_atomico() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let req_con_stock = CrearProductoDto {
            codigo: "VALVOLINE-5W30".to_string(),
            nombre: "Aceite Valvoline 5W30 1L".to_string(),
            descripcion: None,
            categoria: "ACEITE".to_string(),
            unidad_medida: "UND".to_string(),
            stock_minimo: 3,
            precio_venta_cent: 4000,
            cantidad_inicial: Some(10),
            costo_unitario_inicial_micro: Some(25_000_000), // S/ 25.00
            referencia_inicial: Some("REF-INIT-001".to_string()),
        };

        let id_prod =
            InventarioService::registrar_producto(&mut conn, req_con_stock, id_u).unwrap();

        let (stock, valor, costo): (i64, i64, Option<i64>) = conn
            .query_row(
                "SELECT stock_actual, valor_stock_micro, ultimo_costo_micro FROM productos WHERE id_producto = ?1",
                [id_prod],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();

        assert_eq!(stock, 10);
        assert_eq!(valor, 250_000_000);
        assert_eq!(costo, Some(25_000_000));

        let tipo_mov: String = conn
            .query_row(
                "SELECT tipo_movimiento FROM movimientos_inventario WHERE id_producto = ?1",
                [id_prod],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(tipo_mov, "STOCK_INICIAL");
    }

    #[test]
    fn test_compra_ingreso_stock_multiples_items() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        // RUC Peruano Válido según algoritmo RUC SUNAT (20100070970)
        let id_prov = InventarioService::crear_proveedor(
            &conn,
            CrearProveedorDto {
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070970".to_string(),
                nombre_razon_social: "Distribuidora de Lubricantes SAC".to_string(),
                telefono: Some("987654321".to_string()),
                correo: None,
                direccion: None,
                contacto_asesor: None,
            },
        )
        .unwrap();

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "CASTROL-10W40".to_string(),
                nombre: "Aceite Castrol 10W40".to_string(),
                descripcion: None,
                categoria: "ACEITE".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 2,
                precio_venta_cent: 3500,
                cantidad_inicial: None,
                costo_unitario_inicial_micro: None,
                referencia_inicial: None,
            },
            id_u,
        )
        .unwrap();

        let req_compra = RegistrarIngresoDto {
            origen: "COMPRA".to_string(),
            id_proveedor: Some(id_prov),
            tipo_documento: Some("FACTURA".to_string()),
            numero_documento: Some("F001-0001234".to_string()),
            fecha_documento: Some("2026-10-28".to_string()),
            referencia_inicial: None,
            observacion: Some("Compra lote mensual".to_string()),
            detalles: vec![DetalleIngresoRequestDto {
                id_producto: id_prod,
                cantidad: 20,
                costo_efectivo_micro: 20_000_000,
            }],
        };

        let id_ingreso =
            InventarioService::registrar_ingreso_compra(&mut conn, req_compra, id_u).unwrap();
        assert!(id_ingreso > 0);

        let (stock, valor): (i64, i64) = conn
            .query_row(
                "SELECT stock_actual, valor_stock_micro FROM productos WHERE id_producto = ?1",
                [id_prod],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();

        assert_eq!(stock, 20);
        assert_eq!(valor, 400_000_000);
    }

    #[test]
    fn test_ajuste_manual_stock_entrada_y_salida() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        // Crear producto sin stock inicial
        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "GRANEL-10W30".to_string(),
                nombre: "Aceite Granel 10W30".to_string(),
                descripcion: None,
                categoria: "GRANEL".to_string(),
                unidad_medida: "ML".to_string(),
                stock_minimo: 1000,
                precio_venta_cent: 10,
                cantidad_inicial: None,
                costo_unitario_inicial_micro: None,
                referencia_inicial: None,
            },
            id_u,
        )
        .unwrap();

        // 1. Ajuste de ENTRADA: 5,000 ML a S/ 0.01 por ML (10,000 microsoles/ML)
        let adj_entrada = AjustarStockDto {
            id_producto: id_prod,
            tipo_ajuste: "ENTRADA".to_string(),
            cantidad: 5000,
            costo_unitario_micro: Some(10_000),
            motivo: "Carga inicial de tambor".to_string(),
        };
        assert!(InventarioService::ajustar_stock(&mut conn, adj_entrada, id_u).is_ok());

        // 2. Ajuste de SALIDA: 1,000 ML
        let adj_salida = AjustarStockDto {
            id_producto: id_prod,
            tipo_ajuste: "SALIDA".to_string(),
            cantidad: 1000,
            costo_unitario_micro: None,
            motivo: "Merma por derrame en surtidor".to_string(),
        };
        assert!(InventarioService::ajustar_stock(&mut conn, adj_salida, id_u).is_ok());

        let stock_post_salida: i64 = conn
            .query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                [id_prod],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(stock_post_salida, 4000);
    }

    #[test]
    fn test_eliminar_producto_con_movimientos_rechazado() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "PROD-HIST".to_string(),
                nombre: "Producto Con Historial".to_string(),
                descripcion: None,
                categoria: "OTRO".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 1,
                precio_venta_cent: 1500,
                cantidad_inicial: Some(5),
                costo_unitario_inicial_micro: Some(10_000_000),
                referencia_inicial: Some("REF-INIT".to_string()),
            },
            id_u,
        )
        .unwrap();

        let res = InventarioService::eliminar_producto(&conn, id_prod);
        assert!(res.is_err());
    }
}

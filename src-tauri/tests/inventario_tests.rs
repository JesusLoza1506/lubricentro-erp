#[cfg(test)]
mod tests {
    use lubricentro_erp_lib::db::init_db;
    use lubricentro_erp_lib::dto::ingreso_dto::{
        AnularIngresoDto, DetalleIngresoRequestDto, RegistrarIngresoDto,
    };
    use lubricentro_erp_lib::dto::producto_dto::{
        AjustarStockDto, ConteoFisicoDto, CrearProductoDto, ProductoPublicoDto,
    };
    use lubricentro_erp_lib::dto::proveedor_dto::{
        ActualizarProveedorDto, ActualizarServicioDto, CrearProveedorDto, CrearServicioDto,
    };
    use lubricentro_erp_lib::entities::producto::ProductoEntity;
    use lubricentro_erp_lib::errors::AppError;
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
    fn test_idempotencia_registro_ingreso_compra() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_prov = InventarioService::crear_proveedor(
            &conn,
            CrearProveedorDto {
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070970".to_string(),
                nombre_razon_social: "Proveedor Idempotente SAC".to_string(),
                telefono: None,
                correo: None,
                direccion: None,
                contacto_asesor: None,
            },
        )
        .unwrap();

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "PROD-IDEMP".to_string(),
                nombre: "Producto Idempotente Test".to_string(),
                descripcion: None,
                categoria: "ACEITE".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 2,
                precio_venta_cent: 1000,
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
            numero_documento: Some("F001-0009999".to_string()),
            fecha_documento: Some("2026-11-01".to_string()),
            referencia_inicial: None,
            observacion: Some("Prueba Idempotencia".to_string()),
            detalles: vec![DetalleIngresoRequestDto {
                id_producto: id_prod,
                cantidad: 5,
                costo_efectivo_micro: 10_000_000,
            }],
        };

        // Primer intento exitoso
        let res1 = InventarioService::registrar_ingreso_compra(&mut conn, req_compra.clone(), id_u);
        assert!(res1.is_ok());

        // Segundo intento duplicado (mismo proveedor, tipo y número de documento) debe fallar por idempotencia
        let res2 = InventarioService::registrar_ingreso_compra(&mut conn, req_compra, id_u);
        assert!(res2.is_err());
        assert!(res2
            .unwrap_err()
            .to_string()
            .contains("Ya existe un ingreso registrado"));
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
            cantidad_inicial: Some(10.0),
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
    fn test_compra_ingreso_stock_multiples_items_y_sin_documento() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

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

        let req_compra_sd = RegistrarIngresoDto {
            origen: "COMPRA".to_string(),
            id_proveedor: Some(id_prov),
            tipo_documento: Some("SIN_DOCUMENTO".to_string()),
            numero_documento: None,
            fecha_documento: None,
            referencia_inicial: None,
            observacion: Some("Compra interna directa".to_string()),
            detalles: vec![DetalleIngresoRequestDto {
                id_producto: id_prod,
                cantidad: 20,
                costo_efectivo_micro: 20_000_000,
            }],
        };

        let id_ingreso =
            InventarioService::registrar_ingreso_compra(&mut conn, req_compra_sd, id_u).unwrap();
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
    fn test_anulacion_ingreso_compra_exitoso_y_bloqueado_por_trigger_movimientos_posteriores() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_prov = InventarioService::crear_proveedor(
            &conn,
            CrearProveedorDto {
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070970".to_string(),
                nombre_razon_social: "Distribuidora Peruana SRL".to_string(),
                telefono: None,
                correo: None,
                direccion: None,
                contacto_asesor: None,
            },
        )
        .unwrap();

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "TOTAL-5W30".to_string(),
                nombre: "Aceite Total Quartz 5W30".to_string(),
                descripcion: None,
                categoria: "ACEITE".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 5,
                precio_venta_cent: 3000,
                cantidad_inicial: None,
                costo_unitario_inicial_micro: None,
                referencia_inicial: None,
            },
            id_u,
        )
        .unwrap();

        let id_ingreso = InventarioService::registrar_ingreso_compra(
            &mut conn,
            RegistrarIngresoDto {
                origen: "COMPRA".to_string(),
                id_proveedor: Some(id_prov),
                tipo_documento: Some("FACTURA".to_string()),
                numero_documento: Some("F001-0001234".to_string()),
                fecha_documento: Some("2026-10-25".to_string()),
                referencia_inicial: None,
                observacion: Some("Compra lote inicial".to_string()),
                detalles: vec![DetalleIngresoRequestDto {
                    id_producto: id_prod,
                    cantidad: 10,
                    costo_efectivo_micro: 18_000_000,
                }],
            },
            id_u,
        )
        .unwrap();

        let adj_salida = AjustarStockDto {
            id_producto: id_prod,
            tipo_ajuste: "SALIDA".to_string(),
            cantidad: 1,
            costo_unitario_micro: None,
            motivo: "Despacho manual de muestra".to_string(),
        };
        InventarioService::ajustar_stock(&mut conn, adj_salida, id_u).unwrap();

        let res_anulacion_fallida = InventarioService::anular_ingreso_compra(
            &mut conn,
            AnularIngresoDto {
                id_ingreso,
                motivo: "Anulación por error en la factura ingresada".to_string(),
            },
            id_u,
        );

        assert!(
            res_anulacion_fallida.is_err(),
            "Se esperaba que la anulación fallara debido a que el stock disminuyó / movimientos posteriores."
        );

        let err = res_anulacion_fallida.unwrap_err();
        let msg_err = match err {
            AppError::Validation(m) => m,
            AppError::Database(e) => e.to_string(),
            other => panic!("Tipo de error no esperado: {:?}", other),
        };

        let msg_lower = msg_err.to_lowercase();
        assert!(
            msg_lower.contains("disponible")
                || msg_lower.contains("posteriores")
                || msg_lower.contains("no anulable"),
            "El mensaje de error retornado fue: '{}'",
            msg_err
        );
    }

    #[test]
    fn test_concurrencia_optimista_conteo_fisico_stock_observado() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "FILT-CONCURRENTE".to_string(),
                nombre: "Filtro Aire Concurrente".to_string(),
                descripcion: None,
                categoria: "FILTRO".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 2,
                precio_venta_cent: 1500,
                cantidad_inicial: Some(10.0),
                costo_unitario_inicial_micro: Some(8_000_000),
                referencia_inicial: Some("REF-01".to_string()),
            },
            id_u,
        )
        .unwrap();

        InventarioService::ajustar_stock(
            &mut conn,
            AjustarStockDto {
                id_producto: id_prod,
                tipo_ajuste: "SALIDA".to_string(),
                cantidad: 2,
                costo_unitario_micro: None,
                motivo: "Ajuste intermedio por otro usuario".to_string(),
            },
            id_u,
        )
        .unwrap();

        let res_conteo_concurrente = InventarioService::procesar_conteo_fisico(
            &mut conn,
            ConteoFisicoDto {
                id_producto: id_prod,
                stock_fisico_contado: 9.0,
                stock_observado: Some(10.0),
                costo_unitario_micro: None,
                motivo: "Conteo de inventario nocturno".to_string(),
            },
            id_u,
        );

        assert!(res_conteo_concurrente.is_err());
        let err = res_conteo_concurrente.unwrap_err().to_string();
        assert!(err.contains("modificado por otro usuario"));
    }

    #[test]
    fn test_kardex_resumen_saldos_y_arrastre_historico() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_prod = InventarioService::registrar_producto(
            &mut conn,
            CrearProductoDto {
                codigo: "PROD-KARDEX-HIST".to_string(),
                nombre: "Producto Test Arrastre Histórico".to_string(),
                descripcion: None,
                categoria: "ACEITE".to_string(),
                unidad_medida: "UND".to_string(),
                stock_minimo: 1,
                precio_venta_cent: 5000,
                cantidad_inicial: Some(15.0),
                costo_unitario_inicial_micro: Some(30_000_000),
                referencia_inicial: Some("INIT-01".to_string()),
            },
            id_u,
        )
        .unwrap();

        let resumen_futuro = InventarioService::obtener_resumen_saldos_kardex(
            &conn,
            id_prod,
            Some("2030-01-01".to_string()),
            Some("2030-01-10".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(resumen_futuro.saldo_inicial, 15);
        assert_eq!(resumen_futuro.saldo_final, 15);

        let res_exceso_dias = InventarioService::listar_kardex_producto(
            &conn,
            id_prod,
            20,
            0,
            Some("2025-01-01".to_string()),
            Some("2026-02-01".to_string()),
            None,
        );
        assert!(res_exceso_dias.is_err());
        assert!(res_exceso_dias
            .unwrap_err()
            .to_string()
            .contains("366 días"));
    }

    #[test]
    fn test_crud_proveedores_y_validacion_ruc() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        let res_ruc_malo = InventarioService::crear_proveedor(
            &conn,
            CrearProveedorDto {
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070979".to_string(),
                nombre_razon_social: "Proveedor Invalido".to_string(),
                telefono: None,
                correo: None,
                direccion: None,
                contacto_asesor: None,
            },
        );
        assert!(res_ruc_malo.is_err());

        let id_prov = InventarioService::crear_proveedor(
            &conn,
            CrearProveedorDto {
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070970".to_string(),
                nombre_razon_social: "Proveedor Valido SAC".to_string(),
                telefono: Some("999888777".to_string()),
                correo: None,
                direccion: None,
                contacto_asesor: None,
            },
        )
        .unwrap();

        let update_res = InventarioService::actualizar_proveedor(
            &conn,
            ActualizarProveedorDto {
                id_proveedor: id_prov,
                tipo_documento: "RUC".to_string(),
                numero_documento: "20100070970".to_string(),
                nombre_razon_social: "Proveedor Valido SAC Editado".to_string(),
                telefono: Some("999888777".to_string()),
                correo: Some("contacto@proveedor.com".to_string()),
                direccion: Some("Av. Industrial 123".to_string()),
                contacto_asesor: Some("Juan Pérez".to_string()),
            },
        );
        assert!(update_res.is_ok());
    }

    #[test]
    fn test_crud_servicios_mantenimiento() {
        let dir = tempdir().unwrap();
        let mut conn = init_db(dir.path().to_path_buf()).unwrap();
        let id_u = setup_usuario_admin(&conn);

        let id_serv = InventarioService::crear_servicio(
            &mut conn,
            CrearServicioDto {
                codigo: "SERV-ACEITE-01".to_string(),
                nombre: "Cambio de Aceite Sintético Express".to_string(),
                descripcion: Some("Incluye revisión de 10 puntos".to_string()),
                precio_venta_cent: 3500,
                es_cambio_aceite: true,
            },
            id_u,
        )
        .unwrap();

        InventarioService::actualizar_servicio(
            &mut conn,
            ActualizarServicioDto {
                id_servicio: id_serv,
                nombre: "Cambio de Aceite Premium".to_string(),
                descripcion: Some("Incluye alineación de luces".to_string()),
                precio_venta_cent: 4000,
                es_cambio_aceite: true,
            },
            id_u,
        )
        .unwrap();

        assert!(
            InventarioService::cambiar_estado_servicio(&mut conn, id_serv, false, id_u).is_ok()
        );
        assert!(InventarioService::cambiar_estado_servicio(&mut conn, id_serv, true, id_u).is_ok());
    }
}

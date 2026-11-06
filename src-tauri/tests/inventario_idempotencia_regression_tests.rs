use lubricentro_erp_lib::{
    db::init_db,
    dto::{
        ingreso_dto::{AnularIngresoDto, DetalleIngresoRequestDto, RegistrarIngresoDto},
        producto_dto::AjustarStockDto,
    },
    services::inventario_service::InventarioService,
};

fn preparar() -> (tempfile::TempDir, rusqlite::Connection) {
    let dir = tempfile::tempdir().unwrap();
    let c = init_db(dir.path().to_path_buf()).unwrap();
    c.execute_batch("INSERT INTO usuarios (id_usuario,nombre_completo,username,password_hash,rol) VALUES (1,'Admin Prueba','admin_prueba','$argon2id$prueba','ADMINISTRADOR');
    INSERT INTO productos (id_producto,codigo,nombre,categoria,unidad_medida,stock_minimo,precio_venta_cent) VALUES (1,'FILTRO-01','Filtro','FILTRO','UND',1,1500);
    INSERT INTO proveedores (id_proveedor,tipo_documento,numero_documento,nombre_razon_social) VALUES (1,'RUC','20100070970','Proveedor Prueba SAC');").unwrap();
    (dir, c)
}

#[test]
fn ajuste_reintento_responde_ok_sin_duplicar_stock_o_auditoria() {
    let (_dir, mut c) = preparar();
    let mut req = AjustarStockDto {
        id_producto: 1,
        tipo_ajuste: "ENTRADA".into(),
        cantidad: 10,
        costo_unitario_micro: Some(5_000_000),
        motivo: "Ingreso verificado".into(),
    };
    InventarioService::ajustar_stock(&mut c, req.clone(), None, 1).unwrap();
    req.cantidad = 5;
    let id = Some("operacion-ajuste-0001");
    let primera = InventarioService::ajustar_stock(&mut c, req.clone(), id, 1).unwrap();
    let auditoria: i64 = c
        .query_row("SELECT COUNT(*) FROM auditoria", [], |r| r.get(0))
        .unwrap();
    let movimientos: i64 = c
        .query_row("SELECT COUNT(*) FROM movimientos_inventario", [], |r| {
            r.get(0)
        })
        .unwrap();
    let repetida = InventarioService::ajustar_stock(&mut c, req, id, 1).unwrap();
    assert_eq!(primera, repetida);
    assert_eq!(repetida["status"], "OK");
    assert_eq!(repetida["stock_resultante"], 15);
    let stock: i64 = c
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto=1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock, 15);
    assert_eq!(
        auditoria,
        c.query_row("SELECT COUNT(*) FROM auditoria", [], |r| r.get::<_, i64>(0))
            .unwrap()
    );
    assert_eq!(
        movimientos,
        c.query_row("SELECT COUNT(*) FROM movimientos_inventario", [], |r| r
            .get::<_, i64>(0))
            .unwrap()
    );
}

#[test]
fn compra_y_anulacion_reintentan_sin_duplicar_efectos() {
    let (_dir, mut c) = preparar();
    let req = RegistrarIngresoDto {
        origen: "COMPRA".into(),
        id_proveedor: Some(1),
        tipo_documento: Some("FACTURA".into()),
        numero_documento: Some("F001-0001234".into()),
        fecha_documento: Some("2026-11-01".into()),
        referencia_inicial: None,
        observacion: None,
        detalles: vec![DetalleIngresoRequestDto {
            id_producto: 1,
            cantidad: 5,
            costo_efectivo_micro: 10_000_000,
        }],
    };
    let id = Some("operacion-compra-0001");
    let primera = InventarioService::registrar_ingreso_compra(&mut c, req.clone(), id, 1).unwrap();
    assert_eq!(
        primera,
        InventarioService::registrar_ingreso_compra(&mut c, req, id, 1).unwrap()
    );
    let req = AnularIngresoDto {
        id_ingreso: primera["id_ingreso"].as_i64().unwrap(),
        motivo: "Documento registrado por error".into(),
    };
    let id = Some("operacion-anular-0001");
    let primera = InventarioService::anular_ingreso_compra(&mut c, req.clone(), id, 1).unwrap();
    assert_eq!(
        primera,
        InventarioService::anular_ingreso_compra(&mut c, req, id, 1).unwrap()
    );
    let stock: i64 = c
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto=1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock, 0);
    let n: i64 = c
        .query_row("SELECT COUNT(*) FROM movimientos_inventario", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(n, 2);
}

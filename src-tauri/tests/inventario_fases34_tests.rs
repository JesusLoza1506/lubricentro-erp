use lubricentro_erp_lib::{
    db::init_db, dto::producto_dto::CrearProductoDto,
    services::inventario_service::InventarioService,
};
use rusqlite::Connection;

fn preparar() -> (tempfile::TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let c = init_db(dir.path().to_path_buf()).unwrap();
    c.execute("INSERT INTO usuarios (id_usuario,nombre_completo,username,password_hash,rol) VALUES (1,'Admin Prueba','admin_prueba','$argon2id$prueba','ADMINISTRADOR')", []).unwrap();
    (dir, c)
}
fn producto(c: &mut Connection, codigo: &str, cantidad: Option<f64>) -> i64 {
    InventarioService::registrar_producto(
        c,
        CrearProductoDto {
            codigo: codigo.into(),
            nombre: "Filtro de aceite".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 5,
            precio_venta_cent: 2000,
            cantidad_inicial: cantidad,
            costo_unitario_inicial_micro: cantidad.map(|_| 1_230_000),
            referencia_inicial: cantidad.map(|_| format!("REF-{codigo}")),
        },
        1,
    )
    .unwrap()
}
#[test]
fn pagina_publica_serializa_solo_datos_publicos() {
    let (_dir, mut c) = preparar();
    producto(&mut c, "PUBLICO-01", Some(3.0));
    let p = InventarioService::listar_productos_publicos_paginado(&c, None, None, 20, 0).unwrap();
    let json = serde_json::to_value(p).unwrap();
    assert_eq!(json["total_registros"], 1);
    let item = &json["productos"][0];
    assert_eq!(item["precio_venta_cent"], 2000);
    assert!(item.get("valor_stock_micro").is_none());
    assert!(item.get("ultimo_costo_micro").is_none());
    let admin =
        InventarioService::listar_productos_admin_paginado(&c, None, None, None, 20, 0).unwrap();
    assert_eq!(admin.productos[0].ultimo_costo_micro, Some(1_230_000));
}
#[test]
fn criticos_paginados_conteo_busqueda_y_orden_estable() {
    let (_dir, mut c) = preparar();
    for i in 0..45 {
        producto(&mut c, &format!("CRIT-{i:02}"), None);
    }
    producto(&mut c, "NORMAL-01", Some(20.0));
    let mut ids = Vec::new();
    for offset in [0, 20, 40] {
        let p = InventarioService::listar_productos_publicos_paginado_filtrado(
            &c,
            Some("Filtro".into()),
            None,
            20,
            offset,
            true,
        )
        .unwrap();
        assert_eq!(p.total_registros, 45);
        assert_eq!(p.productos.len(), if offset == 40 { 5 } else { 20 });
        ids.extend(p.productos.iter().map(|p| p.id_producto));
    }
    let mut unicos = ids.clone();
    unicos.sort_unstable();
    unicos.dedup();
    assert_eq!(unicos.len(), 45);
    assert!(ids.windows(2).all(|w| w[0] < w[1]));
    let p = InventarioService::listar_productos_publicos_paginado_filtrado(
        &c,
        Some("NO-EXISTE".into()),
        None,
        20,
        0,
        true,
    )
    .unwrap();
    assert_eq!(p.total_registros, 0);
    assert!(p.productos.is_empty());
    InventarioService::eliminar_producto(&mut c, ids[0], "Producto discontinuado", 1).unwrap();
    let admin = InventarioService::listar_productos_admin_paginado_filtrado(
        &c,
        None,
        None,
        Some(true),
        20,
        0,
        true,
    )
    .unwrap();
    assert_eq!(admin.total_registros, 44);
    assert!(admin.productos.iter().all(|p| p.activo));
    assert_eq!(
        InventarioService::listar_productos_admin_paginado(&c, None, None, Some(true), 20, 0)
            .unwrap()
            .total_registros,
        46
    );
    assert!(InventarioService::listar_productos_publicos_paginado(&c, None, None, 0, 0).is_err());
    assert!(InventarioService::listar_productos_publicos_paginado(&c, None, None, 20, -1).is_err());
}
#[test]
fn motivo_producto_es_obligatorio_y_respeta_caracteres_unicode() {
    let (_dir, mut c) = preparar();
    let id = producto(&mut c, "MOTIVO-01", None);
    for motivo in ["", "    ", "abcd"] {
        assert!(InventarioService::eliminar_producto(&mut c, id, motivo, 1).is_err());
    }
    assert!(InventarioService::eliminar_producto(&mut c, id, &"x".repeat(251), 1).is_err());
    InventarioService::eliminar_producto(&mut c, id, &"á".repeat(250), 1).unwrap();
    assert!(InventarioService::reactivar_producto(&mut c, id, "no", 1).is_err());
    InventarioService::reactivar_producto(&mut c, id, "Nueva disponibilidad confirmada", 1)
        .unwrap();
}
#[test]
fn kardex_conserva_valores_y_origen_ingreso_sin_escrituras() {
    let (_dir, mut c) = preparar();
    let id = producto(&mut c, "ORIGEN-01", Some(3.0));
    let cambios = c
        .query_row("SELECT total_changes()", [], |r| r.get::<_, i64>(0))
        .unwrap();
    let movimientos =
        InventarioService::listar_kardex_producto(&c, id, 20, 0, None, None, None).unwrap();
    assert_eq!(movimientos.len(), 1);
    let m = &movimientos[0];
    assert_eq!(m.costo_unitario_micro, 1_230_000);
    assert_eq!(m.valor_delta_micro, 3_690_000);
    assert!(m.id_ingreso.is_some());
    assert!(m.id_ot.is_none());
    assert!(m.codigo_origen.is_some());
    let origen = InventarioService::obtener_origen_kardex(&c, m.id_movimiento).unwrap();
    assert_eq!(origen.tipo, "INGRESO");
    assert_eq!(origen.lineas[0].valor_micro, Some(3_690_000));
    assert_eq!(origen.lineas[0].cantidad, 3);
    assert_eq!(
        c.query_row("SELECT total_changes()", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        cambios
    );
    assert!(InventarioService::obtener_origen_kardex(&c, 999999).is_err());
}
#[test]
fn kardex_valida_rango_y_saldos_independientes_del_filtro_de_tipo() {
    let (_dir, mut c) = preparar();
    let id = producto(&mut c, "SALDOS-01", Some(3.0));
    let normal =
        InventarioService::obtener_resumen_saldos_kardex(&c, id, None, None, None).unwrap();
    let filtrado = InventarioService::obtener_resumen_saldos_kardex(
        &c,
        id,
        None,
        None,
        Some("VENTA_DIRECTA".into()),
    )
    .unwrap();
    assert_eq!(normal.saldo_inicial, filtrado.saldo_inicial);
    assert_eq!(normal.saldo_final, filtrado.saldo_final);
    assert_eq!(filtrado.saldo_final, 3);
    for (a, b) in [
        ("2026-02-30", "2026-03-01"),
        ("2026-03-02", "2026-03-01"),
        ("2020-01-01", "2026-01-01"),
    ] {
        assert!(InventarioService::listar_kardex_producto(
            &c,
            id,
            20,
            0,
            Some(a.into()),
            Some(b.into()),
            None
        )
        .is_err());
        assert!(InventarioService::obtener_resumen_saldos_kardex(
            &c,
            id,
            Some(a.into()),
            Some(b.into()),
            None
        )
        .is_err());
    }
    let antes = InventarioService::obtener_resumen_saldos_kardex(
        &c,
        id,
        None,
        Some("2000-01-01".into()),
        None,
    )
    .unwrap();
    assert_eq!(antes.saldo_inicial, 0);
    assert_eq!(antes.saldo_final, 0);
}

#[test]
fn kardex_resuelve_orden_real_del_consumo() {
    use lubricentro_erp_lib::dto::orden_trabajo_dto::{
        CrearOrdenTrabajoDto, IniciarTrabajoOtDto, RegistrarInsumoOtDto, ServicioInicialDto,
    };
    use lubricentro_erp_lib::services::ordenes_service::OrdenesService;
    let (_dir, mut c) = preparar();
    let id = producto(&mut c, "CONSUMO-01", Some(3.0));
    c.execute_batch("INSERT INTO usuarios (id_usuario,nombre_completo,username,password_hash,rol) VALUES (2,'Mecanico Prueba','mecanico_prueba','$argon2id$prueba','MECANICO');
      INSERT INTO clientes (id_cliente,tipo_documento,numero_documento,nombre_razon_social) VALUES (2,'DNI','12345678','Cliente Prueba');
      INSERT INTO vehiculos (id_vehiculo,placa,id_cliente,marca,modelo,anio,kilometraje_actual,activo) VALUES (1,'ABC123',2,'Toyota','Yaris',2020,1000,1);
      INSERT INTO servicios (id_servicio,codigo,nombre,precio_venta_cent,es_cambio_aceite,activo) VALUES (1,'SERV-01','Cambio de filtro',1500,0,1);").unwrap();
    let ot = OrdenesService::crear_orden_trabajo(
        &mut c,
        CrearOrdenTrabajoDto {
            id_vehiculo: 1,
            id_cliente_atendido: 2,
            kilometraje_ingreso: 1000,
            id_mecanico: Some(2),
            pedido_cliente: Some("Cambiar filtro".into()),
            servicios_iniciales: vec![ServicioInicialDto {
                id_servicio: 1,
                cantidad: 1,
            }],
        },
        1,
        "ADMINISTRADOR",
    )
    .unwrap();
    OrdenesService::iniciar_trabajo_ot(
        &mut c,
        IniciarTrabajoOtDto {
            id_ot: ot.id_ot,
            id_zanja_ejecucion: 1,
            id_mecanico: None,
        },
        2,
        "MECANICO",
    )
    .unwrap();
    OrdenesService::registrar_insumo_ot(
        &mut c,
        RegistrarInsumoOtDto {
            id_ot: ot.id_ot,
            id_producto: id,
            cantidad: 1.0,
            id_operacion: Some("consumo-prueba-origen-0001".into()),
        },
        2,
        "MECANICO",
    )
    .unwrap();
    let movimientos = InventarioService::listar_kardex_producto(
        &c,
        id,
        20,
        0,
        None,
        None,
        Some("CONSUMO_OT".into()),
    )
    .unwrap();
    assert_eq!(movimientos.len(), 1);
    let m = &movimientos[0];
    assert_eq!(m.id_ot, Some(ot.id_ot));
    assert_eq!(m.codigo_origen.as_deref(), Some(ot.codigo_ot.as_str()));
    let origen = InventarioService::obtener_origen_kardex(&c, m.id_movimiento).unwrap();
    assert_eq!(origen.tipo, "OT");
    assert_eq!(origen.documento.as_deref(), Some("ABC123"));
    assert_eq!(origen.lineas.len(), 1);
    assert_eq!(origen.lineas[0].cantidad, 1);
}

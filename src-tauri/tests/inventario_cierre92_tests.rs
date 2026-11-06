use lubricentro_erp_lib::{
    db::init_db,
    dto::producto_dto::{AjustarStockDto, ConteoFisicoDto, CrearProductoDto},
    services::inventario_service::InventarioService,
};
use rusqlite::Connection;
fn preparar() -> (tempfile::TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let c = init_db(dir.path().to_path_buf()).unwrap();
    c.execute("INSERT INTO usuarios (id_usuario,nombre_completo,username,password_hash,rol) VALUES (1,'Administrador prueba','admin_prueba','$argon2id$prueba','ADMINISTRADOR')", []).unwrap();
    (dir, c)
}
fn producto(c: &mut Connection, ml: bool, cantidad: f64, costo: i64) -> i64 {
    InventarioService::registrar_producto(
        c,
        CrearProductoDto {
            codigo: if ml { "GRA-01" } else { "UND-01" }.into(),
            nombre: "Producto de prueba".into(),
            descripcion: None,
            categoria: if ml { "GRANEL" } else { "FILTRO" }.into(),
            unidad_medida: if ml { "ML" } else { "UND" }.into(),
            stock_minimo: 0,
            precio_venta_cent: 2000,
            cantidad_inicial: Some(cantidad),
            costo_unitario_inicial_micro: Some(costo),
            referencia_inicial: Some("Referencia prueba".into()),
        },
        1,
    )
    .unwrap()
}
fn stock(c: &Connection, id: i64) -> (i64, i64) {
    c.query_row(
        "SELECT stock_actual,valor_stock_micro FROM productos WHERE id_producto=?1",
        [id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .unwrap()
}
fn contar(
    c: &mut Connection,
    id: i64,
    fisico: f64,
    observado: f64,
    motivo: &str,
    op: &str,
) -> Result<serde_json::Value, lubricentro_erp_lib::errors::AppError> {
    InventarioService::procesar_conteo_fisico(
        c,
        ConteoFisicoDto {
            id_producto: id,
            stock_fisico_contado: fisico,
            stock_observado: Some(observado),
            costo_unitario_micro: None,
            motivo: motivo.into(),
        },
        Some(op),
        1,
    )
}
#[test]
fn conteo_reintenta_original_antes_de_comprobar_stock_observado() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 10.0, 1230000);
    let primero = contar(
        &mut c,
        id,
        9.0,
        10.0,
        "Conteo mensual",
        "conteo-reintento-01",
    )
    .unwrap();
    let movimientos: i64 = c
        .query_row("SELECT count(*) FROM movimientos_inventario", [], |r| {
            r.get(0)
        })
        .unwrap();
    let segundo = contar(
        &mut c,
        id,
        9.0,
        10.0,
        "Conteo mensual",
        "conteo-reintento-01",
    )
    .unwrap();
    assert_eq!(primero, segundo);
    assert_eq!(stock(&c, id), (9, 11070000));
    assert_eq!(
        movimientos,
        c.query_row("SELECT count(*) FROM movimientos_inventario", [], |r| r
            .get::<_, i64>(0))
            .unwrap()
    );
    assert!(contar(
        &mut c,
        id,
        8.0,
        10.0,
        "Conteo mensual",
        "conteo-reintento-01"
    )
    .is_err());
}
#[test]
fn conteo_sin_diferencia_no_exige_motivo_ni_crea_movimiento() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 10.0, 1230000);
    let antes: i64 = c
        .query_row("SELECT count(*) FROM auditoria", [], |r| r.get(0))
        .unwrap();
    let r = contar(&mut c, id, 10.0, 10.0, "", "conteo-sin-cambio-01").unwrap();
    assert_eq!(r["status"], "OK");
    assert_eq!(
        1,
        c.query_row("SELECT count(*) FROM movimientos_inventario", [], |r| r
            .get::<_, i64>(0))
            .unwrap()
    );
    assert_eq!(
        antes,
        c.query_row("SELECT count(*) FROM auditoria", [], |r| r.get::<_, i64>(0))
            .unwrap()
    );
    contar(&mut c, id, 10.0, 10.0, "", "conteo-sin-cambio-01").unwrap();
}
#[test]
fn costo_inicial_con_mas_de_dos_decimales_es_rechazado_y_revierte_alta() {
    let (_d, mut c) = preparar();
    let r = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "COSTO-01".into(),
            nombre: "Filtro de prueba".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 0,
            precio_venta_cent: 1000,
            cantidad_inicial: Some(10.0),
            costo_unitario_inicial_micro: Some(1234567),
            referencia_inicial: Some("Inicio".into()),
        },
        1,
    );
    assert!(r.is_err());
    assert_eq!(
        0,
        c.query_row("SELECT count(*) FROM productos", [], |r| r.get::<_, i64>(0))
            .unwrap()
    );
    assert_eq!(
        0,
        c.query_row("SELECT count(*) FROM auditoria", [], |r| r.get::<_, i64>(0))
            .unwrap()
    );
}
#[test]
fn conteo_granel_conserva_promedio_en_sobrante_faltante_y_agotamiento() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, true, 10.0, 1230000);
    contar(
        &mut c,
        id,
        11.0,
        10.0,
        "Conteo mensual",
        "conteo-granel-entrada",
    )
    .unwrap();
    assert_eq!(stock(&c, id), (11000, 13530000));
    contar(
        &mut c,
        id,
        9.0,
        11.0,
        "Conteo mensual",
        "conteo-granel-salida",
    )
    .unwrap();
    assert_eq!(stock(&c, id), (9000, 11070000));
    contar(
        &mut c,
        id,
        0.0,
        9.0,
        "Conteo mensual",
        "conteo-granel-agota",
    )
    .unwrap();
    assert_eq!(stock(&c, id), (0, 0));
    contar(
        &mut c,
        id,
        1.0,
        0.0,
        "Conteo mensual",
        "conteo-granel-repone",
    )
    .unwrap();
    assert_eq!(stock(&c, id), (1000, 1230000));
}
#[test]
fn conteo_fallo_de_auditoria_revierte_stock_movimiento_y_solicitud() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 10.0, 1230000);
    c.execute_batch("CREATE TRIGGER fallo_auditoria BEFORE INSERT ON auditoria BEGIN SELECT RAISE(ABORT,'Fallo forzado'); END;").unwrap();
    assert!(contar(
        &mut c,
        id,
        9.0,
        10.0,
        "Conteo mensual",
        "conteo-reversion-01"
    )
    .is_err());
    assert_eq!(stock(&c, id), (10, 12300000));
    assert_eq!(
        0,
        c.query_row("SELECT count(*) FROM solicitudes_operacion", [], |r| r
            .get::<_, i64>(0))
            .unwrap()
    );
    c.execute_batch("DROP TRIGGER fallo_auditoria;").unwrap();
    contar(
        &mut c,
        id,
        9.0,
        10.0,
        "Conteo mensual",
        "conteo-reversion-01",
    )
    .unwrap();
}
#[test]
fn conteo_rechaza_observacion_obsoleta_y_precision_invalida() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 10.0, 1230000);
    for (fisico, obs) in [(9.0, 8.0), (9.1, 10.0), (f64::NAN, 10.0), (9.0, 10.1)] {
        assert!(contar(
            &mut c,
            id,
            fisico,
            obs,
            "Conteo mensual",
            "conteo-invalido-01"
        )
        .is_err());
    }
    assert_eq!(stock(&c, id), (10, 12300000));
}
#[test]
fn ajuste_salida_granel_no_divide_dos_veces_por_mil() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, true, 10.0, 1230000);
    InventarioService::ajustar_stock(
        &mut c,
        AjustarStockDto {
            id_producto: id,
            tipo_ajuste: "SALIDA".into(),
            cantidad: 1000,
            costo_unitario_micro: None,
            motivo: "Ajuste autorizado".into(),
        },
        Some("ajuste-granel-0001"),
        1,
    )
    .unwrap();
    assert_eq!(stock(&c, id), (9000, 11070000));
}
#[test]
fn filtros_combinados_y_codigos_consideran_inactivos() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 10.0, 1230000);
    let p = InventarioService::listar_productos_admin_consulta(
        &c,
        None,
        Some("FILTRO".into()),
        Some(true),
        20,
        0,
        false,
        Some("DISPONIBLE".into()),
        Some("ACTIVO".into()),
    )
    .unwrap();
    assert_eq!(p.total_registros, 1);
    InventarioService::eliminar_producto(&mut c, id, "Producto descontinuado", 1).unwrap();
    let p = InventarioService::listar_productos_admin_consulta(
        &c,
        None,
        None,
        None,
        20,
        0,
        false,
        None,
        Some("INACTIVO".into()),
    )
    .unwrap();
    assert_eq!(p.total_registros, 1);
    assert!(InventarioService::listar_productos_publicos_consulta(
        &c,
        None,
        None,
        20,
        0,
        false,
        None,
        Some("INACTIVO".into())
    )
    .is_err());
    assert_eq!(
        InventarioService::proponer_codigo(&c, "PRODUCTO").unwrap(),
        "PROD-000002"
    );
}
#[test]
fn sobrante_con_stock_usa_promedio_interno_sin_recortarlo_a_centimos() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 1.0, 1230000);
    InventarioService::ajustar_stock(
        &mut c,
        AjustarStockDto {
            id_producto: id,
            tipo_ajuste: "ENTRADA".into(),
            cantidad: 1,
            costo_unitario_micro: Some(1240000),
            motivo: "Ingreso autorizado".into(),
        },
        None,
        1,
    )
    .unwrap();
    contar(&mut c, id, 3.0, 2.0, "Conteo mensual", "conteo-promedio-01").unwrap();
    assert_eq!(stock(&c, id), (3, 3705000));
}
#[test]
fn criticos_priorizan_stock_cero_y_ordenan_por_nombre_codigo() {
    let (_d, mut c) = preparar();
    let id = producto(&mut c, false, 2.0, 1230000);
    c.execute(
        "UPDATE productos SET stock_minimo=5 WHERE id_producto=?1",
        [id],
    )
    .unwrap();
    c.execute("INSERT INTO productos(codigo,nombre,categoria,unidad_medida,stock_minimo,precio_venta_cent) VALUES('VACIO-01','Zeta sin stock','FILTRO','UND',0,1500)",[]).unwrap();
    let p = InventarioService::listar_productos_publicos_consulta(
        &c,
        None,
        None,
        20,
        0,
        false,
        Some("CRITICO".into()),
        None,
    )
    .unwrap();
    assert_eq!(p.total_registros, 2);
    assert_eq!(p.productos[0].codigo, "VACIO-01");
    assert_eq!(p.productos[1].id_producto, id);
}
#[test]
fn entradas_validan_fechas_y_rango_de_366_dias() {
    let (_d, c) = preparar();
    for (a, b) in [
        (Some("2026-10-01"), None),
        (Some("2026-11-01"), Some("2026-10-01")),
        (Some("2020-01-01"), Some("2026-11-01")),
        (Some("2026-99-99"), Some("2026-11-01")),
    ] {
        assert!(InventarioService::listar_ingresos_stock_filtrado(
            &c,
            20,
            0,
            None,
            None,
            a.map(str::to_string),
            b.map(str::to_string)
        )
        .is_err());
    }
    assert!(InventarioService::listar_ingresos_stock_filtrado(
        &c,
        20,
        0,
        None,
        None,
        Some("2026-10-01".into()),
        Some("2026-10-31".into())
    )
    .unwrap()
    .is_empty());
}
#[test]
fn referencia_inicial_se_genera_sin_pedir_captura_manual() {
    let (_d, mut c) = preparar();
    let id = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "REF-AUTO".into(),
            nombre: "Filtro automático".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 0,
            precio_venta_cent: 1500,
            cantidad_inicial: Some(1.0),
            costo_unitario_inicial_micro: Some(1000000),
            referencia_inicial: None,
        },
        1,
    )
    .unwrap();
    assert_eq!(stock(&c, id), (1, 1000000));
    let referencia: String = c
        .query_row("SELECT referencia_inicial FROM ingresos_stock", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(referencia.starts_with("INICIAL-"));
}

#[test]
fn valores_fuera_del_techo_se_rechazan_sin_desbordar_ni_dejar_altas() {
    let (_d, mut c) = preparar();
    let r = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "LIMITE-01".into(),
            nombre: "Producto de valor extremo".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 0,
            precio_venta_cent: 1500,
            cantidad_inicial: Some(9999999.0),
            costo_unitario_inicial_micro: Some(999999990000),
            referencia_inicial: None,
        },
        1,
    );
    assert!(r.is_err());
    assert_eq!(
        0,
        c.query_row("SELECT count(*) FROM productos", [], |r| r.get::<_, i64>(0))
            .unwrap()
    );
}

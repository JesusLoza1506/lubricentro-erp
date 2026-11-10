use lubricentro_erp_lib::{
    db::init_db,
    dto::{pos_dto::*, producto_dto::CrearProductoDto},
    services::{
        inventario_service::InventarioService,
        pos_service::{consultas, devoluciones, ventas},
    },
};
use rusqlite::{params, Connection};
use serde_json::{json, Value};
fn preparar(ml: bool) -> (tempfile::TempDir, Connection, i64) {
    let d = tempfile::tempdir().unwrap();
    let mut c = init_db(d.path().into()).unwrap();
    c.execute("INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES(1,'Admin prueba','admin','$argon2id$prueba','ADMINISTRADOR'),(2,'Cajero prueba','cajero','$argon2id$prueba','CAJERO'),(3,'Mecánico prueba','mecanico','$argon2id$prueba','MECANICO'),(4,'Otro cajero','otro','$argon2id$prueba','CAJERO')",[]).unwrap();
    c.execute("UPDATE configuracion SET ruc='20600695771',razon_social='Empresa de prueba',direccion='Av. Principal 123',ubigeo='110201',nubefact_ruta_api='https://api.nubefact.com/api/v1/prueba',nubefact_token_ref='token_prueba'",[]).unwrap();
    for (tipo, serie) in [
        ("BOLETA", "B001"),
        ("FACTURA", "F001"),
        ("NC_BOLETA", "BC01"),
        ("NC_FACTURA", "FC01"),
    ] {
        c.execute("INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,activa) VALUES('PRODUCCION',?1,?2,1)",params![tipo,serie]).unwrap();
    }
    c.execute("INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent) VALUES('caja-pos-prueba-01','CAJA-POS',2,'PRODUCCION',10000)",[]).unwrap();
    let id = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "PROD-POS".into(),
            nombre: "Filtro de prueba".into(),
            descripcion: None,
            categoria: if ml { "GRANEL" } else { "FILTRO" }.into(),
            unidad_medida: if ml { "ML" } else { "UND" }.into(),
            stock_minimo: 0,
            precio_venta_cent: 2000,
            cantidad_inicial: Some(10.0),
            costo_unitario_inicial_micro: Some(12_000_000),
            referencia_inicial: None,
        },
        1,
    )
    .unwrap();
    (d, c, id)
}
fn solicitud(id: i64, q: i64) -> ConfirmarVentaPos {
    ConfirmarVentaPos {
        id_operacion: uuid::Uuid::new_v4().to_string(),
        id_ot: None,
        id_cliente: 1,
        tipo_comprobante: "BOLETA".into(),
        descuento_bp: 0,
        motivo: "".into(),
        lineas: vec![LineaDirecta {
            tipo: "PRODUCTO".into(),
            id,
            cantidad: q,
            precio_esperado_cent: 2000,
        }],
        pagos: vec![PagoPos {
            medio: "EFECTIVO".into(),
            monto_cent: 10000,
            recibido_cent: Some(12000),
            referencia: None,
        }],
    }
}
fn n(c: &Connection, sql: &str) -> i64 {
    c.query_row(sql, [], |r| r.get(0)).unwrap()
}
fn aceptar(c: &Connection, doc: i64) {
    c.execute("UPDATE comprobantes SET estado_envio='ACEPTADO',fecha_confirmacion=datetime('now','-5 hours') WHERE id_comprobante=?1",[doc]).unwrap();
}
fn retorno(v: &Value, linea: i64, q: i64, rec: i64) -> DevolverVentaPos {
    DevolverVentaPos {
        id_operacion: uuid::Uuid::new_v4().to_string(),
        id_venta: v["id_venta"].as_i64().unwrap(),
        motivo: "Devolución solicitada por cliente".into(),
        medio: "EFECTIVO".into(),
        referencia: None,
        lineas: vec![LineaRetorno {
            id_detalle: linea,
            cantidad: q,
            recuperada: rec,
        }],
    }
}
#[test]
fn venta_directa_atomica_pago_vuelto_y_kardex() {
    let (_d, mut c, id) = preparar(false);
    let r = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    assert_eq!(r["total_cent"], 10000);
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 5);
    assert_eq!(n(&c, "SELECT valor_stock_micro FROM productos"), 60_000_000);
    assert_eq!(n(&c, "SELECT vuelto_cent FROM movimientos_caja"), 2000);
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM comprobantes WHERE estado_envio='PENDIENTE'"
        ),
        1
    );
}
#[test]
fn servicios_directos_respetan_limite_sin_dejar_efectos() {
    let (_d, mut c, id) = preparar(false);
    c.execute("INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent) VALUES(1,'SERV-POS','Servicio directo',100)", []).unwrap();
    let mut p = solicitud(id, 5);
    p.lineas.push(LineaDirecta {
        tipo: "SERVICIO".into(),
        id: 1,
        cantidad: 1,
        precio_esperado_cent: 100,
    });
    assert_eq!(ventas::cotizar(&c, 2, &p).unwrap()["total_cent"], 10_100);
    p.lineas[1].cantidad = 99_999;
    assert_eq!(
        ventas::cotizar(&c, 2, &p).unwrap()["total_cent"],
        10_009_900
    );
    for cantidad in [0, 100_000] {
        p.lineas[1].cantidad = cantidad;
        let cotizacion = ventas::cotizar(&c, 2, &p).unwrap_err().to_string();
        assert!(cotizacion.contains("99 999"));
        let confirmacion = ventas::confirmar(&mut c, 2, None, &p)
            .unwrap_err()
            .to_string();
        assert!(confirmacion.contains("99 999"));
    }
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 0);
    assert_eq!(n(&c, "SELECT count(*) FROM movimientos_caja"), 0);
    assert_eq!(n(&c, "SELECT count(*) FROM comprobantes"), 0);
    assert_eq!(
        n(&c, "SELECT sum(correlativo_actual) FROM series_comprobante"),
        0
    );
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 10);
}

#[test]
fn detalle_conserva_observaciones_y_estado_aceptado() {
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    let doc = v["id_documento"].as_i64().unwrap();
    let venta = v["id_venta"].as_i64().unwrap();
    aceptar(&c, doc);
    for (entrada, esperado) in [
        (
            json!("  Nota del proveedor  "),
            json!(["Nota del proveedor"]),
        ),
        (
            json!(["Primera nota", " ", "Segunda nota"]),
            json!(["Primera nota", "Segunda nota"]),
        ),
        (Value::Null, json!([])),
    ] {
        c.execute(
            "UPDATE comprobantes SET advertencias_json=?1 WHERE id_comprobante=?2",
            params![entrada.to_string(), doc],
        )
        .unwrap();
        let detalle = consultas::detalle(&c, 2, venta).unwrap();
        let documento = &detalle["documentos"][0];
        assert_eq!(documento["estado"], "ACEPTADO");
        assert_eq!(documento["advertencias"], esperado);
    }
}

#[test]
fn reintento_devuelve_resultado_y_rechaza_otra_carga() {
    let (_d, mut c, id) = preparar(false);
    let mut p = solicitud(id, 5);
    let r = ventas::confirmar(&mut c, 2, None, &p).unwrap();
    assert_eq!(r, ventas::confirmar(&mut c, 2, None, &p).unwrap());
    p.lineas[0].cantidad = 4;
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 1);
}
#[test]
fn fallo_auditoria_revierte_venta_stock_serie_y_caja() {
    let (_d, mut c, id) = preparar(false);
    c.execute_batch("CREATE TRIGGER fallo_pos BEFORE INSERT ON auditoria WHEN NEW.modulo='POS' BEGIN SELECT RAISE(ABORT,'Fallo simulado'); END;").unwrap();
    assert!(ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).is_err());
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 10);
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 0);
    assert_eq!(
        n(&c, "SELECT sum(correlativo_actual) FROM series_comprobante"),
        0
    );
    assert_eq!(n(&c, "SELECT count(*) FROM movimientos_caja"), 0);
}
#[test]
fn devolucion_40_y_60_recupera_solo_dos_y_no_repite() {
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    let p = retorno(&v, linea, 2, 2);
    let r = devoluciones::devolver(&mut c, 2, Some(1), &p).unwrap();
    assert_eq!(r["total_cent"], 4000);
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 7);
    assert_eq!(r, devoluciones::devolver(&mut c, 2, None, &p).unwrap());
    assert!(devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 3, 0)).is_err());
    aceptar(&c, r["id_documento"].as_i64().unwrap());
    let r2 = devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 3, 0)).unwrap();
    assert_eq!(r2["total_cent"], 6000);
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 7);
    assert_eq!(
        n(
            &c,
            "SELECT sum(monto_cent) FROM movimientos_caja WHERE tipo='REINTEGRO'"
        ),
        10000
    );
}
#[test]
fn permisos_privacidad_y_venta_ajena() {
    let (_d, mut c, id) = preparar(false);
    let p = solicitud(id, 5);
    assert!(ventas::confirmar(&mut c, 3, None, &p).is_err());
    let r = ventas::confirmar(&mut c, 2, None, &p).unwrap();
    assert!(consultas::detalle(&c, 4, r["id_venta"].as_i64().unwrap()).is_err());
    let publico = consultas::detalle(&c, 2, r["id_venta"].as_i64().unwrap())
        .unwrap()
        .to_string();
    assert!(!publico.contains("costo"));
    assert!(consultas::catalogo(&c, 3, "", "TODOS", 1).is_err());
}
#[test]
fn granel_no_divide_costo_dos_veces() {
    let (_d, mut c, id) = preparar(true);
    let mut p = solicitud(id, 4500);
    p.pagos[0].monto_cent = 9000;
    let r = ventas::confirmar(&mut c, 2, None, &p).unwrap();
    assert_eq!(r["total_cent"], 9000);
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 5500);
    assert_eq!(n(&c, "SELECT valor_stock_micro FROM productos"), 66_000_000);
    assert_eq!(
        n(&c, "SELECT costo_total_micro FROM detalle_venta"),
        54_000_000
    );
}
#[test]
fn precio_cambiado_stock_y_factura_incompleta_no_cobran() {
    let (_d, mut c, id) = preparar(false);
    let mut p = solicitud(id, 5);
    p.lineas[0].precio_esperado_cent = 1800;
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
    p.lineas[0].precio_esperado_cent = 2000;
    p.tipo_comprobante = "FACTURA".into();
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
    p.tipo_comprobante = "BOLETA".into();
    p.lineas[0].cantidad = 11;
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 0);
}
#[test]
fn descuento_y_pago_mixto_concilian() {
    let (_d, mut c, id) = preparar(false);
    let mut p = solicitud(id, 5);
    p.descuento_bp = 501;
    p.motivo = "Descuento autorizado por promoción".into();
    p.pagos = vec![
        PagoPos {
            medio: "YAPE".into(),
            monto_cent: 3000,
            recibido_cent: None,
            referencia: None,
        },
        PagoPos {
            medio: "EFECTIVO".into(),
            monto_cent: 6499,
            recibido_cent: Some(10000),
            referencia: None,
        },
    ];
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
    let r = ventas::confirmar(&mut c, 2, Some(1), &p).unwrap();
    assert_eq!(r["total_cent"], 9499);
    assert_eq!(n(&c, "SELECT sum(monto_cent) FROM movimientos_caja"), 9499);
    assert_eq!(
        n(
            &c,
            "SELECT vuelto_cent FROM movimientos_caja WHERE medio_pago='EFECTIVO'"
        ),
        3501
    );
}
#[test]
fn catalogo_paginado_sin_costos_y_fechas_invalidas() {
    let (_d, c, _id) = preparar(false);
    let p = consultas::catalogo(&c, 2, "Filtro", "TODOS", 1).unwrap();
    assert_eq!(p["total"], 1);
    assert!(!p.to_string().contains("micro"));
    assert!(
        consultas::listado(&c, 2, &json!({"desde":"2024-01-01","hasta":"2026-01-01"})).is_err()
    );
}

#[test]
fn cobrar_ot_con_adicional_no_repite_consumo_y_conserva_precios() {
    use lubricentro_erp_lib::{
        dto::orden_trabajo_dto::*, services::ordenes_service::OrdenesService,
    };
    let (_d, mut c, id) = preparar(true);
    c.execute_batch("INSERT INTO clientes(id_cliente,tipo_documento,numero_documento,nombre_razon_social) VALUES(2,'DNI','12345678','Cliente de taller'); INSERT INTO vehiculos(id_vehiculo,placa,id_cliente,marca,modelo,kilometraje_actual) VALUES(1,'ABC123',2,'Toyota','Yaris',80000); INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent,es_cambio_aceite) VALUES(1,'SERV-POS','Cambio de aceite',2000,1);").unwrap();
    let ot = OrdenesService::crear_orden_trabajo(
        &mut c,
        CrearOrdenTrabajoDto {
            id_vehiculo: 1,
            id_cliente_atendido: 2,
            kilometraje_ingreso: 80000,
            id_mecanico: None,
            pedido_cliente: None,
            servicios_iniciales: vec![ServicioInicialDto {
                id_servicio: 1,
                cantidad: 1,
            }],
        },
        2,
        "CAJERO",
    )
    .unwrap();
    OrdenesService::iniciar_trabajo_ot(
        &mut c,
        IniciarTrabajoOtDto {
            id_ot: ot.id_ot,
            id_zanja_ejecucion: 1,
            id_mecanico: None,
        },
        3,
        "MECANICO",
    )
    .unwrap();
    OrdenesService::registrar_insumo_ot(
        &mut c,
        RegistrarInsumoOtDto {
            id_ot: ot.id_ot,
            id_producto: id,
            cantidad: 4.5,
            id_operacion: Some(uuid::Uuid::new_v4().to_string()),
        },
        3,
        "MECANICO",
    )
    .unwrap();
    OrdenesService::finalizar_ot(
        &mut c,
        FinalizarOtDto {
            id_ot: ot.id_ot,
            kilometraje_salida: 80000,
            resumen_publico: "Servicio completado".into(),
            tipo_aceite: Some("MINERAL".into()),
        },
        3,
        "MECANICO",
    )
    .unwrap();
    c.execute(
        "UPDATE servicios SET precio_venta_cent=3000 WHERE id_servicio=1",
        [],
    )
    .unwrap();
    let mut p = solicitud(id, 1000);
    p.id_ot = Some(ot.id_ot);
    p.pagos[0].monto_cent = 13000;
    p.pagos[0].recibido_cent = Some(15000);
    let disponibles = consultas::ots(&c, 2, "ABC123", 1).unwrap();
    assert_eq!(disponibles["total"], 1);
    let inicio = |db: &Connection, usuario| {
        serde_json::to_value(
            lubricentro_erp_lib::services::dashboard_service::DashboardService::obtener_dashboard(
                db, usuario,
            )
            .unwrap(),
        )
        .unwrap()
    };
    assert_eq!(inicio(&c, 2)["pendientes"]["total"], 1);
    let contexto = &disponibles["filas"][0];
    assert_eq!(contexto["id"], ot.id_ot);
    assert!(contexto["codigo"].as_str().is_some_and(|s| !s.is_empty()));
    assert_eq!(contexto["placa"], "ABC123");
    assert_eq!(contexto["cliente"], "Cliente de taller");
    assert!(contexto["fecha"].as_str().is_some_and(|s| !s.is_empty()));
    assert_eq!(contexto["total_cent"], 11000);
    let cotizacion = ventas::cotizar(&c, 2, &p).unwrap();
    assert_eq!(cotizacion["ot"], *contexto);
    assert_eq!(cotizacion["total_cent"], 13000);
    let r = ventas::confirmar(&mut c, 2, None, &p).unwrap();
    assert_eq!(r["total_cent"], 13000);
    assert_eq!(consultas::ots(&c, 2, "ABC123", 1).unwrap()["total"], 0);
    assert_eq!(inicio(&c, 2)["pendientes"]["total"], 0);
    assert!(inicio(&c, 1)["mediana_cobro_seg"].as_i64().is_some());
    let mov = n(
        &c,
        "SELECT id_movimiento FROM movimientos_inventario WHERE tipo_movimiento='VENTA_DIRECTA'",
    );
    let origen = InventarioService::obtener_origen_kardex(&c, mov).unwrap();
    assert_eq!(origen.tipo, "VENTA");
    assert_eq!(origen.lineas.len(), 3);

    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 4500);
    assert_eq!(
        n(
            &c,
            "SELECT precio_unitario_cent FROM detalle_venta WHERE id_servicio_ot IS NOT NULL"
        ),
        2000
    );
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM movimientos_inventario WHERE tipo_movimiento='CONSUMO_OT'"
        ),
        1
    );
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM movimientos_inventario WHERE tipo_movimiento='VENTA_DIRECTA'"
        ),
        1
    );
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM ordenes_trabajo WHERE estado='FINALIZADO'"
        ),
        1
    );
    p.id_operacion = uuid::Uuid::new_v4().to_string();
    assert!(ventas::confirmar(&mut c, 2, None, &p).is_err());
}

#[test]
fn rechazo_reemite_misma_venta_sin_stock_o_cobro_nuevo() {
    use lubricentro_erp_lib::services::pos_service::documentos;
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    let doc = v["id_documento"].as_i64().unwrap();
    c.execute("UPDATE comprobantes SET estado_envio='RECHAZADO',fecha_confirmacion=datetime('now','-5 hours') WHERE id_comprobante=?1",[doc]).unwrap();
    let p = ReemitirPos {
        id_operacion: uuid::Uuid::new_v4().to_string(),
        id_documento: doc,
        receptor: Value::Null,
        motivo: "Corrección documental de rechazo".into(),
    };
    let nuevo = documentos::reemitir(&mut c, 1, &p).unwrap();
    assert_ne!(nuevo["id_documento"], v["id_documento"]);
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 1);
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM movimientos_caja WHERE tipo='COBRO'"
        ),
        1
    );
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 5);
    assert_eq!(n(&c, "SELECT max(correlativo) FROM comprobantes"), 2);
    assert_eq!(nuevo, documentos::reemitir(&mut c, 1, &p).unwrap());
}

#[test]
fn devolucion_residual_y_recuperacion_inactiva_conservan_costo() {
    let (_d, mut c, id) = preparar(false);
    let mut p = solicitud(id, 3);
    p.descuento_bp = 333;
    p.pagos[0].monto_cent = 5800; // descuento de 200 céntimos
    let v = ventas::confirmar(&mut c, 2, None, &p).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    c.execute("UPDATE productos SET activo=0 WHERE id_producto=?1", [id])
        .unwrap();
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    let mut suma = 0;
    for _ in 0..3 {
        let r = devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 1, 1)).unwrap();
        suma += r["total_cent"].as_i64().unwrap();
        aceptar(&c, r["id_documento"].as_i64().unwrap());
    }
    assert_eq!(suma, 5800);
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 10);
    assert_eq!(
        n(&c, "SELECT valor_stock_micro FROM productos"),
        120_000_000
    );
}

#[test]
fn fallo_nota_revierte_reintegro_retorno_y_devolucion() {
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    c.execute(
        "UPDATE series_comprobante SET activa=0 WHERE tipo_comprobante='NC_BOLETA'",
        [],
    )
    .unwrap();
    assert!(devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 2, 2)).is_err());
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 5);
    assert_eq!(n(&c, "SELECT count(*) FROM devoluciones"), 0);
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM movimientos_caja WHERE tipo='REINTEGRO'"
        ),
        0
    );
}

#[test]
fn comando_de_negocio_deriva_actor_y_exige_autorizacion_real() {
    use lubricentro_erp_lib::services::{
        auth_service::AuthService, pos_service::operaciones, session_manager::SessionState,
    };
    let (_d, mut c, id) = preparar(false);
    let hash = AuthService::hash_password("AdminPass123").unwrap();
    c.execute(
        "UPDATE usuarios SET password_hash=?1 WHERE id_usuario=1",
        [hash],
    )
    .unwrap();
    let mut sesion = SessionState::new();
    sesion.abrir_sesion(2, 0, "cajero".into(), "CAJERO".into(), false);
    let mut p = solicitud(id, 5);
    p.descuento_bp = 600;
    p.motivo = "Descuento de campaña autorizado".into();
    p.pagos[0].monto_cent = 9400;
    let v = serde_json::to_value(&p).unwrap();
    assert!(operaciones::ejecutar(&mut c, &mut sesion, "confirmar", v.clone(), None).is_err());
    assert!(operaciones::ejecutar(
        &mut c,
        &mut sesion,
        "confirmar",
        v.clone(),
        Some(CredencialesPos {
            username: "admin".into(),
            password: "incorrecta".into()
        })
    )
    .is_err());
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM auditoria WHERE modulo='POS' AND resultado='DENEGADO'"
        ),
        1
    );
    let r = operaciones::ejecutar(
        &mut c,
        &mut sesion,
        "confirmar",
        v.clone(),
        Some(CredencialesPos {
            username: "admin".into(),
            password: "AdminPass123".into(),
        }),
    )
    .unwrap();
    assert_eq!(n(&c, "SELECT id_emisor FROM ventas"), 2);
    assert_eq!(n(&c, "SELECT id_autorizante FROM ventas"), 1);
    assert_eq!(sesion.usuario_id, Some(2));
    let reintento = operaciones::ejecutar(&mut c, &mut sesion, "confirmar", v, None).unwrap();
    assert_eq!(r["id_venta"], reintento["id_venta"]);
    assert!(reintento.get("token_envio").is_none());
    let token = r["token_envio"].as_str().unwrap();
    let referencia = r["id_documento"].to_string();
    assert_eq!(
        sesion
            .consumir_autorizacion(token, 2, "POS_ENVIO_INICIAL", &referencia)
            .unwrap(),
        2
    );
    assert!(sesion
        .consumir_autorizacion(token, 2, "POS_ENVIO_INICIAL", &referencia)
        .is_err());
    let mut cambiada = p.clone();
    cambiada.pagos[0].recibido_cent = Some(13000);
    assert!(operaciones::ejecutar(
        &mut c,
        &mut sesion,
        "confirmar",
        serde_json::to_value(cambiada).unwrap(),
        None
    )
    .is_err());
}
#[test]
fn sesion_limitada_mecanico_y_actor_falsificado_no_operan() {
    use lubricentro_erp_lib::services::{pos_service::operaciones, session_manager::SessionState};
    let (_d, mut c, id) = preparar(false);
    let mut sesion = SessionState::new();
    assert!(operaciones::ejecutar(&mut c, &mut sesion, "inicio", json!({}), None).is_err());
    sesion.abrir_sesion(2, 0, "cajero".into(), "CAJERO".into(), true);
    assert!(operaciones::ejecutar(&mut c, &mut sesion, "inicio", json!({}), None).is_err());
    sesion.abrir_sesion(3, 0, "mecanico".into(), "MECANICO".into(), false);
    assert!(operaciones::ejecutar(&mut c, &mut sesion, "inicio", json!({}), None).is_err());
    sesion.abrir_sesion(2, 0, "cajero".into(), "CAJERO".into(), false);
    let mut p = serde_json::to_value(solicitud(id, 5)).unwrap();
    p["id_emisor"] = json!(1);
    assert!(operaciones::ejecutar(&mut c, &mut sesion, "confirmar", p, None).is_err());
    assert_eq!(n(&c, "SELECT count(*) FROM ventas"), 0);
}

#[test]
fn nota_rechazada_se_reemite_sin_repetir_reintegro_y_con_receptor_original() {
    use lubricentro_erp_lib::services::pos_service::documentos;
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    let d = devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 1, 1)).unwrap();
    let nota = d["id_documento"].as_i64().unwrap();
    c.execute("UPDATE comprobantes SET estado_envio='RECHAZADO',fecha_confirmacion=datetime('now','-5 hours') WHERE id_comprobante=?1",[nota]).unwrap();
    let mut p = ReemitirPos {
        id_operacion: uuid::Uuid::new_v4().to_string(),
        id_documento: nota,
        receptor: json!({"tipo":"DNI","documento":"12345678","nombre":"Otra persona","direccion":"Otra dirección"}),
        motivo: "Reemisión de nota por rechazo oficial".into(),
    };
    assert!(documentos::reemitir(&mut c, 1, &p).is_err());
    p.receptor = Value::Null;
    documentos::reemitir(&mut c, 1, &p).unwrap();
    assert_eq!(n(&c, "SELECT count(*) FROM devoluciones"), 1);
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM movimientos_caja WHERE tipo='REINTEGRO'"
        ),
        1
    );
    assert_eq!(n(&c, "SELECT stock_actual FROM productos"), 6);
}

#[test]
fn caja_toma_solo_efectivo_aplicado_y_reintegra_venta_previa_en_turno_actual() {
    use lubricentro_erp_lib::{
        dto::caja_dto::{AbrirCajaDto, CierreCajaDto, FiltroMovimientosCaja},
        services::caja_service::CajaService,
    };
    let (_d, mut c, producto) = preparar(false);
    let mut req = solicitud(producto, 4);
    req.pagos = vec![
        PagoPos {
            medio: "EFECTIVO".into(),
            monto_cent: 5_000,
            recibido_cent: Some(10_000),
            referencia: None,
        },
        PagoPos {
            medio: "YAPE".into(),
            monto_cent: 3_000,
            recibido_cent: None,
            referencia: None,
        },
    ];
    let venta = ventas::confirmar(&mut c, 2, None, &req).unwrap();
    aceptar(&c, venta["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    let anterior = CajaService::detalle(&c, 2, 1).unwrap();
    assert_eq!(anterior["resumen"]["efectivo_esperado_cent"], 15_000);
    assert_eq!(anterior["resumen"]["yape"]["neto_cent"], 3_000);
    assert_eq!(
        n(
            &c,
            "SELECT vuelto_cent FROM movimientos_caja WHERE medio_pago='EFECTIVO'"
        ),
        5_000
    );
    CajaService::cerrar_caja(
        &mut c,
        2,
        CierreCajaDto {
            id_operacion: uuid::Uuid::new_v4().to_string(),
            id_caja: 1,
            version_esperada: anterior["turno"]["version"].as_i64().unwrap(),
            efectivo_contado_cent: 15_000,
            fondo_siguiente_cent: 10_000,
            motivo: None,
        },
    )
    .unwrap();
    let turno = CajaService::abrir_caja(
        &mut c,
        2,
        AbrirCajaDto {
            id_operacion: uuid::Uuid::new_v4().to_string(),
            monto_apertura_cent: 10_000,
        },
    )
    .unwrap();
    let nuevo = turno["id_caja"].as_i64().unwrap();
    let mut req = retorno(&venta, linea, 1, 0);
    req.medio = "YAPE".into();
    devoluciones::devolver(&mut c, 2, Some(1), &req).unwrap();
    let r = CajaService::detalle(&c, 2, nuevo).unwrap();
    assert_eq!(r["resumen"]["efectivo_esperado_cent"], 10_000);
    assert_eq!(r["resumen"]["yape"]["neto_cent"], -2_000);
    let movs = CajaService::movimientos(
        &c,
        2,
        FiltroMovimientosCaja {
            id_caja: nuevo,
            pagina: None,
            origen: Some("POS".into()),
            direccion: Some("EGRESO".into()),
            buscar: None,
        },
    )
    .unwrap();
    assert_eq!(movs["total"], 1);
    assert_eq!(movs["datos"][0]["id_venta"], venta["id_venta"]);
    assert_eq!(movs["datos"][0]["autorizante"], "Admin prueba");
    assert_eq!(
        CajaService::detalle(&c, 2, 1).unwrap()["resumen"]["yape"]["neto_cent"],
        3_000
    );
}

#[test]
fn editar_empresa_con_comprobante_pendiente_conserva_ruc_y_foto_anterior() {
    use lubricentro_erp_lib::services::admin_service::{empresa, guardar_empresa, Empresa};
    let (_d, mut c, id) = preparar(false);
    let venta = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    assert_eq!(venta["total_cent"], 10000);
    let ver = empresa(&c).unwrap();
    assert_eq!(ver["ruc_fijo"], true);
    let datos = |ruc: &str| Empresa {
        ruc: ruc.into(),
        razon_social: "Empresa de prueba".into(),
        nombre_comercial: "Lubricentro demo".into(),
        direccion: "Av. Nueva 500".into(),
        ubigeo: "110201".into(),
        telefono: "+51 987 654 321".into(),
        correo: "demo@example.invalid".into(),
        version: ver["version"].as_i64().unwrap(),
        motivo: Some("Corrección de prueba".into()),
    };
    assert!(guardar_empresa(&mut c, 1, datos("20999999990")).is_err());
    let nueva = guardar_empresa(&mut c, 1, datos("20600695771")).unwrap();
    assert_eq!(nueva["direccion"], "Av. Nueva 500");
    assert_eq!(nueva["telefono"], "987654321");
    let (direccion, estado): (String, String) = c
        .query_row(
            "SELECT json_extract(empresa_snapshot,'$.direccion'),estado_envio FROM comprobantes",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(
        (direccion.as_str(), estado.as_str()),
        ("Av. Principal 123", "PENDIENTE")
    );
}

#[test]
fn reportes_usan_venta_cobro_y_recuperacion_fisica_del_pos() {
    use lubricentro_erp_lib::services::{
        dashboard_service::DashboardService,
        reportes_service::{ahora_lima, consultar, Filtros, Periodo, Reporte},
    };
    let (_d, mut c, _) = preparar(false);
    let id = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "REP-POS".into(),
            nombre: "Producto del ejemplo de Reportes".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 0,
            precio_venta_cent: 1000,
            cantidad_inicial: Some(10.0),
            costo_unitario_inicial_micro: Some(5_000_000),
            referencia_inicial: None,
        },
        1,
    )
    .unwrap();
    let mut q = solicitud(id, 10);
    q.descuento_bp = 1000;
    q.motivo = "Descuento de prueba autorizado".into();
    q.lineas[0].precio_esperado_cent = 1000;
    q.pagos[0].monto_cent = 6000;
    q.pagos[0].recibido_cent = Some(10000);
    q.pagos.push(PagoPos {
        medio: "YAPE".into(),
        monto_cent: 3000,
        recibido_cent: None,
        referencia: None,
    });
    let v = ventas::confirmar(&mut c, 2, Some(1), &q).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 3, 2)).unwrap();
    let ventas = |r| {
        let corte = consultar(
            &c,
            r,
            Filtros {
                periodo: Periodo::Hoy,
                ..Default::default()
            },
            "PRODUCCION",
            ahora_lima(),
        )
        .unwrap();
        serde_json::to_value(corte.pagina(1).unwrap()).unwrap()
    };
    let tot = |p: &Value, nombre: &str| {
        p["totales"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t[0] == nombre)
            .unwrap()[1]["valor"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let r01 = ventas(Reporte::R01);
    assert_eq!(tot(&r01, "Ventas"), "1");
    assert_eq!(tot(&r01, "Bruto (S/)"), "100.00");
    assert_eq!(tot(&r01, "Descuentos (S/)"), "10.00");
    assert_eq!(tot(&r01, "Vendido (S/)"), "90.00");
    assert_eq!(tot(&r01, "Devuelto (S/)"), "27.00");
    assert_eq!(tot(&r01, "Neto (S/)"), "63.00");
    let r02 = ventas(Reporte::R02);
    assert_eq!(tot(&r02, "Movimientos"), "3");
    assert_eq!(tot(&r02, "Neto (S/)"), "63.00");
    let r03 = ventas(Reporte::R03);
    assert_eq!(tot(&r03, "Cantidad (UND)"), "7");
    assert_eq!(tot(&r03, "Subtotal Top 10 sin IGV (S/)"), "53.39");
    assert_eq!(tot(&r03, "Costo reconocido (S/)"), "40.00");
    assert_eq!(tot(&r03, "Margen (S/)"), "13.39");
    assert_eq!(ventas(Reporte::R07)["total_filas"], 2);
    assert_eq!(n(&c,"SELECT count(*) FROM movimientos_caja m JOIN ventas v USING(id_venta) WHERE m.fecha_operacion=v.fecha_venta"),2);
    assert_eq!(n(&c,"SELECT count(*) FROM movimientos_caja m JOIN devoluciones d USING(id_devolucion) WHERE m.fecha_operacion=d.fecha_operacion"),1);
    let dash = serde_json::to_value(DashboardService::obtener_dashboard(&c, 1).unwrap()).unwrap();
    assert_eq!(dash["ventas_netas_hoy_cent"], 6300);
    assert_eq!(dash["ventas_por_dia"][6]["valor"], 6300);
    let caja = serde_json::to_value(DashboardService::obtener_dashboard(&c, 2).unwrap()).unwrap();
    assert_eq!(caja["actividad_turno"]["cobros"], 1); // efectivo + Yape, sin contar el reintegro
    assert!(caja.get("ventas_netas_hoy_cent").is_none());
}

#[test]
#[ignore = "Genera 20.000 ventas confirmadas reales de POS en SQLite temporal en disco"]
fn rendimiento_reportes_con_20000_ventas() {
    use lubricentro_erp_lib::services::reportes_service::{
        ahora_lima, consultar, Filtros, Periodo, Reporte,
    };
    use std::time::{Duration, Instant};
    let (_d, mut c, _) = preparar(false);
    let id = InventarioService::registrar_producto(
        &mut c,
        CrearProductoDto {
            codigo: "REP-VOLUMEN".into(),
            nombre: "Producto de volumen de Reportes".into(),
            descripcion: None,
            categoria: "FILTRO".into(),
            unidad_medida: "UND".into(),
            stock_minimo: 0,
            precio_venta_cent: 1000,
            cantidad_inicial: Some(20_000.0),
            costo_unitario_inicial_micro: Some(5_000_000),
            referencia_inicial: None,
        },
        1,
    )
    .unwrap();
    let mut q = solicitud(id, 1);
    q.lineas[0].precio_esperado_cent = 1000;
    q.pagos[0].monto_cent = 1000;
    q.pagos[0].recibido_cent = Some(1000);
    for i in 0..20_000 {
        if i % 5_000 == 0 {
            println!("Preparación: {i} de 20000 ventas");
        }
        q.id_operacion = uuid::Uuid::new_v4().to_string();
        ventas::confirmar(&mut c, 2, None, &q).unwrap();
    }
    assert_eq!(
        n(
            &c,
            "SELECT count(*) FROM ventas WHERE estado_registro='CONFIRMADA'"
        ),
        20_000
    );
    let mut aptas = 0;
    for i in 1..=20 {
        let inicio = Instant::now();
        let corte = consultar(
            &c,
            Reporte::R01,
            Filtros {
                periodo: Periodo::EsteMes,
                ..Default::default()
            },
            "PRODUCCION",
            ahora_lima(),
        )
        .unwrap();
        let datos = serde_json::to_value(corte.pagina(1).unwrap()).unwrap();
        let duracion = inicio.elapsed();
        println!("Consulta R01 {i}: {duracion:?}");
        assert_eq!(datos["totales"][0][1]["valor"], "20000");
        aptas += usize::from(duracion <= Duration::from_secs(3));
    }
    assert!(aptas >= 19, "{aptas}/20 consultas cumplieron");
}

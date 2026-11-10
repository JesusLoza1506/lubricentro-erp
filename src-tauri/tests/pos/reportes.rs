use super::*;
use lubricentro_erp_lib::services::reportes_service::{
    ahora_lima, consultar, exportar, Corte, Filtros, Periodo, Reporte,
};
use std::io::Read;

fn corte(c: &Connection, r: Reporte, filtros: Filtros) -> Corte {
    consultar(c, r, filtros, "PRODUCCION", ahora_lima()).unwrap()
}

fn pagina(c: &Corte) -> Value {
    serde_json::to_value(c.pagina(1).unwrap()).unwrap()
}

fn total<'a>(p: &'a Value, titulo: &str) -> &'a str {
    p["totales"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t[0] == titulo)
        .unwrap()[1]["valor"]
        .as_str()
        .unwrap()
}

fn producto(c: &mut Connection, codigo: &str, ml: bool, precio: i64) -> i64 {
    InventarioService::registrar_producto(
        c,
        CrearProductoDto {
            codigo: codigo.into(),
            nombre: format!("Insumo {codigo}"),
            descripcion: None,
            categoria: if ml { "GRANEL" } else { "FILTRO" }.into(),
            unidad_medida: if ml { "ML" } else { "UND" }.into(),
            stock_minimo: 0,
            precio_venta_cent: precio,
            cantidad_inicial: Some(5.0),
            costo_unitario_inicial_micro: Some(5_000_000),
            referencia_inicial: None,
        },
        1,
    )
    .unwrap()
}

fn entradas_xlsx(file: std::fs::File) -> (String, String) {
    let mut xlsx = zip::ZipArchive::new(file).unwrap();
    let leer = |archivo: &mut zip::ZipArchive<std::fs::File>, nombre| {
        let mut s = String::new();
        archivo
            .by_name(nombre)
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        s
    };
    (
        leer(&mut xlsx, "xl/sharedStrings.xml"),
        leer(&mut xlsx, "xl/worksheets/sheet1.xml"),
    )
}

#[test]
fn top_diez_y_todos_conservan_desempate_totales_y_filas_en_pdf_y_xlsx() {
    let (_d, mut c, _) = preparar(false);
    let mut q = solicitud(0, 1);
    q.lineas.clear();
    for i in (1..=11).rev() {
        let id = producto(&mut c, &format!("RANK-{i:02}"), false, 1180);
        q.lineas.push(LineaDirecta {
            tipo: "PRODUCTO".into(),
            id,
            cantidad: 1,
            precio_esperado_cent: 1180,
        });
    }
    let granel = producto(&mut c, "RANK-ML", true, 2360);
    q.lineas.push(LineaDirecta {
        tipo: "PRODUCTO".into(),
        id: granel,
        cantidad: 1000,
        precio_esperado_cent: 2360,
    });
    q.pagos[0].monto_cent = 15_340;
    q.pagos[0].recibido_cent = Some(16_000);
    ventas::confirmar(&mut c, 2, None, &q).unwrap();
    let dir = tempfile::tempdir().unwrap();
    for todos in [false, true] {
        let reporte = corte(
            &c,
            Reporte::R03,
            Filtros {
                periodo: Periodo::Hoy,
                ver_todos: todos,
                ..Default::default()
            },
        );
        let p = pagina(&reporte);
        let codigos = p["filas"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["celdas"][0]["valor"].as_str().unwrap())
            .collect::<Vec<_>>();
        let n = if todos { 11 } else { 9 };
        let mut esperados = vec!["RANK-ML".to_owned()];
        esperados.extend((1..=11).rev().take(n).map(|i| format!("RANK-{i:02}")));
        assert_eq!(codigos, esperados);
        assert_eq!(total(&p, "Cantidad (UND)"), n.to_string());
        assert_eq!(total(&p, "Cantidad (L)"), "1.000");
        let (etiqueta, monto) = if todos {
            ("Total del resultado sin IGV (S/)", "130.00")
        } else {
            ("Subtotal Top 10 sin IGV (S/)", "110.00")
        };
        assert_eq!(total(&p, etiqueta), monto);
        let pdf = exportar::preparar(
            &reporte,
            exportar::Formato::Pdf,
            &dir.path().join("ranking.pdf"),
        )
        .unwrap();
        let documento = lopdf::Document::load(pdf.path()).unwrap();
        let texto = documento
            .extract_text(&documento.get_pages().into_keys().collect::<Vec<_>>())
            .unwrap();
        let xlsx = exportar::preparar(
            &reporte,
            exportar::Formato::Xlsx,
            &dir.path().join("ranking.xlsx"),
        )
        .unwrap();
        let (cadenas, hoja) = entradas_xlsx(xlsx.reopen().unwrap());
        for codigo in codigos {
            assert!(texto.contains(codigo), "{texto}");
            assert!(cadenas.contains(codigo), "{cadenas}");
        }
        assert_eq!(texto.contains("RANK-01"), todos);
        assert_eq!(cadenas.contains("RANK-01"), todos);
        assert!(texto.contains(monto));
        assert!(cadenas.contains(etiqueta));
        assert!(hoja.contains(&format!("<v>{}</v>", monto.trim_end_matches(".00"))));
    }
}

#[test]
fn devolucion_de_servicio_no_aplica_y_reintegro_sin_recuperacion_preserva_costo_historico() {
    let (_d, mut c, _) = preparar(false);
    let id = producto(&mut c, "HIST-COSTO", false, 1180);
    c.execute(
        "INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent)
        VALUES(1,'SERV-REP','Revisión reportable',590)",
        [],
    )
    .unwrap();
    let mut q = solicitud(id, 2);
    q.lineas[0].precio_esperado_cent = 1180;
    q.lineas.push(LineaDirecta {
        tipo: "SERVICIO".into(),
        id: 1,
        cantidad: 2,
        precio_esperado_cent: 590,
    });
    q.pagos[0].monto_cent = 3540;
    q.pagos[0].recibido_cent = Some(4000);
    let venta = ventas::confirmar(&mut c, 2, None, &q).unwrap();
    aceptar(&c, venta["id_documento"].as_i64().unwrap());
    let linea = n(
        &c,
        "SELECT id_detalle FROM detalle_venta WHERE id_producto IS NOT NULL",
    );
    let serv = n(
        &c,
        "SELECT id_detalle FROM detalle_venta WHERE id_servicio IS NOT NULL",
    );
    let mut d = retorno(&venta, linea, 1, 0);
    d.lineas.push(LineaRetorno {
        id_detalle: serv,
        cantidad: 1,
        recuperada: 0,
    });
    devoluciones::devolver(&mut c, 2, Some(1), &d).unwrap();
    assert_eq!(
        c.query_row(
            "SELECT stock_actual FROM productos WHERE id_producto=?1",
            [id],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        3
    );
    use lubricentro_erp_lib::dto::ingreso_dto::{DetalleIngresoRequestDto, RegistrarIngresoDto};
    c.execute(
        "INSERT INTO proveedores(id_proveedor,tipo_documento,numero_documento,nombre_razon_social)
        VALUES(1,'DNI','12345678','Proveedor de prueba')",
        [],
    )
    .unwrap();
    InventarioService::registrar_ingreso_compra(
        &mut c,
        RegistrarIngresoDto {
            origen: "COMPRA".into(),
            id_proveedor: Some(1),
            tipo_documento: Some("SIN_DOCUMENTO".into()),
            numero_documento: None,
            fecha_documento: None,
            referencia_inicial: None,
            observacion: None,
            detalles: vec![DetalleIngresoRequestDto {
                id_producto: id,
                cantidad: 1,
                costo_efectivo_micro: 13_000_000,
            }],
        },
        None,
        1,
    )
    .unwrap();
    let (q, valor): (i64, i64) = c
        .query_row(
            "SELECT stock_actual,valor_stock_micro FROM productos WHERE id_producto=?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!((q, valor / q), (4, 7_000_000));
    let f = Filtros {
        periodo: Periodo::Hoy,
        ..Default::default()
    };
    let p = pagina(&corte(&c, Reporte::R03, f.clone()));
    assert_eq!(total(&p, "Costo reconocido (S/)"), "10.00");
    assert_eq!(total(&p, "Cantidad (UND)"), "1");
    assert_eq!(total(&p, "Margen (S/)"), "0.00");
    let s = pagina(&corte(
        &c,
        Reporte::R03,
        Filtros {
            tipo: Some("SERVICIOS".into()),
            ..f
        },
    ));
    let fila = &s["filas"][0]["celdas"];
    assert_eq!(fila[0]["valor"], "SERV-REP");
    assert_eq!(fila[3]["valor"], "1");
    assert_eq!(fila[4]["valor"], "5.00");
    assert_eq!(fila[5]["tipo"], "no_aplica");
    assert_eq!(fila[6]["tipo"], "no_aplica");
}

#[test]
fn devolucion_tras_medianoche_de_venta_anterior_concilia_sin_movimientos_manuales() {
    let mut c = Connection::open_in_memory().unwrap();
    let esquema = include_str!("../../migrations/0001_initial_schema_v7.sql");
    // Solo se fija el reloj de las columnas de prueba; se ejecutan las operaciones POS
    // normales con todas las restricciones del esquema y sin editar registros históricos.
    let esquema = esquema
        .replace(
            "fecha_venta TEXT NOT NULL DEFAULT (datetime('now', '-5 hours'))",
            "fecha_venta TEXT NOT NULL DEFAULT '2026-11-08 23:59:59'",
        )
        .replacen(
            "fecha_operacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours'))",
            "fecha_operacion TEXT NOT NULL DEFAULT '2026-11-09 00:00:00'",
            1,
        );
    assert!(esquema.contains("fecha_operacion TEXT NOT NULL DEFAULT '2026-11-09 00:00:00'"));
    c.execute_batch(&esquema).unwrap();
    let id = preparar_en(&mut c, false);
    let mut q = solicitud(id, 5);
    q.pagos[0].monto_cent = 6_000;
    q.pagos[0].recibido_cent = Some(10_000);
    q.pagos.push(PagoPos {
        medio: "YAPE".into(),
        monto_cent: 4_000,
        recibido_cent: None,
        referencia: None,
    });
    let v = ventas::confirmar(&mut c, 2, None, &q).unwrap();
    aceptar(&c, v["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 2, 0)).unwrap();
    c.execute_batch("
        INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion)
        VALUES('aporte-test',1,2,'INGRESO_MANUAL','EFECTIVO',3000,1,'OTRO','Aporte de prueba','2026-11-09 08:00:00'),
              ('gasto-test',1,2,'EGRESO_MANUAL','EFECTIVO',1000,-1,'OTRO','Gasto de prueba','2026-11-09 08:01:00');
    ").unwrap();
    let ahora = chrono::NaiveDate::from_ymd_opt(2026, 11, 9)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();
    let consultar_dia = |r, dia: &str| {
        pagina(
            &consultar(
                &c,
                r,
                Filtros {
                    periodo: Periodo::Personalizado,
                    desde: Some(dia.into()),
                    hasta: Some(dia.into()),
                    ..Default::default()
                },
                "PRODUCCION",
                ahora,
            )
            .unwrap(),
        )
    };
    let hoy = consultar_dia(Reporte::R01, "2026-11-09");
    assert_eq!(total(&hoy, "Ventas"), "0");
    assert_eq!(total(&hoy, "Devoluciones"), "1");
    assert_eq!(total(&hoy, "Neto (S/)"), "-40.00");
    let caja_hoy = consultar_dia(Reporte::R02, "2026-11-09");
    assert_eq!(total(&caja_hoy, "Movimientos"), "1");
    assert_eq!(total(&caja_hoy, "Neto (S/)"), "-40.00");
    let caja_ayer = consultar_dia(Reporte::R02, "2026-11-08");
    assert_eq!(total(&caja_ayer, "Movimientos"), "2");
    assert_eq!(total(&caja_ayer, "Neto (S/)"), "100.00");
}

#[test]
fn comprobantes_lista_documentos_y_vinculos_sin_contar_intentos_como_ventas() {
    use lubricentro_erp_lib::services::pos_service::documentos;
    let (_d, mut c, id) = preparar(false);
    let v = ventas::confirmar(&mut c, 2, None, &solicitud(id, 5)).unwrap();
    let id_doc = v["id_documento"].as_i64().unwrap();
    c.execute(
        "UPDATE comprobantes SET estado_envio='RECHAZADO',
        fecha_confirmacion=datetime('now','-5 hours') WHERE id_comprobante=?1",
        [id_doc],
    )
    .unwrap();
    let reemitido = documentos::reemitir(
        &mut c,
        1,
        &ReemitirPos {
            id_operacion: uuid::Uuid::new_v4().to_string(),
            id_documento: id_doc,
            receptor: Value::Null,
            motivo: "Corrección documental de prueba".into(),
        },
    )
    .unwrap();
    aceptar(&c, reemitido["id_documento"].as_i64().unwrap());
    let linea = n(&c, "SELECT id_detalle FROM detalle_venta");
    devoluciones::devolver(&mut c, 2, Some(1), &retorno(&v, linea, 1, 0)).unwrap();
    for i in 1..=3 {
        c.execute(
            "INSERT INTO auditoria(actor,modulo,accion,entidad,id_entidad,resultado,motivo)
            VALUES('SISTEMA','POS','ENVIAR','comprobante',?1,'OK',?2)",
            (id_doc.to_string(), format!("Intento de prueba {i}")),
        )
        .unwrap();
    }
    let f = Filtros {
        periodo: Periodo::Hoy,
        ..Default::default()
    };
    let reporte = corte(&c, Reporte::R07, f.clone());
    let p = pagina(&reporte);
    assert_eq!(p["total_filas"], 3);
    let filas = p["filas"].as_array().unwrap();
    let relaciones = filas
        .iter()
        .enumerate()
        .map(|(i, fila)| {
            let detalle = serde_json::to_value(reporte.detalle(i).unwrap()).unwrap();
            let campo = |nombre| {
                detalle["filas"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["celdas"][0]["valor"] == nombre)
                    .unwrap()["celdas"][1]["valor"]
                    .clone()
            };
            (
                fila["celdas"][1]["valor"].clone(),
                (
                    campo("Relación"),
                    campo("Documento reemplazado"),
                    campo("Documento referenciado"),
                ),
            )
        })
        .collect::<Vec<_>>();
    let original = relaciones.iter().find(|(_, d)| d.0 == "ORIGINAL").unwrap();
    let sustituto = relaciones.iter().find(|(_, d)| d.0 == "REEMISION").unwrap();
    let nota = relaciones
        .iter()
        .find(|(_, d)| d.0 == "NOTA_CREDITO")
        .unwrap();
    assert_eq!(sustituto.1 .1, original.0);
    assert_eq!(nota.1 .2, sustituto.0);
    assert_eq!(total(&pagina(&corte(&c, Reporte::R01, f)), "Ventas"), "1");
}

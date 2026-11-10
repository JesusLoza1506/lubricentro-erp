use super::*;
use serde_json::Value;

fn db() -> Connection {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(include_str!(
        "../../../migrations/0001_initial_schema_v7.sql"
    ))
    .unwrap();
    db
}
fn fecha() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 11, 9)
        .unwrap()
        .and_hms_opt(13, 30, 0)
        .unwrap()
}
fn ver(db: &Connection, reporte: Reporte, filtros: Filtros) -> Corte {
    consultar(db, reporte, filtros, "PRODUCCION", fecha())
        .unwrap_or_else(|e| panic!("{reporte:?}: {e:?}"))
}

#[test]
fn los_ocho_reportes_consultan_el_esquema_real_y_permiten_vacios() {
    let db = db();
    for reporte in [
        Reporte::R01,
        Reporte::R02,
        Reporte::R03,
        Reporte::R04,
        Reporte::R05,
        Reporte::R06,
        Reporte::R07,
        Reporte::R08,
    ] {
        let corte = ver(&db, reporte, Filtros::default());
        assert!(corte.datos.filas.is_empty(), "{reporte:?}");
        assert!(corte.pagina(1).is_ok());
        assert!(corte.pagina(2).is_err());
    }
    ver(
        &db,
        Reporte::R03,
        Filtros {
            tipo: Some("SERVICIOS".into()),
            ..Default::default()
        },
    );
    ver(
        &db,
        Reporte::R06,
        Filtros {
            actividad: Some("RECIBIDAS".into()),
            ..Default::default()
        },
    );
}

#[test]
fn rango_lima_366_dias_semana_y_mes() {
    let hoy = fecha().date();
    let personalizada = |desde: &str, hasta: &str| Filtros {
        periodo: Periodo::Personalizado,
        desde: Some(desde.into()),
        hasta: Some(hasta.into()),
        ..Default::default()
    };
    assert!(rango(&personalizada("2025-11-09", "2026-11-09"), hoy).is_ok());
    for (a, b) in [
        ("2025-11-08", "2026-11-09"),
        ("2026-11-09", "2026-11-10"),
        ("2026-11-09", "2026-11-08"),
        ("2026-2-09", "2026-11-09"),
    ] {
        assert!(rango(&personalizada(a, b), hoy).is_err());
    }
    assert_eq!(
        rango(
            &Filtros {
                periodo: Periodo::EstaSemana,
                ..Default::default()
            },
            hoy
        )
        .unwrap(),
        (hoy, hoy)
    );
    let (a, b) = rango(
        &Filtros {
            periodo: Periodo::MesAnterior,
            ..Default::default()
        },
        hoy,
    )
    .unwrap();
    assert_eq!((a.day(), b.day(), a.month()), (1, 31, 10));
}

#[test]
fn corte_inmutable_auditoria_sanea_y_deduplica_restauracion() {
    let db = db();
    db.execute("INSERT INTO restauraciones(id_operacion,evento_externo_id,actor_snapshot,nombre_copia_snapshot,ambiente,sha256_copia,estado,fecha_inicio)
       VALUES('rest','evento1','Administradora','Copia','PRODUCCION',?1,'COMPLETA','2026-11-08 22:00:00')",["a".repeat(64)]).unwrap();
    db.execute_batch(r#"INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento,despues_json,motivo)
        VALUES('SISTEMA','BACKUP','RESTAURAR','OK','2026-11-08 22:00:01','{"evento":"evento1"}',NULL),
        ('SISTEMA','ADMIN','CAMBIAR_ESTADO','OK','2026-11-09 09:00:00','{"estado":"ACTIVA","token":"no-exponer"}','Token: no-exponer');"#).unwrap();
    let estado = EstadoReportes::default();
    let c = estado
        .publicar(
            "sesion-a",
            ver(&db, Reporte::R08, Filtros::default()),
            estado.iniciar_consulta(),
        )
        .unwrap();
    let pagina = serde_json::to_value(c.pagina(1).unwrap()).unwrap();
    assert_eq!(pagina["total_filas"], 2);
    assert!(pagina.to_string().contains("Texto reservado"));
    let detalle = serde_json::to_string(c.detalle(0).unwrap()).unwrap();
    assert!(detalle.contains("ACTIVA"));
    assert!(!detalle.contains("no-exponer"));
    db.execute_batch("INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento) VALUES('SISTEMA','OT','OT_RECIBIDA','OK','2026-11-09 11:00:00');").unwrap();
    let anterior = estado.obtener("sesion-a", "PRODUCCION", &c.id).unwrap();
    assert_eq!(anterior.datos.filas.len(), 2);
    let nuevo = estado
        .publicar(
            "sesion-a",
            ver(&db, Reporte::R08, Filtros::default()),
            estado.iniciar_consulta(),
        )
        .unwrap();
    assert_eq!(nuevo.datos.filas.len(), 3);
    assert!(estado.obtener("sesion-a", "PRODUCCION", &c.id).is_err());
    assert!(estado.obtener("sesion-b", "PRODUCCION", &nuevo.id).is_err());
}

#[test]
fn exportacion_usa_todas_las_filas_y_no_publica_antes_de_confirmar() {
    let db = db();
    for i in 1..=27 {
        db.execute(
            "INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento,motivo)
            VALUES('SISTEMA','OT',?1,'OK','2026-11-09 12:00:00','=1+1')",
            [format!("EVENTO_{i:02}")],
        )
        .unwrap();
    }
    let c = ver(&db, Reporte::R08, Filtros::default());
    let p: Value = serde_json::to_value(c.pagina(1).unwrap()).unwrap();
    assert_eq!(p["filas"].as_array().unwrap().len(), 20);
    let dir = tempfile::tempdir().unwrap();
    for formato in [exportar::Formato::Pdf, exportar::Formato::Xlsx] {
        let destino = dir.path().join(formato.nombre(&c));
        let temporal = exportar::preparar(&c, formato, &destino).unwrap();
        assert!(!destino.exists());
        exportar::publicar(temporal, &destino).unwrap();
        let bytes = std::fs::read(&destino).unwrap();
        if formato.extension() == "pdf" {
            assert!(bytes.starts_with(b"%PDF"));
            let documento = lopdf::Document::load_mem(&bytes).unwrap();
            let paginas: Vec<_> = documento.get_pages().into_keys().collect();
            assert!(paginas.len() >= 2);
            let texto = documento.extract_text(&paginas).unwrap();
            for accion in ["EVENTO_01", "EVENTO_27"] {
                assert!(texto.contains(accion), "{texto}");
            }
            for pagina in paginas {
                assert!(documento
                    .extract_text(&[pagina])
                    .unwrap()
                    .contains("Fecha y hora"));
            }
        } else {
            assert!(bytes.starts_with(b"PK"));
        }
        if let Ok(carpeta) = std::env::var("REPORTES_MUESTRAS") {
            std::fs::copy(
                &destino,
                std::path::Path::new(&carpeta).join(destino.file_name().unwrap()),
            )
            .unwrap();
        }
    }
}

#[test]
#[ignore = "100.000 eventos en SQLite temporal en disco y exportación de 10.000 filas"]
fn rendimiento_reportes_auditoria_y_exportacion() {
    use std::time::{Duration, Instant};
    let dir = tempfile::tempdir().unwrap();
    let db = Connection::open(dir.path().join("reportes.sqlite")).unwrap();
    db.execute_batch(include_str!(
        "../../../migrations/0001_initial_schema_v7.sql"
    ))
    .unwrap();
    db.execute_batch(
        "WITH RECURSIVE serie(i) AS (
        SELECT 1 UNION ALL SELECT i+1 FROM serie WHERE i<100000
      ) INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento)
      SELECT 'SISTEMA',CASE WHEN i<=10000 THEN 'REPORTES' ELSE 'OT' END,
        'EVENTO_PRUEBA','OK','2026-11-09 12:00:00' FROM serie;",
    )
    .unwrap();
    let mut consultas_aptas = 0;
    let mut exportaciones_aptas = [0, 0];
    for i in 1..=20 {
        let desde = Instant::now();
        let c = ver(&db, Reporte::R08, Filtros::default());
        let p = serde_json::to_vec(&c.pagina(1).unwrap()).unwrap();
        let el = desde.elapsed();
        consultas_aptas += usize::from(el <= Duration::from_secs(3));
        assert_eq!(c.datos.filas.len(), 100000);
        assert!(!p.is_empty());
        println!("Consulta R08 {i}: {el:?}");
    }
    let corte = ver(
        &db,
        Reporte::R08,
        Filtros {
            modulo: Some("REPORTES".into()),
            ..Default::default()
        },
    );
    assert_eq!(corte.datos.filas.len(), 10000);
    for i in 1..=5 {
        for (j, formato) in [exportar::Formato::Pdf, exportar::Formato::Xlsx]
            .into_iter()
            .enumerate()
        {
            let destino = dir.path().join(formato.nombre(&corte));
            let desde = Instant::now();
            exportar::publicar(
                exportar::preparar(&corte, formato, &destino).unwrap(),
                &destino,
            )
            .unwrap();
            let el = desde.elapsed();
            exportaciones_aptas[j] += usize::from(el <= Duration::from_secs(30));
            println!("Exportación {} {i}: {el:?}", formato.extension());
        }
    }
    assert!(
        consultas_aptas >= 19,
        "{consultas_aptas}/20 consultas cumplieron"
    );
    assert!(
        exportaciones_aptas.iter().all(|n| *n >= 4),
        "{exportaciones_aptas:?} de 5 exportaciones cumplieron"
    );
}

#[test]
fn autorizacion_rust_exige_rol_y_sesion_normales() {
    use crate::services::session_manager::SessionState;
    let db = db();
    let mut sesion = SessionState::new();
    assert!(matches!(
        autorizar(&db, &mut sesion, true),
        Err(AppError::NoAutenticado)
    ));
    for (i, rol) in ["ADMINISTRADOR", "CAJERO", "MECANICO"]
        .into_iter()
        .enumerate()
    {
        let id = i as i64 + 1;
        let username = format!("user{id}");
        db.execute(
            "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol)
            VALUES(?1,'Usuario de prueba',?2,'$argon2id$prueba',?3)",
            (id, &username, rol),
        )
        .unwrap();
        let version = db
            .query_row(
                "SELECT version FROM usuarios WHERE id_usuario=?1",
                [id],
                |r| r.get(0),
            )
            .unwrap();
        sesion.abrir_sesion(id, version, username.clone(), rol.into(), false);
        if rol == "ADMINISTRADOR" {
            assert_eq!(autorizar(&db, &mut sesion, true).unwrap(), id);
            sesion.abrir_sesion(id, version, username, rol.into(), true);
            assert!(autorizar(&db, &mut sesion, true).is_err());
        } else {
            assert!(matches!(
                autorizar(&db, &mut sesion, true),
                Err(AppError::PermisoDenegado)
            ));
        }
    }
    let denegadas: i64 = db
        .query_row(
            "SELECT count(*) FROM auditoria WHERE modulo='REPORTES' AND resultado='DENEGADO'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(denegadas, 2);
}

#[test]
fn pdf_conserva_texto_unicode_al_extraerlo() {
    let db = db();
    let originales = [
        "Muñoz Łódź",
        "Смирнов",
        "Ελένη",
        "王小明",
        "Cafe\u{301} — ½",
        "Aceite 🚗",
        "Símbolos 𝄞 𝟙",
    ];
    for (i, nombre) in originales.iter().enumerate() {
        db.execute(
            "INSERT INTO productos(codigo,nombre,categoria,unidad_medida,precio_venta_cent)
             VALUES(?1,?2,'OTRO','UND',100)",
            (format!("PR-{i:02}"), nombre),
        )
        .unwrap();
    }
    let c = ver(&db, Reporte::R04, Filtros::default());
    let dir = tempfile::tempdir().unwrap();
    let ruta = dir.path().join("unicode.pdf");
    let archivo = exportar::preparar(&c, exportar::Formato::Pdf, &ruta).unwrap();
    if let Ok(carpeta) = std::env::var("REPORTES_MUESTRAS") {
        std::fs::copy(
            archivo.path(),
            std::path::Path::new(&carpeta).join("unicode.pdf"),
        )
        .unwrap();
    }
    let pdf = lopdf::Document::load(archivo.path()).unwrap();
    let paginas: Vec<_> = pdf.get_pages().into_keys().collect();
    let texto = pdf.extract_text(&paginas).unwrap();
    let texto = texto.split_whitespace().collect::<Vec<_>>().join(" ");
    for nombre in originales {
        assert!(texto.contains(nombre), "Falta {nombre:?} en {texto:?}");
    }
}

#[test]
fn stock_critico_e_inactivo_valoran_en_microsoles_antes_del_redondeo() {
    use crate::dto::producto_dto::CrearProductoDto;
    use crate::services::inventario_service::InventarioService;
    let mut db = db();
    db.execute(
        "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol)
        VALUES(1,'Admin prueba','admin','$argon2id$prueba','ADMINISTRADOR')",
        [],
    )
    .unwrap();
    for (codigo, cantidad, minimo) in [
        ("SIN-STOCK", 0, 2),
        ("EN-MINIMO", 1, 1),
        ("INACTIVO", 1, 1),
        ("SIN-VALOR", 0, 1),
    ] {
        InventarioService::registrar_producto(
            &mut db,
            CrearProductoDto {
                codigo: codigo.into(),
                nombre: format!("Producto {codigo}"),
                descripcion: None,
                categoria: if cantidad > 0 { "GRANEL" } else { "OTRO" }.into(),
                unidad_medida: if cantidad > 0 { "ML" } else { "UND" }.into(),
                stock_minimo: minimo,
                precio_venta_cent: 100,
                cantidad_inicial: (cantidad > 0).then_some(0.001),
                costo_unitario_inicial_micro: (cantidad > 0).then_some(4_990_000),
                referencia_inicial: None,
            },
            1,
        )
        .unwrap();
    }
    db.execute(
        "UPDATE productos SET activo=0,version=version+1 WHERE codigo IN ('INACTIVO','SIN-VALOR')",
        [],
    )
    .unwrap();
    let todos = ver(&db, Reporte::R04, Filtros::default());
    let filas = &todos.datos.filas;
    assert_eq!(
        filas
            .iter()
            .map(|r| r.celdas[0].texto())
            .collect::<Vec<_>>(),
        ["EN-MINIMO", "INACTIVO", "SIN-STOCK"]
    );
    assert_eq!(filas[0].celdas[7].texto(), "Sí");
    assert_eq!(filas[0].celdas[8].texto(), "0.000");
    assert_eq!(filas[0].celdas[9].texto(), "4.990000");
    assert_eq!(filas[0].celdas[10].texto(), "0.00");
    assert_eq!(filas[1].celdas[7].texto(), "No");
    assert_eq!(filas[2].celdas[8].texto(), "2");
    assert_eq!(filas[2].celdas[9].texto(), "No aplica");
    assert_eq!(todos.datos.totales[1].1.texto(), "0.01");
    let criticos = ver(
        &db,
        Reporte::R04,
        Filtros {
            solo_criticos: true,
            ..Default::default()
        },
    );
    assert_eq!(
        criticos
            .datos
            .filas
            .iter()
            .map(|r| r.celdas[0].texto())
            .collect::<Vec<_>>(),
        ["EN-MINIMO", "SIN-STOCK"]
    );
}

fn un_dia() -> Filtros {
    Filtros {
        periodo: Periodo::Personalizado,
        desde: Some("2026-11-09".into()),
        hasta: Some("2026-11-09".into()),
        ..Default::default()
    }
}

#[test]
fn cierre_nocturno_usa_fecha_de_cierre_y_conserva_aclaracion_al_corte() {
    use crate::dto::caja_dto::AclararCajaDto;
    use crate::services::caja_service::CajaService;
    let mut db = db();
    db.execute_batch("
        INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES
          (1,'Admin prueba','admin','$argon2id$prueba','ADMINISTRADOR');
        INSERT INTO caja_chica(id_caja,id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent,fecha_apertura)
          VALUES(1,'turno-nocturno','CAJA-01',1,'PRODUCCION',5000,'2026-11-08 23:59:59');
        UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-11-09 00:00:00',id_operacion_cierre='cierre-nocturno',
          id_usuario_cierre=1,tipo_cierre='NORMAL',motivo_cierre='Diferencia de prueba',efectivo_esperado_cent=5000,
          efectivo_contado_cent=4850,tarjeta_neto_cent=0,yape_neto_cent=0,plin_neto_cent=0,transferencia_neto_cent=0,
          fondo_siguiente_cent=1000,retiro_cierre_cent=3850 WHERE id_caja=1;
    ").unwrap();
    let c = ver(&db, Reporte::R05, un_dia());
    assert_eq!(c.datos.filas.len(), 1);
    let cifras = |c: &Corte| {
        c.datos.filas[0].celdas[6..]
            .iter()
            .map(|v| v.texto().to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(cifras(&c), ["50.00", "48.50", "-1.50", "10.00", "38.50"]);
    let detalle = |c: &Corte| {
        c.detalle(0)
            .unwrap()
            .filas
            .iter()
            .map(|r| {
                (
                    r.celdas[0].texto().to_owned(),
                    r.celdas[1].texto().to_owned(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    assert_eq!(detalle(&c)["Aclaración"], "No registrado");
    assert_eq!(detalle(&c)["Yape neto (S/)"], "0.00");
    assert!(!detalle(&c)
        .keys()
        .any(|k| k.contains("contado") || k.contains("diferencia")));
    CajaService::aclarar(
        &mut db,
        1,
        AclararCajaDto {
            id_operacion: uuid::Uuid::new_v4().to_string(),
            id_caja: 1,
            aclaracion: "Revisión posterior de prueba".into(),
        },
    )
    .unwrap();
    let posterior = ver(&db, Reporte::R05, un_dia());
    assert_eq!(detalle(&c)["Aclaración"], "No registrado");
    assert_eq!(
        detalle(&posterior)["Aclaración"],
        "Revisión posterior de prueba"
    );
    assert_eq!(cifras(&c), cifras(&posterior));
    let ayer = ver(
        &db,
        Reporte::R05,
        Filtros {
            desde: Some("2026-11-08".into()),
            hasta: Some("2026-11-08".into()),
            ..un_dia()
        },
    );
    assert!(ayer.datos.filas.is_empty());
}

#[test]
fn taller_distingue_ingreso_finalizacion_responsable_y_duracion() {
    let db = db();
    db.execute_batch(r#"
        INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES
          (1,'Admin prueba','admin','$argon2id$prueba','ADMINISTRADOR'),
          (2,'Mecánico prueba','mecanico','$argon2id$prueba','MECANICO');
        INSERT INTO clientes(id_cliente,tipo_documento,numero_documento,nombre_razon_social)
          VALUES(2,'DNI','12345678','Cliente prueba');
        INSERT INTO vehiculos(id_vehiculo,placa,id_cliente,marca,modelo)
          VALUES(1,'ABC123',2,'Toyota','Yaris'),(2,'DEF456',2,'Toyota','Yaris');
        INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent,es_cambio_aceite)
          VALUES(1,'SERV-TEST','Revisión',1000,0);
        INSERT INTO ordenes_trabajo(id_ot,id_operacion,codigo_ot,id_vehiculo,id_cliente_atendido,placa_snapshot,
          cliente_snapshot,id_creador,id_mecanico,kilometraje_ingreso,fecha_ingreso)
          VALUES(1,'ot-antigua','OT-ANTERIOR',1,2,'ABC123','{"nombre":"Cliente prueba"}',1,2,100,'2026-11-08 23:00:00'),
                (2,'ot-nueva','OT-RECIBIDA',2,2,'DEF456','{"nombre":"Cliente prueba"}',1,2,100,'2026-11-09 00:00:00');
        UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=1,fecha_inicio='2026-11-08 23:55:00' WHERE id_ot=1;
        INSERT INTO detalle_ot_servicios(id_operacion,id_ot,id_servicio,codigo_snapshot,nombre_snapshot,
          cantidad,precio_unitario_cent,es_cambio_aceite,id_usuario)
          VALUES('servicio-ot',1,1,'SERV-TEST','Revisión',1,1000,0,2);
        UPDATE ordenes_trabajo SET estado='FINALIZADO',id_mecanico_terminal=2,kilometraje_salida=100,
          resumen_publico='Revisión finalizada',fecha_fin='2026-11-09 00:25:00' WHERE id_ot=1;
    "#).unwrap();
    let fin = ver(
        &db,
        Reporte::R06,
        Filtros {
            usuario: Some(2),
            ..un_dia()
        },
    );
    assert_eq!(fin.datos.filas.len(), 1);
    let f = &fin.datos.filas[0].celdas;
    assert_eq!(f[0].texto(), "OT-ANTERIOR");
    assert_eq!(f[4].texto(), "Mecánico prueba");
    assert_eq!(f[5].texto(), "Zanja 1");
    assert_eq!(f[9].texto(), "0:30:00");
    let recibidas = ver(
        &db,
        Reporte::R06,
        Filtros {
            actividad: Some("RECIBIDAS".into()),
            ..un_dia()
        },
    );
    assert_eq!(recibidas.datos.filas.len(), 1);
    assert_eq!(recibidas.datos.filas[0].celdas[0].texto(), "OT-RECIBIDA");
    assert!(!recibidas
        .datos
        .columnas
        .iter()
        .any(|c| c.contains("Duración")));
}

#[test]
fn duracion_sin_inicio_valido_o_con_fechas_invertidas_no_aplica() {
    let inicio = "2026-11-09 09:00:00";
    let fin = "2026-11-09 10:03:02";
    assert_eq!(
        operativos::duracion_ot(Some(inicio), Some(fin)).texto(),
        "1:03:02"
    );
    for (a, b) in [
        (None, Some(fin)),
        (Some("inválido"), Some(fin)),
        (Some(fin), Some(inicio)),
        (Some(inicio), None),
    ] {
        assert!(matches!(operativos::duracion_ot(a, b), Celda::NoAplica));
    }
}

#[test]
fn corte_multipagina_es_inmutable_y_su_exportacion_se_invalida_al_cerrar() {
    let db = db();
    for i in 1..=25 {
        db.execute(
            "INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento,motivo)
            VALUES('SISTEMA','OT',?1,'OK','2026-11-09 12:00:00','Motivo original')",
            [format!("ORIGINAL-{i:02}")],
        )
        .unwrap();
    }
    let estado = EstadoReportes::default();
    let c = estado
        .publicar(
            "sesion-a",
            ver(&db, Reporte::R08, un_dia()),
            estado.iniciar_consulta(),
        )
        .unwrap();
    let antes = serde_json::to_value(c.pagina(2).unwrap()).unwrap();
    let detalle = serde_json::to_value(c.detalle(23).unwrap()).unwrap();
    db.execute(
        "INSERT INTO auditoria(actor,modulo,accion,resultado,fecha_evento)
        VALUES('SISTEMA','OT','EVENTO_POSTERIOR','OK','2026-11-09 12:30:00')",
        [],
    )
    .unwrap();
    let vigente = estado.obtener("sesion-a", "PRODUCCION", &c.id).unwrap();
    assert_eq!(
        serde_json::to_value(vigente.pagina(2).unwrap()).unwrap(),
        antes
    );
    assert_eq!(
        serde_json::to_value(vigente.detalle(23).unwrap()).unwrap(),
        detalle
    );
    let dir = tempfile::tempdir().unwrap();
    let ruta = dir.path().join("corte.pdf");
    let preparado = exportar::preparar(&vigente, exportar::Formato::Pdf, &ruta).unwrap();
    let doc = lopdf::Document::load(preparado.path()).unwrap();
    let texto = doc
        .extract_text(&doc.get_pages().into_keys().collect::<Vec<_>>())
        .unwrap();
    assert!(texto.contains("ORIGINAL-01"));
    assert!(!texto.contains("EVENTO_POSTERIOR"));
    assert_eq!(ver(&db, Reporte::R08, un_dia()).datos.filas.len(), 26);
    assert_eq!(
        db.query_row("SELECT count(*) FROM auditoria", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        26
    );
    // El comando exige nuevamente este mismo corte antes de publicar el temporal.
    estado.limpiar().unwrap();
    assert!(estado.obtener("sesion-a", "PRODUCCION", &c.id).is_err());
    drop(preparado);
    assert!(!ruta.exists());
    assert_eq!(dir.path().read_dir().unwrap().count(), 0);
    let nuevo = estado
        .publicar(
            "sesion-b",
            ver(&db, Reporte::R08, un_dia()),
            estado.iniciar_consulta(),
        )
        .unwrap();
    assert!(estado.obtener("sesion-b", "PRUEBA", &nuevo.id).is_err());
}

#[test]
fn reemplazo_fallido_no_publica_archivo_incompleto_y_limpia_el_temporal() {
    let c = ver(&db(), Reporte::R08, Filtros::default());
    let dir = tempfile::tempdir().unwrap();
    let destino = dir.path().join("reporte.pdf");
    std::fs::write(&destino, b"anterior").unwrap();
    let nuevo = exportar::preparar(&c, exportar::Formato::Pdf, &destino).unwrap();
    assert_eq!(std::fs::read(&destino).unwrap(), b"anterior");
    exportar::publicar(nuevo, &destino).unwrap();
    assert!(std::fs::read(&destino).unwrap().starts_with(b"%PDF"));
    let bloqueado = dir.path().join("carpeta.pdf");
    std::fs::create_dir(&bloqueado).unwrap();
    let contenido = bloqueado.join("anterior.txt");
    std::fs::write(&contenido, b"contenido previo").unwrap();
    let temp = exportar::preparar(&c, exportar::Formato::Pdf, &bloqueado).unwrap();
    let ruta_temporal = temp.path().to_owned();
    assert!(exportar::publicar(temp, &bloqueado).is_err());
    assert_eq!(std::fs::read(&contenido).unwrap(), b"contenido previo");
    assert!(!ruta_temporal.exists());
    let inexistente = dir.path().join("inexistente").join("reporte.pdf");
    assert!(exportar::preparar(&c, exportar::Formato::Pdf, &inexistente).is_err());
    assert!(!inexistente.exists());
}

use super::*;
use serde_json::{json, to_value, Value};

fn base() -> Connection {
    preparar(Connection::open_in_memory().unwrap())
}

fn preparar(db: Connection) -> Connection {
    db.execute_batch(include_str!(
        "../../../migrations/0001_initial_schema_v7.sql"
    ))
    .unwrap();
    db.execute_batch(
        "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES
         (1,'Admin de prueba','admin','$argon2id$prueba','ADMINISTRADOR'),
         (2,'Cajero propio','cajero','$argon2id$prueba','CAJERO'),
         (3,'Mecánico propio','mecanico','$argon2id$prueba','MECANICO'),
         (4,'Cajero ajeno','otro','$argon2id$prueba','CAJERO'),
         (5,'Mecánico ajeno','mecanico2','$argon2id$prueba','MECANICO');
         INSERT INTO clientes(id_cliente,tipo_documento,numero_documento,nombre_razon_social)
         VALUES(2,'DNI','12345678','Cliente privado');
         INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent,es_cambio_aceite)
         VALUES(1,'SRV-INICIO','Revisión',1000,0);",
    )
    .unwrap();
    db
}

fn fecha(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn ver(db: &Connection, usuario: i64, corte: &str) -> Value {
    to_value(DashboardService::obtener_en(db, usuario, fecha(corte)).unwrap()).unwrap()
}

fn crear_ot(db: &Connection, id: i64, mec: i64) {
    let placa = format!("ABC{id:03}");
    db.execute(
        "INSERT INTO vehiculos(id_vehiculo,placa,id_cliente,marca,modelo)
         VALUES(?1,?2,2,'Toyota','Yaris')",
        params![id, placa],
    )
    .unwrap();
    db.execute(
        "INSERT INTO ordenes_trabajo(id_ot,id_operacion,codigo_ot,id_vehiculo,
         id_cliente_atendido,placa_snapshot,cliente_snapshot,id_creador,id_mecanico,kilometraje_ingreso,fecha_ingreso)
         VALUES(?1,?2,?3,?1,2,?4,?6,1,?5,100,'2026-11-07 08:00:00')",
        params![id, format!("op-{id}"), format!("OT-{id}"), placa, mec, json!({"nombre":"Cliente privado"}).to_string()],
    )
    .unwrap();
}

fn iniciar_ot(db: &Connection, id: i64, inicio: &str) {
    db.execute(
        "UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=1,fecha_inicio=?2 WHERE id_ot=?1",
        params![id, inicio],
    )
    .unwrap();
}

fn finalizar_ot(db: &Connection, id: i64, mec: i64, inicio: &str, fin: &str) {
    crear_ot(db, id, mec);
    iniciar_ot(db, id, inicio);
    db.execute(
        "INSERT INTO detalle_ot_servicios(id_operacion,id_ot,id_servicio,codigo_snapshot,nombre_snapshot,
           cantidad,precio_unitario_cent,es_cambio_aceite,id_usuario)
         VALUES(?1,?2,1,'SRV-INICIO','Revisión',1,1000,0,?3)",
        params![format!("srv-{id}"), id, mec],
    )
    .unwrap();
    db.execute(
        "UPDATE ordenes_trabajo SET estado='FINALIZADO',id_mecanico_terminal=?2,kilometraje_salida=100,
           resumen_publico='Revisión finalizada',fecha_fin=?3 WHERE id_ot=?1",
        params![id, mec, fin],
    )
    .unwrap();
}

#[test]
fn resumen_vacio_y_contrato_por_rol_no_exponen_datos_ajenos() {
    let db = base();
    let corte = "2026-11-10 08:01:00";
    let admin = ver(&db, 1, corte);
    assert_eq!(admin["rol"], "ADMINISTRADOR");
    assert_eq!(admin["fecha"], "2026-11-10");
    assert_eq!(admin["actualizado_en"], corte);
    assert_eq!(admin["ambiente"], "PRODUCCION");
    assert_eq!(admin["ventas_netas_hoy_cent"], 0);
    assert_eq!(admin["mediana_cobro_seg"], Value::Null);
    assert_eq!(admin["ventas_por_dia"].as_array().unwrap().len(), 7);
    assert_eq!(admin["ventas_por_dia"][0]["periodo"], "2026-11-04");
    assert_eq!(admin["trabajos_por_hora"].as_array().unwrap().len(), 9);
    assert!(admin.get("turno").is_none());
    let mec = ver(&db, 3, corte);
    assert_eq!(mec["rol"], "MECANICO");
    assert!(mec.get("ventas_por_dia").is_none());
    assert!(mec.get("turno").is_none());
    let cajero = ver(&db, 2, corte);
    assert_eq!(cajero["turno"], json!({"estado":"SIN_CAJA"}));
    assert_eq!(cajero["actividad_turno"], Value::Null);
    assert_eq!(cajero["pendientes"]["total"], 0);
    assert!(cajero.get("ventas_netas_hoy_cent").is_none());
}

#[test]
fn jornada_mecanica_usa_responsable_y_mediana_y_excluye_futuro() {
    let db = base();
    finalizar_ot(&db, 1, 3, "2026-11-09 23:30:00", "2026-11-10 00:10:00");
    finalizar_ot(&db, 2, 3, "2026-11-10 08:00:00", "2026-11-10 09:00:00");
    finalizar_ot(&db, 3, 5, "2026-11-10 09:30:00", "2026-11-10 10:00:00");
    finalizar_ot(&db, 4, 3, "2026-11-10 12:00:00", "2026-11-10 13:00:00");
    crear_ot(&db, 5, 3);
    crear_ot(&db, 6, 3);
    iniciar_ot(&db, 6, "2026-11-10 10:00:00");
    let mec = ver(&db, 3, "2026-11-10 11:00:00");
    assert_eq!(
        mec["por_estado"],
        json!({"en_espera":1,"en_proceso":1,"finalizadas_hoy":2})
    );
    assert_eq!(mec["trabajos_abiertos"], 2);
    assert_eq!(mec["trabajos_por_dia"][6]["valor"], 2);
    assert_eq!(mec["mediana_trabajo_seg"], 3000);
    let admin = ver(&db, 1, "2026-11-10 11:00:00");
    assert_eq!(admin["trabajos_terminados_hoy"], 3);
    assert_eq!(admin["trabajos_por_hora"][0]["valor"], 1);
    assert_eq!(admin["trabajos_por_hora"][9]["valor"], 1);
    assert_eq!(admin["trabajos_por_hora"][10]["valor"], 1);
}

#[test]
fn espera_por_cobrar_respeta_29_30_60_61_minutos_y_protege_caja_ajena() {
    let db = base();
    for (id, fin) in [
        (1, "11:31:00"),
        (2, "11:30:00"),
        (3, "11:00:00"),
        (4, "10:59:00"),
    ] {
        finalizar_ot(
            &db,
            id,
            3,
            "2026-11-10 08:00:00",
            &format!("2026-11-10 {fin}"),
        );
    }
    db.execute(
        "INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent,fecha_apertura)
         VALUES('apertura-test','CAJA-PRIVADA',2,'PRODUCCION',4000,'2026-11-09 23:00:00')",
        [],
    ).unwrap();
    let propia = ver(&db, 2, "2026-11-10 12:00:00");
    assert_eq!(
        propia["turno"],
        json!({"estado":"PROPIO","apertura":"2026-11-09 23:00:00"})
    );
    assert_eq!(propia["actividad_turno"]["cobros"], 0);
    assert_eq!(
        propia["pendientes"],
        json!({"total":4,"mayor_espera_seg":3660,
        "menos_30":1,"entre_30_y_60":2,"mas_60":1})
    );
    let ajena = ver(&db, 4, "2026-11-10 12:00:00");
    assert_eq!(ajena["turno"], json!({"estado":"AJENO"}));
    assert_eq!(ajena["actividad_turno"], Value::Null);
    assert!(!ajena.to_string().contains("CAJA-PRIVADA"));
}

#[test]
fn ventana_economica_incluye_devolucion_negativa_y_corta_en_lima() {
    let db = base();
    // Se prueba la ventana con la forma de la vista R01; POS integra la vista real en pos_tests.
    db.execute_batch(
        "CREATE TEMP VIEW v_operaciones_comerciales AS
         SELECT 'PRODUCCION' ambiente, '2026-11-03 23:59:59' fecha_operacion, 9000 total_cent
         UNION ALL SELECT 'PRODUCCION','2026-11-04 00:00:00',10000
         UNION ALL SELECT 'PRODUCCION','2026-11-10 00:00:00',-3000
         UNION ALL SELECT 'PRODUCCION','2026-11-10 00:00:02',5000
         UNION ALL SELECT 'PRUEBA','2026-11-10 00:00:00',1200;",
    )
    .unwrap();
    let corte = "2026-11-10 00:00:01";
    let resumen = ver(&db, 1, corte);
    assert_eq!(resumen["ventas_por_dia"][0]["valor"], 10000);
    assert_eq!(resumen["ventas_por_dia"][1]["valor"], 0);
    assert_eq!(resumen["ventas_por_dia"][6]["valor"], -3000);
    assert_eq!(resumen["ventas_netas_hoy_cent"], -3000);
}

#[test]
fn rechaza_sesion_limitada_o_anterior_y_propaga_fallo_de_lectura() {
    let db = base();
    let mut sesion = SessionState::new();
    let limitada = sesion.abrir_sesion(1, 0, "admin".into(), "ADMINISTRADOR".into(), true);
    assert!(matches!(
        DashboardService::obtener_para_sesion(&db, &mut sesion, &limitada),
        Err(AppError::AccessDenied(_))
    ));
    let nueva = sesion.abrir_sesion(2, 0, "cajero".into(), "CAJERO".into(), false);
    assert!(matches!(
        DashboardService::obtener_para_sesion(&db, &mut sesion, &limitada),
        Err(AppError::NoAutenticado)
    ));
    assert_eq!(
        to_value(DashboardService::obtener_para_sesion(&db, &mut sesion, &nueva).unwrap()).unwrap()
            ["rol"],
        "CAJERO"
    );
    db.execute_batch("DROP VIEW v_operaciones_comerciales;")
        .unwrap();
    assert!(matches!(
        DashboardService::obtener_en(&db, 1, fecha("2026-11-10 08:01:00")),
        Err(AppError::Database(_))
    ));
}

fn abrir_caja(db: &Connection) {
    db.execute_batch(
        "INSERT INTO caja_chica(id_caja,id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent,fecha_apertura)
         VALUES(1,'caja-horas','CAJA-01',2,'PRODUCCION',4000,'2026-11-09 22:50:00');",
    ).unwrap();
}

fn confirmar_venta(db: &Connection, id: i64, fecha: &str, mixto: bool) {
    db.execute(
        "INSERT INTO ventas(id_venta,id_operacion,id_cliente,receptor_snapshot,id_emisor,id_caja,
          ambiente,tipo_original,bruto_cent,descuento_cent,base_cent,igv_cent,total_cent,fecha_venta)
         VALUES(?1,?2,1,'{}',2,1,'PRODUCCION','BOLETA',1000,0,847,153,1000,?3)",
        params![id, format!("venta-{id}"), fecha],
    ).unwrap();
    db.execute(
        "INSERT INTO detalle_venta(id_venta,id_servicio,codigo_snapshot,nombre_snapshot,unidad_medida,
          cantidad,precio_unitario_cent,bruto_cent,descuento_cent,base_cent,igv_cent,igv_bp,total_cent)
         VALUES(?1,1,'SRV-INICIO','Revisión','UND',1,1000,1000,0,847,153,1800,1000)",
        [id],
    ).unwrap();
    db.execute(
        "INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,
          signo,id_venta,recibido_cent,vuelto_cent,concepto,fecha_operacion)
         VALUES(?1,1,2,'COBRO','EFECTIVO',?2,1,?3,?2,0,'Cobro de prueba',?4)",
        params![
            format!("pago-{id}"),
            if mixto { 600 } else { 1000 },
            id,
            fecha
        ],
    )
    .unwrap();
    if mixto {
        db.execute(
            "INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,
              signo,id_venta,concepto,fecha_operacion)
             VALUES(?1,1,2,'COBRO','YAPE',400,1,?2,'Cobro de prueba',?3)",
            params![format!("mixto-{id}"), id, fecha],
        ).unwrap();
    }
    db.execute(
        "UPDATE ventas SET estado_registro='CONFIRMADA' WHERE id_venta=?1",
        [id],
    )
    .unwrap();
}

#[test]
fn turno_contabiliza_ventas_reales_por_hora_sin_duplicar_pagos_mixtos() {
    let db = base();
    abrir_caja(&db);
    confirmar_venta(&db, 1, "2026-11-09 23:59:59", true);
    confirmar_venta(&db, 2, "2026-11-10 00:00:00", false);
    confirmar_venta(&db, 3, "2026-11-10 00:10:00", false);
    let corte = "2026-11-10 00:00:01";
    let cajero = ver(&db, 2, corte);
    assert_eq!(
        cajero["actividad_turno"],
        json!({"cobros":2, "cobros_por_hora":[
            {"periodo":"2026-11-09 23","valor":1},{"periodo":"2026-11-10 00","valor":1}
        ]})
    );
    let admin = ver(&db, 1, corte);
    assert_eq!(admin["ventas_netas_hoy_cent"], 1000);
    assert_eq!(admin["ventas_por_dia"][5]["valor"], 1000);
}

#[test]
#[ignore = "Mide cinco lecturas con 1000 productos y 5000 ventas de prueba en SQLite en disco"]
fn rendimiento_dashboard_con_1000_productos_y_5000_ventas() {
    use std::time::{Duration, Instant};
    let temp = tempfile::tempdir().unwrap();
    let mut db = preparar(Connection::open(temp.path().join("inicio.db")).unwrap());
    abrir_caja(&db);
    let tx = db.transaction().unwrap();
    tx.execute_batch(
        "WITH RECURSIVE serie(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM serie WHERE n<1000)
         INSERT INTO productos(codigo,nombre,categoria,unidad_medida,precio_venta_cent)
         SELECT printf('VOL-%04d',n),printf('Producto %d',n),'OTRO','UND',1000 FROM serie;",
    )
    .unwrap();
    for i in 1..=5000 {
        confirmar_venta(&tx, i, "2026-11-10 09:00:00", i % 2 == 0);
    }
    tx.commit().unwrap();
    let corte = fecha("2026-11-10 10:00:00");
    for usuario in [1, 2, 3] {
        for i in 1..=5 {
            let inicio = Instant::now();
            let resumen = DashboardService::obtener_en(&db, usuario, corte).unwrap();
            let transcurrido = inicio.elapsed();
            println!("Inicio · usuario {usuario} · muestra {i}: {transcurrido:?}");
            assert!(transcurrido <= Duration::from_secs(3));
            match resumen.datos {
                DatosDashboard::Administrador {
                    ventas_netas_hoy_cent,
                    ..
                } => assert_eq!(ventas_netas_hoy_cent, 5_000_000),
                DatosDashboard::Cajero {
                    actividad_turno, ..
                } => assert_eq!(actividad_turno.unwrap().cobros, 5000),
                DatosDashboard::Mecanico {
                    trabajos_abiertos, ..
                } => assert_eq!(trabajos_abiertos, 0),
            }
        }
    }
}

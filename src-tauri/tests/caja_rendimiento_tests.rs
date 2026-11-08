use lubricentro_erp_lib::{
    dto::caja_dto::{CierreCajaDto, FiltroMovimientosCaja},
    services::caja_service::CajaService,
};
use rusqlite::{params, Connection};
use std::time::{Duration, Instant};

fn turno_fixture(conn: &Connection, turno: i64) {
    let id_op = format!("fixture-caja-{turno:08}");
    conn.prepare_cached("INSERT INTO caja_chica(id_caja,id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent)
        VALUES(?1,?2,?3,2,'PRODUCCION',10000)").unwrap()
        .execute(params![turno,id_op,format!("CAJA-PRUEBA-{turno:08}")]).unwrap();
    let mut ins = conn.prepare_cached("INSERT INTO movimientos_caja(id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto)
        VALUES(?1,?2,2,'INGRESO_MANUAL','EFECTIVO',100,1,'APORTE_FONDO','Aporte de prueba')").unwrap();
    for m in 1..=20 {
        ins.execute(params![
            format!("fixture-caja-mov-{turno:08}-{m:02}"),
            turno
        ])
        .unwrap();
    }
}
fn preparar(conn: &mut Connection) {
    conn.execute_batch("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL;")
        .unwrap();
    conn.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    conn.execute(
        "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol)
        VALUES(2,'Cajero rendimiento','cajero','$argon2id$prueba','CAJERO')",
        [],
    )
    .unwrap();
    let tx = conn.transaction().unwrap();
    for turno in 1..=5_000 {
        turno_fixture(&tx, turno);
        tx.prepare_cached("UPDATE caja_chica SET estado='CERRADA',fecha_cierre=datetime('now','-5 hours'),id_operacion_cierre=?1,
          id_usuario_cierre=2,tipo_cierre='NORMAL',efectivo_esperado_cent=12000,efectivo_contado_cent=12000,
          tarjeta_neto_cent=0,yape_neto_cent=0,plin_neto_cent=0,transferencia_neto_cent=0,fondo_siguiente_cent=10000,
          retiro_cierre_cent=2000,version=version+1 WHERE id_caja=?2").unwrap()
            .execute(params![format!("fixture-cierre-{turno:08}"),turno]).unwrap();
    }
    tx.commit().unwrap();
}

#[test]
#[ignore = "Genera 5.000 turnos y 100.000 movimientos en un archivo temporal; ejecutar explícitamente"]
fn veinte_mediciones_caja_en_sqlite_en_disco() {
    let dir = tempfile::tempdir().unwrap();
    let mut conn = Connection::open(dir.path().join("rendimiento_caja.db")).unwrap();
    preparar(&mut conn);
    let mut a_tiempo = [0; 3];
    for muestra in 1..=20 {
        let turno = 5_000 + muestra;
        let tx = conn.transaction().unwrap();
        turno_fixture(&tx, turno);
        tx.commit().unwrap();

        let t = Instant::now();
        let d = CajaService::inicio(&conn, 2).unwrap();
        let resumen = t.elapsed();
        assert_eq!(d["detalle"]["resumen"]["efectivo_esperado_cent"], 12_000);

        let t = Instant::now();
        let p = CajaService::movimientos(
            &conn,
            2,
            FiltroMovimientosCaja {
                id_caja: turno,
                pagina: Some(1),
                origen: None,
                direccion: None,
                buscar: None,
            },
        )
        .unwrap();
        let pagina = t.elapsed();
        assert_eq!(p["total"], 20);

        let t = Instant::now();
        let cerrado = CajaService::cerrar_caja(
            &mut conn,
            2,
            CierreCajaDto {
                id_operacion: format!("medicion-cierre-{turno:08}"),
                id_caja: turno,
                version_esperada: d["detalle"]["turno"]["version"].as_i64().unwrap(),
                efectivo_contado_cent: 12_000,
                fondo_siguiente_cent: 10_000,
                motivo: None,
            },
        )
        .unwrap();
        let cierre = t.elapsed();
        assert_eq!(cerrado["turno"]["retiro_cierre_cent"], 2_000);
        for (i, tiempo) in [resumen, pagina, cierre].into_iter().enumerate() {
            if tiempo <= Duration::from_secs(2) {
                a_tiempo[i] += 1;
            }
        }
        println!("Muestra {muestra:02}: resumen={resumen:?}; primera página={pagina:?}; cierre={cierre:?}");
    }
    assert!(
        a_tiempo.iter().all(|&n| n >= 19),
        "Muestras dentro de 2 s: {a_tiempo:?}"
    );
}

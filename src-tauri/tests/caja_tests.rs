use lubricentro_erp_lib::{
    db::init_db,
    dto::caja_dto::{
        AbrirCajaDto, AclararCajaDto, CierreCajaDto, FiltroHistorialCaja, FiltroMovimientosCaja,
        ManualCajaDto, RevertirCajaDto,
    },
    errors::AppError,
    services::{caja_service::CajaService, pos_service::consultas},
};
use rusqlite::Connection;
use serde_json::Value;

fn base() -> (tempfile::TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let conn = init_db(dir.path().into()).unwrap();
    conn.execute_batch(
        "INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES
        (1,'Administrador prueba','admin','$argon2id$prueba','ADMINISTRADOR'),
        (2,'Cajero titular','cajero','$argon2id$prueba','CAJERO'),
        (3,'Otro cajero','otro','$argon2id$prueba','CAJERO'),
        (4,'Mecánico prueba','mecanico','$argon2id$prueba','MECANICO');",
    )
    .unwrap();
    (dir, conn)
}

fn apertura(op: &str) -> AbrirCajaDto {
    AbrirCajaDto {
        id_operacion: op.into(),
        monto_apertura_cent: 10_000,
    }
}
fn id(v: &Value, clave: &str) -> i64 {
    v[clave].as_i64().unwrap()
}
fn escalar(conn: &Connection, sql: &str) -> i64 {
    conn.query_row(sql, [], |r| r.get(0)).unwrap()
}
fn cierre(op: &str, caja: i64, version: i64, contado: i64, motivo: Option<&str>) -> CierreCajaDto {
    CierreCajaDto {
        id_operacion: op.into(),
        id_caja: caja,
        version_esperada: version,
        efectivo_contado_cent: contado,
        fondo_siguiente_cent: 500,
        motivo: motivo.map(str::to_owned),
    }
}
fn movimiento(caja: i64, op: &str, monto: i64) -> ManualCajaDto {
    ManualCajaDto {
        id_operacion: op.into(),
        id_caja: caja,
        tipo: "EGRESO_MANUAL".into(),
        categoria: "GASTO_MENOR".into(),
        concepto: "Compra menor de taller".into(),
        monto_cent: monto,
    }
}

#[test]
fn apertura_idempotente_y_privacidad_en_caja_y_pos() {
    let (_dir, mut conn) = base();
    let inicial = CajaService::abrir_caja(&mut conn, 2, apertura("caja-apertura-test-01")).unwrap();
    assert_eq!(
        inicial,
        CajaService::abrir_caja(&mut conn, 2, apertura("caja-apertura-test-01")).unwrap()
    );
    assert!(CajaService::abrir_caja(&mut conn, 3, apertura("caja-apertura-test-02")).is_err());
    assert!(CajaService::abrir_caja(
        &mut conn,
        2,
        AbrirCajaDto {
            monto_apertura_cent: 20_000,
            ..apertura("caja-apertura-test-01")
        }
    )
    .is_err());
    assert_eq!(escalar(&conn, "SELECT count(*) FROM caja_chica"), 1);
    assert_eq!(
        escalar(&conn, "SELECT count(*) FROM auditoria WHERE modulo='CAJA'"),
        1
    );

    let otra = CajaService::inicio(&conn, 3).unwrap();
    assert_eq!(otra["ocupada"], true);
    assert!(otra["detalle"].is_null());
    assert_eq!(otra["puede_abrir"], false);
    let pos = consultas::inicio(&conn, 3).unwrap();
    assert_eq!(pos["caja_ocupada"], true);
    assert!(pos["caja"].is_null());
    assert!(pos["caja_propia"].is_null());
    let caja = id(&inicial, "id_caja");
    assert!(matches!(
        CajaService::detalle(&conn, 3, caja),
        Err(AppError::PermisoDenegado)
    ));
    assert!(matches!(
        CajaService::inicio(&conn, 4),
        Err(AppError::PermisoDenegado)
    ));
    assert!(matches!(
        CajaService::manual(&mut conn, 1, movimiento(caja, "caja-gasto-ajeno-01", 1)),
        Err(AppError::PermisoDenegado)
    ));
}

#[test]
fn arqueo_obsoleto_rollback_reintento_y_fondo_sugerido() {
    let (_dir, mut conn) = base();
    let caja = id(
        &CajaService::abrir_caja(&mut conn, 2, apertura("caja-apertura-test-01")).unwrap(),
        "id_caja",
    );
    assert!(CajaService::manual(
        &mut conn,
        2,
        movimiento(caja, "caja-gasto-excesivo-01", 10_001)
    )
    .is_err());
    CajaService::manual(
        &mut conn,
        2,
        movimiento(caja, "caja-gasto-correcto-01", 5_001),
    )
    .unwrap();
    let stale = CajaService::cerrar_caja(
        &mut conn,
        2,
        cierre("caja-cierre-correcto-01", caja, 0, 4_998, None),
    )
    .unwrap_err();
    assert!(stale.to_string().contains("RESUMEN_DESACTUALIZADO"));
    assert!(CajaService::cerrar_caja(
        &mut conn,
        2,
        cierre("caja-cierre-correcto-01", caja, 1, 4_998, None)
    )
    .is_err());
    let req = || {
        cierre(
            "caja-cierre-correcto-01",
            caja,
            1,
            4_998,
            Some("Faltante de un céntimo"),
        )
    };
    conn.execute_batch("CREATE TRIGGER forzar_error_caja BEFORE INSERT ON auditoria
      WHEN NEW.modulo='CAJA' AND NEW.accion='CERRAR' BEGIN SELECT RAISE(ABORT,'fallo simulado'); END;").unwrap();
    assert!(CajaService::cerrar_caja(&mut conn, 2, req()).is_err());
    assert_eq!(escalar(&conn, "SELECT version FROM caja_chica"), 1);
    assert_eq!(
        escalar(
            &conn,
            "SELECT count(*) FROM caja_chica WHERE estado='ABIERTA'"
        ),
        1
    );
    conn.execute_batch("DROP TRIGGER forzar_error_caja;")
        .unwrap();

    let finalizado = CajaService::cerrar_caja(&mut conn, 2, req()).unwrap();
    assert_eq!(
        finalizado,
        CajaService::cerrar_caja(&mut conn, 2, req()).unwrap()
    );
    assert_eq!(finalizado["turno"]["diferencia_efectivo_cent"], -1);
    assert_eq!(finalizado["turno"]["retiro_cierre_cent"], 4_498);
    assert_eq!(escalar(&conn, "SELECT count(*) FROM movimientos_caja"), 1);
    assert_eq!(
        CajaService::inicio(&conn, 3).unwrap()["fondo_sugerido_cent"],
        500
    );
    let listado = CajaService::historial(
        &conn,
        2,
        FiltroHistorialCaja {
            pagina: Some(1),
            desde: None,
            hasta: None,
            codigo: None,
            con_diferencia: Some(true),
            id_usuario: None,
            estado: None,
        },
    )
    .unwrap();
    assert_eq!(listado["total"], 1);
    assert_eq!(listado["datos"][0]["id_caja"], caja);
}

#[test]
fn reverso_exacto_filtrado_y_aclaracion_no_edita_cifras() {
    let (_dir, mut conn) = base();
    let caja = id(
        &CajaService::abrir_caja(&mut conn, 2, apertura("caja-apertura-test-01")).unwrap(),
        "id_caja",
    );
    let original = id(
        &CajaService::manual(
            &mut conn,
            2,
            movimiento(caja, "caja-gasto-original-01", 301),
        )
        .unwrap(),
        "id_movimiento",
    );
    let req = |op: &str| RevertirCajaDto {
        id_operacion: op.into(),
        id_movimiento: original,
        motivo: "Registro equivocado".into(),
    };
    let rev = CajaService::revertir(&mut conn, 2, req("caja-reverso-original-01")).unwrap();
    assert_eq!(
        rev,
        CajaService::revertir(&mut conn, 2, req("caja-reverso-original-01")).unwrap()
    );
    assert!(CajaService::revertir(&mut conn, 2, req("caja-reverso-original-02")).is_err());
    let salidas = CajaService::movimientos(
        &conn,
        2,
        FiltroMovimientosCaja {
            id_caja: caja,
            pagina: Some(1),
            origen: Some("MANUAL".into()),
            direccion: Some("EGRESO".into()),
            buscar: Some("taller".into()),
        },
    )
    .unwrap();
    assert_eq!(salidas["total"], 1);
    assert_eq!(salidas["datos"][0]["id_reverso"], rev["id_movimiento"]);
    assert!(CajaService::cerrar_caja(
        &mut conn,
        1,
        cierre("caja-cierre-ausencia-01", caja, 2, 10_000, None)
    )
    .is_err());
    let cerrado = CajaService::cerrar_caja(
        &mut conn,
        1,
        cierre(
            "caja-cierre-ausencia-01",
            caja,
            2,
            10_000,
            Some("Titular ausente"),
        ),
    )
    .unwrap();
    assert_eq!(cerrado["turno"]["tipo_cierre"], "AUSENCIA");
    assert_eq!(cerrado["turno"]["id_usuario"], 2);
    let aclarado = CajaService::aclarar(
        &mut conn,
        1,
        AclararCajaDto {
            id_operacion: "caja-aclaracion-test-01".into(),
            id_caja: caja,
            aclaracion: "Acta de entrega verificada".into(),
        },
    )
    .unwrap();
    assert_eq!(
        aclarado["turno"]["efectivo_esperado_cent"],
        cerrado["turno"]["efectivo_esperado_cent"]
    );
    assert_eq!(
        aclarado["turno"]["retiro_cierre_cent"],
        cerrado["turno"]["retiro_cierre_cent"]
    );
    assert_eq!(
        aclarado["turno"]["aclaracion"],
        "Acta de entrega verificada"
    );
    assert_eq!(
        escalar(
            &conn,
            "SELECT count(*) FROM auditoria WHERE modulo='CAJA' AND accion='ACLARAR'"
        ),
        1
    );
    assert!(CajaService::revertir(&mut conn, 2, req("caja-reverso-cerrado-01")).is_err());
}

#[test]
fn turno_recuperado_se_cierra_con_motivo_y_solo_por_admin() {
    let (_dir, mut conn) = base();
    let caja = id(
        &CajaService::abrir_caja(&mut conn, 2, apertura("caja-apertura-test-01")).unwrap(),
        "id_caja",
    );
    conn.execute_batch(
        "UPDATE caja_chica SET por_conciliar=1,version=version+1;
        UPDATE configuracion SET recuperacion_pendiente=1 WHERE id_config=1;",
    )
    .unwrap();
    assert!(CajaService::inicio(&conn, 2).unwrap()["detalle"].is_null());
    assert_eq!(
        CajaService::inicio(&conn, 1).unwrap()["detalle"]["modo_cierre"],
        "RECUPERACION"
    );
    let req = || {
        cierre(
            "caja-cierre-recuperado-01",
            caja,
            1,
            10_000,
            Some("Arqueo de caja restaurada"),
        )
    };
    assert!(matches!(
        CajaService::cerrar_caja(&mut conn, 2, req()),
        Err(AppError::PermisoDenegado)
    ));
    let resultado = CajaService::cerrar_caja(&mut conn, 1, req()).unwrap();
    assert_eq!(resultado["turno"]["tipo_cierre"], "RECUPERACION");
    assert_eq!(
        escalar(&conn, "SELECT recuperacion_pendiente FROM configuracion"),
        1
    );
}

use lubricentro_erp_lib::services::{
    admin_service::{self as admin, ambiente, facturacion as fact, usuarios},
    auth_service::AuthService,
    session_manager::SessionState,
};
use rusqlite::Connection;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

fn base() -> Connection {
    let c = Connection::open_in_memory().unwrap();
    c.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Admin Demo','admin','$argon2id$demo','ADMINISTRADOR')",[]).unwrap();
    c
}

#[test]
fn rol_y_sesion_requeridos_aunque_se_invoque_directamente() {
    let c = base();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Cajero Demo','demo','$argon2id$demo','CAJERO')",[]).unwrap();
    let mut s = SessionState::new();
    assert!(admin::autorizar(&c, &mut s).is_err());
    s.abrir_sesion(2, 0, "demo".into(), "CAJERO".into(), false);
    assert!(admin::autorizar(&c, &mut s).is_err());
    s.abrir_sesion(1, 0, "admin".into(), "ADMINISTRADOR".into(), true);
    assert!(admin::autorizar(&c, &mut s).is_err());
    s.abrir_sesion(1, 0, "admin".into(), "ADMINISTRADOR".into(), false);
    assert_eq!(admin::autorizar(&c, &mut s).unwrap(), 1);
}

#[test]
fn temporales_solo_se_entregan_una_vez_y_no_se_auditan() {
    let mut c = base();
    let alta = uuid::Uuid::new_v4().to_string();
    let res = usuarios::crear(&mut c, 1, &alta, "Demo", "Cajero Demo", "CAJERO").unwrap();
    let clave = res["temporal"].as_str().unwrap();
    assert_eq!(clave.len(), 12);
    let id = res["id"].as_i64().unwrap();
    let (hash, obligatoria): (String, bool) = c
        .query_row(
            "SELECT password_hash,debe_cambiar_clave FROM usuarios WHERE id_usuario=?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert!(obligatoria && AuthService::verify_password(clave, &hash));
    assert!(
        usuarios::crear(&mut c, 1, &alta, "demo", "Cajero Demo", "CAJERO").unwrap()["temporal"]
            .is_null()
    );
    assert!(usuarios::crear(
        &mut c,
        1,
        &uuid::Uuid::new_v4().to_string(),
        "demo",
        "Cajero Demo",
        "CAJERO"
    )
    .is_err());
    let reset = uuid::Uuid::new_v4().to_string();
    let nueva = usuarios::restablecer(&mut c, 1, id, 0, &reset).unwrap();
    let nueva = nueva["temporal"].as_str().unwrap();
    assert!(usuarios::restablecer(&mut c, 1, id, 0, &reset).unwrap()["temporal"].is_null());
    let logs:String=c.query_row("SELECT coalesce(group_concat(resultado_json),'')||(SELECT coalesce(group_concat(despues_json),'') FROM auditoria) FROM solicitudes_operacion",[],|r|r.get(0)).unwrap();
    assert!(!logs.contains(clave) && !logs.contains(nueva) && !logs.contains("$argon2id$"));
}

#[test]
fn caja_abierta_impide_baja_y_reactivacion_conserva_la_clave() {
    let mut c = base();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Cajero Demo','demo','$argon2id$demo','CAJERO')",[]).unwrap();
    c.execute("INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent) VALUES('apertura-admin-tests','CAJA-TEST',2,'PRODUCCION',10000)",[]).unwrap();
    let cambiar = |id, version, activo| usuarios::Edicion {
        id,
        version,
        nombre: "Nombre Demo".into(),
        rol: "CAJERO".into(),
        activo,
        motivo: Some("Cambio de acceso".into()),
    };
    assert!(usuarios::editar(&mut c, 1, cambiar(2, 0, false))
        .unwrap_err()
        .to_string()
        .contains("CAJA-TEST"));
    assert!(usuarios::editar(&mut c, 1, cambiar(1, 0, false)).is_err());
    // Una cuenta ajena sin turno puede desactivarse y volver con el mismo hash.
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Otro Cajero','otro','$argon2id$otro','CAJERO')",[]).unwrap();
    usuarios::editar(&mut c, 1, cambiar(3, 0, false)).unwrap();
    usuarios::editar(&mut c, 1, cambiar(3, 1, true)).unwrap();
    assert_eq!(
        c.query_row(
            "SELECT password_hash FROM usuarios WHERE id_usuario=3",
            [],
            |r| r.get::<_, String>(0)
        )
        .unwrap(),
        "$argon2id$otro"
    );
}

#[derive(Default)]
struct Almacen {
    datos: RefCell<HashMap<(String, String), String>>,
    falla: Cell<bool>,
}
impl fact::Secretos for Almacen {
    fn existe(&self, a: &str, r: &str) -> bool {
        self.datos.borrow().contains_key(&(a.into(), r.into()))
    }
    fn guardar(&self, a: &str, r: &str, v: &str) -> admin::R<()> {
        if self.falla.get() {
            return Err(admin::error("No se pudo proteger el token."));
        }
        self.datos
            .borrow_mut()
            .insert((a.into(), r.into()), v.into());
        Ok(())
    }
    fn borrar(&self, a: &str, r: &str) {
        self.datos.borrow_mut().remove(&(a.into(), r.into()));
    }
}
#[test]
fn token_no_sale_en_la_consulta_y_el_fallo_conserva_la_credencial() {
    let mut c = base();
    let store = Almacen::default();
    let ruta = "https://api.nubefact.com/api/v1/demo";
    fact::guardar_token(
        &mut c,
        1,
        fact::ModoToken::Administracion(0),
        ruta,
        "token-ficticio-original",
        &store,
    )
    .unwrap();
    let vista = fact::configuracion(&c, &store).unwrap();
    assert_eq!(vista["token_configurado"], true);
    assert!(!vista.to_string().contains("token-ficticio"));
    fact::guardar_token(
        &mut c,
        1,
        fact::ModoToken::Administracion(1),
        ruta,
        "",
        &store,
    )
    .unwrap();
    store.falla.set(true);
    assert!(fact::guardar_token(
        &mut c,
        1,
        fact::ModoToken::Administracion(2),
        ruta,
        "reemplazo",
        &store
    )
    .is_err());
    assert_eq!(fact::configuracion(&c, &store).unwrap()["version"], 2);
    assert_eq!(
        store.datos.borrow().values().next().unwrap(),
        "token-ficticio-original"
    );
    assert!(fact::url("https://api.nubefact.com@ejemplo.invalid/v1").is_err());
}

#[test]
fn pos_reserva_el_siguiente_maximo_y_reemplazo_es_atomico() {
    let mut c = base();
    let a = fact::registrar(&mut c, 1, "BOLETA", "B001", 120, 135, Some("NF-120")).unwrap()["id"]
        .as_i64()
        .unwrap();
    fact::estado_serie(&mut c, 1, a, true, None).unwrap();
    let tx = c.transaction().unwrap();
    let ajustada = fact::ajustar(&tx, 1, a, 138, "NF-138", Some(135), None).unwrap();
    assert_eq!(ajustada["siguiente"], 139);
    assert_eq!(fact::reservar(&tx, "BOLETA").unwrap().2, 139);
    tx.commit().unwrap();
    let b = fact::registrar(&mut c, 1, "BOLETA", "B002", 0, 0, None).unwrap()["id"]
        .as_i64()
        .unwrap();
    assert!(fact::estado_serie(&mut c, 1, b, true, None).is_err());
    fact::estado_serie(&mut c, 1, b, true, Some(a)).unwrap();
    assert_eq!(fact::reservar(&c, "BOLETA").unwrap(), (b, "B002".into(), 1));
    assert_eq!(
        c.query_row(
            "SELECT correlativo_actual FROM series_comprobante WHERE id_serie=?1",
            [a],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        139
    );
}

#[test]
fn destino_invalido_no_cambia_preferencia_y_exito_separa_cuentas() {
    let mut c = base();
    let tmp = tempfile::tempdir().unwrap();
    let ruta = ambiente::directorio(tmp.path(), "PRUEBA").unwrap();
    std::fs::create_dir_all(&ruta).unwrap();
    std::fs::write(ruta.join("lubricentro_secure.db"), b"no es una base sqlite").unwrap();
    assert!(ambiente::preparar_cambio(&mut c, 1, tmp.path(), "PRUEBA").is_err());
    assert!(!tmp.path().join("ambiente-activo.txt").exists());
    std::fs::remove_file(ruta.join("lubricentro_secure.db")).unwrap();
    ambiente::preparar_cambio(&mut c, 1, tmp.path(), "PRUEBA").unwrap();
    assert_eq!(ambiente::preferido(tmp.path()).unwrap(), "PRUEBA");
    let destino = Connection::open(ruta.join("lubricentro_secure.db")).unwrap();
    assert_eq!(admin::ambiente(&destino).unwrap(), "PRUEBA");
    assert_eq!(
        destino
            .query_row("SELECT count(*) FROM usuarios", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        0
    );
    assert_eq!(admin::ambiente(&c).unwrap(), "PRODUCCION");
}

#[test]
fn zanja_ocupada_y_trabajos_en_cola_preservan_identidad() {
    let mut c = base();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Mecánico Demo','mecanico','$argon2id$demo','MECANICO')",[]).unwrap();
    c.execute_batch("
        INSERT INTO clientes(id_cliente,tipo_documento,numero_documento,nombre_razon_social) VALUES(2,'DNI','12345678','Cliente Demo');
        INSERT INTO vehiculos(placa,id_cliente,marca,modelo) VALUES ('AAA111',2,'Marca','Modelo'),('BBB222',2,'Marca','Modelo');
        INSERT INTO ordenes_trabajo(id_operacion,codigo_ot,id_vehiculo,id_cliente_atendido,placa_snapshot,cliente_snapshot,id_creador,id_mecanico,kilometraje_ingreso)
        VALUES ('operacion-ot-taller-001','OT-UNO',1,2,'AAA111','{}',1,2,0),('operacion-ot-taller-002','OT-DOS',2,2,'BBB222','{}',1,2,0);
        UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=2,fecha_inicio=fecha_ingreso WHERE id_ot=1;
    ").unwrap();
    assert!(admin::guardar_taller(&mut c, 1, 1, 2)
        .unwrap_err()
        .to_string()
        .contains("OT-UNO"));
    assert!(usuarios::editar(
        &mut c,
        1,
        usuarios::Edicion {
            id: 2,
            version: 0,
            nombre: "Mecánico Demo".into(),
            rol: "CAJERO".into(),
            activo: true,
            motivo: Some("Cambio de puesto".into())
        }
    )
    .unwrap_err()
    .to_string()
    .contains("OT-DOS"));
    admin::guardar_taller(&mut c, 1, 3, 2).unwrap();
    assert_eq!(
        c.query_row(
            "SELECT id_zanja_ejecucion FROM ordenes_trabajo WHERE id_ot=2",
            [],
            |r| r.get::<_, Option<i64>>(0)
        )
        .unwrap(),
        None
    );
    assert_eq!(
        c.query_row(
            "SELECT id_zanja_ejecucion FROM ordenes_trabajo WHERE id_ot=1",
            [],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        2
    );
}

#[test]
#[ignore = "Medición local: ejecutar en la PC de referencia"]
fn rendimiento_administracion_200_cuentas_20_series() {
    use std::time::{Duration, Instant};
    let mut c = base();
    c.execute_batch(
        "
        WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i+1 FROM n WHERE i<199)
        INSERT INTO usuarios(nombre_completo,username,password_hash,rol)
        SELECT 'Cajero de prueba',printf('cajero%03d',i),'$argon2id$demo','CAJERO' FROM n;
        WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i+1 FROM n WHERE i<20)
        INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie)
        SELECT 'PRODUCCION','BOLETA',printf('B%03d',i) FROM n;
    ",
    )
    .unwrap();
    let mut lecturas = 0;
    let mut guardados = 0;
    let limite = Duration::from_secs(2);
    for i in 0..20 {
        let inicio = Instant::now();
        assert_eq!(
            usuarios::listar(&c, 1, "", "", "", 1).unwrap()["usuarios"]
                .as_array()
                .unwrap()
                .len(),
            20
        );
        assert_eq!(fact::series(&c).unwrap().len(), 20);
        let lectura = inicio.elapsed();
        let inicio = Instant::now();
        usuarios::restablecer(&mut c, 1, i + 2, 0, &uuid::Uuid::new_v4().to_string()).unwrap();
        let guardado = inicio.elapsed();
        lecturas += if lectura <= limite { 1 } else { 0 };
        guardados += if guardado <= limite { 1 } else { 0 };
        println!(
            "{}: primera página + series {:.2?}; guardado con hash {:.2?}",
            i + 1,
            lectura,
            guardado
        );
    }
    assert!(
        lecturas >= 19 && guardados >= 19,
        "Lecturas {lecturas}/20; guardados {guardados}/20"
    );
}

fn datos_empresa(version: i64, ubigeo: &str) -> admin::Empresa {
    admin::Empresa {
        ruc: "20600695771".into(),
        razon_social: "Empresa de prueba".into(),
        nombre_comercial: "Lubricentro Prueba".into(),
        direccion: "Avenida Principal 123".into(),
        ubigeo: ubigeo.into(),
        telefono: String::new(),
        correo: String::new(),
        version,
        motivo: None,
    }
}

#[test]
fn empresa_admite_distritos_reales_y_rechaza_ubigeos_invalidos_sin_escribir() {
    let mut c = base();
    for (version, codigo) in [(0, "010101"), (1, "110201"), (2, "120307")] {
        let guardado = admin::guardar_empresa(&mut c, 1, datos_empresa(version, codigo)).unwrap();
        assert_eq!(guardado["ubigeo"], codigo);
    }
    let vigente = admin::empresa(&c).unwrap();
    for (codigo, mensaje) in [
        ("119999", "no corresponde a un distrito"),
        ("110200", "no corresponde a un distrito"),
        ("11020", "seis dígitos"),
        ("1102001", "seis dígitos"),
        ("11A201", "seis dígitos"),
        ("１１１１１１", "seis dígitos"),
    ] {
        let mut propuesta = datos_empresa(3, codigo);
        propuesta.nombre_comercial = "Este cambio no debe guardarse".into();
        let error = admin::guardar_empresa(&mut c, 1, propuesta).unwrap_err();
        assert!(error.to_string().contains(mensaje), "{codigo}: {error}");
        assert_eq!(admin::empresa(&c).unwrap(), vigente);
    }
    let auditorias: i64 = c
        .query_row(
            "SELECT count(*) FROM auditoria WHERE accion='EDITAR_EMPRESA'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(auditorias, 3);
}

#[test]
fn empresa_rechaza_version_obsoleta_y_revierte_si_falla_auditoria() {
    let mut c = base();
    let vigente = admin::guardar_empresa(&mut c, 1, datos_empresa(0, "110201")).unwrap();
    let obsoleto = admin::guardar_empresa(&mut c, 1, datos_empresa(0, "010101")).unwrap_err();
    assert!(obsoleto.to_string().contains("La empresa cambió"));
    assert_eq!(admin::empresa(&c).unwrap(), vigente);

    c.execute_batch(
        "CREATE TEMP TRIGGER bloquear_auditoria_empresa BEFORE INSERT ON auditoria
         WHEN NEW.accion='EDITAR_EMPRESA' BEGIN SELECT RAISE(ABORT, 'fallo de prueba'); END;",
    )
    .unwrap();
    assert!(admin::guardar_empresa(&mut c, 1, datos_empresa(1, "010101")).is_err());
    assert_eq!(admin::empresa(&c).unwrap(), vigente);
}

#[test]
fn edicion_revalida_version_y_dependencias_aparecidas_despues_de_abrir() {
    let mut c = base();
    c.execute(
        "INSERT INTO usuarios(nombre_completo,username,password_hash,rol)
         VALUES('Cajero Demo','demo','$argon2id$demo','CAJERO')",
        [],
    )
    .unwrap();
    let abierta = usuarios::detalle(&c, 2).unwrap();
    assert!(abierta["pendientes"]["caja"].is_null());
    let version = abierta["usuario"]["version"].as_i64().unwrap();
    let editar = |version, activo, nombre: &str| usuarios::Edicion {
        id: 2,
        version,
        nombre: nombre.into(),
        rol: "CAJERO".into(),
        activo,
        motivo: (!activo).then(|| "Desactivar cuenta".into()),
    };

    c.execute(
        "INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent)
         VALUES('apertura-tardia-admin','CAJA-TARDIA',2,'PRODUCCION',10000)",
        [],
    )
    .unwrap();
    let bloqueo = usuarios::editar(&mut c, 1, editar(version, false, "Cajero Demo")).unwrap_err();
    assert!(bloqueo.to_string().contains("CAJA-TARDIA"));
    assert_eq!(
        usuarios::detalle(&c, 2).unwrap()["usuario"],
        abierta["usuario"]
    );

    let vigente = usuarios::editar(&mut c, 1, editar(version, true, "Cambio válido")).unwrap();
    let obsoleto =
        usuarios::editar(&mut c, 1, editar(version, true, "Edición antigua")).unwrap_err();
    assert!(obsoleto.to_string().contains("La cuenta cambió"));
    assert_eq!(usuarios::detalle(&c, 2).unwrap()["usuario"], vigente);
}

#[test]
fn alta_revierte_cuenta_y_solicitud_si_falla_auditoria() {
    let mut c = base();
    c.execute_batch(
        "CREATE TEMP TRIGGER bloquear_auditoria_alta BEFORE INSERT ON auditoria
         WHEN NEW.accion='CREAR_CUENTA' BEGIN SELECT RAISE(ABORT, 'fallo de prueba'); END;",
    )
    .unwrap();
    let op = uuid::Uuid::new_v4().to_string();
    assert!(usuarios::crear(&mut c, 1, &op, "nuevo", "Cajero Nuevo", "CAJERO").is_err());
    let (cuentas, solicitudes): (i64, i64) = c
        .query_row(
            "SELECT (SELECT count(*) FROM usuarios WHERE username='nuevo'),
                    (SELECT count(*) FROM solicitudes_operacion)",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!((cuentas, solicitudes), (0, 0));
    c.execute_batch("DROP TRIGGER bloquear_auditoria_alta;")
        .unwrap();
    let creado = usuarios::crear(&mut c, 1, &op, "nuevo", "Cajero Nuevo", "CAJERO").unwrap();
    assert_eq!(creado["temporal"].as_str().unwrap().len(), 12);
}

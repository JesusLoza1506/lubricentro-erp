use super::*;

const FRASE: &str = "frase de prueba de respaldo";

#[test]
fn snapshot_wal_y_rechazo_de_frase_o_archivo_alterado() {
    let t = tempfile::tempdir().unwrap();
    let origen = t.path().join("lubricentro_secure.db");
    let c = crate::db::abrir_db(&origen, true).unwrap();
    c.execute(
        "UPDATE configuracion SET razon_social='Empresa de prueba' WHERE id_config=1",
        [],
    )
    .unwrap();
    let a = Actor {
        id: None,
        nombre: "Prueba".into(),
    };
    let b = BackupState::new(t.path()).unwrap();
    let (archivo, m, hash) = b
        .producir(&origen, manifiesto(&c, &a, "MANUAL").unwrap(), FRASE)
        .unwrap();
    assert_eq!(m.archivos[0].nombre, "base.sqlite");
    assert_eq!(paquete::huella(&archivo).unwrap(), hash);
    let dest = tempfile::tempdir().unwrap();
    assert!(paquete::abrir(&archivo, dest.path(), "una frase equivocada").is_err());
    assert!(!dest.path().join("base.sqlite").exists());
    paquete::abrir(&archivo, dest.path(), FRASE).unwrap();
    let snap = paquete::lectura(&dest.path().join("base.sqlite")).unwrap();
    let nombre: String = snap
        .query_row(
            "SELECT razon_social FROM configuracion WHERE id_config=1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(nombre, "Empresa de prueba");
    let mut datos = fs::read(&archivo).unwrap();
    *datos.last_mut().unwrap() ^= 1;
    let corrupta = t.path().join("alterada.lgbackup");
    fs::write(&corrupta, datos).unwrap();
    assert!(paquete::abrir(&corrupta, tempfile::tempdir().unwrap().path(), FRASE).is_err());
}

#[test]
fn seleccion_inicial_no_publica_y_restauracion_persiste_al_reabrir() {
    let t = tempfile::tempdir().unwrap();
    let origen = t.path().join("origen");
    let destino = t.path().join("destino");
    fs::create_dir_all(&origen).unwrap();
    fs::create_dir_all(&destino).unwrap();
    let original = origen.join("lubricentro_secure.db");
    let c = crate::db::abrir_db(&original, true).unwrap();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Admin de prueba','admin','$argon2id$prueba','ADMINISTRADOR')", []).unwrap();
    let a = Actor {
        id: None,
        nombre: "Prueba".into(),
    };
    let src = BackupState::new(&origen).unwrap();
    let (p, _, _) = src
        .producir(&original, manifiesto(&c, &a, "MANUAL").unwrap(), FRASE)
        .unwrap();
    let b = BackupState::new(&destino).unwrap();
    let anterior = ruta_activa(&destino).unwrap();
    let db = Mutex::new(crate::db::abrir_db(&anterior, true).unwrap());
    assert!(vacia(&db.lock().unwrap()).unwrap());
    let vista = b
        .comprobar(&db, &a, None, None, Some(p), Some(FRASE))
        .unwrap();
    assert_eq!(vista["compatible"], true);
    assert_eq!(ruta_activa(&destino).unwrap(), anterior);
    {
        let mut base = db.lock().unwrap();
        assert!(b
            .restaurar(
                &mut base,
                &a,
                Some("otra sesión"),
                vista["seleccion"].as_str().unwrap()
            )
            .is_err());
        assert_eq!(ruta_activa(&destino).unwrap(), anterior);
        b.restaurar(&mut base, &a, None, vista["seleccion"].as_str().unwrap())
            .unwrap();
        assert!(!vacia(&base).unwrap());
    }
    let activa = ruta_activa(&destino).unwrap();
    assert_ne!(activa, anterior);
    let reabierta = crate::db::abrir_db(&activa, false).unwrap();
    assert!(b.resumen(&reabierta).unwrap()["pendiente"]
        .as_bool()
        .unwrap());
    assert!(vacia(&paquete::lectura(&anterior).unwrap()).unwrap());
}

#[cfg(windows)]
#[test]
fn frase_protegida_por_dpapi() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("frase.dpapi");
    proteccion::guardar(&p, FRASE).unwrap();
    assert_ne!(fs::read(&p).unwrap(), FRASE.as_bytes());
    assert_eq!(proteccion::leer(&p).unwrap().as_str(), FRASE);
}

#[test]
fn historial_y_aviso_coinciden_si_se_retira_el_archivo() {
    let tmp = tempfile::tempdir().unwrap();
    let b = BackupState::new(tmp.path()).unwrap();
    let c = crate::db::abrir_db(&tmp.path().join("lubricentro_secure.db"), true).unwrap();
    let a = Actor {
        id: None,
        nombre: "Sistema".into(),
    };
    let m = manifiesto(&c, &a, "MANUAL").unwrap();
    let ruta = b.root.join("copias").join("prueba.lgbackup");
    fs::write(&ruta, b"archivo").unwrap();
    let key =
        BackupState::insertar(&c, &ruta, &m, &huella(&ruta).unwrap(), None, "MANUAL").unwrap();
    assert_eq!(
        b.estado(&c, None, 1).unwrap()["copias"][0]["estado"],
        "VALIDO"
    );
    assert!(!BackupState::aviso(&c).unwrap());
    fs::remove_file(&ruta).unwrap();
    let estado = b.estado(&c, None, 1).unwrap();
    assert_eq!(estado["copias"][0]["estado"], "AUSENTE");
    assert!(estado["copias"][0].get("rutaInterna").is_none());
    assert!(estado["ultima"].is_null());
    assert!(BackupState::aviso(&c).unwrap());
    let operador = Actor {
        id: Some(123),
        nombre: "Admin".into(),
    };
    assert!(b
        .comprobar(
            &Mutex::new(c),
            &operador,
            Some("sesion".into()),
            Some(key),
            None,
            Some(FRASE)
        )
        .unwrap_err()
        .to_string()
        .contains("Archivo ausente"));
}

#[test]
fn diaria_usa_limite_de_lima_y_espacio_rechaza_antes_de_escribir() {
    let a = chrono::DateTime::parse_from_rfc3339("2026-11-09T03:59:59Z")
        .unwrap()
        .with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
    let b = a + chrono::Duration::seconds(1);
    assert_eq!(dia_programado_en(a).to_string(), "2026-11-07");
    assert_eq!(dia_programado_en(b).to_string(), "2026-11-08");
    assert!(espacio::comprobar(1023, 1024).is_err());
    assert!(espacio::comprobar(1024, 1024).is_ok());
}

#[test]
fn no_recrea_base_de_una_instalacion_que_perdio_su_archivo() {
    let tmp = tempfile::tempdir().unwrap();
    let b = BackupState::new(tmp.path()).unwrap();
    assert!(ruta_activa(tmp.path()).is_ok());
    b.registrar_instalacion().unwrap();
    assert!(ruta_activa(tmp.path()).is_err());
    assert!(!tmp.path().join("lubricentro_secure.db").exists());
}

#[test]
fn reinicio_en_los_tres_limites_de_publicacion() {
    let t = tempfile::tempdir().unwrap();
    let src = BackupState::new(&t.path().join("origen")).unwrap();
    let original = t.path().join("origen/lubricentro_secure.db");
    let c = crate::db::abrir_db(&original, true).unwrap();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Admin de prueba','admin','$argon2id$prueba','ADMINISTRADOR')", []).unwrap();
    let a = Actor {
        id: None,
        nombre: "Prueba".into(),
    };
    let (p, _, _) = src
        .producir(&original, manifiesto(&c, &a, "MANUAL").unwrap(), FRASE)
        .unwrap();
    for fase in 1..=3 {
        let base = t.path().join(format!("destino-{fase}"));
        let b = BackupState::new(&base).unwrap();
        let anterior = ruta_activa(&base).unwrap();
        let db = Mutex::new(crate::db::abrir_db(&anterior, true).unwrap());
        let info = b
            .comprobar(&db, &a, None, None, Some(p.clone()), Some(FRASE))
            .unwrap();
        b.corte.store(fase, std::sync::atomic::Ordering::SeqCst);
        {
            let tarea = b.ocupacion().unwrap();
            tarea.proteger_cierre();
            assert!(b.cierre_bloqueado());
            assert!(b
                .restaurar(
                    &mut db.lock().unwrap(),
                    &a,
                    None,
                    info["seleccion"].as_str().unwrap()
                )
                .unwrap_err()
                .to_string()
                .contains("Interrupción simulada"));
        }
        assert!(!b.cierre_bloqueado());
        drop(db);
        drop(b);
        let reiniciado = BackupState::new(&base).unwrap();
        let activa = ruta_activa(&base).unwrap();
        assert_eq!(activa == anterior, fase < 3);
        let c = crate::db::abrir_db(&activa, false).unwrap();
        if fase == 3 {
            reiniciado.validar_generacion_activa(&c, &activa).unwrap();
            assert_eq!(reiniciado.resumen(&c).unwrap()["pendiente"], true);
        } else {
            assert!(vacia(&c).unwrap());
        }
        drop(c);
        if fase == 3 {
            fs::remove_file(reiniciado.root.join("activo.json")).unwrap();
            assert!(ruta_activa(&base).is_err());
        }
    }
}

#[test]
fn conciliacion_admite_limites_y_conserva_la_referencia_literal() {
    let tmp = tempfile::tempdir().unwrap();
    let b = BackupState::new(tmp.path()).unwrap();
    let mut c = crate::db::abrir_db(&tmp.path().join("lubricentro_secure.db"), true).unwrap();
    c.execute_batch("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Admin de prueba','admin','$argon2id$prueba','ADMINISTRADOR');
      UPDATE configuracion SET recuperacion_pendiente=1;
      INSERT INTO restauraciones(id_operacion,evento_externo_id,actor_snapshot,nombre_copia_snapshot,ambiente,sha256_copia,estado)
      VALUES('operacion','evento','prueba','copia',(SELECT ambiente_actual FROM configuracion),printf('%064d',0),'CONTROLES_PENDIENTES');").unwrap();
    let a = Actor {
        id: Some(1),
        nombre: "admin".into(),
    };
    for (i, (referencia, valida)) in [
        ("NF13".into(), false),
        ("NF135".into(), true),
        ("X".repeat(120), true),
        ("X".repeat(121), false),
    ]
    .into_iter()
    .enumerate()
    {
        let id = i as i64 + 1;
        c.execute("INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,correlativo_actual,mayor_previo,requiere_conciliacion,activa)
           VALUES((SELECT ambiente_actual FROM configuracion),?1,?2,120,135,1,1)",
           params![["BOLETA","FACTURA","NC_BOLETA","NC_FACTURA"][i],["B001","F001","B002","F002"][i]]).unwrap();
        assert_eq!(
            b.conciliar(&mut c, &a, id, 135, &referencia).is_ok(),
            valida
        );
        if valida {
            let (refe, correlativo, evidencia): (String,i64,String) = c.query_row(
              "SELECT s.referencia_verificacion,s.correlativo_actual,cs.evidencia FROM series_comprobante s JOIN conciliaciones_series cs USING(id_serie) WHERE id_serie=?1",
              [id], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?))).unwrap();
            assert_eq!(refe, referencia);
            assert_eq!(correlativo, 135);
            assert_eq!(
                serde_json::from_str::<Value>(&evidencia).unwrap()["referencia"],
                referencia
            );
        }
    }
}

#[test]
fn arranque_bloqueado_solo_recupera_previa_verificada_con_su_administrador() {
    let t = tempfile::tempdir().unwrap();
    let b = BackupState::new(t.path()).unwrap();
    let original = t.path().join("lubricentro_secure.db");
    let c = crate::db::abrir_db(&original, true).unwrap();
    c.execute("UPDATE configuracion SET ruc='20999999990'", [])
        .unwrap();
    let hash = crate::services::auth_service::AuthService::hash_password("clave-prueba").unwrap();
    c.execute("INSERT INTO usuarios(nombre_completo,username,password_hash,rol) VALUES('Admin de prueba','admin',?1,'ADMINISTRADOR')", [hash]).unwrap();
    c.execute(
        "INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,correlativo_actual,activa)
        VALUES('PRODUCCION','BOLETA','B001',120,1)",
        [],
    )
    .unwrap();
    let actor = Actor {
        id: None,
        nombre: "Prueba".into(),
    };
    let (previa, m, h) = b
        .producir(&original, manifiesto(&c, &actor, "PREVIA").unwrap(), FRASE)
        .unwrap();
    let registro = json!({
        "generacion": id(), "anterior": original, "identidad": m.identidad, "fase": "ACTIVA_CON_CONTROLES",
        "fecha": ahora(), "ejecutor": "Prueba", "copia": "anterior.lgbackup",
        "previa": previa, "previa_sha": h.1, "maximos": { "BOLETA:B001": 135 }
    });
    reemplazar(
        &b.root.join("retorno.json"),
        &serde_json::to_vec(&registro).unwrap(),
    )
    .unwrap();
    fs::write(b.root.join("activo.json"), b"registro roto").unwrap();
    assert!(ruta_activa(t.path()).is_err());
    assert!(!b.previa_disponible("PRUEBA"));
    assert!(b.previa_disponible("PRODUCCION"));
    assert!(b
        .recuperar_previa("PRODUCCION", FRASE, "admin", "incorrecta")
        .is_err());
    assert!(ruta_activa(t.path()).is_err());

    let recuperada = b
        .recuperar_previa("PRODUCCION", FRASE, "admin", "clave-prueba")
        .unwrap();
    let ruta = ruta_activa(t.path()).unwrap();
    assert_ne!(ruta, original);
    b.validar_generacion_activa(&recuperada, &ruta).unwrap();
    let estado = b.resumen(&recuperada).unwrap();
    assert_eq!(estado["pendiente"], true);
    assert_eq!(estado["series"][0]["siguiente"], 136);
    assert_eq!(
        c.query_row(
            "SELECT correlativo_actual FROM series_comprobante WHERE serie='B001'",
            [],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        120
    );
}

#[test]
fn retencion_de_15_rutinarias_conserva_previa_y_ultima_valida() {
    let t = tempfile::tempdir().unwrap();
    let b = BackupState::new(t.path()).unwrap();
    let c = crate::db::abrir_db(&t.path().join("lubricentro_secure.db"), true).unwrap();
    let actor = Actor {
        id: None,
        nombre: "Prueba".into(),
    };
    let mut paths = Vec::new();
    for i in 0..=15 {
        let tipo = if i == 0 { "PREVIA" } else { "MANUAL" };
        let p = b.root.join("copias").join(format!("{i}.lgbackup"));
        fs::write(&p, b"copia").unwrap();
        let mut m = manifiesto(&c, &actor, tipo).unwrap();
        m.fecha = format!("2026-11-08 00:{i:02}:00");
        BackupState::insertar(&c, &p, &m, &huella(&p).unwrap(), None, tipo).unwrap();
        paths.push(p);
    }
    b.retencion(&c).unwrap();
    assert!(paths[0].is_file());
    assert!(!paths[1].exists());
    assert!(paths[2..].iter().all(|p| p.is_file()));
    assert_eq!(b.estado(&c, None, 1).unwrap()["total"], 15);
}

use lubricentro_erp_lib::{
    commands::auth_commands::iniciar_sesion_local,
    dto::usuario_dto::*,
    errors::AppError,
    services::{
        auth_service::AuthService,
        session_manager::{EstadoSesion, SessionState},
    },
};
use rusqlite::Connection;
use std::time::{Duration, Instant};

fn setup() -> (Connection, SessionState) {
    let mut c = Connection::open_in_memory().unwrap();
    c.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
    c.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    c.execute(
        "INSERT OR IGNORE INTO configuracion(id_config) VALUES(1)",
        [],
    )
    .unwrap();
    AuthService::instalacion_inicial(
        &mut c,
        InstalacionInicialRequestDto {
            nombre_completo: "Admin Prueba".into(),
            username: "admin".into(),
            password: "Original123".into(),
        },
    )
    .unwrap();
    let mut s = SessionState::new();
    iniciar_sesion_local(&c, &mut s, login()).unwrap();
    (c, s)
}
fn login() -> LoginRequestDto {
    LoginRequestDto {
        username: "admin".into(),
        password: "Original123".into(),
    }
}
fn fail_audit(c: &Connection) {
    c.execute_batch("CREATE TRIGGER audit_fault BEFORE INSERT ON auditoria BEGIN SELECT RAISE(ABORT,'audit fault'); END;").unwrap();
}
fn credentials(c: &Connection) -> (String, i64, i64) {
    c.query_row(
        "SELECT password_hash,version,debe_cambiar_clave FROM usuarios WHERE username='admin'",
        [],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )
    .unwrap()
}
#[test]
fn serialized_contract_has_single_prefix() {
    assert_eq!(
        serde_json::to_value(AppError::NoAutenticado)
            .unwrap()
            .as_str()
            .unwrap(),
        "NO_AUTENTICADO: Debes iniciar sesión para realizar esta acción."
    );
    let e = serde_json::to_string(&AppError::Database(rusqlite::Error::InvalidColumnName(
        "private_column".into(),
    )))
    .unwrap();
    assert!(!e.contains("private_column"));
}
#[test]
fn fresh_interaction_after_14m30s_resets_inactivity() {
    let (c, mut s) = setup();
    let id = s.session_id.clone().unwrap();
    s.ultima_actividad = Instant::now() - Duration::from_secs(870);
    s.registrar_interaccion(&c, &id).unwrap();
    assert!(s.ultima_actividad.elapsed() < Duration::from_secs(1));
}
#[test]
fn expired_session_cannot_be_revived_and_audits_once() {
    let (c, mut s) = setup();
    let id = s.session_id.clone().unwrap();
    s.ultima_actividad = Instant::now() - Duration::from_secs(899);
    assert!(!s.verificar_inactividad_con_auditoria(Some(&c)));
    s.ultima_actividad = Instant::now() - Duration::from_secs(900);
    assert!(matches!(
        s.registrar_interaccion(&c, &id),
        Err(AppError::NoAutenticado)
    ));
    assert!(s.registrar_interaccion(&c, &id).is_err());
    assert_eq!(s.estado_sesion, EstadoSesion::SinSesion);
    let count: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM auditoria WHERE accion='EXPIRACION_INACTIVIDAD'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
}
#[test]
fn delayed_activity_does_not_renew_another_session() {
    let (c, mut s) = setup();
    let old = s.session_id.clone().unwrap();
    s.cerrar_sesion();
    iniciar_sesion_local(&c, &mut s, login()).unwrap();
    let before = Instant::now() - Duration::from_secs(100);
    s.ultima_actividad = before;
    assert!(s.registrar_interaccion(&c, &old).is_err());
    assert_eq!(s.ultima_actividad, before);
    assert_eq!(s.estado_sesion, EstadoSesion::Normal);
}
#[test]
fn revoked_account_cannot_renew_activity() {
    let (c, mut s) = setup();
    let id = s.session_id.clone().unwrap();
    c.execute("INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) SELECT 2,'Admin Respaldo','respaldo',password_hash,'ADMINISTRADOR' FROM usuarios WHERE id_usuario=1",[]).unwrap();
    c.execute("UPDATE usuarios SET activo=0 WHERE id_usuario=1", [])
        .unwrap();
    assert!(s.registrar_interaccion(&c, &id).is_err());
    assert_eq!(s.estado_sesion, EstadoSesion::SinSesion);
}
#[test]
fn missing_environment_leaves_no_session_or_login_success_and_retry_works() {
    let (c, mut s) = setup();
    s.cerrar_sesion();
    let before: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM auditoria WHERE accion='LOGIN_OK'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    c.execute(
        "ALTER TABLE configuracion RENAME COLUMN ambiente_actual TO ambiente_indisponible",
        [],
    )
    .unwrap();
    assert!(iniciar_sesion_local(&c, &mut s, login()).is_err());
    assert_eq!(s.estado_sesion, EstadoSesion::SinSesion);
    let count: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM auditoria WHERE accion='LOGIN_OK'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, before);
    c.execute(
        "ALTER TABLE configuracion RENAME COLUMN ambiente_indisponible TO ambiente_actual",
        [],
    )
    .unwrap();
    let dto = iniciar_sesion_local(&c, &mut s, login()).unwrap();
    assert_eq!(s.session_id.as_deref(), Some(dto.session_id.as_str()));
    assert!(matches!(
        iniciar_sesion_local(&c, &mut s, login()),
        Err(AppError::SessionActive)
    ));
}

// Origen ficticio exclusivo del test: fija la acción y deriva la referencia del ítem.
fn anular_item(
    c: &Connection,
    s: &mut SessionState,
    token: &str,
    item: i64,
) -> Result<(), AppError> {
    s.ejecutar_operacion_autorizada(c,token,"DESCUENTO_POS",&format!("ITEM-{item}"),|c,ejecutor,autorizante| {
        let updated=c.execute("UPDATE items_prueba SET anulado=1,ejecutor=?1,autorizante=?2 WHERE id=?3 AND anulado=0",rusqlite::params![ejecutor,autorizante,item])?;
        if updated!=1 { return Err(AppError::Validation("Ítem no disponible".into())); } Ok(())
    })
}
#[test]
fn fictional_origin_authenticates_preserves_actor_and_applies_one_effect() {
    let (c, mut s) = setup();
    let hash = AuthService::hash_password("Cajero123").unwrap();
    c.execute("INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol,version) VALUES(2,'Cajero Prueba','cajero',?1,'CAJERO',1)",[hash]).unwrap();
    s.cerrar_sesion();
    s.abrir_sesion(2, 1, "cajero".into(), "CAJERO".into(), false);
    c.execute_batch("CREATE TABLE items_prueba(id INTEGER PRIMARY KEY,anulado INTEGER NOT NULL DEFAULT 0,ejecutor INTEGER,autorizante INTEGER); INSERT INTO items_prueba(id) VALUES(1),(2);").unwrap();
    let identity = s.session_id.clone();
    let request = |password: &str| AutorizarDescuentoRequestDto {
        username_admin: "admin".into(),
        password_admin: password.into(),
        referencia_venta: "ITEM-1".into(),
        descuento_bp: 100,
    };
    for _ in 0..5 {
        assert!(AuthService::autorizar_descuento(&c, &mut s, request("Incorrecta123")).is_err());
    }
    assert!(AuthService::autorizar_descuento(&c, &mut s, request("Original123")).is_err());
    assert_eq!(s.session_id, identity);
    assert_eq!(s.usuario_id, Some(2));
    assert!(anular_item(&c, &mut s, "missing", 1).is_err());
    let count: i64 = c
        .query_row("SELECT SUM(anulado) FROM items_prueba", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    // La pausa de verificación no bloquea el ingreso del administrador.
    assert!(AuthService::login(&c, login()).is_ok());
    s.limpiar_fallos_verificacion_admin(1);
    let admin = AuthService::autorizar_descuento(&c, &mut s, request("Original123")).unwrap();
    s.guardar_autorizacion(
        "token".into(),
        admin,
        2,
        "DESCUENTO_POS".into(),
        "ITEM-1".into(),
    );
    assert!(anular_item(&c, &mut s, "token", 2).is_err());
    anular_item(&c, &mut s, "token", 1).unwrap();
    assert!(anular_item(&c, &mut s, "token", 1).is_err());
    let effect: (i64, i64, i64) = c
        .query_row(
            "SELECT anulado,ejecutor,autorizante FROM items_prueba WHERE id=1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!(effect, (1, 2, admin));
    assert_eq!(s.session_id, identity);
}
#[test]
fn changing_password_rolls_back_if_audit_fails() {
    for mandatory in [false, true] {
        let (mut c, mut s) = setup();
        if mandatory {
            c.execute("UPDATE usuarios SET debe_cambiar_clave=1", [])
                .unwrap();
            s.estado_sesion = EstadoSesion::CambioObligatorio;
        }
        let before = credentials(&c);
        let state = s.estado_sesion.clone();
        fail_audit(&c);
        let result = if mandatory {
            AuthService::cambiar_clave_obligatoria(
                &mut c,
                &mut s,
                1,
                CambiarClaveObligatoriaRequestDto {
                    nueva_password: "Different123".into(),
                },
            )
        } else {
            AuthService::cambiar_clave_voluntaria(
                &mut c,
                &mut s,
                1,
                CambiarClaveVoluntariaRequestDto {
                    password_actual: "Original123".into(),
                    nueva_password: "Different123".into(),
                },
            )
        };
        assert!(result.is_err());
        assert_eq!(credentials(&c), before);
        assert_eq!(s.estado_sesion, state);
        assert_eq!(s.usuario_version, Some(before.1));
    }
}
#[test]
fn unicode_and_user_format_are_consistent() {
    assert!(AuthService::normalizar_nombre_completo(&"Á".repeat(100)).is_ok());
    for name in ["abc def", "abc\tdef", "ábc", "abc!"] {
        assert!(AuthService::normalizar_username(name).is_err());
    }
    assert_eq!(
        AuthService::normalizar_username(" ADMIN._-1 ").unwrap(),
        "admin._-1"
    );
    assert!(AuthService::validar_politica_password("Abcdefg١", "admin").is_err());
    assert!(
        AuthService::validar_politica_password(&format!("A1{}", "😀".repeat(126)), "admin").is_ok()
    );
}
#[test]
fn recovery_rolls_back_password_version_and_code_if_audit_fails() {
    let (mut c, _s) = setup();
    let code = "23456789ABCDEFGHJKLMNPQRSTUVWXYZ23";
    let hash = AuthService::hash_password(code).unwrap();
    c.execute("UPDATE configuracion SET recuperacion_hash=?1", [&hash])
        .unwrap();
    let before = credentials(&c);
    fail_audit(&c);
    assert!(AuthService::recuperar_acceso_admin(
        &mut c,
        RecuperarAccesoAdminRequestDto {
            username: "admin".into(),
            codigo_recuperacion: code.into(),
            nueva_password: "Different123".into()
        }
    )
    .is_err());
    assert_eq!(credentials(&c), before);
    let after: String = c
        .query_row("SELECT recuperacion_hash FROM configuracion", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(after, hash);
}
#[test]
fn installation_returns_no_success_or_secrets_if_audit_fails() {
    let mut c = Connection::open_in_memory().unwrap();
    c.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    c.execute(
        "INSERT OR IGNORE INTO configuracion(id_config) VALUES(1)",
        [],
    )
    .unwrap();
    fail_audit(&c);
    assert!(AuthService::instalacion_inicial(
        &mut c,
        InstalacionInicialRequestDto {
            nombre_completo: "Admin Prueba".into(),
            username: "admin".into(),
            password: "Original123".into()
        }
    )
    .is_err());
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM usuarios", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    let hash: Option<String> = c
        .query_row("SELECT recuperacion_hash FROM configuracion", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(hash.is_none());
}
#[test]
fn installation_rolls_back_if_configuration_update_is_ignored() {
    let mut c = Connection::open_in_memory().unwrap();
    c.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    c.execute(
        "INSERT OR IGNORE INTO configuracion(id_config) VALUES(1)",
        [],
    )
    .unwrap();
    c.execute_batch("CREATE TRIGGER config_fault BEFORE UPDATE ON configuracion BEGIN SELECT RAISE(IGNORE); END;").unwrap();
    assert!(AuthService::instalacion_inicial(
        &mut c,
        InstalacionInicialRequestDto {
            nombre_completo: "Admin Prueba".into(),
            username: "admin".into(),
            password: "Original123".into()
        }
    )
    .is_err());
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM usuarios", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

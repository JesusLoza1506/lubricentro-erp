use rusqlite::Connection;
use std::time::{Duration, Instant};

use lubricentro_erp_lib::dto::usuario_dto::{
    AutorizarDescuentoRequestDto, CambiarClaveObligatoriaRequestDto,
    CambiarClaveVoluntariaRequestDto, InstalacionInicialRequestDto, LoginRequestDto,
    RecuperarAccesoAdminRequestDto,
};
use lubricentro_erp_lib::errors::AppError;
use lubricentro_erp_lib::services::auth_service::AuthService;
use lubricentro_erp_lib::services::session_manager::{
    EstadoSesion, IntentoVerificacionAdmin, SessionState,
};

/// Carga en tiempo de compilación el esquema SQL oficial V7 desde el archivo de migración físico
const INITIAL_SCHEMA_V7: &str = include_str!("../migrations/0001_initial_schema_v7.sql");

/// Inicializa una base de datos SQLite en memoria ejecutando directamente la migración oficial V7.0
fn setup_inmemory_db_v7() -> Connection {
    let conn = Connection::open_in_memory().unwrap();

    // Activar soporte de claves foráneas
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    // Ejecutar el script DDL de la migración oficial
    conn.execute_batch(INITIAL_SCHEMA_V7).unwrap();

    // Garantizar que la fila inicial de configuración esté presente para las pruebas
    let count_config: i64 = conn
        .query_row("SELECT COUNT(*) FROM configuracion", [], |r| r.get(0))
        .unwrap_or(0);

    if count_config == 0 {
        conn.execute("INSERT INTO configuracion (id_config) VALUES (1);", [])
            .unwrap();
    }

    conn
}

// =========================================================================
// BLOQUE 1: Argon2id y Hash Señuelo (TC-LOG-01 y TC-LOG-02)
// =========================================================================

#[test]
fn test_tc_log_01_argon2id_hashing_y_verificacion() {
    let raw_pass = "Admin12345";
    let hash = AuthService::hash_password(raw_pass).unwrap();

    assert!(hash.starts_with("$argon2id$"));
    assert!(AuthService::verify_password(raw_pass, &hash));
    assert!(!AuthService::verify_password("WrongPass123", &hash));
}

#[test]
fn test_tc_log_02_dummy_hash_timing_attack_protection() {
    let dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";
    let resultado = AuthService::verify_password("CualquierPassword", dummy_hash);
    assert!(!resultado);
}

// =========================================================================
// BLOQUE 2: Instalación Inicial y Atomicidad (RF-LOG-01 / RB06)
// =========================================================================

#[test]
fn test_tc_log_03_instalacion_inicial_primer_admin_y_auditoria_atomica() {
    let mut conn = setup_inmemory_db_v7();

    let res = AuthService::instalacion_inicial(
        &mut conn,
        InstalacionInicialRequestDto {
            nombre_completo: "Administrador Principal".to_string(),
            username: "admin".to_string(),
            password: "Admin2026Password".to_string(),
        },
    )
    .unwrap();

    assert_eq!(res.username, "admin");
    assert_eq!(res.codigo_recuperacion.len(), 39);

    let aud_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM auditoria WHERE accion = 'INSTALACION_INICIAL'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(aud_count, 1);

    let err_duplicado = AuthService::instalacion_inicial(
        &mut conn,
        InstalacionInicialRequestDto {
            nombre_completo: "Segundo Admin".to_string(),
            username: "admin2".to_string(),
            password: "Admin2026Password".to_string(),
        },
    );
    assert!(err_duplicado.is_err());
}

// =========================================================================
// BLOQUE 3: Login, Control de Versión y Reinicio de Bloqueos (RF-LOG-02 / RB02)
// =========================================================================

#[test]
fn test_tc_log_04_bloqueo_5_fallos_y_limpieza_al_ingresar() {
    let conn = setup_inmemory_db_v7();
    let hash = AuthService::hash_password("ClaveValida123").unwrap();

    conn.execute(
        "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo) VALUES ('Test User', 'testuser', ?1, 'CAJERO', 1)",
        [&hash],
    )
    .unwrap();

    for _ in 1..=5 {
        let _ = AuthService::login(
            &conn,
            LoginRequestDto {
                username: "testuser".to_string(),
                password: "ClaveErronea123".to_string(),
            },
        );
    }

    let (intentos, bloqueado_hasta): (i32, Option<String>) = conn
        .query_row(
            "SELECT intentos_fallidos, bloqueado_hasta FROM usuarios WHERE username = 'testuser'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();

    assert_eq!(intentos, 5);
    assert!(bloqueado_hasta.is_some());

    let err_bloqueo = AuthService::login(
        &conn,
        LoginRequestDto {
            username: "testuser".to_string(),
            password: "ClaveValida123".to_string(),
        },
    );
    assert!(err_bloqueo.is_err());
}

#[test]
fn test_tc_log_04_b_reinicio_contadores_post_expiracion_15_minutos() {
    let conn = setup_inmemory_db_v7();
    let hash = AuthService::hash_password("ClaveValida123").unwrap();

    conn.execute(
        "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo, intentos_fallidos, bloqueado_hasta) 
         VALUES ('Cajero Exp', 'cajeroexp', ?1, 'CAJERO', 1, 5, datetime('now', '-5 hours', '-16 minutes'))",
        [&hash],
    )
    .unwrap();

    let _ = AuthService::login(
        &conn,
        LoginRequestDto {
            username: "cajeroexp".to_string(),
            password: "ClaveWrong123".to_string(),
        },
    );

    let (intentos, bloqueado_hasta): (i32, Option<String>) = conn
        .query_row(
            "SELECT intentos_fallidos, bloqueado_hasta FROM usuarios WHERE username = 'cajeroexp'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();

    assert_eq!(intentos, 1);
    assert!(bloqueado_hasta.is_none());
}

// =========================================================================
// BLOQUE 4: Cambio Obligatorio y Revocación por Versión (RF-LOG-03 / RF-LOG-04)
// =========================================================================

#[test]
fn test_tc_log_05_cambio_obligatorio_y_voluntario_con_incremento_version() {
    let mut conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();

    let hash = AuthService::hash_password("Temporal123").unwrap();
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, debe_cambiar_clave, version) 
         VALUES (1, 'User Temp', 'usertemp', ?1, 'MECANICO', 1, 1)",
        [&hash],
    )
    .unwrap();

    session.abrir_sesion(1, 1, "usertemp".to_string(), "MECANICO".to_string(), true);
    assert_eq!(session.estado_sesion, EstadoSesion::CambioObligatorio);

    assert!(session.validar_sesion_operativa(&conn).is_err());

    AuthService::cambiar_clave_obligatoria(
        &mut conn,
        &mut session,
        1,
        CambiarClaveObligatoriaRequestDto {
            nueva_password: "NuevaClaveValida2026".to_string(),
        },
    )
    .unwrap();

    assert_eq!(session.estado_sesion, EstadoSesion::Normal);
    assert_eq!(session.validar_sesion_operativa(&conn).unwrap(), 1);

    let err_vol = AuthService::cambiar_clave_voluntaria(
        &mut conn,
        &mut session,
        1,
        CambiarClaveVoluntariaRequestDto {
            password_actual: "ClaveInvalida123".to_string(),
            nueva_password: "OtraClave2026".to_string(),
        },
    );
    assert!(err_vol.is_err());
}

#[test]
fn test_tc_log_05_b_rechazar_sesion_temporal_anterior_a_restablecimiento() {
    let mut conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();

    let hash = AuthService::hash_password("TempPassword123").unwrap();
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, debe_cambiar_clave, version) 
         VALUES (10, 'Temp User', 'tempuser', ?1, 'CAJERO', 1, 1)",
        [&hash],
    )
    .unwrap();

    session.abrir_sesion(10, 1, "tempuser".to_string(), "CAJERO".to_string(), true);

    // Se restablece la cuenta externamente incrementando 'version' a 2 en la BD
    conn.execute(
        "UPDATE usuarios SET version = version + 1 WHERE id_usuario = 10",
        [],
    )
    .unwrap();

    let res = AuthService::cambiar_clave_obligatoria(
        &mut conn,
        &mut session,
        10,
        CambiarClaveObligatoriaRequestDto {
            nueva_password: "NuevaPassword2026".to_string(),
        },
    );

    assert!(matches!(res, Err(AppError::NoAutenticado)));
    assert_eq!(session.estado_sesion, EstadoSesion::SinSesion);
}

// =========================================================================
// BLOQUE 5: Verificaciones Administrativas e Pausas (RB03 / RB08)
// =========================================================================

#[test]
fn test_tc_log_06_limite_independiente_verificaciones_admin() {
    let mut session = SessionState::new();
    let admin_id = 100;

    for _ in 1..=4 {
        assert!(!session.registrar_fallo_verificacion_admin(admin_id));
    }
    assert!(!session.esta_verificacion_admin_pausada(admin_id));

    assert!(session.registrar_fallo_verificacion_admin(admin_id));
    assert!(session.esta_verificacion_admin_pausada(admin_id));

    session.limpiar_fallos_verificacion_admin(admin_id);
    assert!(!session.esta_verificacion_admin_pausada(admin_id));
}

#[test]
fn test_tc_log_06_b_reiniciar_pausa_admin_al_vencer() {
    let mut session = SessionState::new();
    let admin_id = 101;

    session.verificaciones_admin_fallos.insert(
        admin_id,
        IntentoVerificacionAdmin {
            fallos: 5,
            pausado_hasta: Some(Instant::now() - Duration::from_secs(960)),
        },
    );

    assert!(!session.esta_verificacion_admin_pausada(admin_id));

    let entry = session.verificaciones_admin_fallos.get(&admin_id).unwrap();
    assert_eq!(entry.fallos, 0);
    assert!(entry.pausado_hasta.is_none());
}

// =========================================================================
// BLOQUE 6: Inactividad y Autorización Vincular en Origen (RF-LOG-06 / RB04 / Punto 10)
// =========================================================================

#[test]
fn test_tc_log_07_inactividad_15_minutos_y_vinculacion_origen_single_use() {
    let mut session = SessionState::new();
    let id_cajero = 10;
    let id_admin = 1;

    session.abrir_sesion(
        id_cajero,
        1,
        "cajero1".to_string(),
        "CAJERO".to_string(),
        false,
    );

    session.ultima_actividad = Instant::now() - Duration::from_secs(901);
    assert!(session.verificar_inactividad());
    assert_eq!(session.estado_sesion, EstadoSesion::SinSesion);

    session.abrir_sesion(
        id_cajero,
        1,
        "cajero1".to_string(),
        "CAJERO".to_string(),
        false,
    );
    let token = "desc_token_123".to_string();

    // Guardar token vinculando autorizante (1) con solicitante (id_cajero = 10)
    session.guardar_autorizacion(
        token.clone(),
        id_admin,
        id_cajero,
        "DESCUENTO_POS".to_string(),
        "REF-1001".to_string(),
    );

    // Intentar consumir desde otro usuario/solicitante debe ser rechazado
    let res_impostor = session.consumir_autorizacion(&token, 999, "DESCUENTO_POS", "REF-1001");
    assert!(res_impostor.is_err());

    // Consumo legítimo por parte del cajero titular
    let res_valido = session.consumir_autorizacion(&token, id_cajero, "DESCUENTO_POS", "REF-1001");
    assert_eq!(res_valido.unwrap(), id_admin);

    // Reutilización inmediata (segundo intento) debe ser rechazada (Single-Use)
    let res_reuso = session.consumir_autorizacion(&token, id_cajero, "DESCUENTO_POS", "REF-1001");
    assert!(res_reuso.is_err());
}

// =========================================================================
// BLOQUE 7: Recuperación de Acceso Administrador (RF-LOG-07)
// =========================================================================

#[test]
fn test_tc_log_08_recuperar_acceso_admin_renovacion_codigo() {
    let mut conn = setup_inmemory_db_v7();

    let inst_res = AuthService::instalacion_inicial(
        &mut conn,
        InstalacionInicialRequestDto {
            nombre_completo: "Admin Root".to_string(),
            username: "adminroot".to_string(),
            password: "OldPassword2026".to_string(),
        },
    )
    .unwrap();

    let codigo_original = inst_res.codigo_recuperacion;

    let rec_res = AuthService::recuperar_acceso_admin(
        &mut conn,
        RecuperarAccesoAdminRequestDto {
            username: "adminroot".to_string(),
            codigo_recuperacion: codigo_original.clone(),
            nueva_password: "NewPassword2026".to_string(),
        },
    )
    .unwrap();

    assert_ne!(codigo_original, rec_res.codigo_recuperacion);

    let err_rec = AuthService::recuperar_acceso_admin(
        &mut conn,
        RecuperarAccesoAdminRequestDto {
            username: "adminroot".to_string(),
            codigo_recuperacion: codigo_original,
            nueva_password: "OtraPassword2026".to_string(),
        },
    );
    assert!(err_rec.is_err());
}

// =========================================================================
// BLOQUE 8: Pruebas V7.0 (Aislamiento de Roles y Revocación por Versión)
// =========================================================================

#[test]
fn test_tc_log_01_v7_invocacion_sin_sesion_activa_es_rechazada() {
    let conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();
    assert!(session.validar_sesion_operativa(&conn).is_err());
}

#[test]
fn test_tc_log_03_v7_cambio_obligatorio_en_sesion_normal_es_rechazado() {
    let mut conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();

    let hash = AuthService::hash_password("ClaveBase123").unwrap();
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, debe_cambiar_clave, version) 
         VALUES (2, 'Usuario Normal', 'usernormal', ?1, 'CAJERO', 0, 1)",
        [&hash],
    )
    .unwrap();

    session.abrir_sesion(2, 1, "usernormal".to_string(), "CAJERO".to_string(), false);
    assert_eq!(session.estado_sesion, EstadoSesion::Normal);

    let res = AuthService::cambiar_clave_obligatoria(
        &mut conn,
        &mut session,
        2,
        CambiarClaveObligatoriaRequestDto {
            nueva_password: "NuevaClaveProhibida123".to_string(),
        },
    );

    assert!(res.is_err());
}

#[test]
fn test_tc_log_07_v7_verificacion_admin_activa_pausa_tras_5_fallos() {
    let conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();

    let hash_admin = AuthService::hash_password("AdminClaveValida123").unwrap();
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo, debe_cambiar_clave, version) 
         VALUES (99, 'Admin Supervisor', 'adminsupervisor', ?1, 'ADMINISTRADOR', 1, 0, 1)",
        [&hash_admin],
    )
    .unwrap();

    for _ in 1..=4 {
        let res = AuthService::autorizar_descuento(
            &conn,
            &mut session,
            AutorizarDescuentoRequestDto {
                username_admin: "adminsupervisor".to_string(),
                password_admin: "ClaveInvalidaErronea".to_string(),
                referencia_venta: "OT-500".to_string(),
                descuento_bp: 1000,
            },
        );
        assert!(res.is_err());
    }

    assert!(!session.esta_verificacion_admin_pausada(99));

    let res_quinto = AuthService::autorizar_descuento(
        &conn,
        &mut session,
        AutorizarDescuentoRequestDto {
            username_admin: "adminsupervisor".to_string(),
            password_admin: "ClaveInvalidaErronea".to_string(),
            referencia_venta: "OT-500".to_string(),
            descuento_bp: 1000,
        },
    );
    assert!(res_quinto.is_err());
    assert!(session.esta_verificacion_admin_pausada(99));

    let res_bloqueado = AuthService::autorizar_descuento(
        &conn,
        &mut session,
        AutorizarDescuentoRequestDto {
            username_admin: "adminsupervisor".to_string(),
            password_admin: "AdminClaveValida123".to_string(),
            referencia_venta: "OT-500".to_string(),
            descuento_bp: 1000,
        },
    );
    assert!(res_bloqueado.is_err());
}

#[test]
fn test_tc_log_10_v7_revocacion_inmediata_por_desalineacion_de_version() {
    let conn = setup_inmemory_db_v7();
    let mut session = SessionState::new();

    let hash = AuthService::hash_password("ClaveSecret123").unwrap();
    conn.execute(
        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo, version) 
         VALUES (50, 'Juan Perez', 'jperez', ?1, 'CAJERO', 1, 1)",
        [&hash],
    )
    .unwrap();

    session.abrir_sesion(50, 1, "jperez".to_string(), "CAJERO".to_string(), false);
    assert!(session.validar_sesion_operativa(&conn).is_ok());

    conn.execute(
        "UPDATE usuarios SET version = version + 1 WHERE id_usuario = 50",
        [],
    )
    .unwrap();

    assert!(matches!(
        session.validar_sesion_operativa(&conn),
        Err(AppError::NoAutenticado)
    ));
    assert_eq!(session.estado_sesion, EstadoSesion::SinSesion);
}

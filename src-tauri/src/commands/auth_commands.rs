use crate::dto::usuario_dto::{
    AutorizarDescuentoRequestDto, CambiarClaveObligatoriaRequestDto,
    CambiarClaveVoluntariaRequestDto, InstalacionInicialRequestDto, InstalacionInicialResponseDto,
    LoginRequestDto, LoginResponseDto, ReautenticarRequestDto, RecuperarAccesoAdminRequestDto,
    SesionResumenDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::auth_service::AuthService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

fn obtener_ambiente_actual_strict(conn: &Connection) -> Result<String, AppError> {
    conn.query_row(
        "SELECT ambiente_actual FROM configuracion LIMIT 1",
        [],
        |row| row.get(0),
    )
    .map_err(AppError::Database)
}

#[tauri::command]
pub fn obtener_ambiente_inicial(
    db_state: State<'_, Mutex<Connection>>,
) -> Result<String, AppError> {
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    obtener_ambiente_actual_strict(&conn)
}

#[tauri::command]
pub fn requiere_instalacion(db_state: State<'_, Mutex<Connection>>) -> Result<bool, AppError> {
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))?;
    Ok(count == 0)
}

#[tauri::command]
pub fn login_usuario(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: LoginRequestDto,
) -> Result<LoginResponseDto, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    iniciar_sesion_local(&conn, &mut session, req)
}

pub fn iniciar_sesion_local(
    conn: &Connection,
    session: &mut SessionState,
    req: LoginRequestDto,
) -> Result<LoginResponseDto, AppError> {
    if !session.verificar_inactividad_con_auditoria(Some(conn)) {
        return Err(AppError::SessionActive);
    }

    // 1. CORRECCIÓN R7 (Punto 5): Consultar ambiente ANTES de invocar AuthService::login
    let ambiente = obtener_ambiente_actual_strict(conn)?;

    // 2. Autenticar credenciales y registrar auditoría solo si el ambiente responde
    let (mut res, usuario_version) = AuthService::login(conn, req)?;
    res.ambiente_actual = ambiente;

    // 3. Abrir la sesión en memoria una vez que la operación entera ha sido exitosa
    res.session_id = session.abrir_sesion(
        res.id_usuario,
        usuario_version,
        res.username.clone(),
        res.rol.clone(),
        res.debe_cambiar_clave,
    );

    Ok(res)
}

#[tauri::command]
pub fn instalacion_inicial(
    db_state: State<'_, Mutex<Connection>>,
    req: InstalacionInicialRequestDto,
) -> Result<InstalacionInicialResponseDto, AppError> {
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    AuthService::instalacion_inicial(&mut conn, req)
}

#[tauri::command]
pub fn recuperar_acceso_admin(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: RecuperarAccesoAdminRequestDto,
) -> Result<InstalacionInicialResponseDto, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    let res = AuthService::recuperar_acceso_admin(&mut conn, req)?;

    session.cerrar_sesion();

    Ok(res)
}

#[tauri::command]
pub fn obtener_sesion(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<SesionResumenDto, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    if session.verificar_inactividad_con_auditoria(Some(&conn)) {
        return Err(AppError::NoAutenticado);
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;
    let estado_str = session.estado_sesion.as_str().to_string();

    let mut res =
        AuthService::obtener_usuario_sesion(&conn, &mut session, id_usuario, &estado_str)?;
    res.ambiente_actual = obtener_ambiente_actual_strict(&conn)?;

    Ok(res)
}

#[tauri::command]
pub fn cambiar_clave_obligatoria(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: CambiarClaveObligatoriaRequestDto,
) -> Result<(), AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    if session.verificar_inactividad_con_auditoria(Some(&conn)) {
        return Err(AppError::NoAutenticado);
    }

    if session.estado_sesion != crate::services::session_manager::EstadoSesion::CambioObligatorio {
        return Err(AppError::AccessDenied(
            "El cambio obligatorio de clave solo está permitido para sesiones en flujo de cambio de clave.".to_string(),
        ));
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;

    let mut conn_mut = conn;
    AuthService::cambiar_clave_obligatoria(&mut conn_mut, &mut session, id_usuario, req)?;

    Ok(())
}

#[tauri::command]
pub fn cambiar_clave_voluntaria(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: CambiarClaveVoluntariaRequestDto,
) -> Result<(), AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    if session.verificar_inactividad_con_auditoria(Some(&conn)) {
        return Err(AppError::NoAutenticado);
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;

    let mut conn_mut = conn;
    AuthService::cambiar_clave_voluntaria(&mut conn_mut, &mut session, id_usuario, req)?;

    Ok(())
}

#[tauri::command]
pub fn registrar_actividad(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    sesion_id: String,
) -> Result<(), AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    session.registrar_interaccion(&conn, &sesion_id)
}

#[tauri::command]
pub fn cerrar_sesion(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<(), AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    if let (Some(id), Some(usr), Some(rol)) = (
        session.usuario_id,
        session.username.clone(),
        session.rol.clone(),
    ) {
        if let Ok(conn) = db_state.lock() {
            if let Err(e) = AuditoriaService::registrar_cierre_sesion(&conn, id, &usr, &rol) {
                tracing::error!(
                    "Fallo de trazabilidad al registrar auditoria de logout: {}",
                    e
                );
            }
        }
    }

    session.cerrar_sesion();
    Ok(())
}

#[tauri::command]
pub fn reautenticar(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: ReautenticarRequestDto,
) -> Result<String, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    let id_usuario = session.validar_sesion_operativa(&conn)?;

    if session.esta_verificacion_admin_pausada(id_usuario) {
        return Err(AppError::Auth(
            "Demasiados intentos de reautenticación. Intente nuevamente en 15 minutos.".to_string(),
        ));
    }

    let (password_hash, debe_cambiar_clave, bloqueado_hasta): (
        String,
        i32,
        Option<String>,
    ) = conn.query_row(
        "SELECT password_hash, debe_cambiar_clave, bloqueado_hasta FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
        [id_usuario],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    ).map_err(|_| AppError::NoAutenticado)?;

    if debe_cambiar_clave == 1 {
        return Err(AppError::AccessDenied(
            "Debe cambiar su contraseña antes de realizar esta operación.".to_string(),
        ));
    }

    if let Some(ref hasta) = bloqueado_hasta {
        let esta_bloqueado: bool = conn
            .query_row("SELECT datetime('now', '-5 hours') < ?1", [hasta], |r| {
                r.get(0)
            })
            .unwrap_or(false);

        if esta_bloqueado {
            session.cerrar_sesion();
            return Err(AppError::Auth(
                "Cuenta bloqueada temporalmente.".to_string(),
            ));
        }
    }

    if !AuthService::verify_password(&req.password_actual, &password_hash) {
        let se_pauso = session.registrar_fallo_verificacion_admin(id_usuario);
        if se_pauso {
            return Err(AppError::Auth(
                "Demasiados intentos de reautenticación. Intente nuevamente en 15 minutos."
                    .to_string(),
            ));
        }
        return Err(AppError::Auth(
            "La contraseña actual es incorrecta.".to_string(),
        ));
    }

    session.limpiar_fallos_verificacion_admin(id_usuario);

    let token = format!("auth_{}", uuid::Uuid::new_v4().simple());
    session.guardar_autorizacion(
        token.clone(),
        id_usuario,
        id_usuario,
        req.accion,
        req.referencia,
    );

    Ok(token)
}

#[tauri::command]
pub fn autorizar_descuento(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: AutorizarDescuentoRequestDto,
) -> Result<String, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;

    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;

    let id_cajero = session.validar_sesion_operativa(&conn)?;

    let id_admin = AuthService::autorizar_descuento(&conn, &mut session, req.clone())?;

    let token = format!("desc_{}", uuid::Uuid::new_v4().simple());

    session.guardar_autorizacion(
        token.clone(),
        id_admin,
        id_cajero,
        "DESCUENTO_POS".to_string(),
        req.referencia_venta.clone(),
    );

    let username_cajero = session.username.as_deref();
    let rol_cajero = session.rol.as_deref();
    if let Err(e) = AuditoriaService::registrar_v5(
        &conn,
        Some(id_cajero),
        "USUARIO",
        username_cajero,
        rol_cajero,
        Some(id_admin),
        "POS",
        "AUTORIZAR_DESCUENTO",
        Some("ventas"),
        Some(&req.referencia_venta),
        Some(&format!("Descuento de {} bp autorizado", req.descuento_bp)),
        "OK",
    ) {
        tracing::error!(
            "Fallo de trazabilidad al registrar auditoria de autorizacion: {}",
            e
        );
    }

    Ok(token)
}

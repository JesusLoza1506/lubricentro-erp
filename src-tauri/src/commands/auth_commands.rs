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

#[tauri::command]
pub fn requiere_instalacion(db_state: State<'_, Mutex<Connection>>) -> Result<bool, AppError> {
    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))?;
    Ok(count == 0)
}

#[tauri::command]
pub fn login_usuario(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: LoginRequestDto,
) -> Result<LoginResponseDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if !session.verificar_inactividad() {
        return Err(AppError::Validation(
            "Ya existe una sesión activa en el sistema. Cierre la sesión actual antes de ingresar."
                .to_string(),
        ));
    }

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let res = AuthService::login(&conn, req)?;

    session.abrir_sesion(
        res.id_usuario,
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
    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::instalacion_inicial(&mut conn, req)
}

#[tauri::command]
pub fn recuperar_acceso_admin(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: RecuperarAccesoAdminRequestDto,
) -> Result<InstalacionInicialResponseDto, AppError> {
    let mut session = session_state.lock().ok();

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let res = AuthService::recuperar_acceso_admin(&mut conn, req)?;

    if let Some(ref mut s) = session {
        s.cerrar_sesion();
    }

    Ok(res)
}

#[tauri::command]
pub fn obtener_sesion(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<SesionResumenDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if session.verificar_inactividad() {
        return Err(AppError::NoAutenticado);
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;
    let estado_str = session.estado_sesion.as_str().to_string();

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::obtener_usuario_sesion(&conn, &mut session, id_usuario, &estado_str)
}

#[tauri::command]
pub fn cambiar_clave_obligatoria(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: CambiarClaveObligatoriaRequestDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if session.verificar_inactividad() {
        return Err(AppError::NoAutenticado);
    }

    // H02: Validar que la sesión esté explícitamente retenida en estado de cambio obligatorio
    if session.estado_sesion != crate::services::session_manager::EstadoSesion::CambioObligatorio {
        return Err(AppError::Validation(
            "Acceso denegado: El cambio obligatorio de clave solo está permitido para sesiones en flujo de cambio de clave.".to_string(),
        ));
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::cambiar_clave_obligatoria(&conn, &mut session, id_usuario, req)?;

    Ok(())
}

#[tauri::command]
pub fn cambiar_clave_voluntaria(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: CambiarClaveVoluntariaRequestDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if session.verificar_inactividad() {
        return Err(AppError::NoAutenticado);
    }

    let id_usuario = session.usuario_id.ok_or(AppError::NoAutenticado)?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    AuthService::cambiar_clave_voluntaria(&conn, &mut session, id_usuario, req)?;

    Ok(())
}

#[tauri::command]
pub fn registrar_actividad(session_state: State<'_, Mutex<SessionState>>) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if session.verificar_inactividad() {
        return Err(AppError::NoAutenticado);
    }

    session.renovar_actividad();
    Ok(())
}

#[tauri::command]
pub fn cerrar_sesion(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    if let (Some(id), Some(usr), Some(rol)) = (
        session.usuario_id,
        session.username.clone(),
        session.rol.clone(),
    ) {
        if let Ok(conn) = db_state.lock() {
            let _ = AuditoriaService::registrar_cierre_sesion(&conn, id, &usr, &rol);
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
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

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
        return Err(AppError::Validation(
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
    session.guardar_autorizacion(token.clone(), id_usuario, req.accion, req.referencia);

    Ok(token)
}

#[tauri::command]
pub fn autorizar_descuento(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    req: AutorizarDescuentoRequestDto,
) -> Result<String, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _ = session.validar_sesion_operativa(&conn)?;

    let id_admin = AuthService::autorizar_descuento(&conn, &mut session, req.clone())?;

    let token = format!("desc_{}", uuid::Uuid::new_v4().simple());
    session.guardar_autorizacion(
        token.clone(),
        id_admin,
        "DESCUENTO_POS".to_string(),
        req.referencia_venta,
    );

    Ok(token)
}

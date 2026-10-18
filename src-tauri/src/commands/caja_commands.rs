use crate::dto::caja_dto::{AbrirCajaDto, CajaChicaDto, CierreCajaDto};
use crate::errors::AppError;
use crate::services::caja_service::CajaService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn abrir_caja_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    mut payload: AbrirCajaDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    if session.verificar_inactividad_con_auditoria(Some(&conn)) {
        return Err(AppError::NoAutenticado);
    }

    let user_id = session.validar_sesion_operativa(&conn)?;
    payload.id_usuario = user_id;

    CajaService::abrir_caja(&conn, payload)
}

#[tauri::command]
pub fn cerrar_caja_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CierreCajaDto,
) -> Result<CajaChicaDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    if session.verificar_inactividad_con_auditoria(Some(&conn)) {
        return Err(AppError::NoAutenticado);
    }

    let user_id = session.validar_sesion_operativa(&conn)?;

    CajaService::cerrar_caja(&conn, payload, user_id)
}

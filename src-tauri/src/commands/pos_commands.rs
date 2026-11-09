use crate::{
    dto::pos_dto::CredencialesPos,
    errors::AppError,
    services::{pos_service::operaciones, session_manager::SessionState},
};
use rusqlite::Connection;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;
#[tauri::command]
pub fn pos_operacion_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    accion: String,
    payload: Value,
    credenciales: Option<CredencialesPos>,
) -> Result<Value, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    operaciones::ejecutar(&mut conn, &mut session, &accion, payload, credenciales)
}

#[tauri::command]
pub fn pos_archivo_local_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_documento: i64,
    tipo: String,
) -> Result<(), AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    let actor = session.validar_sesion_operativa(&conn)?;
    let (venta, pdf, xml, cdr): (i64, Option<String>, Option<String>, Option<String>) = conn
        .query_row(
            "SELECT id_venta,ruta_pdf,ruta_xml,ruta_cdr FROM comprobantes WHERE id_comprobante=?1",
            [id_documento],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;
    crate::services::pos_service::consultas::visible(&conn, actor, venta)?;
    let referencia = match tipo.as_str() {
        "pdf" => pdf,
        "xml" => xml,
        "cdr" => cdr,
        _ => None,
    }
    .ok_or_else(|| AppError::NotFound("Archivo oficial no disponible.".into()))?;
    let ruta = crate::services::backup_service::ruta_comprobante_local(&conn, &referencia)?;
    tauri_plugin_opener::open_path(ruta, None::<&str>)
        .map_err(|_| AppError::Validation("No se pudo abrir el archivo oficial local.".into()))
}

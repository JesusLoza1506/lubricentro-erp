use crate::{
    dto::caja_dto::*,
    errors::AppError,
    services::{caja_service::CajaService, session_manager::SessionState},
};
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

fn recibir<T: DeserializeOwned>(v: Value) -> Result<T, AppError> {
    serde_json::from_value(v)
        .map_err(|_| AppError::Validation("Datos de Caja Chica inválidos.".into()))
}

#[tauri::command]
pub fn caja_operacion_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    accion: String,
    payload: Value,
) -> Result<Value, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let mut conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    let actor = session.validar_sesion_operativa(&conn)?;
    match accion.as_str() {
        "inicio" => CajaService::inicio(&conn, actor),
        "detalle" => {
            let id = payload
                .get("id_caja")
                .and_then(Value::as_i64)
                .ok_or_else(|| AppError::Validation("Indica el turno.".into()))?;
            CajaService::detalle(&conn, actor, id)
        }
        "movimientos" => {
            CajaService::movimientos(&conn, actor, recibir::<FiltroMovimientosCaja>(payload)?)
        }
        "historial" => {
            CajaService::historial(&conn, actor, recibir::<FiltroHistorialCaja>(payload)?)
        }
        "abrir" => CajaService::abrir_caja(&mut conn, actor, recibir::<AbrirCajaDto>(payload)?),
        "manual" => CajaService::manual(&mut conn, actor, recibir::<ManualCajaDto>(payload)?),
        "revertir" => CajaService::revertir(&mut conn, actor, recibir::<RevertirCajaDto>(payload)?),
        "cerrar" => CajaService::cerrar_caja(&mut conn, actor, recibir::<CierreCajaDto>(payload)?),
        "aclarar" => CajaService::aclarar(&mut conn, actor, recibir::<AclararCajaDto>(payload)?),
        _ => Err(AppError::Validation(
            "Operación de Caja Chica desconocida.".into(),
        )),
    }
}

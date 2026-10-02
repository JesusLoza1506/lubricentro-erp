use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto};
use crate::dto::vehiculo_dto::{CrearVehiculoDto, VehiculoDto};
use crate::errors::AppError;
use crate::services::cliente_service::ClienteService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn registrar_cliente_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearClienteDto,
) -> Result<i64, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    ClienteService::registrar_cliente(&conn, payload)
}

#[tauri::command]
pub fn buscar_cliente_por_doc_cmd(
    state: State<'_, Mutex<Connection>>,
    doc: String,
) -> Result<ClienteDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    ClienteService::buscar_cliente_por_documento(&conn, &doc)
}

#[tauri::command]
pub fn registrar_vehiculo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearVehiculoDto,
) -> Result<String, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    ClienteService::registrar_vehiculo(&conn, payload)
}

#[tauri::command]
pub fn listar_vehiculos_por_cliente_cmd(
    state: State<'_, Mutex<Connection>>,
    id_cliente: i64,
) -> Result<Vec<VehiculoDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    ClienteService::listar_vehiculos_por_cliente(&conn, id_cliente)
}

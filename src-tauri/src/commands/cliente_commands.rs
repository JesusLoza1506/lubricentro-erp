use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto};
use crate::dto::vehiculo_dto::{
    CrearVehiculoPayloadDto, EditarVehiculoPayloadDto, FichaVehicularCompletaDto,
    VehiculoCompletoDto,
};
use crate::errors::AppError;
use crate::services::cliente_service::ClienteService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

fn verificar_permiso_rol(
    conn: &Connection,
    id_usuario: Option<i64>,
    roles_permitidos: &[&str],
) -> Result<(), AppError> {
    let id_u = match id_usuario {
        Some(id) if id > 0 => id,
        _ => {
            return Err(AppError::Validation(
                "Acceso denegado: Se requiere un usuario autenticado para esta operación."
                    .to_string(),
            ))
        }
    };

    let nombre_rol: String = conn
        .query_row(
            "SELECT UPPER(rol) FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
            [id_u],
            |row| row.get(0),
        )
        .map_err(|_| {
            AppError::Validation("Acceso denegado: Usuario no encontrado o inactivo.".to_string())
        })?;

    if !roles_permitidos.contains(&nombre_rol.as_str()) {
        return Err(AppError::Validation(format!(
            "Acceso denegado: El rol '{}' no tiene permisos para realizar esta operación.",
            nombre_rol
        )));
    }

    Ok(())
}

#[tauri::command]
pub fn registrar_cliente_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearClienteDto,
    id_usuario: Option<i64>,
) -> Result<i64, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    // Restringido estrictamente a Administrador y Cajero (El mecánico es solo lectura)
    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO"])?;

    ClienteService::registrar_cliente(&conn, payload)
}

#[tauri::command]
pub fn buscar_cliente_por_doc_cmd(
    state: State<'_, Mutex<Connection>>,
    doc: String,
    id_usuario: Option<i64>,
) -> Result<ClienteDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    ClienteService::buscar_cliente_por_documento(&conn, &doc)
}

#[tauri::command]
pub fn registrar_vehiculo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearVehiculoPayloadDto,
    id_usuario: Option<i64>,
) -> Result<String, AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let placa = ClienteService::registrar_vehiculo(&tx, payload, id_usuario)?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(placa)
}

#[tauri::command]
pub fn actualizar_vehiculo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: EditarVehiculoPayloadDto,
    id_usuario: Option<i64>,
) -> Result<(), AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::actualizar_vehiculo(&tx, payload, id_usuario)?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn obtener_vehiculo_por_placa_cmd(
    state: State<'_, Mutex<Connection>>,
    placa: String,
    id_usuario: Option<i64>,
) -> Result<VehiculoCompletoDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    ClienteService::obtener_vehiculo_por_placa(&conn, &placa)
}

#[tauri::command]
pub fn obtener_ficha_vehicular_completa_cmd(
    state: State<'_, Mutex<Connection>>,
    placa: String,
    id_usuario: Option<i64>,
) -> Result<FichaVehicularCompletaDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    ClienteService::obtener_ficha_vehicular_completa(&conn, &placa)
}

#[tauri::command]
pub fn eliminar_vehiculo_cmd(
    state: State<'_, Mutex<Connection>>,
    placa: String,
    id_usuario: Option<i64>,
) -> Result<(), AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR"])?;

    ClienteService::eliminar_vehiculo(&conn, &placa, id_usuario)
}

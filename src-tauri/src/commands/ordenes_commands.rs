use crate::dto::orden_trabajo_dto::{
    CambiarEstadoOtDto, CrearOrdenTrabajoDto, OrdenTrabajoHistorialDto,
};
use crate::errors::AppError;
use crate::services::ordenes_service::OrdenesService;
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
pub fn crear_orden_trabajo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearOrdenTrabajoDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::crear_orden_trabajo(&conn, payload)
}

#[tauri::command]
pub fn cambiar_estado_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CambiarEstadoOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::cambiar_estado_ot(&conn, payload)
}

#[tauri::command]
pub fn obtener_historial_por_placa_cmd(
    state: State<'_, Mutex<Connection>>,
    placa: String,
    id_usuario: Option<i64>,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_rol(&conn, id_usuario, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::obtener_historial_por_placa(&conn, &placa)
}

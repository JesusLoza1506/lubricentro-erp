use crate::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, AgregarServicioOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto,
    OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto, ServicioResumenDto,
};
use crate::dto::usuario_dto::MecanicoResumenDto;
use crate::errors::AppError;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

/// Extrae de forma segura el ID de usuario desde Option<i64> y valida que sea > 0.
/// Elimina cualquier tipo de fallback o usuario por defecto.
fn obtener_id_usuario_valido(id_usuario: Option<i64>) -> Result<i64, AppError> {
    id_usuario.filter(|&id| id > 0).ok_or_else(|| {
        AppError::Validation(
            "Acceso denegado: Se requiere un id_usuario válido y autenticado para esta operación."
                .to_string(),
        )
    })
}

fn verificar_permiso_rol(
    conn: &Connection,
    id_usuario: i64,
    roles_permitidos: &[&str],
) -> Result<(), AppError> {
    let nombre_rol: String = conn
        .query_row(
            "SELECT UPPER(rol) FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
            [id_usuario],
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
pub fn listar_ordenes_trabajo_cmd(
    state: State<'_, Mutex<Connection>>,
    id_usuario: Option<i64>,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;

    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::listar_ordenes_trabajo(&conn, Some(user_id))
}

#[tauri::command]
pub fn crear_orden_trabajo_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearOrdenTrabajoDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;

    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "CAJERO"])?;

    OrdenesService::crear_orden_trabajo(&mut conn, payload)
}

#[tauri::command]
pub fn cambiar_estado_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CambiarEstadoOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;

    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::cambiar_estado_ot(&mut conn, payload, Some(user_id))
}

#[tauri::command]
pub fn reasignar_mecanico_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: ReasignarMecanicoOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;

    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR"])?;

    OrdenesService::reasignar_mecanico_ot(&conn, payload)
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

    let user_id = obtener_id_usuario_valido(id_usuario)?;

    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::obtener_historial_por_placa(&conn, &placa)
}

#[tauri::command]
pub fn listar_mecanicos_cmd(
    state: State<'_, Mutex<Connection>>,
    id_usuario: Option<i64>,
) -> Result<Vec<MecanicoResumenDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    let mut stmt = conn
        .prepare(
            "SELECT id_usuario, nombre_completo 
             FROM usuarios 
             WHERE UPPER(rol) = 'MECANICO' AND activo = 1 
             ORDER BY nombre_completo ASC",
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(MecanicoResumenDto {
                id_usuario: row.get(0)?,
                nombre_completo: row.get(1)?,
            })
        })
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut lista = Vec::new();
    for r in rows {
        lista.push(r.map_err(|e| AppError::Validation(e.to_string()))?);
    }

    Ok(lista)
}

#[tauri::command]
pub fn agregar_producto_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: AgregarProductoOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::agregar_producto_ot(&mut conn, payload, user_id)
}

#[tauri::command]
pub fn eliminar_producto_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    id_detalle: i64,
    id_ot: i64,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::eliminar_producto_ot(&mut conn, id_detalle, id_ot, user_id)
}

#[tauri::command]
pub fn agregar_servicio_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: AgregarServicioOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::agregar_servicio_ot(&conn, payload, user_id)
}

#[tauri::command]
pub fn eliminar_servicio_ot_cmd(
    state: State<'_, Mutex<Connection>>,
    id_detalle: i64,
    id_ot: i64,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::eliminar_servicio_ot(&conn, id_detalle, id_ot, user_id)
}

#[tauri::command]
pub fn listar_servicios_cmd(
    state: State<'_, Mutex<Connection>>,
    id_usuario: Option<i64>,
) -> Result<Vec<ServicioResumenDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = obtener_id_usuario_valido(id_usuario)?;
    verificar_permiso_rol(&conn, user_id, &["ADMINISTRADOR", "CAJERO", "MECANICO"])?;

    OrdenesService::listar_servicios(&conn)
}

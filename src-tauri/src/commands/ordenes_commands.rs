use crate::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, AgregarServicioOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto,
    OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto, ServicioResumenDto,
};
use crate::dto::usuario_dto::MecanicoResumenDto;
use crate::errors::AppError;
use crate::services::ordenes_service::OrdenesService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

fn verificar_permiso_sesion(
    session: &mut SessionState,
    conn: &Connection,
    roles_permitidos: &[&str],
) -> Result<i64, AppError> {
    let user_id = session.validar_sesion_operativa(conn)?;
    let rol = session.rol.as_deref().unwrap_or("").to_uppercase();

    if !roles_permitidos.contains(&rol.as_str()) {
        return Err(AppError::Validation(format!(
            "Acceso denegado: El rol '{}' no tiene permisos para realizar esta operación.",
            rol
        )));
    }

    Ok(user_id)
}

#[tauri::command]
pub fn listar_ordenes_trabajo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::listar_ordenes_trabajo(&conn, Some(user_id))
}

#[tauri::command]
pub fn crear_orden_trabajo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearOrdenTrabajoDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    OrdenesService::crear_orden_trabajo(&mut conn, payload)
}

#[tauri::command]
pub fn cambiar_estado_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CambiarEstadoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::cambiar_estado_ot(&mut conn, payload, Some(user_id))
}

#[tauri::command]
pub fn reasignar_mecanico_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ReasignarMecanicoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    OrdenesService::reasignar_mecanico_ot(&conn, payload)
}

#[tauri::command]
pub fn obtener_historial_por_placa_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _user_id = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::obtener_historial_por_placa(&conn, &placa)
}

#[tauri::command]
pub fn listar_mecanicos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<MecanicoResumenDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _user_id = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

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
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AgregarProductoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::agregar_producto_ot(&mut conn, payload, user_id)
}

#[tauri::command]
pub fn eliminar_producto_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_detalle: i64,
    id_ot: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::eliminar_producto_ot(&mut conn, id_detalle, id_ot, user_id)
}

#[tauri::command]
pub fn agregar_servicio_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AgregarServicioOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::agregar_servicio_ot(&conn, payload, user_id)
}

#[tauri::command]
pub fn eliminar_servicio_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_detalle: i64,
    id_ot: i64,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let user_id = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::eliminar_servicio_ot(&conn, id_detalle, id_ot, user_id)
}

#[tauri::command]
pub fn listar_servicios_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ServicioResumenDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let _user_id = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::listar_servicios(&conn)
}

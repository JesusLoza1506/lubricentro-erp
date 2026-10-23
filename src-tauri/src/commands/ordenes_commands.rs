use crate::dto::orden_trabajo_dto::{
    AgregarServicioOtDto, CancelarOtDto, CrearOrdenTrabajoDto, DevolverInsumoOtDto, FinalizarOtDto,
    IniciarTrabajoOtDto, ModificarServicioOtDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto,
    RegistrarInsumoOtDto, ServicioResumenDto, TableroZanjasDto,
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
) -> Result<(i64, String), AppError> {
    let user_id = session.validar_sesion_operativa(conn)?;
    let rol = session.rol.as_deref().unwrap_or("").to_uppercase();

    if !roles_permitidos.contains(&rol.as_str()) {
        return Err(AppError::Validation(format!(
            "Acceso denegado: El rol '{}' no tiene permisos para realizar esta operación.",
            rol
        )));
    }

    Ok((user_id, rol))
}

#[tauri::command]
pub fn obtener_tablero_zanjas_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<TableroZanjasDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::obtener_tablero_zanjas(&conn, user_id, &rol)
}

#[tauri::command]
pub fn listar_ordenes_trabajo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::listar_ordenes_trabajo(&conn, user_id, &rol)
}

#[tauri::command]
pub fn crear_orden_trabajo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearOrdenTrabajoDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    OrdenesService::crear_orden_trabajo(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn iniciar_trabajo_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: IniciarTrabajoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::iniciar_trabajo_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn reasignar_mecanico_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ReasignarMecanicoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    OrdenesService::reasignar_mecanico_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn registrar_insumo_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: RegistrarInsumoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::registrar_insumo_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn devolver_insumo_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: DevolverInsumoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    OrdenesService::devolver_insumo_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn agregar_servicio_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AgregarServicioOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::agregar_servicio_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn modificar_servicio_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ModificarServicioOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::modificar_servicio_ot(&mut conn, payload, user_id, &rol)
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
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::eliminar_servicio_ot(&mut conn, id_detalle, id_ot, user_id, &rol)
}

#[tauri::command]
pub fn finalizar_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: FinalizarOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "MECANICO"])?;

    OrdenesService::finalizar_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn cancelar_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CancelarOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    OrdenesService::cancelar_ot(&mut conn, payload, user_id, &rol)
}

#[tauri::command]
pub fn obtener_historial_por_placa_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (user_id, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::obtener_historial_por_placa(&conn, &placa, user_id, &rol)
}

#[tauri::command]
pub fn listar_mecanicos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<MecanicoResumenDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (_user_id, _rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    let mut stmt = conn.prepare(
        "SELECT id_usuario, nombre_completo 
         FROM usuarios 
         WHERE UPPER(rol) = 'MECANICO' AND activo = 1 
         ORDER BY nombre_completo ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(MecanicoResumenDto {
            id_usuario: row.get(0)?,
            nombre_completo: row.get(1)?,
        })
    })?;

    let mut lista = Vec::new();
    for r in rows {
        lista.push(r?);
    }

    Ok(lista)
}

#[tauri::command]
pub fn listar_servicios_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ServicioResumenDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error DB".into()))?;

    let (_user_id, _rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    OrdenesService::listar_servicios(&conn)
}

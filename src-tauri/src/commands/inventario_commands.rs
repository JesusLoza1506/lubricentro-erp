use crate::dto::producto_dto::{
    ActualizarProductoDto, AjustarStockDto, CrearProductoDto, DespacharProductoDto,
    ProductoAdminDto, ProductoPublicoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::errors::AppError;
use crate::services::inventario_service::InventarioService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

fn verificar_permiso_sesion(
    session: &mut SessionState,
    conn: &Connection,
    roles_permitidos: &[&str],
) -> Result<(i64, String), AppError> {
    let id_usuario = session.validar_sesion_operativa(conn)?;
    let rol = session.rol.as_deref().unwrap_or("").to_uppercase();

    if !roles_permitidos.contains(&rol.as_str()) {
        return Err(AppError::Validation(format!(
            "Acceso denegado: El rol '{}' no tiene permisos para realizar esta operación.",
            rol
        )));
    }

    Ok((id_usuario, rol))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ListarProductosResponse {
    Admin(Vec<ProductoAdminDto>),
    Publico(Vec<ProductoPublicoDto>),
}

#[tauri::command]
pub fn listar_productos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<ListarProductosResponse, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (_, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    if rol == "ADMINISTRADOR" {
        let prods = InventarioService::listar_productos_admin(&conn)?;
        Ok(ListarProductosResponse::Admin(prods))
    } else {
        let prods = InventarioService::listar_productos_publicos(&conn)?;
        Ok(ListarProductosResponse::Publico(prods))
    }
}

#[tauri::command]
pub fn listar_productos_publicos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ProductoPublicoDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_productos_publicos(&conn)
}

#[tauri::command]
pub fn listar_productos_criticos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<ListarProductosResponse, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (_, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    if rol == "ADMINISTRADOR" {
        let prods = InventarioService::listar_productos_criticos_admin(&conn)?;
        Ok(ListarProductosResponse::Admin(prods))
    } else {
        let prods = InventarioService::listar_productos_criticos_publicos(&conn)?;
        Ok(ListarProductosResponse::Publico(prods))
    }
}

#[tauri::command]
pub fn registrar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearProductoDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::registrar_producto(&conn, payload, id_u)
}

#[tauri::command]
pub fn actualizar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ActualizarProductoDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::actualizar_producto(&conn, payload)
}

#[tauri::command]
pub fn eliminar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::eliminar_producto(&conn, id_producto)
}

#[tauri::command]
pub fn despachar_producto_granel_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: DespacharProductoDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::despachar_producto_granel(&conn, payload, id_u)
}

#[tauri::command]
pub fn ajustar_stock_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AjustarStockDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::ajustar_stock(&conn, payload, id_u)
}

#[tauri::command]
pub fn listar_categorias_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<CategoriaEntity>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_categorias(&conn)
}

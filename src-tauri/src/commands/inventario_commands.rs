use crate::dto::producto_dto::{
    ActualizarProductoDto, CrearProductoDto, ProductoAdminDto, ProductoPublicoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::errors::AppError;
use crate::services::inventario_service::InventarioService;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn listar_productos_cmd(
    state: State<'_, Mutex<Connection>>,
    _rol_usuario: Option<String>,
) -> Result<Vec<ProductoAdminDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::listar_productos_admin(&conn)
}

#[tauri::command]
pub fn listar_productos_publicos_cmd(
    state: State<'_, Mutex<Connection>>,
) -> Result<Vec<ProductoPublicoDto>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::listar_productos_publicos(&conn)
}

#[tauri::command]
pub fn registrar_producto_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: CrearProductoDto,
    id_usuario: i64,
) -> Result<i64, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::registrar_producto(&conn, payload, id_usuario)
}

#[tauri::command]
pub fn actualizar_producto_cmd(
    state: State<'_, Mutex<Connection>>,
    payload: ActualizarProductoDto,
) -> Result<(), AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::actualizar_producto(&conn, payload)
}

#[tauri::command]
pub fn eliminar_producto_cmd(
    state: State<'_, Mutex<Connection>>,
    id_producto: i64,
) -> Result<(), AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::eliminar_producto(&conn, id_producto)
}

#[tauri::command]
pub fn listar_categorias_cmd(
    state: State<'_, Mutex<Connection>>,
) -> Result<Vec<CategoriaEntity>, AppError> {
    let conn = state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    InventarioService::listar_categorias(&conn)
}

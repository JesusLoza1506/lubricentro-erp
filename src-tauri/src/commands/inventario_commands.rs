use crate::dto::ingreso_dto::{AnularIngresoDto, IngresoStockResponseDto, RegistrarIngresoDto};
use crate::dto::producto_dto::{
    ActualizarProductoDto, AjustarStockDto, ConteoFisicoDto, CrearProductoDto, MovimientoKardexDto,
    OrigenKardexDto, ProductoAdminDto, ProductoPublicoDto,
};
use crate::dto::proveedor_dto::{
    ActualizarProveedorDto, ActualizarServicioDto, CrearProveedorDto, CrearServicioDto,
    ProveedorDto, ServicioMantenimientoDto,
};
use crate::entities::categoria::CategoriaEntity;
use crate::errors::AppError;
use crate::services::inventario_service::{
    InventarioService, ProductosPaginadosDto, ProductosPublicosPaginadosDto, ResumenSaldosKardexDto,
};
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;
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
            "Acceso denegado: El rol '{}' no tiene permisos para esta operación.",
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
    incluir_inactivos: Option<bool>,
) -> Result<ListarProductosResponse, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (_, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    if rol == "ADMINISTRADOR" {
        Ok(ListarProductosResponse::Admin(
            InventarioService::listar_productos_admin(&conn, incluir_inactivos.unwrap_or(false))?,
        ))
    } else {
        Ok(ListarProductosResponse::Publico(
            InventarioService::listar_productos_publicos(&conn)?,
        ))
    }
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn listar_productos_admin_paginado_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    busqueda: Option<String>,
    categoria: Option<String>,
    incluir_inactivos: Option<bool>,
    limite: Option<i64>,
    offset: Option<i64>,
    solo_criticos: Option<bool>,
    stock: Option<String>,
    estado: Option<String>,
) -> Result<ProductosPaginadosDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::listar_productos_admin_consulta(
        &conn,
        busqueda,
        categoria,
        incluir_inactivos,
        limite.unwrap_or(20),
        offset.unwrap_or(0),
        solo_criticos.unwrap_or(false),
        stock,
        estado,
    )
}

#[tauri::command]
pub fn listar_productos_publicos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ProductoPublicoDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_productos_publicos(&conn)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn listar_productos_publicos_paginado_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    busqueda: Option<String>,
    categoria: Option<String>,
    limite: Option<i64>,
    offset: Option<i64>,
    solo_criticos: Option<bool>,
    stock: Option<String>,
    estado: Option<String>,
) -> Result<ProductosPublicosPaginadosDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_productos_publicos_consulta(
        &conn,
        busqueda,
        categoria,
        limite.unwrap_or(20),
        offset.unwrap_or(0),
        solo_criticos.unwrap_or(false),
        stock,
        estado,
    )
}

#[tauri::command]
pub fn listar_productos_criticos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<ListarProductosResponse, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (_, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    if rol == "ADMINISTRADOR" {
        Ok(ListarProductosResponse::Admin(
            InventarioService::listar_productos_criticos_admin(&conn)?,
        ))
    } else {
        Ok(ListarProductosResponse::Publico(
            InventarioService::listar_productos_criticos_publicos(&conn)?,
        ))
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
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::registrar_producto(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn actualizar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ActualizarProductoDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::actualizar_producto(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn eliminar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
    motivo: String,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::eliminar_producto(&mut conn, id_producto, &motivo, id_u)
}

#[tauri::command]
pub fn reactivar_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
    motivo: String,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::reactivar_producto(&mut conn, id_producto, &motivo, id_u)
}

#[tauri::command]
pub fn despachar_producto_granel_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
    cantidad: i64,
    motivo: Option<String>,
) -> Result<Value, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;
    let _ = (id_producto, cantidad, motivo);
    Err(AppError::Validation(
        "Registre el consumo de materiales desde su orden de trabajo o la venta desde POS.".into(),
    ))
}

#[tauri::command]
pub fn registrar_ingreso_compra_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: RegistrarIngresoDto,
    id_operacion: Option<String>,
) -> Result<Value, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::registrar_ingreso_compra(&mut conn, payload, id_operacion.as_deref(), id_u)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn listar_ingresos_stock_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    limite: Option<i64>,
    offset: Option<i64>,
    estado: Option<String>,
    origen: Option<String>,
    fecha_inicio: Option<String>,
    fecha_fin: Option<String>,
) -> Result<Vec<IngresoStockResponseDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::listar_ingresos_stock_filtrado(
        &conn,
        limite.unwrap_or(20),
        offset.unwrap_or(0),
        estado,
        origen,
        fecha_inicio,
        fecha_fin,
    )
}

#[tauri::command]
pub fn anular_ingreso_compra_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AnularIngresoDto,
    id_operacion: Option<String>,
) -> Result<Value, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::anular_ingreso_compra(&mut conn, payload, id_operacion.as_deref(), id_u)
}

#[tauri::command]
pub fn ajustar_stock_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: AjustarStockDto,
    id_operacion: Option<String>,
) -> Result<Value, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::ajustar_stock(&mut conn, payload, id_operacion.as_deref(), id_u)
}

#[tauri::command]
pub fn procesar_conteo_fisico_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ConteoFisicoDto,
    id_operacion: Option<String>,
) -> Result<Value, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::procesar_conteo_fisico(&mut conn, payload, id_operacion.as_deref(), id_u)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn listar_kardex_producto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
    limite: Option<i64>,
    offset: Option<i64>,
    fecha_inicio: Option<String>,
    fecha_fin: Option<String>,
    tipo_movimiento: Option<String>,
) -> Result<Vec<MovimientoKardexDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::listar_kardex_producto(
        &conn,
        id_producto,
        limite.unwrap_or(20),
        offset.unwrap_or(0),
        fecha_inicio,
        fecha_fin,
        tipo_movimiento,
    )
}

#[tauri::command]
pub fn obtener_resumen_saldos_kardex_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_producto: i64,
    fecha_inicio: Option<String>,
    fecha_fin: Option<String>,
    tipo_movimiento: Option<String>,
) -> Result<ResumenSaldosKardexDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::obtener_resumen_saldos_kardex(
        &conn,
        id_producto,
        fecha_inicio,
        fecha_fin,
        tipo_movimiento,
    )
}

#[tauri::command]
pub fn listar_categorias_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<CategoriaEntity>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_categorias(&conn)
}

#[tauri::command]
pub fn listar_proveedores_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ProveedorDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::listar_proveedores(&conn)
}

#[tauri::command]
pub fn crear_proveedor_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearProveedorDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::crear_proveedor(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn actualizar_proveedor_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ActualizarProveedorDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::actualizar_proveedor(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn listar_servicios_admin_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ServicioMantenimientoDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::listar_servicios_admin(&conn)
}

#[tauri::command]
pub fn listar_servicios_publicos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<ServicioMantenimientoDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    InventarioService::listar_servicios_publicos(&conn)
}

#[tauri::command]
pub fn crear_servicio_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearServicioDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::crear_servicio(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn actualizar_servicio_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: ActualizarServicioDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::actualizar_servicio(&mut conn, payload, id_u)
}

#[tauri::command]
pub fn cambiar_estado_servicio_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_servicio: i64,
    activo: bool,
    motivo: String,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    InventarioService::cambiar_estado_servicio(&mut conn, id_servicio, activo, &motivo, id_u)
}

#[tauri::command]
pub fn obtener_origen_kardex_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    id_movimiento: i64,
) -> Result<OrigenKardexDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Error de sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Error de BD".into()))?;
    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;
    InventarioService::obtener_origen_kardex(&conn, id_movimiento)
}

#[tauri::command]
pub fn proponer_codigo_inventario_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    tipo: String,
) -> Result<String, AppError> {
    let mut session = session_state.lock().map_err(|_| AppError::DbLock)?;
    let conn = db_state.lock().map_err(|_| AppError::DbLock)?;
    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;
    InventarioService::proponer_codigo(&conn, &tipo)
}

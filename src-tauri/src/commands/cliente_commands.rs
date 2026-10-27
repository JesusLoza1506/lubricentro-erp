use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto, EditarClienteDto};
use crate::dto::vehiculo_dto::{
    CambiarEstadoVehiculoPayloadDto, CorregirHistorialOtPayloadDto, CrearVehiculoPayloadDto,
    EditarVehiculoPayloadDto, FichaVehicularCompletaDto, ProximoCambioAceiteDto,
    ResultadoPaginadoDto, VehiculoCompletoDto,
};
use crate::errors::AppError;
use crate::services::cliente_service::ClienteService;
use crate::services::session_manager::SessionState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

fn verificar_permiso_sesion(
    session: &mut SessionState,
    conn: &Connection,
    roles_permitidos: &[&str],
) -> Result<(i64, String, String), AppError> {
    let id_usuario = session.validar_sesion_operativa(conn)?;
    let rol = session.rol.as_deref().unwrap_or("").to_uppercase();
    let username = session.username.clone().unwrap_or_default();

    if !roles_permitidos.contains(&rol.as_str()) {
        return Err(AppError::Validation(format!(
            "Acceso denegado: El rol '{}' no tiene permisos para realizar esta operación.",
            rol
        )));
    }

    Ok((id_usuario, username, rol))
}

#[tauri::command]
pub fn registrar_cliente_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearClienteDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let id_cliente =
        ClienteService::registrar_cliente(&tx, payload, Some(id_u), Some(&username), Some(&rol))?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(id_cliente)
}

#[tauri::command]
pub fn actualizar_cliente_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: EditarClienteDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::actualizar_cliente(&tx, payload, Some(id_u), Some(&username), Some(&rol))?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn buscar_cliente_por_doc_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    tipo_documento: String,
    numero_documento: String,
) -> Result<ClienteDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;
    ClienteService::buscar_cliente_por_documento(
        &conn,
        &tipo_documento,
        &numero_documento,
        Some(&rol),
    )
}

#[tauri::command]
pub fn buscar_clientes_por_termino_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    termino: String,
) -> Result<Vec<ClienteDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;
    ClienteService::buscar_clientes_por_termino(&conn, &termino, Some(&rol))
}

#[tauri::command]
pub fn registrar_vehiculo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearVehiculoPayloadDto,
) -> Result<String, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let placa =
        ClienteService::registrar_vehiculo(&tx, payload, Some(id_u), Some(&username), Some(&rol))?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(placa)
}

#[tauri::command]
pub fn actualizar_vehiculo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: EditarVehiculoPayloadDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) =
        verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::actualizar_vehiculo(&tx, payload, Some(id_u), Some(&username), &rol)?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn cambiar_estado_vehiculo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CambiarEstadoVehiculoPayloadDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::cambiar_estado_vehiculo(&tx, payload, Some(id_u), Some(&username), Some(&rol))?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn corregir_historial_ot_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CorregirHistorialOtPayloadDto,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let mut conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (id_u, username, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::corregir_historial_ot(&tx, payload, Some(id_u), Some(&username), Some(&rol))?;
    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn obtener_vehiculo_por_placa_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
) -> Result<VehiculoCompletoDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;
    ClienteService::obtener_vehiculo_por_placa(&conn, &placa, Some(&rol))
}

#[tauri::command]
pub fn obtener_ficha_vehicular_completa_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
    pagina: Option<u32>,
    limite: Option<u32>,
) -> Result<FichaVehicularCompletaDto, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;
    ClienteService::obtener_ficha_vehicular_completa(
        &conn,
        &placa,
        Some(&rol),
        pagina.unwrap_or(1),
        limite.unwrap_or(20),
    )
}

#[tauri::command]
pub fn obtener_proximos_cambios_aceite_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    pagina: Option<u32>,
    limite: Option<u32>,
    busqueda: Option<String>,
    filtro_causa: Option<String>,
) -> Result<ResultadoPaginadoDto<ProximoCambioAceiteDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;
    crate::services::cliente_service::ficha_vehicular::obtener_proximos_cambios_aceite_paginado(
        &conn,
        Some(&rol),
        pagina.unwrap_or(1),
        limite.unwrap_or(20),
        busqueda.as_deref(),
        filtro_causa.as_deref(),
    )
}

#[tauri::command]
pub fn buscar_vehiculos_candidatos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    busqueda: String,
) -> Result<Vec<VehiculoCompletoDto>, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo sesión".into()))?;
    let conn = db_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo BD".into()))?;

    let (_, _, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    let termino = format!("%{}%", busqueda.trim().to_uppercase());

    let mut stmt = conn.prepare(
        "SELECT v.placa
         FROM vehiculos v
         JOIN clientes c ON c.id_cliente = v.id_cliente
         WHERE v.activo = 1 
           AND (v.placa LIKE ?1 OR c.nombre_razon_social LIKE ?1 OR c.numero_documento LIKE ?1)
         ORDER BY v.placa ASC
         LIMIT 15",
    )?;

    let iter = stmt.query_map([termino], |row| row.get::<_, String>(0))?;

    let mut lista = Vec::new();
    for placa_res in iter {
        let placa = placa_res?;
        if let Ok(vehiculo_dto) =
            ClienteService::obtener_vehiculo_por_placa(&conn, &placa, Some(&rol))
        {
            lista.push(vehiculo_dto);
        }
    }

    Ok(lista)
}

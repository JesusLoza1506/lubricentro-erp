use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto};
use crate::dto::vehiculo_dto::{
    CrearVehiculoPayloadDto, EditarVehiculoPayloadDto, FichaVehicularCompletaDto,
    VehiculoCompletoDto,
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

#[tauri::command]
pub fn registrar_cliente_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearClienteDto,
) -> Result<i64, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    ClienteService::registrar_cliente(&conn, payload)
}

#[tauri::command]
pub fn buscar_cliente_por_doc_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    doc: String,
) -> Result<ClienteDto, AppError> {
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

    ClienteService::buscar_cliente_por_documento(&conn, &doc, Some(&rol))
}

#[tauri::command]
pub fn registrar_vehiculo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    payload: CrearVehiculoPayloadDto,
) -> Result<String, AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let placa = ClienteService::registrar_vehiculo(&tx, payload, Some(id_u))?;
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
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let mut conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR", "CAJERO"])?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    ClienteService::actualizar_vehiculo(&tx, payload, Some(id_u))?;
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
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (_, rol) = verificar_permiso_sesion(
        &mut session,
        &conn,
        &["ADMINISTRADOR", "CAJERO", "MECANICO"],
    )?;

    ClienteService::obtener_vehiculo_por_placa(&conn, &placa, Some(&rol))
}

#[tauri::command]
pub fn obtener_todos_vehiculos_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<VehiculoCompletoDto>, AppError> {
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

    let mut stmt = conn
        .prepare(
            "SELECT 
                v.placa, 
                v.marca, 
                v.modelo, 
                v.anio, 
                v.tipo_motor, 
                v.kilometraje_actual, 
                v.id_cliente, 
                v.activo,
                c.nombre_razon_social,
                c.tipo_documento,
                c.numero_documento,
                c.direccion,
                c.telefono
             FROM vehiculos v
             LEFT JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.activo = 1
             ORDER BY v.placa ASC",
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let es_mecanico = rol == "MECANICO";

    let rows = stmt
        .query_map([], |row| {
            let id_cliente: Option<i64> = row.get(6)?;
            let nombre_cliente: Option<String> = row.get(8)?;
            let tipo_doc: Option<String> = row.get(9)?;
            let num_doc: Option<String> = row.get(10)?;
            let mut direccion: Option<String> = row.get(11)?;
            let mut telefono: Option<String> = row.get(12)?;

            // Filtrar datos privados si es MECANICO
            if es_mecanico {
                telefono = None;
                direccion = None;
            }

            let cliente_struct = if id_cliente.is_some() {
                Some(crate::dto::vehiculo_dto::ClienteResumenDto {
                    id_cliente: id_cliente.unwrap_or(0),
                    tipo_documento: tipo_doc.clone().unwrap_or_default(),
                    numero_documento: num_doc.clone().unwrap_or_default(),
                    nombre_razon_social: nombre_cliente.clone().unwrap_or_default(),
                    telefono: telefono.clone(),
                    direccion: direccion.clone(),
                })
            } else {
                None
            };

            Ok(VehiculoCompletoDto {
                placa: row.get(0)?,
                marca: row.get(1)?,
                modelo: row.get(2)?,
                anio: row.get(3)?,
                tipo_motor: row.get(4)?,
                kilometraje_actual: row.get(5)?,
                id_cliente: id_cliente.unwrap_or(0),
                activo: row.get::<_, i32>(7)?,
                cliente: cliente_struct,
                nombre_cliente,
                telefono_cliente: telefono,
                tipo_documento_cliente: tipo_doc,
                documento_cliente: num_doc,
                direccion_cliente: direccion,
            })
        })
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut lista = Vec::new();
    for v in rows.flatten() {
        lista.push(v);
    }

    Ok(lista)
}

#[tauri::command]
pub fn obtener_ficha_vehicular_completa_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
) -> Result<FichaVehicularCompletaDto, AppError> {
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

    ClienteService::obtener_ficha_vehicular_completa(&conn, &placa, Some(&rol))
}

#[tauri::command]
pub fn eliminar_vehiculo_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
    placa: String,
) -> Result<(), AppError> {
    let mut session = session_state
        .lock()
        .map_err(|_| AppError::Validation("Fallo al obtener estado de sesión".to_string()))?;

    let conn = db_state.lock().map_err(|_| {
        AppError::Validation("Fallo al obtener estado de la base de datos".to_string())
    })?;

    let (id_u, _) = verificar_permiso_sesion(&mut session, &conn, &["ADMINISTRADOR"])?;

    ClienteService::eliminar_vehiculo(&conn, &placa, Some(id_u))
}

pub mod calculos;
pub mod consultas;
pub mod devoluciones;
pub mod documentos;
mod stock;
pub mod ventas;

use crate::errors::AppError;
use rusqlite::{params, Connection};
use serde_json::{json, Value};

pub fn validar_rol(conn: &Connection, actor: i64) -> Result<String, AppError> {
    let rol: String = conn.query_row("SELECT rol FROM usuarios WHERE id_usuario=?1 AND activo=1 AND debe_cambiar_clave=0 AND (bloqueado_hasta IS NULL OR bloqueado_hasta<=datetime('now','-5 hours'))",[actor],|r|r.get(0))?;
    if !["ADMINISTRADOR", "CAJERO"].contains(&rol.as_str()) {
        return Err(AppError::PermisoDenegado);
    }
    Ok(rol)
}
pub fn motivo(s: &str) -> Result<(), AppError> {
    if !(5..=250).contains(&s.trim().chars().count()) {
        return Err(error("El motivo debe tener entre 5 y 250 caracteres."));
    }
    Ok(())
}
pub fn error(s: &str) -> AppError {
    AppError::Validation(s.into())
}
pub fn json_error(e: serde_json::Error) -> AppError {
    error(&format!("Datos de POS inválidos: {e}"))
}
pub fn caja_propia(conn: &Connection, actor: i64) -> Result<i64, AppError> {
    conn.query_row("SELECT id_caja FROM caja_chica WHERE id_usuario=?1 AND estado='ABIERTA' AND por_conciliar=0",[actor],|r|r.get(0))
        .map_err(|_|error("Necesitas una caja propia abierta y operativa para cobrar o reintegrar."))
}
#[allow(clippy::too_many_arguments)]
pub fn auditar(
    conn: &Connection,
    actor: i64,
    op: &str,
    accion: &str,
    entidad: &str,
    id: i64,
    datos: &Value,
    razon: &str,
    admin: Option<i64>,
) -> Result<(), AppError> {
    conn.execute("INSERT INTO auditoria(id_operacion,id_usuario,actor,username_snapshot,rol_snapshot,id_autorizante,modulo,accion,entidad,id_entidad,despues_json,motivo,resultado) SELECT ?1,id_usuario,'USUARIO',username,rol,?2,'POS',?3,?4,?5,?6,?7,'OK' FROM usuarios WHERE id_usuario=?8",params![op,admin,accion,entidad,id.to_string(),datos.to_string(),if razon.is_empty(){None}else{Some(razon)},actor])?;
    Ok(())
}
pub fn configuracion(conn: &Connection) -> Result<Value, AppError> {
    Ok(conn.query_row("SELECT ambiente_actual,ruc,razon_social,direccion,ubigeo,nubefact_ruta_api,nubefact_token_ref,recuperacion_pendiente+revision_caja_pendiente+revision_token_pendiente+revision_series_pendiente FROM configuracion WHERE id_config=1",[],|r|Ok(json!({"ambiente":r.get::<_,String>(0)?,"ruc":r.get::<_,Option<String>>(1)?,"razon_social":r.get::<_,Option<String>>(2)?,"direccion":r.get::<_,Option<String>>(3)?,"ubigeo":r.get::<_,Option<String>>(4)?,"ruta":r.get::<_,Option<String>>(5)?,"token_ref":r.get::<_,Option<String>>(6)?,"recuperacion":r.get::<_,i64>(7)?})))?)
}
pub fn validar_config(c: &Value) -> Result<(), AppError> {
    for k in [
        "ruc",
        "razon_social",
        "direccion",
        "ubigeo",
        "ruta",
        "token_ref",
    ] {
        if c[k].as_str().unwrap_or("").trim().is_empty() {
            return Err(error("Completa empresa, ubicación, ruta y referencia del token en Administración antes de cobrar."));
        }
    }
    if c["recuperacion"].as_i64() != Some(0) {
        return Err(error(
            "Completa los requisitos de recuperación antes de operar.",
        ));
    }
    if !c["ruta"].as_str().unwrap_or("").starts_with("https://") {
        return Err(error("La ruta NubeFact debe utilizar HTTPS."));
    }
    Ok(())
}

pub fn auditar_denegacion(
    conn: &Connection,
    actor: i64,
    accion: &str,
    op: &str,
) -> Result<(), AppError> {
    conn.execute("INSERT INTO auditoria(id_operacion,id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,resultado) SELECT ?1,id_usuario,'USUARIO',username,rol,'POS',?2,'DENEGADO' FROM usuarios WHERE id_usuario=?3",params![op,format!("AUTORIZACION_DENEGADA_{accion}"),actor])?;
    Ok(())
}

pub mod operaciones;

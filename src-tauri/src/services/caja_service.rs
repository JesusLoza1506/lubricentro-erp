mod consultas;
mod escrituras;

use crate::{entities::caja_chica::CajaChicaEntity, errors::AppError};
use rusqlite::{Connection, OptionalExtension};

pub struct CajaService;

const LIMITE_FONDO: i64 = 1_000_000;
const LIMITE_CONTEO: i64 = 9_999_999_999;

fn invalido(mensaje: &str) -> AppError {
    AppError::Validation(mensaje.into())
}

fn es_administrador(conn: &Connection, actor: i64) -> Result<bool, AppError> {
    let rol: Option<String> = conn
        .query_row(
            "SELECT rol FROM usuarios WHERE id_usuario=?1 AND activo=1 AND debe_cambiar_clave=0 AND (bloqueado_hasta IS NULL OR bloqueado_hasta<=datetime('now','-5 hours'))",
            [actor],
            |r| r.get(0),
        )
        .optional()?;
    match rol.as_deref() {
        Some("ADMINISTRADOR") => Ok(true),
        Some("CAJERO") => Ok(false),
        _ => Err(AppError::PermisoDenegado),
    }
}

fn configuracion(conn: &Connection) -> Result<(String, bool), AppError> {
    Ok(conn.query_row(
        "SELECT ambiente_actual, recuperacion_pendiente FROM configuracion WHERE id_config=1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?)
}

fn modo_cierre(
    turno: &CajaChicaEntity,
    actor: i64,
    admin: bool,
    recuperacion: bool,
) -> Option<&'static str> {
    if turno.estado != "ABIERTA" {
        return None;
    }
    if turno.por_conciliar {
        return admin.then_some("RECUPERACION");
    }
    if recuperacion {
        return None;
    }
    if turno.id_usuario == actor {
        Some("NORMAL")
    } else {
        admin.then_some("AUSENCIA")
    }
}

use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    Auth(String),
    AccessDenied(String),
    Validation(String),
    NotFound(String),
    NoAutenticado,
    PermisoDenegado,
}

/// Mapea los errores técnicos de SQLite a mensajes amigables para el usuario final (2.4.2)
fn mapear_error_sqlite(err: &rusqlite::Error) -> String {
    match err {
        rusqlite::Error::SqliteFailure(_sqlite_err, Some(msg)) => {
            let msg_lower = msg.to_lowercase();
            if msg_lower.contains("foreign key constraint failed") {
                "No se pudo completar la operación debido a dependencias activas con otros registros.".to_string()
            } else if msg_lower.contains("unique constraint failed") {
                "El registro ya existe en el sistema. Verifique que los datos no estén duplicados."
                    .to_string()
            } else if msg_lower.contains("database is locked")
                || msg_lower.contains("database table is locked")
            {
                "La base de datos se encuentra ocupada procesando otra transacción. Por favor, reintente en unos segundos.".to_string()
            } else {
                format!("Error en el almacenamiento de datos: {}", msg)
            }
        }
        rusqlite::Error::SqliteFailure(_, None) => {
            "Ocurrió un error en la base de datos local.".to_string()
        }
        rusqlite::Error::QueryReturnedNoRows => {
            "No se encontró el registro solicitado en la base de datos.".to_string()
        }
        _ => format!("Error de Base de Datos: {}", err),
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "{}", mapear_error_sqlite(err)),
            AppError::Auth(msg) => write!(f, "{}", msg),
            AppError::AccessDenied(msg) => write!(f, "Acceso denegado: {}", msg),
            AppError::Validation(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
            AppError::NoAutenticado => write!(f, "Debes iniciar sesión para realizar esta acción."),
            AppError::PermisoDenegado => write!(
                f,
                "No tienes permisos suficientes para realizar esta operación."
            ),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

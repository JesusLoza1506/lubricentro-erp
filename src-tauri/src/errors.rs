use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    Auth(String),
    AccessDenied(String),
    Validation(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "Error de Base de Datos: {}", err),
            AppError::Auth(msg) => write!(f, "Error de Autenticación: {}", msg),
            AppError::AccessDenied(msg) => write!(f, "Acceso Denegado: {}", msg),
            AppError::Validation(msg) => write!(f, "Error de Validación: {}", msg),
            AppError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err)
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

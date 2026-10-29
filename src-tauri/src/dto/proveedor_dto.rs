use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveedorDto {
    pub id_proveedor: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub contacto_asesor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CrearProveedorDto {
    pub tipo_documento: String, // DNI | RUC | CE
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub contacto_asesor: Option<String>,
}

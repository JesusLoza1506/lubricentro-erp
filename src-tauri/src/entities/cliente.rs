#[derive(Debug, Clone, PartialEq)]
pub struct ClienteEntity {
    pub id_cliente: i32,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre_razon_social: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColaEnvioSunatEntity {
    pub id_cola: i32,
    pub id_comprobante: i32,
    pub estado_envio: String,
    pub intentos: i32,
    pub ultima_error: Option<String>,
    pub respuesta_cdr: Option<String>,
    pub fecha_creacion: Option<String>,
    pub fecha_ultimo_intento: Option<String>,
    pub fecha_confirmacion: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuditoriaEntity {
    pub id_auditoria: i32,
    pub id_usuario: Option<i32>,
    pub tabla_afectada: String,
    pub accion: String,
    pub detalle: Option<String>,
    pub fecha: Option<String>,
}

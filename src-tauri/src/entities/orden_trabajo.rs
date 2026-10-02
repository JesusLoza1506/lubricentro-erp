use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdenTrabajoEntity {
    pub id_ot: i64,
    pub codigo_ot: String,
    pub placa: String,
    pub id_mecanico: i64,
    pub zanja: Option<i32>,
    pub estado: String,
    pub kilometraje_ingreso: i64,
    pub proximo_kilometraje: i64,
    pub observaciones: Option<String>,
    pub fecha_ingreso: Option<String>,
}

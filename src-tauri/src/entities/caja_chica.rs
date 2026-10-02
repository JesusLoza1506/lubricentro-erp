use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CajaChicaEntity {
    pub id_caja: i64,
    pub id_usuario: i64,
    pub monto_apertura: i64,
    pub monto_cierre_efectivo: Option<i64>,
    pub monto_cierre_digital: Option<i64>,
    pub monto_diferencia: Option<i64>,
    pub estado: String,
    pub fecha_apertura: Option<String>,
    pub fecha_cierre: Option<String>,
}

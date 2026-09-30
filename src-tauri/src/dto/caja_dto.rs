use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CierreCajaDTO {
    pub id_caja: i32,
    pub monto_cierre_efectivo: f64,
    pub monto_cierre_digital: f64,
    pub monto_diferencia: f64,
    pub estado: String,
}

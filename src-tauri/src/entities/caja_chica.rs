#[derive(Debug, Clone, PartialEq)]
pub struct CajaChicaEntity {
    pub id_caja: i32,
    pub id_usuario: i32,
    pub monto_apertura: f64,
    pub monto_cierre_efectivo: Option<f64>,
    pub monto_cierre_digital: Option<f64>,
    pub monto_diferencia: Option<f64>,
    pub estado: String,
    pub fecha_apertura: Option<String>,
    pub fecha_cierre: Option<String>,
}

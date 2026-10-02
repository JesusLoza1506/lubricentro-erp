use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardDto {
    pub rol: String,
    pub ventas_del_dia: f64,
    pub comprobantes_emitidos: i64,
    pub ots_activas: i64,
    pub productos_stock_critico: i64,
    pub estado_caja: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertaFidelizacionDto {
    pub placa: String,
    pub marca: String,
    pub modelo: String,
    pub kilometraje_actual: i64,
    pub proximo_kilometraje: i64,
    pub nombre_cliente: String,
    pub telefono: Option<String>,
}

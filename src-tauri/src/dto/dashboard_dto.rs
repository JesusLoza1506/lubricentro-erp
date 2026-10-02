use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardDto {
    pub rol: String,
    pub total_ventas_hoy: Option<i64>, // En céntimos (Solo Admin/Cajero)
    pub ot_en_proceso: i64,
    pub productos_stock_bajo: i64,
    pub alertas_fidelizacion: i64, // Vehículos que superan el próximo kilometraje
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

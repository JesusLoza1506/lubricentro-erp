pub mod ajustes;
pub mod compras;
pub mod idempotencia;
pub mod kardex;
pub mod productos;
pub mod proveedores;
pub mod servicios;

use crate::dto::producto_dto::ProductoAdminDto;
use crate::errors::AppError;
use chrono::{FixedOffset, Utc};
use serde::Serialize;

pub struct InventarioService;

pub(crate) const CATEGORIAS_VALIDAS: [&str; 5] = ["ACEITE", "FILTRO", "ADITIVO", "GRANEL", "OTRO"];

/// Retorna la hora actual en la zona horaria de Lima (UTC-5)
pub(crate) fn obtener_fecha_lima() -> chrono::DateTime<FixedOffset> {
    let offset = FixedOffset::west_opt(5 * 3600).expect("Offset de Lima válido");
    Utc::now().with_timezone(&offset)
}

#[derive(Debug, Serialize)]
pub struct ResumenSaldosKardexDto {
    pub saldo_inicial: i64,
    pub saldo_final: i64,
    pub saldo_actual: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductosPaginadosDto {
    pub productos: Vec<ProductoAdminDto>,
    pub total_registros: i64,
}

pub fn calcular_promedio_ponderado_micro(
    valor_anterior_micro: i64,
    valor_aporte_micro: i64,
    stock_total: i64,
) -> Result<i64, AppError> {
    if stock_total <= 0 {
        return Ok(0);
    }
    let total_valor = (valor_anterior_micro as i128) + (valor_aporte_micro as i128);
    let num = total_valor;
    let den = stock_total as i128;

    let resultado = (num + (den / 2)) / den;
    Ok(resultado as i64)
}

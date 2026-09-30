use crate::dto::caja_dto::CierreCajaDTO;
use crate::services::caja_service::CajaService;

#[tauri::command]
pub async fn cerrar_caja_cmd(
    id_caja: i32,
    monto_apertura: f64,
    ventas_efectivo: f64,
    efectivo_conteo: f64,
    digital_conteo: f64,
) -> Result<CierreCajaDTO, String> {
    Ok(CajaService::calcular_cierre(
        id_caja,
        monto_apertura,
        ventas_efectivo,
        efectivo_conteo,
        digital_conteo,
    ))
}

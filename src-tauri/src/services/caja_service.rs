use crate::dto::caja_dto::CierreCajaDTO;

pub struct CajaService;

impl CajaService {
    pub fn calcular_cierre(
        id_caja: i32,
        monto_apertura: f64,
        ventas_efectivo: f64,
        efectivo_conteo: f64,
        digital_conteo: f64,
    ) -> CierreCajaDTO {
        let saldo_teorico = monto_apertura + ventas_efectivo;
        let diferencia = efectivo_conteo - saldo_teorico;

        CierreCajaDTO {
            id_caja,
            monto_cierre_efectivo: efectivo_conteo,
            monto_cierre_digital: digital_conteo,
            monto_diferencia: diferencia,
            estado: "CERRADA".to_string(),
        }
    }
}

use crate::dto::comprobante_dto::ComprobanteDTO;
use crate::entities::comprobante::ComprobanteEntity;

pub struct ComprobanteService;

impl ComprobanteService {
    pub fn mapear_a_dto(entity: ComprobanteEntity, serie_str: Option<String>) -> ComprobanteDTO {
        ComprobanteDTO {
            id_comprobante: entity.id_comprobante,
            tipo_comprobante: entity.tipo_comprobante,
            serie: serie_str,
            correlativo: entity.correlativo,
            monto_subtotal: entity.monto_subtotal,
            monto_igv: entity.monto_igv,
            monto_total: entity.monto_total,
            medio_pago: entity.medio_pago,
            estado_sunat: entity.estado_sunat,
            fecha_emision: entity.fecha_emision,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapear_comprobante_a_dto() {
        let entity = ComprobanteEntity {
            id_comprobante: 1,
            id_ot: None,
            id_cliente: 1,
            id_cajero: 1,
            tipo_comprobante: "01".to_string(),
            id_serie: Some(1),
            correlativo: 10,
            monto_subtotal: 100.0,
            monto_igv: 18.0,
            monto_total: 118.0,
            medio_pago: "Efectivo".to_string(),
            estado_sunat: "PENDIENTE".to_string(),
            fecha_emision: None,
        };

        let dto = ComprobanteService::mapear_a_dto(entity, Some("FFF1".to_string()));
        assert_eq!(dto.correlativo, 10);
        assert_eq!(dto.serie.unwrap(), "FFF1");
    }
}

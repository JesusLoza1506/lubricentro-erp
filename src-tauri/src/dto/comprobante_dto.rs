use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComprobanteDto {
    pub id_comprobante: i64,
    pub tipo_comprobante: String,
    pub serie: Option<String>,
    pub correlativo: i64,
    pub monto_subtotal: i64,
    pub monto_igv: i64,
    pub monto_total: i64,
    pub estado_sunat: String,
    pub fecha_emision: Option<String>,
}

/// DTO de entrada para emitir comprobantes con montos en céntimos enteros.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmitirComprobanteDto {
    pub id_ot: Option<i64>,
    pub id_cliente: i64,
    pub id_cajero: i64,
    pub id_caja: i64,
    pub tipo_comprobante: String, // 'BOLETA', 'FACTURA', 'PROFORMA'
    pub id_serie: Option<i64>,
    pub monto_subtotal: i64,
    pub monto_igv: i64,
    pub monto_total: i64,
    pub medio_pago: String, // 'EFECTIVO', 'YAPE', 'PLIN', 'TARJETA'
    pub monto_pago: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dto_entrada_centimos() {
        let payload_json = r#"{
            "id_ot": null,
            "id_cliente": 1,
            "id_cajero": 1,
            "id_caja": 1,
            "tipo_comprobante": "BOLETA",
            "id_serie": 1,
            "monto_subtotal": 10000,
            "monto_igv": 1800,
            "monto_total": 11800,
            "medio_pago": "EFECTIVO",
            "monto_pago": 11800
        }"#;

        let dto: Result<EmitirComprobanteDto, _> = serde_json::from_str(payload_json);
        assert!(dto.is_ok());
        assert_eq!(dto.unwrap().monto_total, 11800);
    }
}

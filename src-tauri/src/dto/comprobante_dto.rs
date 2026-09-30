use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComprobanteDTO {
    pub id_comprobante: i32,
    pub tipo_comprobante: String,
    pub serie: Option<String>,
    pub correlativo: i32,
    pub monto_subtotal: f64,
    pub monto_igv: f64,
    pub monto_total: f64,
    pub medio_pago: String,
    pub estado_sunat: String,
    pub fecha_emision: Option<String>,
}

/// DTO de entrada para emitir comprobantes. Omite id_serie y correlativo para obligar a su cálculo atómico.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmitirComprobanteDTO {
    pub id_cliente: i32,
    pub id_cajero: i32,
    pub tipo_comprobante: String,
    pub medio_pago: String,
    pub monto_total: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dto_entrada_omitir_correlativo_manual() {
        let payload_json = r#"{
            "id_cliente": 1,
            "id_cajero": 1,
            "tipo_comprobante": "01",
            "medio_pago": "Efectivo",
            "monto_total": 118.0
        }"#;

        let dto: Result<EmitirComprobanteDTO, _> = serde_json::from_str(payload_json);
        assert!(dto.is_ok());
    }
}

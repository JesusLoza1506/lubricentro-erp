use crate::dto::comprobante_dto::{ComprobanteDTO, EmitirComprobanteDTO};

#[tauri::command]
pub async fn emitir_comprobante_cmd(
    payload: EmitirComprobanteDTO,
) -> Result<ComprobanteDTO, String> {
    let subtotal = payload.monto_total / 1.18;
    let igv = payload.monto_total - subtotal;

    Ok(ComprobanteDTO {
        id_comprobante: 100,
        tipo_comprobante: payload.tipo_comprobante,
        serie: Some("FFF1".to_string()),
        correlativo: 69,
        monto_subtotal: subtotal,
        monto_igv: igv,
        monto_total: payload.monto_total,
        medio_pago: payload.medio_pago,
        estado_sunat: "PENDIENTE".to_string(),
        fecha_emision: Some("2026-09-29 22:30:00".to_string()),
    })
}

use lubricentro_erp_lib::nubefact::{
    enviar_a_nubefact, ComprobantePayload, EstadoEnvio, ItemPayload,
};

#[tokio::test]
async fn test_envio_nubefact_sandbox_aceptado() {
    let payload = ComprobantePayload {
        operacion: "generar_comprobante".to_string(),
        tipo_de_comprobante: 1,
        serie: "FFF1".to_string(),
        numero: 999,
        sunat_transaction: 1,
        cliente_tipo_de_documento: 6,
        cliente_numero_de_documento: "20600695771".to_string(),
        cliente_denominacion: "NUBEFACT SA".to_string(),
        cliente_direccion: "AV. LIBERTAD 123 - LIMA".to_string(),
        cliente_email: "prueba@gmail.com".to_string(),
        fecha_de_emision: chrono::Local::now().format("%d-%m-%Y").to_string(),
        moneda: 1,
        porcentaje_de_igv: 18.0,
        total_gravada: 100.0,
        total_igv: 18.0,
        total: 118.0,
        detraccion: false,
        enviar_automaticamente_a_la_sunat: true,
        enviar_automaticamente_al_cliente: false,
        cancelado: true,
        items: vec![ItemPayload {
            unidad_de_medida: "NIU".to_string(),
            codigo: "001".to_string(),
            codigo_producto_sunat: "10000000".to_string(),
            descripcion: "Servicio Test CI/CD".to_string(),
            cantidad: 1.0,
            valor_unitario: 100.0,
            precio_unitario: 118.0,
            descuento: None,
            subtotal: 100.0,
            tipo_de_igv: 1,
            igv: 18.0,
            total: 118.0,
            anticipo_regularizacion: false,
            anticipo_documento_serie: "".to_string(),
            anticipo_documento_numero: "".to_string(),
        }],
    };

    let res = enviar_a_nubefact(&payload).await;
    assert!(res.is_ok());
    let estado = res.unwrap();
    assert!(estado == EstadoEnvio::Aceptado || estado == EstadoEnvio::Rechazado);
}

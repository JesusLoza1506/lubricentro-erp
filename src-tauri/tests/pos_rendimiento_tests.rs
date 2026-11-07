use lubricentro_erp_lib::{
    db::init_db,
    dto::pos_dto::*,
    services::pos_service::{consultas, ventas},
};
use serde_json::json;
use std::time::{Duration, Instant};
#[test]
#[ignore = "Medición explícita con 5000 ventas en SQLite temporal en disco; no mide WebView ni NubeFact"]
fn cinco_mediciones_pos_con_5000_ventas() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = init_db(dir.path().into()).unwrap();
    c.execute_batch("INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES(1,'Administrador prueba','admin','$argon2id$prueba','ADMINISTRADOR'); UPDATE configuracion SET ruc='20600695771',razon_social='Empresa de prueba',direccion='Av. Principal 123',ubigeo='110201',nubefact_ruta_api='https://api.nubefact.com/api/v1/prueba',nubefact_token_ref='token_prueba'; INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,activa) VALUES('PRODUCCION','BOLETA','B001',1); INSERT INTO caja_chica(id_operacion,codigo_caja,id_usuario,ambiente,monto_apertura_cent) VALUES('apertura-perf-pos','CAJA-PERF',1,'PRODUCCION',10000); INSERT INTO servicios(id_servicio,codigo,nombre,precio_venta_cent) VALUES(1,'SER-PERF','Servicio de prueba',100);").unwrap();
    let solicitud = || ConfirmarVentaPos {
        id_operacion: uuid::Uuid::new_v4().to_string(),
        id_ot: None,
        id_cliente: 1,
        tipo_comprobante: "BOLETA".into(),
        descuento_bp: 0,
        motivo: "".into(),
        lineas: vec![LineaDirecta {
            tipo: "SERVICIO".into(),
            id: 1,
            cantidad: 1,
            precio_esperado_cent: 100,
        }],
        pagos: vec![PagoPos {
            medio: "EFECTIVO".into(),
            monto_cent: 100,
            recibido_cent: Some(100),
            referencia: None,
        }],
    };
    for _ in 0..5000 {
        ventas::confirmar(&mut c, 1, None, &solicitud()).unwrap();
    }
    for i in 1..=5 {
        let inicio = Instant::now();
        let r = consultas::listado(&c, 1, &json!({"pagina":1})).unwrap();
        let consulta = inicio.elapsed();
        assert_eq!(r["filas"].as_array().unwrap().len(), 20);
        let inicio = Instant::now();
        ventas::confirmar(&mut c, 1, None, &solicitud()).unwrap();
        let confirmar = inicio.elapsed();
        println!(
            "Muestra {i}: listado={:.3} ms; confirmación local={:.3} ms",
            consulta.as_secs_f64() * 1000.0,
            confirmar.as_secs_f64() * 1000.0
        );
        assert!(consulta < Duration::from_secs(3));
        assert!(confirmar < Duration::from_secs(3));
    }
}

use lubricentro_erp_lib::{
    dto::producto_dto::AjustarStockDto, services::inventario_service::InventarioService,
};
use rusqlite::{params, Connection};
use std::time::Instant;

/// Solo datos temporales. Ejecutar explícitamente; no toca la BD del usuario.
#[test]
#[ignore = "Medición explícita de consultas con 1000 productos y 10000 movimientos"]
fn cinco_mediciones_catalogo_y_kardex_con_volumen() {
    let mut c = Connection::open_in_memory().unwrap();
    c.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
    c.execute_batch(include_str!("../migrations/0001_initial_schema_v7.sql"))
        .unwrap();
    c.execute("INSERT INTO usuarios(id_usuario,nombre_completo,username,password_hash,rol) VALUES(1,'Administrador prueba','admin_prueba','$argon2id$prueba','ADMINISTRADOR')",[]).unwrap();
    for id in 1..=1000 {
        c.execute("INSERT INTO productos(id_producto,codigo,nombre,categoria,unidad_medida,stock_minimo,precio_venta_cent) VALUES(?1,?2,?3,'FILTRO','UND',15,2000)",params![id,format!("PROD-{id:06}"),format!("Filtro de aceite {id:04}")]).unwrap();
        for _ in 0..10 {
            InventarioService::ajustar_stock(
                &mut c,
                AjustarStockDto {
                    id_producto: id,
                    tipo_ajuste: "ENTRADA".into(),
                    cantidad: 1,
                    costo_unitario_micro: Some(1230000),
                    motivo: "Ingreso para medición de rendimiento".into(),
                },
                None,
                1,
            )
            .unwrap();
        }
    }
    assert_eq!(
        10000,
        c.query_row("SELECT count(*) FROM movimientos_inventario", [], |r| r
            .get::<_, i64>(0))
            .unwrap()
    );
    for muestra in 1..=5 {
        let inicio = Instant::now();
        let pagina = InventarioService::listar_productos_publicos_consulta(
            &c,
            Some("Filtro".into()),
            Some("FILTRO".into()),
            20,
            0,
            false,
            None,
            None,
        )
        .unwrap();
        assert_eq!(pagina.total_registros, 1000);
        assert_eq!(pagina.productos.len(), 20);
        let catalogo_ms = inicio.elapsed().as_secs_f64() * 1000.0;
        let inicio = Instant::now();
        let pagina = InventarioService::listar_productos_admin_consulta(
            &c, None, None, None, 20, 0, true, None, None,
        )
        .unwrap();
        assert_eq!(pagina.total_registros, 1000);
        let criticos_ms = inicio.elapsed().as_secs_f64() * 1000.0;
        let inicio = Instant::now();
        assert_eq!(
            InventarioService::listar_kardex_producto(&c, 1, 20, 0, None, None, None)
                .unwrap()
                .len(),
            10
        );
        let kardex_ms = inicio.elapsed().as_secs_f64() * 1000.0;
        println!("Medición {muestra}: catálogo={catalogo_ms:.3} ms; críticos={criticos_ms:.3} ms; kardex={kardex_ms:.3} ms");
        assert!(catalogo_ms < 3000.0 && criticos_ms < 3000.0 && kardex_ms < 3000.0);
    }
}

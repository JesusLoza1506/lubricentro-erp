use lubricentro_erp_lib::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto, ReasignarMecanicoOtDto,
};
use lubricentro_erp_lib::services::ordenes_service::OrdenesService;
use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    // Cargar esquema DDL V5 oficial
    let schema_v5 = include_str!("../migrations/0001_initial_schema_v7.sql");
    conn.execute_batch(schema_v5).unwrap();

    // Insertar datos de prueba sin colisionar con id_cliente = 1 (Cliente varios ya insertado en V5)
    conn.execute_batch(
        "
        -- Configuración inicial
        UPDATE configuracion SET revision_cuentas_pendiente = 0, revision_series_pendiente = 0 WHERE id_config = 1;

        -- Usuarios para pruebas
        INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES
        (1, 'Admin Principal', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'ADMINISTRADOR', 1),
        (2, 'Juan Mecanico', 'mecanico1', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 1),
        (3, 'Pedro Cajero', 'cajero', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'CAJERO', 1),
        (4, 'Carlos Mecanico 2', 'mecanico2', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 1),
        (5, 'Mecanico Inactivo', 'mecanico_inactivo', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 0),
        (6, 'Admin Inactivo', 'admin_inactivo', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'ADMINISTRADOR', 0);

        -- Cliente identificado con ID 2
        INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, activo) VALUES
        (2, 'DNI', '12345678', 'Cliente Prueba', 1);

        -- Vehículo vinculado al cliente ID 2
        INSERT INTO vehiculos (id_vehiculo, placa, id_cliente, marca, modelo, anio, kilometraje_actual, activo) VALUES
        (1, 'ABC123', 2, 'Toyota', 'Yaris', 2020, 50000, 1);

        -- Productos y Servicios
        INSERT INTO productos (id_producto, codigo, nombre, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, precio_venta_cent, activo) VALUES
        (1, 'PROD-001', 'Aceite 10W-40 Sintetico', 'ACEITE', 'UND', 0, 0, 0, 12000, 1),
        (2, 'PROD-002', 'Filtro de Aceite', 'FILTRO', 'UND', 0, 0, 0, 2500, 1);

        INSERT INTO servicios (id_servicio, codigo, nombre, precio_venta_cent, es_cambio_aceite, activo) VALUES
        (1, 'SERV-001', 'Cambio de Aceite Basico', 2500, 1, 1);
        ",
    )
    .unwrap();

    // Carga de stock en DDL V5 respetando CHECK constraint de origen='INICIAL'
    let id_op = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO ingresos_stock (id_operacion, codigo, origen, referencia_inicial, total_capturado_micro, total_costo_micro, id_usuario, motivo, estado)
         VALUES (?1, 'ING-001', 'INICIAL', 'CARGA-INICIAL-001', 0, 0, 1, 'Carga inicial de inventario para pruebas', 'PREPARANDO')",
        [&id_op],
    ).unwrap();
    let id_ingreso = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_capturado_micro, costo_efectivo_micro, incluye_igv, igv_recuperable, igv_bp, valor_capturado_micro, valor_aporte_micro)
         VALUES (?1, 1, 'PROD-001', 'Aceite 10W-40', 'UND', 10, 0, 0, 0, 0, 1800, 0, 0)",
        [id_ingreso],
    ).unwrap();
    let id_det1 = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_capturado_micro, costo_efectivo_micro, incluye_igv, igv_recuperable, igv_bp, valor_capturado_micro, valor_aporte_micro)
         VALUES (?1, 2, 'PROD-002', 'Filtro de Aceite', 'UND', 2, 0, 0, 0, 0, 1800, 0, 0)",
        [id_ingreso],
    ).unwrap();
    let id_det2 = conn.last_insert_rowid();

    let op_k1 = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
         VALUES (?1, 1, 1, 'INGRESO', ?2, 'UND', 0, 10, 10, 0, 0, 0, 0)",
        [&op_k1, &id_det1.to_string()],
    ).unwrap();

    let op_k2 = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
         VALUES (?1, 2, 1, 'INGRESO', ?2, 'UND', 0, 2, 2, 0, 0, 0, 0)",
        [&op_k2, &id_det2.to_string()],
    ).unwrap();

    conn.execute(
        "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
        [id_ingreso],
    )
    .unwrap();

    conn
}

#[test]
fn test_01_crear_ot_mecanico_inexistente() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 999,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

#[test]
fn test_02_crear_ot_mecanico_inactivo() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 5,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

#[test]
fn test_03_crear_ot_usuario_sin_rol_mecanico() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 3,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

#[test]
fn test_04_reasignar_mecanico_inexistente() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot: ot.id_ot,
        nuevo_id_mecanico: 888,
    };
    assert!(OrdenesService::reasignar_mecanico_ot(&conn, req_reasignar).is_err());
}

#[test]
fn test_05_reasignar_mecanico_inactivo() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot: ot.id_ot,
        nuevo_id_mecanico: 5,
    };
    assert!(OrdenesService::reasignar_mecanico_ot(&conn, req_reasignar).is_err());
}

#[test]
fn test_06_mecanico_bloqueado_en_ot_ajena() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 1.0,
    };

    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 4).is_err());
}

#[test]
fn test_07_admin_agrega_producto_y_auditoria() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 1.0,
    };

    OrdenesService::agregar_producto_ot(&mut conn, payload, 1).unwrap();

    let id_u_mov: i64 = conn
        .query_row(
            "SELECT id_usuario FROM movimientos_inventario ORDER BY id_movimiento DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(id_u_mov, 1);
}

#[test]
fn test_08_stock_insuficiente() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 2,
        cantidad: 5.0,
    };

    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 2).is_err());
}

#[test]
fn test_09_devolucion_stock_al_eliminar_producto() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 2.0,
    };
    let ot_actualizada = OrdenesService::agregar_producto_ot(&mut conn, payload, 2).unwrap();
    let id_detalle = ot_actualizada.detalles[0].id_detalle;

    let stock_despues_agregar: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_despues_agregar, 8.0);

    OrdenesService::eliminar_producto_ot(&mut conn, id_detalle, ot.id_ot, 2).unwrap();

    let stock_restaurado: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_restaurado, 10.0);
}

#[test]
fn test_10_devolucion_stock_al_cancelar_ot() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 3.0,
    };
    OrdenesService::agregar_producto_ot(&mut conn, payload, 2).unwrap();

    let cambio = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "CANCELADO".to_string(),
        zanja: None,
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio, Some(1)).unwrap();

    let stock_final: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_final, 10.0);
}

#[test]
fn test_11_flujo_transiciones_validas() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();
    assert_eq!(ot.estado, "EN_ESPERA");

    let cambio_proceso = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "EN_PROCESO".to_string(),
        zanja: Some(1),
    };
    let ot_proceso = OrdenesService::cambiar_estado_ot(&mut conn, cambio_proceso, Some(2)).unwrap();
    assert_eq!(ot_proceso.estado, "EN_PROCESO");

    let cambio_finalizado = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "FINALIZADO".to_string(),
        zanja: None,
    };
    let ot_fin = OrdenesService::cambiar_estado_ot(&mut conn, cambio_finalizado, Some(2)).unwrap();
    assert_eq!(ot_fin.estado, "FINALIZADO");
}

#[test]
fn test_12_bloqueo_modificacion_estado_terminal() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let cambio_cancelar = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "CANCELADO".to_string(),
        zanja: None,
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio_cancelar, Some(1)).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 1.0,
    };
    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 1).is_err());
}

#[test]
fn test_13_usuario_none_rechazado() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let cambio = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "EN_PROCESO".to_string(),
        zanja: Some(1),
    };
    assert!(OrdenesService::cambiar_estado_ot(&mut conn, cambio, None).is_err());
}

#[test]
fn test_14_usuario_ejecutor_inactivo_rechazado() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 1.0,
    };
    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 6).is_err());
}

#[test]
fn test_15_tipo_aceite_invalido_rechazado() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "DESCONOCIDO".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

#[test]
fn test_16_kilometraje_igual_a_ficha_permitido() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_ok());
}

#[test]
fn test_17_kilometraje_inferior_a_ficha_rechazado() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 40000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

#[test]
fn test_18_finalizacion_libera_zanja() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let cambio_proc = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "EN_PROCESO".to_string(),
        zanja: Some(1),
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio_proc, Some(2)).unwrap();

    let cambio_fin = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "FINALIZADO".to_string(),
        zanja: None,
    };
    let ot_finalizada = OrdenesService::cambiar_estado_ot(&mut conn, cambio_fin, Some(2)).unwrap();

    assert!(ot_finalizada.zanja.is_none());
}

#[test]
fn test_19_indice_unico_parcial_zanja_activa() {
    let conn = setup_test_db();
    let op1 = uuid::Uuid::new_v4().to_string();
    let op2 = uuid::Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO ordenes_trabajo (id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot, id_creador, id_mecanico, id_zanja_ejecucion, estado, kilometraje_ingreso, fecha_inicio)
         VALUES (?1, 'OT-TEST-1', 1, 2, 'ABC123', '{\"nombre\":\"Cliente Prueba\"}', 1, 2, 1, 'EN_PROCESO', 50000, datetime('now', '-5 hours'))",
        [&op1],
    ).unwrap();

    let res = conn.execute(
        "INSERT INTO ordenes_trabajo (id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot, id_creador, id_mecanico, id_zanja_ejecucion, estado, kilometraje_ingreso, fecha_inicio)
         VALUES (?2, 'OT-TEST-2', 1, 2, 'ABC123', '{\"nombre\":\"Cliente Prueba\"}', 1, 2, 1, 'EN_PROCESO', 50000, datetime('now', '-5 hours'))",
        [&op2],
    );

    assert!(res.is_err());
}

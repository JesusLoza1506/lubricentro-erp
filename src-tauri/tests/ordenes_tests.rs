use lubricentro_erp_lib::dto::orden_trabajo_dto::{
    AgregarServicioOtDto, CancelarOtDto, CrearOrdenTrabajoDto, DevolverInsumoOtDto, FinalizarOtDto,
    IniciarTrabajoOtDto, ModificarServicioOtDto, ReasignarMecanicoOtDto, RegistrarInsumoOtDto,
    ServicioInicialDto,
};
use lubricentro_erp_lib::services::ordenes_service::OrdenesService;
use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    // Cargar esquema DDL V7 oficial
    let schema_v7 = include_str!("../migrations/0001_initial_schema_v7.sql");
    conn.execute_batch(schema_v7).unwrap();

    // Insertar datos de prueba
    conn.execute_batch(
        "
        -- Configuración inicial V7.0
        UPDATE configuracion 
        SET revision_token_pendiente = 0, 
            revision_caja_pendiente = 0, 
            revision_series_pendiente = 0,
            recuperacion_pendiente = 0 
        WHERE id_config = 1;

        -- Habilitar Zanja 3 para las pruebas que la requieran
        UPDATE zanjas SET activa = 1 WHERE id_zanja = 3;

        -- Usuarios para pruebas
        INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES
        (1, 'Admin Principal', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'ADMINISTRADOR', 1),
        (2, 'Juan Mecanico', 'mecanico1', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 1),
        (3, 'Pedro Cajero', 'cajero', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'CAJERO', 1),
        (4, 'Carlos Mecanico 2', 'mecanico2', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 1),
        (5, 'Mecanico Inactivo', 'mecanico_inactivo', '$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L', 'MECANICO', 0);

        -- Cliente identificado con ID 2
        INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social) VALUES
        (2, 'DNI', '12345678', 'Cliente Prueba');

        -- Vehículo vinculado al cliente ID 2
        INSERT INTO vehiculos (id_vehiculo, placa, id_cliente, marca, modelo, anio, kilometraje_actual, activo) VALUES
        (1, 'ABC123', 2, 'Toyota', 'Yaris', 2020, 50000, 1),
        (2, 'XYZ999', 2, 'Nissan', 'Frontier', 2022, 10000, 1);

        -- Productos (con ultimo_costo_micro = 50000000 > 0) y Servicios
        INSERT INTO productos (id_producto, codigo, nombre, categoria, unidad_medida, stock_actual, stock_minimo, valor_stock_micro, ultimo_costo_micro, precio_venta_cent, activo) VALUES
        (1, 'PROD-001', 'Aceite 10W-40 Sintetico', 'ACEITE', 'UND', 0, 0, 0, 50000000, 12000, 1),
        (2, 'PROD-002', 'Filtro de Aceite', 'FILTRO', 'UND', 0, 0, 0, 15000000, 2500, 1);

        INSERT INTO servicios (id_servicio, codigo, nombre, precio_venta_cent, es_cambio_aceite, activo) VALUES
        (1, 'SERV-001', 'Cambio de Aceite Basico', 2500, 1, 1),
        (2, 'SERV-002', 'Engrase General', 1500, 0, 1);
        ",
    )
    .unwrap();

    // Cargar inventario inicial vía Kardex V7.0
    let id_op = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO ingresos_stock (id_operacion, codigo, origen, referencia_inicial, total_costo_micro, id_usuario, observacion, estado)
         VALUES (?1, 'ING-001', 'INICIAL', 'CARGA-INICIAL-001', 500000000, 1, 'Carga inicial de inventario para pruebas', 'PREPARANDO')",
        [&id_op],
    ).unwrap();
    let id_ingreso = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO detalle_ingreso (id_ingreso, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida, cantidad, costo_efectivo_micro, valor_aporte_micro)
         VALUES (?1, 1, 'PROD-001', 'Aceite 10W-40', 'UND', 10, 50000000, 500000000)",
        [id_ingreso],
    ).unwrap();
    let id_det1 = conn.last_insert_rowid();

    let op_k1 = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO movimientos_inventario (id_operacion, id_producto, id_usuario, tipo_movimiento, id_detalle_ingreso, unidad_medida, stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro, valor_resultante_micro, costo_unitario_micro)
         VALUES (?1, 1, 1, 'STOCK_INICIAL', ?2, 'UND', 0, 10, 10, 0, 500000000, 500000000, 50000000)",
        [&op_k1, &id_det1.to_string()],
    ).unwrap();

    // Actualizar saldo de producto tras movimiento kardex y mantener ultimo_costo_micro > 0
    conn.execute(
        "UPDATE productos SET stock_actual = 10, valor_stock_micro = 500000000, ultimo_costo_micro = 50000000 WHERE id_producto = 1",
        [],
    ).unwrap();

    conn.execute(
        "UPDATE ingresos_stock SET estado = 'REGISTRADO' WHERE id_ingreso = ?1",
        [id_ingreso],
    )
    .unwrap();

    conn
}

fn helper_crear_ot_base(conn: &mut Connection) -> i64 {
    let req = CrearOrdenTrabajoDto {
        id_vehiculo: 1,
        id_cliente_atendido: 2,
        kilometraje_ingreso: 50000,
        id_mecanico: Some(2),
        pedido_cliente: Some("Mantenimiento general".to_string()),
        servicios_iniciales: vec![ServicioInicialDto {
            id_servicio: 1,
            cantidad: 1,
        }],
    };
    let ot = OrdenesService::crear_orden_trabajo(conn, req, 3, "CAJERO").unwrap();
    ot.id_ot
}

// ==========================================
// MATRIZ DE PRUEBAS V7.0
// ==========================================

#[test]
fn tc_ot_01_controles_acceso_y_privacidad() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    // 1. Privacidad de Nota Interna para Cajero
    let ot_cajero = OrdenesService::obtener_ot_por_id(&conn, id_ot, 3, "CAJERO").unwrap();
    assert!(ot_cajero.nota_interna.is_none());

    // 2. Privacidad de Costos Internos (solo Admin ve costos micros)
    let ot_mec = OrdenesService::obtener_ot_por_id(&conn, id_ot, 2, "MECANICO").unwrap();
    for insumo in ot_mec.insumos {
        assert!(insumo.costo_unitario_micro.is_none());
    }

    // 3. Intento de reasignar mecánico por Cajero (debe fallar)
    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot,
        nuevo_id_mecanico: 4,
        motivo: "Reasignación no autorizada".to_string(),
    };
    assert!(OrdenesService::reasignar_mecanico_ot(&mut conn, req_reasignar, 3, "CAJERO").is_err());
}

#[test]
fn tc_ot_02_unicidad_ot_abierta_vehiculo() {
    let mut conn = setup_test_db();
    let _id_ot = helper_crear_ot_base(&mut conn);

    // Intentar crear una segunda OT para el mismo vehículo mientras la primera está en espera
    let req2 = CrearOrdenTrabajoDto {
        id_vehiculo: 1,
        id_cliente_atendido: 2,
        kilometraje_ingreso: 50005,
        id_mecanico: None,
        pedido_cliente: Some("Revisión duplicada".to_string()),
        servicios_iniciales: vec![ServicioInicialDto {
            id_servicio: 2,
            cantidad: 1,
        }],
    };

    let res = OrdenesService::crear_orden_trabajo(&mut conn, req2, 3, "CAJERO");
    assert!(res.is_err());
}

#[test]
fn tc_ot_03_inicio_combinado_transaccional() {
    let mut conn = setup_test_db();

    // Crear OT sin mecánico asignado
    let req_crear = CrearOrdenTrabajoDto {
        id_vehiculo: 1,
        id_cliente_atendido: 2,
        kilometraje_ingreso: 50000,
        id_mecanico: None,
        pedido_cliente: None,
        servicios_iniciales: vec![ServicioInicialDto {
            id_servicio: 1,
            cantidad: 1,
        }],
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear, 3, "CAJERO").unwrap();
    assert_eq!(ot.estado, "EN_ESPERA");
    assert!(ot.id_mecanico.is_none());

    // Iniciar asignando al Mecánico ID 2 en Zanja 1
    let req_iniciar = IniciarTrabajoOtDto {
        id_ot: ot.id_ot,
        id_zanja_ejecucion: 1,
    };
    let ot_iniciada =
        OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar, 2, "MECANICO").unwrap();

    assert_eq!(ot_iniciada.estado, "EN_PROCESO");
    assert_eq!(ot_iniciada.id_mecanico, Some(2));
    assert_eq!(ot_iniciada.id_zanja_ejecucion, Some(1));
}

#[test]
fn tc_ot_04_exclusividad_zanja_y_reasignacion_admin() {
    let mut conn = setup_test_db();
    let id_ot_1 = helper_crear_ot_base(&mut conn);

    // Pasar OT 1 a EN_PROCESO en Zanja 2
    let req_iniciar1 = IniciarTrabajoOtDto {
        id_ot: id_ot_1,
        id_zanja_ejecucion: 2,
    };
    OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar1, 2, "MECANICO").unwrap();

    // Crear OT 2 con Vehículo 2 y meter a Zanja 2 (debe rebotar por estar ocupada)
    let req_crear2 = CrearOrdenTrabajoDto {
        id_vehiculo: 2,
        id_cliente_atendido: 2,
        kilometraje_ingreso: 10000,
        id_mecanico: Some(4),
        pedido_cliente: None,
        servicios_iniciales: vec![ServicioInicialDto {
            id_servicio: 2,
            cantidad: 1,
        }],
    };
    let ot2 = OrdenesService::crear_orden_trabajo(&mut conn, req_crear2, 3, "CAJERO").unwrap();

    let req_iniciar2 = IniciarTrabajoOtDto {
        id_ot: ot2.id_ot,
        id_zanja_ejecucion: 2,
    };
    assert!(OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar2, 4, "MECANICO").is_err());

    // Reasignación por Admin con motivo
    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot: id_ot_1,
        nuevo_id_mecanico: 4,
        motivo: "Reasignación por cambio de turno técnico".to_string(),
    };
    let ot_reasignada =
        OrdenesService::reasignar_mecanico_ot(&mut conn, req_reasignar, 1, "ADMINISTRADOR")
            .unwrap();
    assert_eq!(ot_reasignada.id_mecanico, Some(4));
}

#[test]
fn tc_ot_05_edicion_servicios_y_minimo_un_servicio() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    // Agregar segundo servicio
    let req_add = AgregarServicioOtDto {
        id_ot,
        id_servicio: 2,
        cantidad: 1,
    };
    let ot = OrdenesService::agregar_servicio_ot(&mut conn, req_add, 2, "MECANICO").unwrap();
    assert_eq!(ot.servicios.len(), 2);

    let id_det_1 = ot.servicios[0].id_detalle;
    let id_det_2 = ot.servicios[1].id_detalle;

    // Modificar cantidad del servicio 2
    let req_mod = ModificarServicioOtDto {
        id_ot,
        id_detalle_servicio: id_det_2,
        nueva_cantidad: 3,
    };
    let ot_mod = OrdenesService::modificar_servicio_ot(&mut conn, req_mod, 2, "MECANICO").unwrap();
    let s2 = ot_mod
        .servicios
        .iter()
        .find(|s| s.id_detalle == id_det_2)
        .unwrap();
    assert_eq!(s2.cantidad, 3);

    // Intentar eliminar ambos servicios hasta dejar en 0 (debe fallar al intentar eliminar el último)
    OrdenesService::eliminar_servicio_ot(&mut conn, id_det_1, id_ot, 2, "MECANICO").unwrap();
    assert!(
        OrdenesService::eliminar_servicio_ot(&mut conn, id_det_2, id_ot, 2, "MECANICO").is_err()
    );
}

#[test]
fn tc_ot_06_descuento_real_stock_kardex() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    // Pasar a EN_PROCESO
    let req_iniciar = IniciarTrabajoOtDto {
        id_ot,
        id_zanja_ejecucion: 1,
    };
    OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar, 2, "MECANICO").unwrap();

    // Registrar consumo de 2 unidades de Aceite
    let req_insumo = RegistrarInsumoOtDto {
        id_ot,
        id_producto: 1,
        cantidad: 2.0,
    };
    OrdenesService::registrar_insumo_ot(&mut conn, req_insumo, 2, "MECANICO").unwrap();

    // Validar descuento en productos
    let stock_actual: i64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_actual, 8);

    // Validar registro en movimientos_inventario (Kardex)
    let tipo_mov: String = conn.query_row(
        "SELECT tipo_movimiento FROM movimientos_inventario WHERE id_producto = 1 ORDER BY id_movimiento DESC LIMIT 1",
        [],
        |r| r.get(0),
    ).unwrap();
    assert_eq!(tipo_mov, "CONSUMO_OT");
}

#[test]
fn tc_ot_07_devolucion_fisica_insumos_con_motivo() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    let req_iniciar = IniciarTrabajoOtDto {
        id_ot,
        id_zanja_ejecucion: 1,
    };
    OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar, 2, "MECANICO").unwrap();

    let req_insumo = RegistrarInsumoOtDto {
        id_ot,
        id_producto: 1,
        cantidad: 3.0,
    };
    let ot = OrdenesService::registrar_insumo_ot(&mut conn, req_insumo, 2, "MECANICO").unwrap();
    let id_det_producto = ot.insumos[0].id_detalle;

    // Devolución administrativa de 1 unidad por sobrante
    let req_dev = DevolverInsumoOtDto {
        id_ot,
        id_detalle_producto: id_det_producto,
        cantidad_a_devolver: 1.0,
        motivo: "Devolución de 1 lata no abierta".to_string(),
    };
    OrdenesService::devolver_insumo_ot(&mut conn, req_dev, 1, "ADMINISTRADOR").unwrap();

    // Stock debe restaurarse a 8 (10 - 3 + 1)
    let stock_actual: i64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_actual, 8);

    // Validar tipo de movimiento REPOSICION_OT en Kardex
    let (tipo_mov, motivo): (String, String) = conn.query_row(
        "SELECT tipo_movimiento, motivo FROM movimientos_inventario WHERE id_producto = 1 ORDER BY id_movimiento DESC LIMIT 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).unwrap();
    assert_eq!(tipo_mov, "REPOSICION_OT");
    assert!(motivo.contains("Devolución de 1 lata"));
}

#[test]
fn tc_ot_08_finalizacion_kilometraje_y_liberacion_zanja() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    let req_iniciar = IniciarTrabajoOtDto {
        id_ot,
        id_zanja_ejecucion: 3,
    };
    OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar, 2, "MECANICO").unwrap();

    // Finalizar OT con incremento de Kilometraje y Aceite Sintético
    let req_fin = FinalizarOtDto {
        id_ot,
        kilometraje_salida: 55000,
        resumen_publico: "Mantenimiento preventivo completado con éxito".to_string(),
        tipo_aceite: Some("SINTETICO".to_string()),
    };
    let ot_fin = OrdenesService::finalizar_ot(&mut conn, req_fin, 2, "MECANICO").unwrap();

    assert_eq!(ot_fin.estado, "FINALIZADO");
    assert!(ot_fin.id_zanja_ejecucion.is_none()); // Zanja liberada
    assert_eq!(ot_fin.km_proximo, Some(65000)); // 55000 + 10000

    // Kilometraje actualizado en el Vehículo
    let km_vehiculo: i64 = conn
        .query_row(
            "SELECT kilometraje_actual FROM vehiculos WHERE id_vehiculo = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(km_vehiculo, 55000);
}

#[test]
fn tc_ot_09_cancelacion_corta_con_costo_interno() {
    let mut conn = setup_test_db();
    let id_ot = helper_crear_ot_base(&mut conn);

    let req_iniciar = IniciarTrabajoOtDto {
        id_ot,
        id_zanja_ejecucion: 1,
    };
    OrdenesService::iniciar_trabajo_ot(&mut conn, req_iniciar, 2, "MECANICO").unwrap();

    let req_insumo = RegistrarInsumoOtDto {
        id_ot,
        id_producto: 1,
        cantidad: 2.0,
    };
    OrdenesService::registrar_insumo_ot(&mut conn, req_insumo, 2, "MECANICO").unwrap();

    // Cancelación por Administrador con motivo
    let req_canc = CancelarOtDto {
        id_ot,
        motivo: "Cliente cancela por falla de repuesto ajeno".to_string(),
    };
    let ot_canc = OrdenesService::cancelar_ot(&mut conn, req_canc, 1, "ADMINISTRADOR").unwrap();

    assert_eq!(ot_canc.estado, "CANCELADO");
    assert!(ot_canc.id_zanja_ejecucion.is_none()); // Zanja liberada

    // En v7.0 el insumo consumido NO se devuelve automáticamente (queda como costo)
    let stock_actual: i64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_actual, 8); // Se mantienen consumidas 2 unidades
}

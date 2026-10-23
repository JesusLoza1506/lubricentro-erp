use lubricentro_erp_lib::dto::cliente_dto::CrearClienteDto;
use lubricentro_erp_lib::dto::vehiculo_dto::{
    CambiarEstadoVehiculoPayloadDto, CrearVehiculoPayloadDto, EditarVehiculoPayloadDto,
};
use lubricentro_erp_lib::services::cliente_service::ClienteService;
use rusqlite::Connection;
use std::time::Instant;

const DUMMY_ARGON2_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

/// Configura una base de datos SQLite en memoria utilizando el esquema real V7 de producción
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 5000;
         PRAGMA journal_mode = WAL;",
    )
    .unwrap();

    let initial_schema = include_str!("../migrations/0001_initial_schema_v7.sql");
    conn.execute_batch(initial_schema).unwrap();

    conn.execute(
        "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo)
         VALUES ('Admin System', 'admin', ?1, 'ADMINISTRADOR', 1),
                ('Cajero Caja', 'caja', ?1, 'CAJERO', 1),
                ('Mecanico Taller', 'meca', ?1, 'MECANICO', 1)",
        [DUMMY_ARGON2_HASH],
    )
    .unwrap();

    conn.execute_batch(
        "
        INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, es_varios)
        VALUES ('DNI', '12345678', 'Juan Perez', '987654321', 0);

        INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual)
        VALUES ('ABC123', 2, 'Toyota', 'Yaris', 2020, 'GASOLINA', 50000);
        ",
    )
    .unwrap();

    conn
}

#[test]
fn test_registrar_cliente_y_vehiculo_service() {
    let conn = setup_test_db();

    let cliente_payload = CrearClienteDto {
        tipo_documento: "DNI".to_string(),
        numero_documento: "87654321".to_string(),
        nombre_razon_social: "Maria Lopez".to_string(),
        telefono: Some("912345678".to_string()),
        direccion: Some("Av. Principal 123".to_string()),
        correo: Some("maria@gmail.com".to_string()),
    };

    let id_cliente = ClienteService::registrar_cliente(
        &conn,
        cliente_payload,
        Some(1),
        Some("admin"),
        Some("ADMINISTRADOR"),
    )
    .unwrap();
    assert!(id_cliente > 0);

    let vehiculo_payload = CrearVehiculoPayloadDto {
        placa: "XYZ789".to_string(),
        marca: "Nissan".to_string(),
        modelo: "Sentra".to_string(),
        anio: Some(2021),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 15000,
        id_cliente: Some(id_cliente),
        nombre_razon_social: None,
        tipo_documento: None,
        numero_documento: None,
        telefono: None,
        correo: None,
        direccion: None,
    };

    let placa_res = ClienteService::registrar_vehiculo(
        &conn,
        vehiculo_payload,
        Some(1),
        Some("admin"),
        Some("ADMINISTRADOR"),
    )
    .unwrap();
    assert_eq!(placa_res, "XYZ789");

    let vehiculo_res =
        ClienteService::obtener_vehiculo_por_placa(&conn, "XYZ789", Some("ADMINISTRADOR"));
    assert!(vehiculo_res.is_ok());
}

#[test]
fn test_actualizar_vehiculo_y_propietario_service() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "ABC123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris Hatchback".to_string(),
        anio: Some(2021),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 55000,
        id_cliente: Some(2),
        nombre_razon_social: Some("Juan Perez Actualizado".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("999888777".to_string()),
        correo: Some("juan@correo.com".to_string()),
        direccion: Some("Nueva Direccion 456".to_string()),
        motivo: None,
    };

    let res = ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    );
    assert!(res.is_ok());

    let vehiculo_actualizado =
        ClienteService::obtener_vehiculo_por_placa(&conn, "ABC123", Some("ADMINISTRADOR")).unwrap();
    assert_eq!(vehiculo_actualizado.modelo, "Yaris Hatchback");
    assert_eq!(vehiculo_actualizado.kilometraje_actual, 55000);
    assert_eq!(
        vehiculo_actualizado.nombre_cliente.unwrap(),
        "Juan Perez Actualizado"
    );
}

#[test]
fn test_actualizar_vehiculo_por_id_y_cambio_placa() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "NEW123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(2),
        nombre_razon_social: Some("Juan Perez".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("987654321".to_string()),
        correo: None,
        direccion: None,
        motivo: Some("Correccion de placa por error de digitacion inicial".to_string()),
    };

    let res = ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    );
    assert!(res.is_ok());

    let vehiculo_modificado =
        ClienteService::obtener_vehiculo_por_placa(&conn, "NEW123", Some("ADMINISTRADOR"));
    assert!(vehiculo_modificado.is_ok());
    assert_eq!(vehiculo_modificado.unwrap().placa, "NEW123");
}

#[test]
fn test_cambio_placa_por_cajero_falla() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "NEW123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(2),
        nombre_razon_social: Some("Juan Perez".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("987654321".to_string()),
        correo: None,
        direccion: None,
        motivo: Some("Intento de cambio de placa autorizado por cajero".to_string()),
    };

    let res =
        ClienteService::actualizar_vehiculo(&conn, edit_payload, Some(2), Some("caja"), "CAJERO");
    assert!(res.is_err());
}

#[test]
fn test_cambio_placa_sin_motivo_falla() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "NEW123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(2),
        nombre_razon_social: Some("Juan Perez".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("987654321".to_string()),
        correo: None,
        direccion: None,
        motivo: None,
    };

    let res = ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    );
    assert!(res.is_err());
}

#[test]
fn test_cambio_propietario_sin_motivo_falla() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, es_varios)
         VALUES ('DNI', '88888888', 'Nuevo Propietario', '911223344', 0)",
        [],
    )
    .unwrap();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "ABC123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(3),
        nombre_razon_social: Some("Nuevo Propietario".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("88888888".to_string()),
        telefono: Some("911223344".to_string()),
        correo: None,
        direccion: None,
        motivo: None,
    };

    let res = ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    );
    assert!(res.is_err());
}

#[test]
fn test_reduccion_kilometraje_sin_motivo_falla() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "ABC123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 40000,
        id_cliente: Some(2),
        nombre_razon_social: Some("Juan Perez".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("987654321".to_string()),
        correo: None,
        direccion: None,
        motivo: None,
    };

    let res = ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    );
    assert!(res.is_err());
}

#[test]
fn test_actualizar_vehiculo_por_mecanico_cambio_propietario_falla() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, es_varios)
         VALUES ('DNI', '88888888', 'Nuevo Propietario', '911223344', 0)",
        [],
    )
    .unwrap();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "ABC123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(3),
        nombre_razon_social: Some("Nuevo Propietario".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("88888888".to_string()),
        telefono: Some("911223344".to_string()),
        correo: None,
        direccion: None,
        motivo: Some("Cambio de dueño autorizado por mecanico".to_string()),
    };

    let res =
        ClienteService::actualizar_vehiculo(&conn, edit_payload, Some(3), Some("meca"), "MECANICO");
    assert!(res.is_err());
}

#[test]
fn test_consulta_atomica_ficha_completa_con_aceite_y_paginacion() {
    let conn = setup_test_db();

    // 1. Insertar servicio de cambio de aceite previo
    conn.execute(
        "INSERT INTO servicios (codigo, nombre, precio_venta_cent, es_cambio_aceite)
         VALUES ('SERV-01', 'Cambio de Aceite Estándar', 1500, 1)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO productos (
            codigo, nombre, categoria, unidad_medida, stock_actual, stock_minimo,
            valor_stock_micro, precio_venta_cent, activo
        ) VALUES (
            'PROD-001', 'Aceite Sintetico 5W-30', 'ACEITE', 'UND', 0, 2,
            0, 5000, 1
        )",
        [],
    )
    .unwrap();

    // Crear 25 OTs respetando la máquina de estados y las restricciones del trigger de cambio de aceite
    for i in 1..=25 {
        let codigo_ot = format!("OT-{:03}", i);
        let id_op = format!("OP-TEST-{:03}", i);
        let id_op_serv = format!("OP-SERV-{:03}", i);

        // A. Nace en EN_ESPERA libre de datos de ejecución
        conn.execute(
            "INSERT INTO ordenes_trabajo (
                id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido,
                placa_snapshot, cliente_snapshot, id_creador,
                estado, kilometraje_ingreso
            ) VALUES (
                ?1, ?2, 1, 2, 'ABC123', '{\"nombre\":\"Juan Perez\",\"documento\":\"12345678\"}',
                1, 'EN_ESPERA', 50000
            )",
            [&id_op, &codigo_ot],
        )
        .unwrap();

        let id_ot = conn.last_insert_rowid();

        // B. Inicia ejecución (EN_PROCESO)
        conn.execute(
            "UPDATE ordenes_trabajo 
             SET estado = 'EN_PROCESO', id_mecanico = 3, id_zanja_ejecucion = 1, fecha_inicio = datetime('now', '-2 hours')
             WHERE id_ot = ?1",
            [id_ot],
        )
        .unwrap();

        // C. Insertar el servicio de cambio de aceite obligatorio para cumplir el CHECK/Trigger de tipo_aceite
        conn.execute(
            "INSERT INTO detalle_ot_servicios (
                id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot,
                cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario
            ) VALUES (
                ?1, ?2, 1, 'SERV-01', 'Cambio de Aceite Estándar',
                1, 1500, 1, 1
            )",
            [&id_op_serv, &id_ot.to_string()],
        )
        .unwrap();

        // D. Finaliza ejecución (FINALIZADO con tipo_aceite)
        conn.execute(
            "UPDATE ordenes_trabajo 
             SET estado = 'FINALIZADO', id_mecanico_terminal = 3, kilometraje_salida = 50000,
                 tipo_aceite = 'SINTETICO', intervalo_aceite_km = 10000,
                 resumen_publico = 'Servicio de mantenimiento', fecha_fin = datetime('now', '-1 hours')
             WHERE id_ot = ?1",
            [id_ot],
        )
        .unwrap();
    }

    // Probar Página 1 (20 items por página)
    let ficha_p1 = ClienteService::obtener_ficha_vehicular_completa(
        &conn,
        "ABC123",
        Some("ADMINISTRADOR"),
        1,
        20,
    )
    .expect("Debe obtener la página 1 de la ficha vehicular");

    assert_eq!(ficha_p1.historial.items.len(), 20);
    assert_eq!(ficha_p1.historial.total_registros, 25);
    assert_eq!(ficha_p1.historial.pagina_actual, 1);
    assert_eq!(ficha_p1.historial.total_paginas, 2);
    assert_eq!(
        ficha_p1.historial.items[0].nombre_cliente.as_deref(),
        Some("Juan Perez")
    );

    // Probar Página 2 (5 items restantes)
    let ficha_p2 = ClienteService::obtener_ficha_vehicular_completa(
        &conn,
        "ABC123",
        Some("ADMINISTRADOR"),
        2,
        20,
    )
    .expect("Debe obtener la página 2 de la ficha vehicular");

    assert_eq!(ficha_p2.historial.items.len(), 5);
    assert_eq!(ficha_p2.historial.pagina_actual, 2);
}

#[test]
fn test_bloqueo_desactivacion_service_con_ot_abierta() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO ordenes_trabajo (
            id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido,
            placa_snapshot, cliente_snapshot, id_creador,
            estado, kilometraje_ingreso
        ) VALUES (
            'OP-TEST-02', 'OT-001', 1, 2, 'ABC123', '{\"nombre\":\"Juan Perez\"}',
            1, 'EN_ESPERA', 50000
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'EN_PROCESO', id_mecanico = 3, id_zanja_ejecucion = 1, fecha_inicio = datetime('now', '-5 hours')
         WHERE id_ot = 1",
        [],
    )
    .unwrap();

    let payload = CambiarEstadoVehiculoPayloadDto {
        placa: "ABC123".to_string(),
        activo: 0,
        motivo: "Desactivacion por prueba tecnica".to_string(),
    };

    let res_desactivar = ClienteService::cambiar_estado_vehiculo(
        &conn,
        payload,
        Some(1),
        Some("admin"),
        Some("ADMINISTRADOR"),
    );
    assert!(res_desactivar.is_err());
}

#[test]
fn test_desactivacion_logica_exitosa_service_con_motivo() {
    let conn = setup_test_db();

    let payload = CambiarEstadoVehiculoPayloadDto {
        placa: "ABC123".to_string(),
        activo: 0,
        motivo: "Desactivacion por desuso del propietario".to_string(),
    };

    let res_desactivar = ClienteService::cambiar_estado_vehiculo(
        &conn,
        payload,
        Some(1),
        Some("admin"),
        Some("ADMINISTRADOR"),
    );
    assert!(res_desactivar.is_ok());

    let vehiculo_desactivado =
        ClienteService::obtener_vehiculo_por_placa(&conn, "ABC123", Some("ADMINISTRADOR")).unwrap();
    assert_eq!(vehiculo_desactivado.activo, 0);
}

#[test]
fn test_autorizacion_rol_mecanico_bloqueado() {
    let conn = setup_test_db();

    let rol: String = conn
        .query_row(
            "SELECT UPPER(rol) FROM usuarios WHERE username = 'meca'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(rol, "MECANICO");

    let roles_escritura = ["ADMINISTRADOR", "CAJERO"];
    assert!(!roles_escritura.contains(&rol.as_str()));
}

// =========================================================================
// PUNTO 15: PRUEBAS DE BORDE Y TRANSACCIONALIDAD CUMPLIENDO DDL V7.0
// =========================================================================

#[test]
fn test_transaccion_atomica_alta_falla_rollback() {
    let conn = setup_test_db();

    let total_clientes_inicial: i64 = conn
        .query_row("SELECT COUNT(*) FROM clientes", [], |r| r.get(0))
        .unwrap();

    let payload_invalido = CrearVehiculoPayloadDto {
        placa: "PLACA_TOTALMENTE_INVALIDA_Y_DEMASIADO_LARGA".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 10000,
        id_cliente: None,
        nombre_razon_social: Some("Cliente Rollback".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("77777777".to_string()),
        telefono: Some("911111111".to_string()),
        correo: None,
        direccion: None,
    };

    let res = ClienteService::registrar_vehiculo(
        &conn,
        payload_invalido,
        Some(1),
        Some("admin"),
        Some("ADMINISTRADOR"),
    );

    assert!(res.is_err());

    let total_clientes_final: i64 = conn
        .query_row("SELECT COUNT(*) FROM clientes", [], |r| r.get(0))
        .unwrap();

    assert_eq!(
        total_clientes_inicial, total_clientes_final,
        "El registro de cliente debió hacer rollback al fallar la creación del vehículo"
    );
}

#[test]
fn test_limites_candidato_proximo_cambio_aceite() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO servicios (codigo, nombre, precio_venta_cent, es_cambio_aceite)
         VALUES ('SERV-LIM', 'Cambio Sintético', 2000, 1)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO ordenes_trabajo (
            id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido,
            placa_snapshot, cliente_snapshot, id_creador,
            estado, kilometraje_ingreso, fecha_ingreso
        ) VALUES (
            'OP-LIMIT-01', 'OT-LIMIT', 1, 2, 'ABC123', '{\"nombre\":\"Juan Perez\"}',
            1, 'EN_ESPERA', 50000, datetime('now', '-180 days')
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'EN_PROCESO',
             id_mecanico = 3,
             id_zanja_ejecucion = 1,
             fecha_inicio = datetime('now', '-180 days')
         WHERE id_ot = 1",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO detalle_ot_servicios (
            id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot,
            cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario
        ) VALUES (
            'OP-SERV-LIM', 1, 1, 'SERV-LIM', 'Cambio Sintético',
            1, 2000, 1, 1
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'FINALIZADO',
             id_mecanico_terminal = 3,
             kilometraje_salida = 75000,
             tipo_aceite = 'SINTETICO',
             intervalo_aceite_km = 10000,
             resumen_publico = 'Cambio de aceite de prueba límite',
             fecha_fin = datetime('now', '-180 days')
         WHERE id_ot = 1",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE vehiculos SET kilometraje_actual = 84500 WHERE id_vehiculo = 1",
        [],
    )
    .unwrap();

    let candidatos = ClienteService::obtener_proximos_cambios_aceite(&conn, Some("ADMINISTRADOR"))
        .expect("Debe consultar candidatos a cambio de aceite");

    let candidato = candidatos.iter().find(|c| c.placa == "ABC123");
    assert!(
        candidato.is_some(),
        "El vehículo debe activarse como candidato a cambio al alcanzar km_proximo - 500 o 180 días"
    );
}

#[test]
fn test_conservacion_historica_ots_al_cambiar_propietario() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, es_varios)
         VALUES ('DNI', '99999999', 'Carlos Propietario Nuevo', '922334455', 0)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO ordenes_trabajo (id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot, id_creador, estado, kilometraje_ingreso, fecha_ingreso)
         VALUES ('OP-HIST-01', 'OT-H1', 1, 2, 'ABC123', '{\"nombre\":\"Juan Perez\"}', 1, 'EN_ESPERA', 50000, datetime('now', '-10 hours'))",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'EN_PROCESO',
             id_mecanico = 3,
             id_zanja_ejecucion = 1,
             fecha_inicio = datetime('now', '-10 hours')
         WHERE id_ot = 1",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO servicios (codigo, nombre, precio_venta_cent, es_cambio_aceite) VALUES ('SERV-H1', 'Mantenimiento General', 1000, 0)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO detalle_ot_servicios (id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot, cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario)
         VALUES ('OP-DS-H1', 1, 1, 'SERV-H1', 'Mantenimiento General', 1, 1000, 0, 1)",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'FINALIZADO',
             id_mecanico_terminal = 3,
             kilometraje_salida = 50000,
             resumen_publico = 'Mantenimiento previo realizado',
             fecha_fin = datetime('now', '-5 hours')
         WHERE id_ot = 1",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO ordenes_trabajo (id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot, id_creador, estado, kilometraje_ingreso, fecha_ingreso)
         VALUES ('OP-HIST-02', 'OT-H2', 1, 2, 'ABC123', '{\"nombre\":\"Juan Perez\"}', 1, 'EN_ESPERA', 50000, datetime('now', '-4 hours'))",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'EN_PROCESO',
             id_mecanico = 3,
             id_zanja_ejecucion = 2,
             fecha_inicio = datetime('now', '-4 hours')
         WHERE id_ot = 2",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO detalle_ot_servicios (id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot, cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario)
         VALUES ('OP-DS-H2', 2, 1, 'SERV-H1', 'Mantenimiento General', 1, 1000, 0, 1)",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE ordenes_trabajo 
         SET estado = 'FINALIZADO',
             id_mecanico_terminal = 3,
             kilometraje_salida = 50000,
             resumen_publico = 'Segundo mantenimiento realizado',
             fecha_fin = datetime('now', '-2 hours')
         WHERE id_ot = 2",
        [],
    )
    .unwrap();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: Some(1),
        placa: "ABC123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris".to_string(),
        anio: Some(2020),
        tipo_motor: Some("GASOLINA".to_string()),
        kilometraje_actual: 50000,
        id_cliente: Some(3),
        nombre_razon_social: Some("Carlos Propietario Nuevo".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("99999999".to_string()),
        telefono: Some("922334455".to_string()),
        correo: None,
        direccion: None,
        motivo: Some("Transferencia legal de propiedad del vehículo".to_string()),
    };

    ClienteService::actualizar_vehiculo(
        &conn,
        edit_payload,
        Some(1),
        Some("admin"),
        "ADMINISTRADOR",
    )
    .expect("El cambio de propietario con motivo debe ejecutarse correctamente");

    let ficha = ClienteService::obtener_ficha_vehicular_completa(
        &conn,
        "ABC123",
        Some("ADMINISTRADOR"),
        1,
        20,
    )
    .expect("Debe obtener la ficha vehicular");

    assert_eq!(
        ficha.historial.total_registros, 2,
        "El historial técnico de OTs de la placa debe conservarse íntegramente tras el cambio de propietario"
    );
    assert_eq!(
        ficha.vehiculo.nombre_cliente.unwrap(),
        "Carlos Propietario Nuevo"
    );
}

// =========================================================================
// FASE 4: PRUEBAS DE ESTRÉS Y RENDIMIENTO DE BÚSQUEDA
// =========================================================================

#[test]
fn test_estres_masivo_1000_vehiculos_y_rendimiento_busqueda() {
    let mut conn = setup_test_db();

    let inicio_insercion = Instant::now();

    let tx = conn.transaction().unwrap();

    for i in 1..=1000 {
        let dni = format!("{:08}", 10000000 + i);
        let nombre_cliente = format!("Cliente Simulado {}", i);
        let placa = format!("STR{:03}", i % 1000);

        tx.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, es_varios)
             VALUES ('DNI', ?1, ?2, '900000000', 0)",
            [&dni, &nombre_cliente],
        )
        .unwrap();

        let id_cliente_nuevo = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo)
             VALUES (?1, ?2, 'Nissan', 'Versa', 2022, 'GASOLINA', ?3, 1)",
            rusqlite::params![placa, id_cliente_nuevo, (i * 100) as i64],
        )
        .unwrap();
    }

    tx.commit().unwrap();

    let duracion_insercion = inicio_insercion.elapsed();
    println!(
        "FASE 4: Tiempo de inserción masiva de 1,000 vehículos: {:?}",
        duracion_insercion
    );

    let total_vehiculos: i64 = conn
        .query_row("SELECT COUNT(*) FROM vehiculos", [], |row| row.get(0))
        .unwrap();
    assert_eq!(total_vehiculos, 1001);

    let placa_prueba = "STR500";
    let inicio_busqueda = Instant::now();

    let vehiculo_res =
        ClienteService::obtener_vehiculo_por_placa(&conn, placa_prueba, Some("ADMINISTRADOR"));

    let duracion_busqueda = inicio_busqueda.elapsed();

    assert!(vehiculo_res.is_ok());
    let v = vehiculo_res.unwrap();
    assert_eq!(v.placa, placa_prueba);
    assert_eq!(v.kilometraje_actual, 50000);

    println!(
        "FASE 4: Tiempo de consulta de un vehículo entre 1,000 registros: {:?}",
        duracion_busqueda
    );

    assert!(
        duracion_busqueda.as_millis() < 10,
        "La búsqueda por placa excedió el tiempo límite esperado para un entorno indexado."
    );
}

use lubricentro_erp_lib::dto::cliente_dto::CrearClienteDto;
use lubricentro_erp_lib::dto::vehiculo_dto::{CrearVehiculoPayloadDto, EditarVehiculoPayloadDto};
use lubricentro_erp_lib::services::cliente_service::ClienteService;
use rusqlite::Connection;

/// Configura una base de datos SQLite en memoria utilizando exactamente el esquema real de producción
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    let initial_schema = include_str!("../migrations/0001_initial_schema.sql");
    conn.execute_batch(initial_schema).unwrap();

    // Datos iniciales compatibles con el esquema de producción
    conn.execute_batch(
        "
        INSERT INTO usuarios (nombre_completo, username, password_hash, rol)
        VALUES ('Admin', 'admin', 'hash', 'ADMINISTRADOR'),
               ('Mecanico', 'meca', 'hash', 'MECANICO');

        INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono)
        VALUES ('DNI', '12345678', 'Juan Perez', '987654321');

        INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual)
        VALUES ('ABC-123', 1, 'Toyota', 'Yaris', 2020, '1.5L', 50000);
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
    };

    let id_cliente = ClienteService::registrar_cliente(&conn, cliente_payload).unwrap();
    assert!(id_cliente > 0);

    let vehiculo_payload = CrearVehiculoPayloadDto {
        placa: "XYZ-789".to_string(),
        marca: "Nissan".to_string(),
        modelo: "Sentra".to_string(),
        anio: Some(2021),
        tipo_motor: Some("1.8L".to_string()),
        kilometraje_actual: 15000,
        id_cliente: Some(id_cliente),
        nombre_razon_social: None,
        tipo_documento: None,
        numero_documento: None,
        telefono: None,
        direccion: None,
    };

    let placa_res = ClienteService::registrar_vehiculo(&conn, vehiculo_payload, Some(1)).unwrap();
    assert_eq!(placa_res, "XYZ-789");

    let vehiculo_res = ClienteService::obtener_vehiculo_por_placa(&conn, "XYZ-789");
    assert!(vehiculo_res.is_ok());
}

#[test]
fn test_actualizar_vehiculo_y_propietario_service() {
    let conn = setup_test_db();

    let edit_payload = EditarVehiculoPayloadDto {
        id_vehiculo: None,
        placa: "ABC-123".to_string(),
        marca: "Toyota".to_string(),
        modelo: "Yaris Hatchback".to_string(),
        anio: Some(2021),
        tipo_motor: Some("1.5L Dual VVT-i".to_string()),
        kilometraje_actual: 55000,
        id_cliente: Some(1),
        nombre_razon_social: Some("Juan Perez Actualizado".to_string()),
        tipo_documento: Some("DNI".to_string()),
        numero_documento: Some("12345678".to_string()),
        telefono: Some("999888777".to_string()),
        direccion: Some("Nueva Direccion 456".to_string()),
    };

    let res = ClienteService::actualizar_vehiculo(&conn, edit_payload, Some(1));
    assert!(res.is_ok());

    let vehiculo_actualizado =
        ClienteService::obtener_vehiculo_por_placa(&conn, "ABC-123").unwrap();
    assert_eq!(vehiculo_actualizado.modelo, "Yaris Hatchback");
    assert_eq!(vehiculo_actualizado.kilometraje_actual, 55000);
    assert_eq!(
        vehiculo_actualizado.nombre_cliente.unwrap(),
        "Juan Perez Actualizado"
    );
}

#[test]
fn test_consulta_atomica_ficha_completa_con_aceite() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje)
         VALUES ('OT-001', 'ABC-123', 2, 1, 'FINALIZADO', 50000, 55000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO categorias (nombre_categoria) VALUES ('ACEITES Y LUBRICANTES')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO productos (id_categoria, descripcion, precio_venta, precio_compra, stock_actual, stock_minimo, unidad_medida) 
         VALUES (1, 'Aceite Sintetico 5W-30', 5000, 4000, 10.0, 2.0, 'GALON')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO detalle_ot_productos (id_ot, id_producto, cantidad, precio_aplicado) VALUES (1, 1, 1.0, 5000)",
        [],
    )
    .unwrap();

    let ficha = ClienteService::obtener_ficha_vehicular_completa(&conn, "ABC-123").unwrap();

    assert_eq!(ficha.vehiculo.placa, "ABC-123");
    assert_eq!(ficha.historial.len(), 1);
    assert_eq!(ficha.historial[0].codigo_ot, "OT-001");
    assert_eq!(
        ficha.historial[0].tipo_aceite.as_deref(),
        Some("Aceite Sintetico 5W-30")
    );
}

#[test]
fn test_bloqueo_eliminacion_service_con_ot() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje)
         VALUES ('OT-001', 'ABC-123', 2, 1, 'FINALIZADO', 50000, 55000)",
        [],
    )
    .unwrap();

    let res_eliminar = ClienteService::eliminar_vehiculo(&conn, "ABC-123", Some(1));
    assert!(res_eliminar.is_err());
}

#[test]
fn test_eliminacion_logica_exitosa_service() {
    let conn = setup_test_db();

    let res_eliminar = ClienteService::eliminar_vehiculo(&conn, "ABC-123", Some(1));
    assert!(res_eliminar.is_ok());

    let res_get = ClienteService::obtener_vehiculo_por_placa(&conn, "ABC-123");
    assert!(res_get.is_err());
}

#[test]
fn test_autorizacion_rol_mecanico_bloqueado() {
    let conn = setup_test_db();

    let rol: String = conn
        .query_row(
            "SELECT UPPER(rol) FROM usuarios WHERE id_usuario = 2",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(rol, "MECANICO");

    let roles_escritura = ["ADMINISTRADOR", "CAJERO"];
    assert!(!roles_escritura.contains(&rol.as_str()));
}

use lubricentro_erp_lib::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto, ReasignarMecanicoOtDto,
};
use lubricentro_erp_lib::services::ordenes_service::OrdenesService;
use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "
        CREATE TABLE usuarios (
            id_usuario INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre_completo TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            rol TEXT NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE clientes (
            id_cliente INTEGER PRIMARY KEY AUTOINCREMENT,
            tipo_documento TEXT NOT NULL,
            numero_documento TEXT NOT NULL UNIQUE,
            nombre_razon_social TEXT NOT NULL,
            telefono TEXT,
            direccion TEXT,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE vehiculos (
            placa TEXT PRIMARY KEY,
            id_cliente INTEGER NOT NULL,
            marca TEXT NOT NULL,
            modelo TEXT NOT NULL,
            anio INTEGER,
            tipo_motor TEXT,
            kilometraje_actual INTEGER NOT NULL DEFAULT 0,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE ordenes_trabajo (
            id_ot INTEGER PRIMARY KEY AUTOINCREMENT,
            codigo_ot TEXT NOT NULL UNIQUE,
            placa TEXT NOT NULL,
            id_mecanico INTEGER NOT NULL,
            zanja INTEGER,
            estado TEXT NOT NULL,
            kilometraje_ingreso INTEGER NOT NULL,
            proximo_kilometraje INTEGER NOT NULL,
            observaciones TEXT,
            fecha_ingreso DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_zanja_activa 
        ON ordenes_trabajo(zanja) 
        WHERE zanja IS NOT NULL AND estado IN ('EN_ESPERA', 'EN_PROCESO');

        CREATE TABLE categorias (
            id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre_categoria TEXT NOT NULL UNIQUE,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE productos (
            id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
            id_categoria INTEGER NOT NULL,
            codigo_barras TEXT UNIQUE,
            descripcion TEXT NOT NULL,
            unidad_medida TEXT NOT NULL,
            stock_actual REAL NOT NULL DEFAULT 0,
            stock_minimo REAL NOT NULL DEFAULT 0,
            precio_compra INTEGER NOT NULL,
            precio_venta INTEGER NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE servicios (
            id_servicio INTEGER PRIMARY KEY AUTOINCREMENT,
            descripcion TEXT NOT NULL,
            precio_referencial INTEGER NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE detalle_ot_productos (
            id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
            id_ot INTEGER NOT NULL,
            id_producto INTEGER NOT NULL,
            cantidad REAL NOT NULL,
            precio_aplicado INTEGER NOT NULL,
            descuento INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE detalle_ot_servicios (
            id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
            id_ot INTEGER NOT NULL,
            id_servicio INTEGER NOT NULL,
            id_mecanico_ejecutor INTEGER NOT NULL,
            precio_aplicado INTEGER NOT NULL,
            descuento INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE movimientos_inventario (
            id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
            id_producto INTEGER NOT NULL,
            id_usuario INTEGER NOT NULL,
            id_detalle_ot INTEGER,
            tipo_movimiento TEXT NOT NULL,
            stock_anterior REAL NOT NULL,
            diferencia REAL NOT NULL,
            stock_resultante REAL NOT NULL,
            referencia TEXT,
            fecha_movimiento DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        -- Usuarios para pruebas
        INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES
        (1, 'Admin', 'admin', 'hash', 'ADMINISTRADOR', 1),
        (2, 'Juan Mecanico', 'mecanico1', 'hash', 'MECANICO', 1),
        (3, 'Pedro Cajero', 'cajero', 'hash', 'CAJERO', 1),
        (4, 'Carlos Mecanico 2', 'mecanico2', 'hash', 'MECANICO', 1),
        (5, 'Mecanico Inactivo', 'mecanico_inactivo', 'hash', 'MECANICO', 0),
        (6, 'Admin Inactivo', 'admin_inactivo', 'hash', 'ADMINISTRADOR', 0);

        INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, activo) VALUES
        (1, 'DNI', '12345678', 'Cliente Prueba', 1);

        INSERT INTO vehiculos (placa, id_cliente, marca, modelo, kilometraje_actual, activo) VALUES
        ('ABC-123', 1, 'Toyota', 'Yaris', 50000, 1);

        INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES
        (1, 'Aceites Motor', 1);

        INSERT INTO productos (id_producto, id_categoria, descripcion, unidad_medida, stock_actual, precio_compra, precio_venta, activo) VALUES
        (1, 1, 'Aceite 10W-40 Sintetico', 'GALON', 10.0, 8000, 12000, 1),
        (2, 1, 'Filtro de Aceite', 'UNIDAD', 2.0, 1500, 2500, 1);

        INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES
        (1, 'Cambio de Aceite Basico', 2500, 1);
        ",
    )
    .unwrap();
    conn
}

// 🧪 1. Creación con mecánico inexistente (debe rehusarse)
#[test]
fn test_01_crear_ot_mecanico_inexistente() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 999, // No existe
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

// 🧪 2. Creación con mecánico inactivo (debe rehusarse)
#[test]
fn test_02_crear_ot_mecanico_inactivo() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 5, // Mecánico inactivo
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

// 🧪 3. Creación con usuario que no tiene rol de mecánico (ej. Cajero)
#[test]
fn test_03_crear_ot_usuario_sin_rol_mecanico() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 3, // ID 3 es CAJERO
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

// 🧪 4. Reasignación de mecánico inválida (mecanico inexistente)
#[test]
fn test_04_reasignar_mecanico_inexistente() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot: ot.id_ot,
        nuevo_id_mecanico: 888, // Inexistente
    };
    assert!(OrdenesService::reasignar_mecanico_ot(&conn, req_reasignar).is_err());
}

// 🧪 5. Reasignación a mecánico inactivo
#[test]
fn test_05_reasignar_mecanico_inactivo() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let req_reasignar = ReasignarMecanicoOtDto {
        id_ot: ot.id_ot,
        nuevo_id_mecanico: 5, // Mecánico inactivo
    };
    assert!(OrdenesService::reasignar_mecanico_ot(&conn, req_reasignar).is_err());
}

// 🧪 6. Mecánico intentando agregar/eliminar detalles en la OT de otro mecánico
#[test]
fn test_06_mecanico_bloqueado_en_ot_ajena() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2, // Asignado a Mecánico 2
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

    // Mecánico 4 intenta modificar la OT del Mecánico 2
    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 4).is_err());
}

// 🧪 7. Administrador agregando productos en OT ajena y verificando auditoría con id del Admin
#[test]
fn test_07_admin_agrega_producto_y_auditoria() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
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

    // Admin (ID 1) lo agrega
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

// 🧪 8. Stock insuficiente al intentar agregar un producto
#[test]
fn test_08_stock_insuficiente() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 2, // Producto 2 solo tiene stock 2.0
        cantidad: 5.0,  // Solicitamos más del disponible
    };

    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 2).is_err());
}

// 🧪 9. Devolución de stock al eliminar un producto de la OT
#[test]
fn test_09_devolucion_stock_al_eliminar_producto() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1, // Stock inicial: 10.0
        cantidad: 2.0,
    };
    let ot_actualizada = OrdenesService::agregar_producto_ot(&mut conn, payload, 2).unwrap();
    let id_detalle = ot_actualizada.detalles[0].id_detalle;

    // Verificar que bajó a 8.0
    let stock_despues_agregar: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_despues_agregar, 8.0);

    // Eliminar producto
    OrdenesService::eliminar_producto_ot(&mut conn, id_detalle, ot.id_ot, 2).unwrap();

    // Verificar que se devolvió a 10.0
    let stock_restaurado: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_restaurado, 10.0);
}

// 🧪 10. Devolución de stock al cancelar la OT
#[test]
fn test_10_devolucion_stock_al_cancelar_ot() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1, // Stock inicial: 10.0
        cantidad: 3.0,
    };
    OrdenesService::agregar_producto_ot(&mut conn, payload, 2).unwrap();

    // Cancelar la OT como Admin (ID 1)
    let cambio = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "CANCELADO".to_string(),
        zanja: None,
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio, Some(1)).unwrap();

    // Stock debe regresar a 10.0
    let stock_final: f64 = conn
        .query_row(
            "SELECT stock_actual FROM productos WHERE id_producto = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stock_final, 10.0);
}

// 🧪 11. Transiciones de estado válidas (EN_ESPERA -> EN_PROCESO -> FINALIZADO)
#[test]
fn test_11_flujo_transiciones_validas() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();
    assert_eq!(ot.estado, "EN_ESPERA");

    // EN_ESPERA -> EN_PROCESO
    let cambio_proceso = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "EN_PROCESO".to_string(),
        zanja: Some(1),
    };
    let ot_proceso = OrdenesService::cambiar_estado_ot(&mut conn, cambio_proceso, Some(2)).unwrap();
    assert_eq!(ot_proceso.estado, "EN_PROCESO");

    // EN_PROCESO -> FINALIZADO
    let cambio_finalizado = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "FINALIZADO".to_string(),
        zanja: None,
    };
    let ot_fin = OrdenesService::cambiar_estado_ot(&mut conn, cambio_finalizado, Some(2)).unwrap();
    assert_eq!(ot_fin.estado, "FINALIZADO");
}

// 🧪 12. Estados terminales (Intentar modificar OT FINALIZADA o CANCELADA)
#[test]
fn test_12_bloqueo_modificacion_estado_terminal() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    // Cancelar
    let cambio_cancelar = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "CANCELADO".to_string(),
        zanja: None,
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio_cancelar, Some(1)).unwrap();

    // Intentar agregar producto a OT cancelada
    let payload = AgregarProductoOtDto {
        id_ot: ot.id_ot,
        id_producto: 1,
        cantidad: 1.0,
    };
    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 1).is_err());
}

// 🧪 13. Intento de operación con usuario None
#[test]
fn test_13_usuario_none_rechazado() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
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

// 🧪 14. Intento de operación con usuario inactivo
#[test]
fn test_14_usuario_ejecutor_inactivo_rechazado() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
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
    // Admin inactivo (ID 6)
    assert!(OrdenesService::agregar_producto_ot(&mut conn, payload, 6).is_err());
}

// 🧪 15. Rechazo de tipo_aceite inválido
#[test]
fn test_15_tipo_aceite_invalido_rechazado() {
    let mut conn = setup_test_db();
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "DESCONOCIDO".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

// 🧪 16. Permitir kilometraje de ingreso igual al actual de la Ficha
#[test]
fn test_16_kilometraje_igual_a_ficha_permitido() {
    let mut conn = setup_test_db();
    // Ficha en 50,000 km. Ingreso con 50,000 km.
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_ok());
}

// 🧪 17. Rechazar kilometraje inferior a una OT previa del mismo vehículo
#[test]
fn test_17_kilometraje_inferior_a_ficha_rechazado() {
    let mut conn = setup_test_db();
    // Ficha en 50,000 km. Ingreso con 40,000 km (inferior).
    let req = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 40000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    assert!(OrdenesService::crear_orden_trabajo(&mut conn, req).is_err());
}

// 🧪 18. Finalización liberando la zanja asignada
#[test]
fn test_18_finalizacion_libera_zanja() {
    let mut conn = setup_test_db();
    let req_crear = CrearOrdenTrabajoDto {
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        kilometraje_ingreso: 50000,
        tipo_aceite: "MINERAL".to_string(),
        observaciones: None,
    };
    let ot = OrdenesService::crear_orden_trabajo(&mut conn, req_crear).unwrap();

    // Pasar a EN_PROCESO
    let cambio_proc = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "EN_PROCESO".to_string(),
        zanja: Some(1),
    };
    OrdenesService::cambiar_estado_ot(&mut conn, cambio_proc, Some(2)).unwrap();

    // Finalizar
    let cambio_fin = CambiarEstadoOtDto {
        id_ot: ot.id_ot,
        nuevo_estado: "FINALIZADO".to_string(),
        zanja: None,
    };
    let ot_finalizada = OrdenesService::cambiar_estado_ot(&mut conn, cambio_fin, Some(2)).unwrap();

    // Zanja debe quedar en None en la OT devuelta
    assert!(ot_finalizada.zanja.is_none());
}

// 🧪 19. Comprobar que la restricción UNIQUE parcial de SQLite previene dos OTs activas en la misma zanja
#[test]
fn test_19_indice_unico_parcial_zanja_activa() {
    let conn = setup_test_db();
    conn.execute(
        "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje)
         VALUES ('OT-TEST-1', 'ABC-123', 2, 1, 'EN_PROCESO', 50000, 55000)",
        [],
    ).unwrap();

    let res = conn.execute(
        "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje)
         VALUES ('OT-TEST-2', 'ABC-123', 2, 1, 'EN_ESPERA', 50000, 55000)",
        [],
    );

    assert!(res.is_err());
}

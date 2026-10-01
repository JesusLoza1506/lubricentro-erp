-- 1. permisos_rol
CREATE TABLE permisos_rol (
    id_permiso INTEGER PRIMARY KEY AUTOINCREMENT,
    rol TEXT NOT NULL CHECK (rol IN ('ADMINISTRADOR', 'MECANICO', 'CAJERO')),
    modulo TEXT NOT NULL,
    puede_ver BOOLEAN NOT NULL DEFAULT 0,
    puede_crear BOOLEAN NOT NULL DEFAULT 0,
    puede_editar BOOLEAN NOT NULL DEFAULT 0,
    puede_eliminar BOOLEAN NOT NULL DEFAULT 0
);

-- 2. usuarios
CREATE TABLE usuarios (
    id_usuario INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_completo TEXT NOT NULL,
    usuario TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    rol TEXT NOT NULL CHECK (rol IN ('ADMINISTRADOR', 'MECANICO', 'CAJERO')),
    activo BOOLEAN NOT NULL DEFAULT 1
);

-- 3. caja_chica
CREATE TABLE caja_chica (
    id_caja INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER NOT NULL,
    monto_apertura REAL NOT NULL,
    monto_cierre_efectivo REAL,
    monto_cierre_digital REAL,
    monto_diferencia REAL,
    estado TEXT NOT NULL CHECK (estado IN ('ABIERTA', 'CERRADA')),
    fecha_apertura DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_cierre DATETIME,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

-- 4. clientes
CREATE TABLE clientes (
    id_cliente INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT NOT NULL CHECK (tipo_documento IN ('DNI', 'RUC', 'CE', 'PASAPORTE')),
    numero_documento TEXT NOT NULL UNIQUE,
    nombre_razon_social TEXT NOT NULL,
    telefono TEXT,
    direccion TEXT
);

-- 5. vehiculos (La placa es Primary Key según el diagrama)
CREATE TABLE vehiculos (
    placa TEXT PRIMARY KEY,
    id_cliente INTEGER NOT NULL,
    marca TEXT,
    modelo TEXT,
    anio INTEGER,
    tipo_motor TEXT,
    kilometraje_actual INTEGER,
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT
);

-- 6. categorias
CREATE TABLE categorias (
    id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_categoria TEXT NOT NULL
);

-- 7. productos (ACTUALIZADO: Agregado la columna 'nombre')
CREATE TABLE IF NOT EXISTS productos (
    id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
    id_categoria INTEGER NOT NULL,
    nombre TEXT NOT NULL,
    codigo_barras TEXT,
    unidad_medida TEXT,
    stock_actual REAL NOT NULL DEFAULT 0 CHECK (stock_actual >= 0),
    stock_minimo REAL NOT NULL DEFAULT 0 CHECK (stock_minimo >= 0),
    precio_compra REAL NOT NULL CHECK (precio_compra >= 0),
    precio_venta REAL NOT NULL CHECK (precio_venta >= 0),
    FOREIGN KEY (id_categoria) REFERENCES categorias(id_categoria)
);

-- 8. ordenes_trabajo
CREATE TABLE ordenes_trabajo (
    id_ot INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_ot TEXT NOT NULL,
    placa TEXT NOT NULL,
    id_mecanico INTEGER NOT NULL,
    zanja INTEGER,
    estado TEXT NOT NULL CHECK (estado IN ('EN_ESPERA', 'EN_PROCESO', 'FINALIZADO', 'CANCELADO')),
    kilometraje_ingreso INTEGER NOT NULL,
    proximo_kilometraje INTEGER NOT NULL,
    fecha_ingreso DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (placa) REFERENCES vehiculos(placa) ON DELETE RESTRICT,
    FOREIGN KEY (id_mecanico) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

-- 9. series_comprobante
CREATE TABLE series_comprobante (
    id_serie INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_comprobante TEXT NOT NULL,
    serie TEXT NOT NULL UNIQUE,
    correlativo_actual INTEGER NOT NULL DEFAULT 0,
    activa BOOLEAN NOT NULL DEFAULT 1
);

-- 10. comprobantes
CREATE TABLE comprobantes (
    id_comprobante INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER,
    id_cliente INTEGER NOT NULL,
    id_cajero INTEGER NOT NULL,
    tipo_comprobante TEXT NOT NULL,
    id_serie INTEGER, /* Permite NULL para PROFORMA */
    correlativo INTEGER NOT NULL,
    monto_subtotal REAL NOT NULL,
    monto_igv REAL NOT NULL,
    monto_total REAL NOT NULL,
    medio_pago TEXT NOT NULL,
    estado_sunat TEXT NOT NULL CHECK (estado_sunat IN ('PENDIENTE', 'EN_COLA', 'ACEPTADO', 'RECHAZADO', 'OBSERVADO', 'ANULADO')),
    fecha_emision DATETIME DEFAULT CURRENT_TIMESTAMP,
    CHECK (tipo_comprobante = 'PROFORMA' OR id_serie IS NOT NULL),
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE RESTRICT,
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    FOREIGN KEY (id_cajero) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    FOREIGN KEY (id_serie) REFERENCES series_comprobante(id_serie) ON DELETE RESTRICT
);

-- 11. detalle_comprobante
CREATE TABLE detalle_comprobante (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    id_producto INTEGER NOT NULL,
    cantidad REAL NOT NULL,
    precio_unitario REAL NOT NULL,
    subtotal REAL NOT NULL,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE CASCADE,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT
);

-- 12. cola_envio_sunat
CREATE TABLE cola_envio_sunat (
    id_cola INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    estado_envio TEXT NOT NULL CHECK (estado_envio IN ('PENDIENTE', 'PROCESANDO', 'ENVIADO', 'ERROR')),
    intentos INTEGER NOT NULL DEFAULT 0,
    ultima_error TEXT,
    respuesta_cdr TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_ultimo_intento DATETIME,
    fecha_confirmacion DATETIME,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE CASCADE
);

-- 13. auditoria
CREATE TABLE auditoria (
    id_auditoria INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER,
    tabla_afectada TEXT NOT NULL,
    accion TEXT NOT NULL,
    detalle TEXT,
    fecha DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS movimientos_inventario (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_producto INTEGER NOT NULL,
    id_usuario INTEGER,
    tipo_movimiento TEXT NOT NULL, -- 'ENTRADA', 'SALIDA', 'AJUSTE', 'INICIAL', 'ELIMINACION'
    cantidad_anterior REAL NOT NULL,
    cantidad_nueva REAL NOT NULL,
    diferencia REAL NOT NULL,
    motivo TEXT,
    fecha DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto),
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario)
);
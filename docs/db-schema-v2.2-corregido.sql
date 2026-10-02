-- ============================================================
-- ERP DESKTOP - LUBRICENTRO GLORIA S.R.L.
-- Esquema SQL v2.2 corregido
-- Diseño independiente de la aplicación
-- ============================================================

PRAGMA foreign_keys = ON;

CREATE TABLE permisos_rol (
    id_permiso INTEGER PRIMARY KEY AUTOINCREMENT,
    rol TEXT NOT NULL CHECK (rol IN ('ADMINISTRADOR', 'MECANICO', 'CAJERO')),
    modulo TEXT NOT NULL,
    puede_ver INTEGER NOT NULL DEFAULT 0 CHECK (puede_ver IN (0, 1)),
    puede_crear INTEGER NOT NULL DEFAULT 0 CHECK (puede_crear IN (0, 1)),
    puede_editar INTEGER NOT NULL DEFAULT 0 CHECK (puede_editar IN (0, 1)),
    puede_eliminar INTEGER NOT NULL DEFAULT 0 CHECK (puede_eliminar IN (0, 1)),
    UNIQUE (rol, modulo)
);

CREATE TABLE usuarios (
    id_usuario INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_completo TEXT NOT NULL CHECK (length(trim(nombre_completo)) > 0),
    usuario TEXT NOT NULL COLLATE NOCASE UNIQUE,
    password_hash TEXT NOT NULL,
    rol TEXT NOT NULL CHECK (rol IN ('ADMINISTRADOR', 'MECANICO', 'CAJERO')),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    fecha_creacion TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE caja_chica (
    id_caja INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER NOT NULL,
    monto_apertura REAL NOT NULL CHECK (monto_apertura >= 0),
    monto_cierre_efectivo REAL CHECK (monto_cierre_efectivo >= 0),
    monto_cierre_digital REAL CHECK (monto_cierre_digital >= 0),
    monto_diferencia REAL,
    estado TEXT NOT NULL CHECK (estado IN ('ABIERTA', 'CERRADA')),
    fecha_apertura TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_cierre TEXT,
    CHECK ((estado = 'ABIERTA' AND fecha_cierre IS NULL) OR
           (estado = 'CERRADA' AND fecha_cierre IS NOT NULL)),
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX uq_caja_abierta_usuario
    ON caja_chica(id_usuario) WHERE estado = 'ABIERTA';

CREATE TABLE movimientos_caja (
    id_movimiento_caja INTEGER PRIMARY KEY AUTOINCREMENT,
    id_caja INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    tipo_movimiento TEXT NOT NULL CHECK (tipo_movimiento IN
        ('APERTURA', 'VENTA', 'INGRESO', 'EGRESO', 'RETIRO', 'CIERRE', 'DEVOLUCION')),
    medio_pago TEXT CHECK (medio_pago IN ('EFECTIVO', 'YAPE', 'PLIN', 'TARJETA')),
    monto REAL NOT NULL CHECK (monto >= 0),
    motivo TEXT,
    id_comprobante INTEGER,
    fecha TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_caja) REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE TABLE clientes (
    id_cliente INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT NOT NULL CHECK (tipo_documento IN ('DNI', 'RUC', 'CE', 'PASAPORTE')),
    numero_documento TEXT NOT NULL UNIQUE,
    nombre_razon_social TEXT NOT NULL CHECK (length(trim(nombre_razon_social)) > 0),
    telefono TEXT,
    direccion TEXT,
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

CREATE TABLE vehiculos (
    placa TEXT PRIMARY KEY COLLATE NOCASE,
    id_cliente INTEGER NOT NULL,
    marca TEXT,
    modelo TEXT,
    anio INTEGER CHECK (anio IS NULL OR anio BETWEEN 1900 AND  a strftime('%Y', 'now')),
    tipo_motor TEXT,
    kilometraje_actual INTEGER NOT NULL DEFAULT 0 CHECK (kilometraje_actual >= 0),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT
);

CREATE TABLE categorias (
    id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_categoria TEXT NOT NULL COLLATE NOCASE,
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    UNIQUE (nombre_categoria)
);

CREATE TABLE productos (
    id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
    id_categoria INTEGER NOT NULL,
    nombre TEXT NOT NULL COLLATE NOCASE,
    codigo_barras TEXT COLLATE NOCASE,
    unidad_medida TEXT NOT NULL CHECK (unidad_medida IN ('LITRO', 'GALON', 'UNIDAD', 'BARRIL')),
    stock_actual REAL NOT NULL DEFAULT 0 CHECK (stock_actual >= 0),
    stock_minimo REAL NOT NULL DEFAULT 0 CHECK (stock_minimo >= 0),
    precio_compra REAL NOT NULL CHECK (precio_compra >= 0),
    precio_venta REAL NOT NULL CHECK (precio_venta >= 0),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    fecha_creacion TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_categoria) REFERENCES categorias(id_categoria) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX uq_producto_nombre_activo
    ON productos(lower(trim(nombre))) WHERE activo = 1;
CREATE UNIQUE INDEX uq_producto_codigo_barras
    ON productos(codigo_barras) WHERE codigo_barras IS NOT NULL AND activo = 1;

CREATE TABLE ordenes_trabajo (
    id_ot INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_ot TEXT NOT NULL UNIQUE,
    placa TEXT NOT NULL,
    id_mecanico INTEGER NOT NULL,
    zanja INTEGER NOT NULL CHECK (zanja IN (1, 2)),
    estado TEXT NOT NULL CHECK (estado IN ('EN_ESPERA', 'EN_PROCESO', 'FINALIZADO', 'CANCELADO')),
    kilometraje_ingreso INTEGER NOT NULL CHECK (kilometraje_ingreso >= 0),
    proximo_kilometraje INTEGER NOT NULL CHECK (proximo_kilometraje >= kilometraje_ingreso),
    observaciones TEXT,
    fecha_ingreso TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_cierre TEXT,
    FOREIGN KEY (placa) REFERENCES vehiculos(placa) ON DELETE RESTRICT,
    FOREIGN KEY (id_mecanico) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE TABLE ordenes_trabajo_detalle (
    id_detalle_ot INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER NOT NULL,
    tipo_linea TEXT NOT NULL CHECK (tipo_linea IN ('SERVICIO', 'PRODUCTO')),
    id_producto INTEGER,
    descripcion TEXT NOT NULL,
    cantidad REAL NOT NULL CHECK (cantidad > 0),
    precio_unitario REAL NOT NULL CHECK (precio_unitario >= 0),
    subtotal REAL NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE CASCADE,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT,
    CHECK ((tipo_linea = 'PRODUCTO' AND id_producto IS NOT NULL) OR
           (tipo_linea = 'SERVICIO' AND id_producto IS NULL))
);

CREATE TABLE series_comprobante (
    id_serie INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_comprobante TEXT NOT NULL CHECK (tipo_comprobante IN ('BOLETA', 'FACTURA')),
    serie TEXT NOT NULL UNIQUE,
    correlativo_actual INTEGER NOT NULL DEFAULT 0 CHECK (correlativo_actual >= 0),
    activa INTEGER NOT NULL DEFAULT 1 CHECK (activa IN (0, 1))
);

CREATE TABLE comprobantes (
    id_comprobante INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER,
    id_cliente INTEGER NOT NULL,
    id_cajero INTEGER NOT NULL,
    tipo_comprobante TEXT NOT NULL CHECK (tipo_comprobante IN ('BOLETA', 'FACTURA', 'PROFORMA')),
    id_serie INTEGER,
    correlativo INTEGER NOT NULL CHECK (correlativo >= 0),
    monto_subtotal REAL NOT NULL CHECK (monto_subtotal >= 0),
    monto_igv REAL NOT NULL CHECK (monto_igv >= 0),
    monto_total REAL NOT NULL CHECK (monto_total >= 0),
    medio_pago TEXT NOT NULL CHECK (medio_pago IN ('EFECTIVO', 'YAPE', 'PLIN', 'TARJETA')),
    estado_sunat TEXT NOT NULL CHECK (estado_sunat IN
        ('PENDIENTE', 'EN_COLA', 'ACEPTADO', 'RECHAZADO', 'OBSERVADO', 'ANULADO')),
    fecha_emision TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (tipo_comprobante = 'PROFORMA' OR id_serie IS NOT NULL),
    CHECK (tipo_comprobante = 'PROFORMA' OR monto_total >= monto_subtotal),
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE SET NULL,
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    FOREIGN KEY (id_cajero) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    FOREIGN KEY (id_serie) REFERENCES series_comprobante(id_serie) ON DELETE RESTRICT,
    UNIQUE (id_serie, correlativo)
);

CREATE TABLE detalle_comprobante (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    id_producto INTEGER,
    descripcion TEXT NOT NULL,
    cantidad REAL NOT NULL CHECK (cantidad > 0),
    precio_unitario REAL NOT NULL CHECK (precio_unitario >= 0),
    subtotal REAL NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE CASCADE,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE SET NULL
);

CREATE TABLE cola_envio_sunat (
    id_cola INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL UNIQUE,
    estado_envio TEXT NOT NULL CHECK (estado_envio IN
        ('EN_COLA', 'ENVIANDO', 'ENVIADO', 'RECHAZADO', 'ERROR')),
    intentos INTEGER NOT NULL DEFAULT 0 CHECK (intentos >= 0),
    ultimo_error TEXT,
    respuesta_cdr TEXT,
    fecha_creacion TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_ultimo_intento TEXT,
    fecha_confirmacion TEXT,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE CASCADE
);

CREATE TABLE auditoria (
    id_auditoria INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER,
    tabla_afectada TEXT NOT NULL,
    accion TEXT NOT NULL CHECK (accion IN ('INSERT', 'UPDATE', 'DELETE', 'LOGIN', 'ANULAR')),
    detalle TEXT,
    fecha TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL
);

CREATE TABLE movimientos_inventario (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_producto INTEGER NOT NULL,
    id_usuario INTEGER,
    tipo_movimiento TEXT NOT NULL CHECK (tipo_movimiento IN
        ('ENTRADA', 'SALIDA', 'AJUSTE', 'INICIAL', 'ELIMINACION')),
    cantidad_anterior REAL NOT NULL CHECK (cantidad_anterior >= 0),
    cantidad_nueva REAL NOT NULL CHECK (cantidad_nueva >= 0),
    diferencia REAL NOT NULL,
    motivo TEXT,
    id_comprobante INTEGER,
    fecha TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE SET NULL
);

CREATE INDEX idx_vehiculos_cliente ON vehiculos(id_cliente);
CREATE INDEX idx_ot_placa ON ordenes_trabajo(placa);
CREATE INDEX idx_ot_estado ON ordenes_trabajo(estado);
CREATE INDEX idx_ot_detalle_ot ON ordenes_trabajo_detalle(id_ot);
CREATE INDEX idx_comprobantes_cliente ON comprobantes(id_cliente);
CREATE INDEX idx_comprobantes_estado ON comprobantes(estado_sunat);
CREATE INDEX idx_cola_estado ON cola_envio_sunat(estado_envio);
CREATE INDEX idx_productos_categoria ON productos(id_categoria);
CREATE INDEX idx_movimientos_producto ON movimientos_inventario(id_producto);
CREATE INDEX idx_movimientos_caja ON movimientos_caja(id_caja);

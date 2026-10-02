PRAGMA foreign_keys = ON;

-- ============================================================
-- ERP LUBRICENTRO GLORIA S.R.L. - ESQUEMA SQL V3
-- SQLite. Importes monetarios en centimos.
-- ============================================================

CREATE TABLE usuarios (
    id_usuario INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_completo TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    rol TEXT NOT NULL CHECK (rol IN ('ADMINISTRADOR', 'MECANICO', 'CAJERO')),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

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

CREATE TABLE clientes (
    id_cliente INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT NOT NULL CHECK (tipo_documento IN ('DNI', 'RUC')),
    numero_documento TEXT NOT NULL UNIQUE,
    nombre_razon_social TEXT NOT NULL,
    telefono TEXT,
    direccion TEXT,
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

CREATE TABLE vehiculos (
    placa TEXT PRIMARY KEY,
    id_cliente INTEGER NOT NULL,
    marca TEXT NOT NULL,
    modelo TEXT NOT NULL,
    anio INTEGER CHECK (anio IS NULL OR anio BETWEEN 1900 AND 2100),
    tipo_motor TEXT,
    kilometraje_actual INTEGER NOT NULL DEFAULT 0 CHECK (kilometraje_actual >= 0),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT
);

CREATE TABLE proveedores (
    id_proveedor INTEGER PRIMARY KEY AUTOINCREMENT,
    ruc TEXT NOT NULL UNIQUE,
    razon_social TEXT NOT NULL,
    telefono TEXT,
    contacto_asesor TEXT,
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

CREATE TABLE categorias (
    id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_categoria TEXT NOT NULL UNIQUE,
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

CREATE TABLE productos (
    id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
    id_categoria INTEGER NOT NULL,
    codigo_barras TEXT UNIQUE,
    descripcion TEXT NOT NULL,
    unidad_medida TEXT NOT NULL CHECK (unidad_medida IN ('LITRO', 'GALON', 'UNIDAD', 'BARRIL')),
    stock_actual REAL NOT NULL DEFAULT 0 CHECK (stock_actual >= 0),
    stock_minimo REAL NOT NULL DEFAULT 0 CHECK (stock_minimo >= 0),
    precio_compra INTEGER NOT NULL CHECK (precio_compra >= 0),
    precio_venta INTEGER NOT NULL CHECK (precio_venta >= 0),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1)),
    FOREIGN KEY (id_categoria) REFERENCES categorias(id_categoria) ON DELETE RESTRICT
);

CREATE TABLE servicios (
    id_servicio INTEGER PRIMARY KEY AUTOINCREMENT,
    descripcion TEXT NOT NULL,
    precio_referencial INTEGER NOT NULL CHECK (precio_referencial >= 0),
    activo INTEGER NOT NULL DEFAULT 1 CHECK (activo IN (0, 1))
);

CREATE TABLE compras (
    id_compra INTEGER PRIMARY KEY AUTOINCREMENT,
    id_proveedor INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    numero_factura TEXT NOT NULL,
    monto_total INTEGER NOT NULL CHECK (monto_total >= 0),
    estado TEXT NOT NULL CHECK (estado IN ('PENDIENTE', 'COMPLETADO', 'ANULADO')),
    fecha_compra DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_proveedor) REFERENCES proveedores(id_proveedor) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    UNIQUE (id_proveedor, numero_factura)
);

CREATE TABLE detalle_compra (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_compra INTEGER NOT NULL,
    id_producto INTEGER NOT NULL,
    cantidad REAL NOT NULL CHECK (cantidad > 0),
    precio_unitario INTEGER NOT NULL CHECK (precio_unitario >= 0),
    subtotal INTEGER NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (id_compra) REFERENCES compras(id_compra) ON DELETE RESTRICT,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT
);

CREATE TABLE ordenes_trabajo (
    id_ot INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_ot TEXT NOT NULL UNIQUE,
    placa TEXT NOT NULL,
    id_mecanico INTEGER NOT NULL,
    zanja INTEGER CHECK (zanja IN (1, 2)),
    estado TEXT NOT NULL CHECK (estado IN ('EN_ESPERA', 'EN_PROCESO', 'FINALIZADO', 'CANCELADO')),
    kilometraje_ingreso INTEGER NOT NULL CHECK (kilometraje_ingreso >= 0),
    proximo_kilometraje INTEGER NOT NULL,
    observaciones TEXT,
    fecha_ingreso DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (proximo_kilometraje >= kilometraje_ingreso),
    FOREIGN KEY (placa) REFERENCES vehiculos(placa) ON DELETE RESTRICT,
    FOREIGN KEY (id_mecanico) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE TABLE detalle_ot_productos (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER NOT NULL,
    id_producto INTEGER NOT NULL,
    cantidad REAL NOT NULL CHECK (cantidad > 0),
    precio_aplicado INTEGER NOT NULL CHECK (precio_aplicado >= 0),
    descuento INTEGER NOT NULL DEFAULT 0 CHECK (descuento >= 0),
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE CASCADE,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT
);

CREATE TABLE detalle_ot_servicios (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER NOT NULL,
    id_servicio INTEGER NOT NULL,
    id_mecanico_ejecutor INTEGER NOT NULL,
    precio_aplicado INTEGER NOT NULL CHECK (precio_aplicado >= 0),
    descuento INTEGER NOT NULL DEFAULT 0 CHECK (descuento >= 0),
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE CASCADE,
    FOREIGN KEY (id_servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (id_mecanico_ejecutor) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE TABLE caja_chica (
    id_caja INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER NOT NULL,
    monto_apertura INTEGER NOT NULL CHECK (monto_apertura >= 0),
    monto_cierre_efectivo INTEGER CHECK (monto_cierre_efectivo >= 0),
    monto_cierre_digital INTEGER CHECK (monto_cierre_digital >= 0),
    monto_diferencia INTEGER,
    estado TEXT NOT NULL CHECK (estado IN ('ABIERTA', 'CERRADA')),
    fecha_apertura DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_cierre DATETIME,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX uq_caja_abierta_usuario
ON caja_chica (id_usuario) WHERE estado = 'ABIERTA';

CREATE TABLE series_comprobante (
    id_serie INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_comprobante TEXT NOT NULL CHECK (tipo_comprobante IN ('BOLETA', 'FACTURA')),
    serie TEXT NOT NULL UNIQUE,
    correlativo_actual INTEGER NOT NULL DEFAULT 0 CHECK (correlativo_actual >= 0),
    activa INTEGER NOT NULL DEFAULT 1 CHECK (activa IN (0, 1)),
    UNIQUE (id_serie, tipo_comprobante)
);

CREATE TABLE comprobantes (
    id_comprobante INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ot INTEGER,
    id_cliente INTEGER NOT NULL,
    id_cajero INTEGER NOT NULL,
    id_caja INTEGER NOT NULL,
    tipo_comprobante TEXT NOT NULL CHECK (tipo_comprobante IN ('BOLETA', 'FACTURA', 'PROFORMA')),
    id_serie INTEGER,
    correlativo INTEGER NOT NULL CHECK (correlativo >= 0),
    monto_subtotal INTEGER NOT NULL CHECK (monto_subtotal >= 0),
    monto_igv INTEGER NOT NULL CHECK (monto_igv >= 0),
    monto_total INTEGER NOT NULL CHECK (monto_total >= 0),
    estado_sunat TEXT NOT NULL CHECK (estado_sunat IN ('PENDIENTE', 'EN_COLA', 'ACEPTADO', 'RECHAZADO', 'OBSERVADO', 'ANULADO')),
    fecha_emision DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_ot) REFERENCES ordenes_trabajo(id_ot) ON DELETE SET NULL,
    FOREIGN KEY (id_cliente) REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    FOREIGN KEY (id_cajero) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    FOREIGN KEY (id_caja) REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    FOREIGN KEY (id_serie, tipo_comprobante) REFERENCES series_comprobante(id_serie, tipo_comprobante) ON DELETE RESTRICT,
    UNIQUE (id_serie, correlativo),
    CHECK ((tipo_comprobante = 'PROFORMA' AND id_serie IS NULL) OR (tipo_comprobante IN ('BOLETA', 'FACTURA') AND id_serie IS NOT NULL)),
    CHECK (monto_total = monto_subtotal + monto_igv)
);

CREATE TABLE detalle_comprobante (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    id_producto INTEGER,
    id_servicio INTEGER,
    cantidad REAL NOT NULL CHECK (cantidad > 0),
    precio_unitario INTEGER NOT NULL CHECK (precio_unitario >= 0),
    subtotal INTEGER NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT,
    FOREIGN KEY (id_servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    CHECK ((id_producto IS NOT NULL AND id_servicio IS NULL) OR (id_producto IS NULL AND id_servicio IS NOT NULL))
);

CREATE TABLE pagos_comprobante (
    id_pago INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    medio_pago TEXT NOT NULL CHECK (medio_pago IN ('EFECTIVO', 'YAPE', 'PLIN', 'TARJETA')),
    monto INTEGER NOT NULL CHECK (monto > 0),
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT
);

CREATE TABLE movimientos_caja (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_caja INTEGER NOT NULL,
    id_comprobante INTEGER,
    tipo_movimiento TEXT NOT NULL CHECK (tipo_movimiento IN ('INGRESO_VENTA', 'EGRESO_GASTO', 'RETIRO', 'DEVOLUCION')),
    monto INTEGER NOT NULL CHECK (monto > 0),
    descripcion TEXT NOT NULL,
    referencia TEXT,
    fecha_movimiento DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_caja) REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT
);

CREATE TABLE movimientos_inventario (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_producto INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    id_detalle_compra INTEGER,
    id_detalle_ot INTEGER,
    id_detalle_comprobante INTEGER,
    tipo_movimiento TEXT NOT NULL CHECK (tipo_movimiento IN ('INGRESO_COMPRA', 'EGRESO_VENTA', 'MERMA', 'AJUSTE')),
    stock_anterior REAL NOT NULL CHECK (stock_anterior >= 0),
    diferencia REAL NOT NULL CHECK (diferencia <> 0),
    stock_resultante REAL NOT NULL CHECK (stock_resultante >= 0),
    referencia TEXT,
    fecha_movimiento DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_producto) REFERENCES productos(id_producto) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    FOREIGN KEY (id_detalle_compra) REFERENCES detalle_compra(id_detalle) ON DELETE RESTRICT,
    FOREIGN KEY (id_detalle_ot) REFERENCES detalle_ot_productos(id_detalle) ON DELETE RESTRICT,
    FOREIGN KEY (id_detalle_comprobante) REFERENCES detalle_comprobante(id_detalle) ON DELETE RESTRICT,
    CHECK (stock_anterior + diferencia = stock_resultante),
    CHECK (((id_detalle_compra IS NOT NULL) + (id_detalle_ot IS NOT NULL) + (id_detalle_comprobante IS NOT NULL)) <= 1)
);

CREATE TABLE seguimientos_fidelizacion (
    id_seguimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    placa TEXT NOT NULL,
    fecha_programada DATE NOT NULL,
    fecha_contacto DATETIME,
    contactado INTEGER NOT NULL DEFAULT 0 CHECK (contactado IN (0, 1)),
    observacion TEXT,
    id_usuario INTEGER,
    FOREIGN KEY (placa) REFERENCES vehiculos(placa) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL
);

CREATE TABLE cola_envio_sunat (
    id_cola INTEGER PRIMARY KEY AUTOINCREMENT,
    id_comprobante INTEGER NOT NULL,
    estado_envio TEXT NOT NULL CHECK (estado_envio IN ('EN_COLA', 'ENVIANDO', 'ENVIADO', 'RECHAZADO', 'ERROR')),
    intentos INTEGER NOT NULL DEFAULT 0 CHECK (intentos >= 0),
    ultimo_error TEXT,
    respuesta_cdr TEXT,
    fecha_creacion DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_ultimo_intento DATETIME,
    fecha_confirmacion DATETIME,
    FOREIGN KEY (id_comprobante) REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT
);

CREATE TABLE backups (
    id_backup INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_archivo TEXT NOT NULL,
    ruta_archivo TEXT NOT NULL,
    tamano_bytes INTEGER NOT NULL CHECK (tamano_bytes >= 0),
    tipo TEXT NOT NULL CHECK (tipo IN ('AUTOMATICO', 'MANUAL')),
    fecha_creacion DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id_usuario INTEGER,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL
);

CREATE TABLE auditoria (
    id_auditoria INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario INTEGER,
    tabla_afectada TEXT NOT NULL,
    accion TEXT NOT NULL CHECK (accion IN ('INSERT', 'UPDATE', 'DELETE')),
    detalle TEXT,
    fecha DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_usuario) REFERENCES usuarios(id_usuario) ON DELETE SET NULL
);

-- Permisos: ausencia de fila significa acceso denegado.
INSERT INTO permisos_rol (rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES
('ADMINISTRADOR', 'FICHA_VEHICULAR', 1, 1, 1, 1),
('MECANICO', 'FICHA_VEHICULAR', 1, 0, 0, 0),
('CAJERO', 'FICHA_VEHICULAR', 1, 1, 1, 0),
('ADMINISTRADOR', 'OT', 1, 1, 1, 1),
('MECANICO', 'OT', 1, 0, 1, 0),
('CAJERO', 'OT', 1, 1, 0, 0),
('ADMINISTRADOR', 'INVENTARIO', 1, 1, 1, 1),
('MECANICO', 'INVENTARIO', 1, 0, 0, 0),
('CAJERO', 'INVENTARIO', 1, 0, 0, 0),
('ADMINISTRADOR', 'POS', 1, 1, 1, 1),
('CAJERO', 'POS', 1, 1, 0, 0),
('ADMINISTRADOR', 'CAJA', 1, 1, 1, 0),
('CAJERO', 'CAJA', 1, 1, 0, 0),
('ADMINISTRADOR', 'FIDELIZACION', 1, 0, 1, 0),
('CAJERO', 'FIDELIZACION', 1, 0, 1, 0),
('ADMINISTRADOR', 'COLA_SUNAT', 1, 0, 0, 0),
('CAJERO', 'COLA_SUNAT', 1, 0, 0, 0),
('ADMINISTRADOR', 'BACKUP', 1, 1, 0, 1);

CREATE UNIQUE INDEX uq_fidelizacion_pendiente
ON seguimientos_fidelizacion (placa, fecha_programada)
WHERE contactado = 0;

CREATE INDEX idx_vehiculos_cliente ON vehiculos(id_cliente);
CREATE INDEX idx_ot_placa_estado ON ordenes_trabajo(placa, estado);
CREATE INDEX idx_ot_mecanico_estado ON ordenes_trabajo(id_mecanico, estado);
CREATE INDEX idx_productos_categoria ON productos(id_categoria);
CREATE INDEX idx_movimientos_producto_fecha ON movimientos_inventario(id_producto, fecha_movimiento);
CREATE INDEX idx_comprobantes_cliente_fecha ON comprobantes(id_cliente, fecha_emision);
CREATE INDEX idx_cola_estado ON cola_envio_sunat(estado_envio);
CREATE INDEX idx_movimientos_caja_caja_fecha ON movimientos_caja(id_caja, fecha_movimiento);

CREATE TRIGGER trg_valida_rol_mecanico_insert
BEFORE INSERT ON ordenes_trabajo
FOR EACH ROW
WHEN (SELECT rol FROM usuarios WHERE id_usuario = NEW.id_mecanico) <> 'MECANICO'
BEGIN
    SELECT RAISE(ABORT, 'El usuario asignado no tiene rol MECANICO');
END;

CREATE TRIGGER trg_valida_rol_cajero_insert
BEFORE INSERT ON comprobantes
FOR EACH ROW
WHEN (SELECT rol FROM usuarios WHERE id_usuario = NEW.id_cajero) <> 'CAJERO'
BEGIN
    SELECT RAISE(ABORT, 'El usuario asignado no tiene rol CAJERO');
END;

CREATE TRIGGER trg_valida_kardex_insert
BEFORE INSERT ON movimientos_inventario
FOR EACH ROW
WHEN NEW.stock_anterior + NEW.diferencia <> NEW.stock_resultante
BEGIN
    SELECT RAISE(ABORT, 'El calculo del kardex no es valido');
END;

CREATE TRIGGER trg_valida_pago_insert
AFTER INSERT ON pagos_comprobante
FOR EACH ROW
WHEN (SELECT COALESCE(SUM(monto), 0) FROM pagos_comprobante WHERE id_comprobante = NEW.id_comprobante) >
     (SELECT monto_total FROM comprobantes WHERE id_comprobante = NEW.id_comprobante)
BEGIN
    SELECT RAISE(ABORT, 'La suma de pagos supera el total del comprobante');
END;

CREATE TRIGGER trg_producto_activo_compra
BEFORE INSERT ON detalle_compra
FOR EACH ROW
WHEN (SELECT activo FROM productos WHERE id_producto = NEW.id_producto) <> 1
BEGIN
    SELECT RAISE(ABORT, 'El producto esta inactivo');
END;

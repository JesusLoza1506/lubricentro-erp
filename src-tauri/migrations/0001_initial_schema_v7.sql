-- LUBRICENTRO GLORIA S.R.L. — ESQUEMA INICIAL V7.0
-- Contrato: diez módulos simplificados v7 + informe general v7 (15-10-2026).
-- 27 tablas de aplicación. Las vistas NO almacenan datos ni cuentan como tablas.
-- SOLO BASE VACÍA. NO es una migración de datos desde v6.1.
-- NO sustituir una migración ya aplicada ni ejecutar encima de la BD real.
-- Runner: habilitar foreign_keys=ON antes de BEGIN IMMEDIATE; ejecutar TODO y COMMIT.
-- Si falla: ROLLBACK; no continuar con un esquema parcial. WAL fuera de migración.
-- SQLite >=3.38 compilado con JSON; STRICT. busy_timeout=5000 en cada conexión.
-- Base independiente por ambiente. Para Producción, sustituir SOLO el literal del
-- INSERT de configuracion por PRODUCCION antes de crearla. Ambiente luego inmutable.
--
-- FECHAS: TEXT YYYY-MM-DD HH:MM:SS de America/Lima (UTC-05:00).
-- DEFAULT datetime('now','-5 hours') parte del reloj UTC y NO depende de Windows.
-- Rust convierte un instante externo UNA vez; jamás resta 5 h a texto ya local.
-- CHECK verifica formato/calendario, no puede detectar UTC mal etiquetado como local.
-- Fechas explícitas/actualizaciones, filtros y cruce de medianoche: mismo reloj Lima.
--
-- Dinero comercial: céntimos INTEGER. Stock: UND/ml INTEGER; precio granel por L.
-- Costos/valores: microsoles (6 decimales); promedio NO se persiste duplicado.
-- Rust usa i128/Decimal y redondeo mitad hacia arriba. No REAL monetario.
-- Impuesto: igv_bp/igv_cent conservan nombres del adaptador y representan la tasa
-- TOTAL 18% (IGV+IPM) de las operaciones ordinarias incluidas, no otros regímenes.
--
-- Rust es responsable de autenticar, rol/sesión/propiedad y auditoría antes/después.
-- Triggers validan integridad; id_usuario no prueba quién abrió la sesión.
-- Una transacción por alta+inicial, recepción+servicios, consumo+kardex+saldo,
-- finalización+km del vehículo, venta+líneas+pagos+stock+número+documento,
-- devolución+reintegro+recuperables+nota, y cierre+resúmenes+auditoría.
-- PREPARANDO es interno: no permitir COMMIT sin completar cabeceras y documento.
-- Kardex se inserta ANTES del UPDATE producto al saldo resultante. No auto-stock.
-- La nota interna y correcciones públicas viven en la OT, con auditoría común.
-- Servicios de OT editables abiertos; conservar al menos uno y precio histórico.
-- Cajero solo agrega servicios como parte del comando de recepción; después,
-- administrador/mecánico propio. No usar segundos de reloj como prueba de permiso.
--
-- Documento: estado de envío aquí mismo; SIN cola, agenda ni reintentos por reloj.
-- HTTP fuera de transacción; marcar ENVIANDO e intento incierto ANTES de invocar.
-- Tras reinicio/restauración: exigir consulta por identidad antes de reenviar.
-- Solo ausencia CONFIRMADA permite limpiar flags y reenviar mismo payload/número.
-- Rechazo oficial: nuevo número y documento ligado; mismo tipo y economía.
-- Revisar pendientes: admin, máximo 20 secuenciales por acción, sin doble envío.
-- Auditoría común registra intentos/transiciones sin secretos; no tabla de intentos.
-- No confirmar devolución si original o última nota de cada devolución no aceptados.
--
-- Backup: 23:00 Lima fijo, 14 rutinarias; Guardar fuera MANUAL elige destino cada vez.
-- Frase y token: DPAPI/archivos protegidos fuera de SQLite, nunca secretos aquí.
-- Referencias de backup/restauración y máximos previos se reconstituyen desde journal
-- durable externo después de sustituir BD; no reutilizar IDs de otra fotografía.
-- Recuperación: cerrar turno recuperado + token si falta + confirmar series activas;
-- incertidumbre de un documento no mantiene bloqueado todo el negocio al finalizar.
-- Cifrado/archivos/retención/restauración no los implementa SQL.
-- SIN usuarios ni credenciales predeterminadas. Primer admin se crea desde Login.
-- Fuentes técnicas: https://www.sqlite.org/lang_datefunc.html
-- https://www.sqlite.org/foreignkeys.html y https://www.sqlite.org/stricttables.html
-- Consultadas 15-10-2026. Ver notas finales de responsabilidades de implementación.

PRAGMA foreign_keys = ON;

PRAGMA busy_timeout = 5000;

CREATE TABLE usuarios (
    id_usuario INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_completo TEXT NOT NULL CHECK(length(trim(nombre_completo)) BETWEEN 3 AND 100),
    username TEXT NOT NULL UNIQUE CHECK(length(username) BETWEEN 3 AND 30 AND username=lower(trim(username)) AND username NOT GLOB '*[^a-z0-9._-]*'),
    password_hash TEXT NOT NULL CHECK(password_hash LIKE '$argon2id$%'),
    rol TEXT NOT NULL CHECK(rol IN ('ADMINISTRADOR','MECANICO','CAJERO')),
    activo INTEGER NOT NULL DEFAULT 1 CHECK(activo IN (0,1)),
    debe_cambiar_clave INTEGER NOT NULL DEFAULT 0 CHECK(debe_cambiar_clave IN (0,1)),
    intentos_fallidos INTEGER NOT NULL DEFAULT 0 CHECK(intentos_fallidos BETWEEN 0 AND 5),
    bloqueado_hasta TEXT CHECK (bloqueado_hasta IS NULL OR (length(bloqueado_hasta)=19 AND datetime(bloqueado_hasta, '+0 seconds') IS bloqueado_hasta)),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    fecha_cambio_clave TEXT CHECK (fecha_cambio_clave IS NULL OR (length(fecha_cambio_clave)=19 AND datetime(fecha_cambio_clave, '+0 seconds') IS fecha_cambio_clave)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0)
) STRICT;

CREATE TABLE configuracion (
    id_config INTEGER PRIMARY KEY CHECK(id_config=1),
    version_esquema INTEGER NOT NULL DEFAULT 700 CHECK(version_esquema=700),
    zona_horaria TEXT NOT NULL DEFAULT 'America/Lima' CHECK(zona_horaria='America/Lima'),
    ruc TEXT CHECK(ruc IS NULL OR (length(ruc)=11 AND ruc NOT GLOB '*[^0-9]*')),
    razon_social TEXT CHECK(razon_social IS NULL OR length(trim(razon_social)) BETWEEN 3 AND 150),
    nombre_comercial TEXT CHECK(nombre_comercial IS NULL OR length(trim(nombre_comercial)) BETWEEN 3 AND 100),
    direccion TEXT CHECK(direccion IS NULL OR length(trim(direccion)) BETWEEN 3 AND 200),
    ubigeo TEXT CHECK(ubigeo IS NULL OR (length(ubigeo)=6 AND ubigeo NOT GLOB '*[^0-9]*')),
    telefono TEXT,
    correo TEXT,
    ambiente_actual TEXT NOT NULL DEFAULT 'PRUEBA' CHECK(ambiente_actual IN ('PRUEBA','PRODUCCION')),
    igv_bp INTEGER NOT NULL DEFAULT 1800 CHECK(igv_bp=1800),
    descuento_cajero_bp INTEGER NOT NULL DEFAULT 500 CHECK(descuento_cajero_bp=500),
    ciclo_mineral_km INTEGER NOT NULL DEFAULT 5000 CHECK(ciclo_mineral_km=5000),
    ciclo_sintetico_km INTEGER NOT NULL DEFAULT 10000 CHECK(ciclo_sintetico_km=10000),
    margen_km INTEGER NOT NULL DEFAULT 500 CHECK(margen_km=500),
    plazo_dias INTEGER NOT NULL DEFAULT 180 CHECK(plazo_dias=180),
    hora_backup TEXT NOT NULL DEFAULT '23:00' CHECK(hora_backup='23:00'),
    retencion_backups INTEGER NOT NULL DEFAULT 14 CHECK(retencion_backups=14),
    recuperacion_hash TEXT CHECK(recuperacion_hash IS NULL OR recuperacion_hash LIKE '$argon2id$%'),
    fecha_codigo_recuperacion TEXT CHECK (fecha_codigo_recuperacion IS NULL OR (length(fecha_codigo_recuperacion)=19 AND datetime(fecha_codigo_recuperacion, '+0 seconds') IS fecha_codigo_recuperacion)),
    recuperacion_fallos INTEGER NOT NULL DEFAULT 0 CHECK(recuperacion_fallos BETWEEN 0 AND 5),
    recuperacion_bloqueada_hasta TEXT CHECK (recuperacion_bloqueada_hasta IS NULL OR (length(recuperacion_bloqueada_hasta)=19 AND datetime(recuperacion_bloqueada_hasta, '+0 seconds') IS recuperacion_bloqueada_hasta)),
    recuperacion_pendiente INTEGER NOT NULL DEFAULT 0 CHECK(recuperacion_pendiente IN (0,1)),
    revision_token_pendiente INTEGER NOT NULL DEFAULT 0 CHECK(revision_token_pendiente IN (0,1)),
    revision_caja_pendiente INTEGER NOT NULL DEFAULT 0 CHECK(revision_caja_pendiente IN (0,1)),
    nubefact_ruta_api TEXT CHECK(nubefact_ruta_api IS NULL OR (length(nubefact_ruta_api) BETWEEN 9 AND 500 AND nubefact_ruta_api LIKE 'https://%' AND instr(nubefact_ruta_api,'@')=0)),
    nubefact_token_ref TEXT CHECK(nubefact_token_ref IS NULL OR length(nubefact_token_ref) BETWEEN 1 AND 128),
    revision_series_pendiente INTEGER NOT NULL DEFAULT 0 CHECK(revision_series_pendiente IN (0,1)),
    fecha_actualizacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_actualizacion IS NULL OR (length(fecha_actualizacion)=19 AND datetime(fecha_actualizacion, '+0 seconds') IS fecha_actualizacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0)
) STRICT;

INSERT INTO configuracion(id_config,ambiente_actual) VALUES(1,'PRODUCCION');

CREATE TABLE zanjas (
    id_zanja INTEGER PRIMARY KEY CHECK(id_zanja BETWEEN 1 AND 6),
    activa INTEGER NOT NULL CHECK(activa IN (0,1))
) STRICT;

INSERT INTO zanjas VALUES (1,1),(2,1),(3,0),(4,0),(5,0),(6,0);

CREATE TABLE clientes (
    id_cliente INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT CHECK(tipo_documento IN ('DNI','RUC','CE')),
    numero_documento TEXT,
    nombre_razon_social TEXT NOT NULL CHECK(length(trim(nombre_razon_social)) BETWEEN 3 AND 150),
    telefono TEXT CHECK(telefono IS NULL OR (length(telefono) IN (7,9) AND telefono NOT GLOB '*[^0-9]*')),
    correo TEXT CHECK(correo IS NULL OR length(correo)<=150),
    direccion TEXT CHECK(direccion IS NULL OR length(direccion)<=200),
    es_varios INTEGER NOT NULL DEFAULT 0 CHECK(es_varios IN (0,1)),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    UNIQUE(tipo_documento,numero_documento),
    CHECK((tipo_documento='DNI' AND length(numero_documento)=8 AND numero_documento NOT GLOB '*[^0-9]*') OR (tipo_documento='RUC' AND length(numero_documento)=11 AND numero_documento NOT GLOB '*[^0-9]*') OR (tipo_documento='CE' AND length(numero_documento) BETWEEN 6 AND 12 AND numero_documento NOT GLOB '*[^A-Za-z0-9]*') OR (es_varios=1 AND tipo_documento IS NULL AND numero_documento IS NULL)),
    CHECK((es_varios=0 AND tipo_documento IS NOT NULL AND numero_documento IS NOT NULL) OR (es_varios=1 AND tipo_documento IS NULL AND numero_documento IS NULL))
) STRICT;

CREATE TABLE proveedores (
    id_proveedor INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT NOT NULL CHECK(tipo_documento IN ('DNI','RUC','CE')),
    numero_documento TEXT NOT NULL,
    nombre_razon_social TEXT NOT NULL CHECK(length(trim(nombre_razon_social)) BETWEEN 3 AND 150),
    telefono TEXT CHECK(telefono IS NULL OR (length(telefono) IN (7,9) AND telefono NOT GLOB '*[^0-9]*')),
    correo TEXT CHECK(correo IS NULL OR length(correo)<=150),
    direccion TEXT CHECK(direccion IS NULL OR length(direccion)<=200),
    contacto_asesor TEXT CHECK(contacto_asesor IS NULL OR length(contacto_asesor)<=100),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    UNIQUE(tipo_documento,numero_documento),
    CHECK((tipo_documento='DNI' AND length(numero_documento)=8 AND numero_documento NOT GLOB '*[^0-9]*') OR (tipo_documento='RUC' AND length(numero_documento)=11 AND numero_documento NOT GLOB '*[^0-9]*') OR (tipo_documento='CE' AND length(numero_documento) BETWEEN 6 AND 12 AND numero_documento NOT GLOB '*[^A-Za-z0-9]*')),
    CHECK(numero_documento IS NOT NULL)
) STRICT;

CREATE UNIQUE INDEX uq_cliente_varios ON clientes(es_varios) WHERE es_varios=1;

INSERT INTO clientes(nombre_razon_social,es_varios) VALUES('Cliente varios',1);

CREATE TABLE vehiculos (
    id_vehiculo INTEGER PRIMARY KEY AUTOINCREMENT,
    placa TEXT NOT NULL UNIQUE CHECK(length(placa)=6 AND placa=upper(placa) AND placa NOT GLOB '*[^A-Z0-9]*'),
    id_cliente INTEGER NOT NULL REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    marca TEXT NOT NULL CHECK(length(trim(marca)) BETWEEN 2 AND 60),
    modelo TEXT NOT NULL CHECK(length(trim(modelo)) BETWEEN 2 AND 60),
    anio INTEGER CHECK(anio IS NULL OR anio BETWEEN 1900 AND 2100),
    tipo_motor TEXT CHECK(tipo_motor IS NULL OR tipo_motor IN ('GASOLINA','DIESEL','GLP','GNV','HIBRIDO','ELECTRICO','OTRO')),
    kilometraje_actual INTEGER NOT NULL DEFAULT 0 CHECK(kilometraje_actual BETWEEN 0 AND 9999999),
    activo INTEGER NOT NULL DEFAULT 1 CHECK(activo IN (0,1)),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0)
) STRICT;

CREATE TABLE productos (
    id_producto INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE CHECK(length(codigo) BETWEEN 3 AND 30 AND codigo=upper(codigo) AND codigo NOT GLOB '*[^A-Z0-9-]*'),
    nombre TEXT NOT NULL CHECK(length(trim(nombre)) BETWEEN 3 AND 150),
    descripcion TEXT CHECK(descripcion IS NULL OR length(descripcion)<=500),
    categoria TEXT NOT NULL CHECK(categoria IN ('ACEITE','FILTRO','ADITIVO','GRANEL','OTRO')),
    unidad_medida TEXT NOT NULL CHECK(unidad_medida IN ('UND','ML')),
    stock_actual INTEGER NOT NULL DEFAULT 0 CHECK(stock_actual BETWEEN 0 AND 9999999999),
    stock_minimo INTEGER NOT NULL DEFAULT 0 CHECK(stock_minimo BETWEEN 0 AND 9999999999),
    valor_stock_micro INTEGER NOT NULL DEFAULT 0 CHECK(valor_stock_micro BETWEEN 0 AND 9000000000000000),
    ultimo_costo_micro INTEGER CHECK(ultimo_costo_micro BETWEEN 1 AND 999999999999),
    precio_venta_cent INTEGER NOT NULL CHECK(precio_venta_cent BETWEEN 1 AND 99999999),
    activo INTEGER NOT NULL DEFAULT 1 CHECK(activo IN (0,1)),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    CHECK((categoria='GRANEL' AND unidad_medida='ML') OR (categoria<>'GRANEL' AND unidad_medida='UND')),
    CHECK((stock_actual=0 AND valor_stock_micro=0) OR (stock_actual>0 AND valor_stock_micro>0)),
    CHECK(unidad_medida='ML' OR (stock_actual<=9999999 AND stock_minimo<=9999999))
) STRICT;

CREATE TABLE servicios (
    id_servicio INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE CHECK(length(codigo) BETWEEN 3 AND 30 AND codigo=upper(codigo) AND codigo NOT GLOB '*[^A-Z0-9-]*'),
    nombre TEXT NOT NULL CHECK(length(trim(nombre)) BETWEEN 3 AND 150),
    descripcion TEXT CHECK(descripcion IS NULL OR length(descripcion)<=500),
    precio_venta_cent INTEGER NOT NULL CHECK(precio_venta_cent BETWEEN 1 AND 99999999),
    es_cambio_aceite INTEGER NOT NULL DEFAULT 0 CHECK(es_cambio_aceite IN (0,1)),
    activo INTEGER NOT NULL DEFAULT 1 CHECK(activo IN (0,1)),
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0)
) STRICT;

CREATE TABLE ingresos_stock (
    id_ingreso INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    codigo TEXT NOT NULL UNIQUE,
    origen TEXT NOT NULL CHECK(origen IN ('COMPRA','INICIAL')),
    id_proveedor INTEGER REFERENCES proveedores(id_proveedor) ON DELETE RESTRICT,
    proveedor_snapshot TEXT CHECK(proveedor_snapshot IS NULL OR json_valid(proveedor_snapshot)),
    tipo_documento TEXT CHECK(tipo_documento IN ('FACTURA','BOLETA','OTRO','SIN_DOCUMENTO')),
    numero_documento TEXT CHECK(numero_documento IS NULL OR length(numero_documento) BETWEEN 1 AND 40),
    fecha_documento TEXT CHECK (fecha_documento IS NULL OR (length(fecha_documento)=10 AND date(fecha_documento, '+0 days') IS fecha_documento)),
    referencia_inicial TEXT UNIQUE CHECK(referencia_inicial IS NULL OR length(trim(referencia_inicial)) BETWEEN 3 AND 40),
    estado TEXT NOT NULL DEFAULT 'PREPARANDO' CHECK(estado IN ('PREPARANDO','REGISTRADO','ANULADO')),
    total_costo_micro INTEGER NOT NULL CHECK(total_costo_micro BETWEEN 0 AND 9000000000000000),
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    observacion TEXT CHECK(observacion IS NULL OR length(observacion)<=500),
    fecha_registro TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_registro IS NULL OR (length(fecha_registro)=19 AND datetime(fecha_registro, '+0 seconds') IS fecha_registro)),
    fecha_anulacion TEXT CHECK (fecha_anulacion IS NULL OR (length(fecha_anulacion)=19 AND datetime(fecha_anulacion, '+0 seconds') IS fecha_anulacion)),
    id_usuario_anulacion INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    motivo_anulacion TEXT,
    UNIQUE(id_proveedor,tipo_documento,numero_documento),
    CHECK((origen='COMPRA' AND id_proveedor IS NOT NULL AND proveedor_snapshot IS NOT NULL AND tipo_documento IS NOT NULL AND numero_documento IS NOT NULL AND fecha_documento IS NOT NULL) OR (origen='INICIAL' AND id_proveedor IS NULL AND tipo_documento IS NULL AND referencia_inicial IS NOT NULL)),
    CHECK(estado<>'ANULADO' OR (fecha_anulacion IS NOT NULL AND id_usuario_anulacion IS NOT NULL AND (motivo_anulacion IS NOT NULL AND length(trim(motivo_anulacion)) BETWEEN 5 AND 250)))
) STRICT;

CREATE TABLE detalle_ingreso (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_ingreso INTEGER NOT NULL REFERENCES ingresos_stock(id_ingreso) ON DELETE RESTRICT,
    id_producto INTEGER NOT NULL REFERENCES productos(id_producto) ON DELETE RESTRICT,
    codigo_snapshot TEXT NOT NULL,
    nombre_snapshot TEXT NOT NULL,
    unidad_medida TEXT NOT NULL CHECK(unidad_medida IN ('UND','ML')),
    cantidad INTEGER NOT NULL CHECK(cantidad BETWEEN 1 AND 9999999999),
    costo_efectivo_micro INTEGER NOT NULL CHECK(costo_efectivo_micro BETWEEN 1 AND 999999999999),
    valor_aporte_micro INTEGER NOT NULL CHECK(valor_aporte_micro BETWEEN 0 AND 9000000000000000),
    UNIQUE(id_ingreso,id_producto)
) STRICT;

CREATE TABLE ordenes_trabajo (
    id_ot INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    codigo_ot TEXT NOT NULL UNIQUE,
    id_vehiculo INTEGER NOT NULL REFERENCES vehiculos(id_vehiculo) ON DELETE RESTRICT,
    id_cliente_atendido INTEGER NOT NULL REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    placa_snapshot TEXT NOT NULL,
    cliente_snapshot TEXT NOT NULL CHECK(json_valid(cliente_snapshot)),
    id_creador INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_mecanico INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_mecanico_terminal INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_zanja_ejecucion INTEGER REFERENCES zanjas(id_zanja) ON DELETE RESTRICT,
    estado TEXT NOT NULL DEFAULT 'EN_ESPERA' CHECK(estado IN ('EN_ESPERA','EN_PROCESO','FINALIZADO','CANCELADO')),
    kilometraje_ingreso INTEGER NOT NULL CHECK(kilometraje_ingreso BETWEEN 0 AND 9999999),
    kilometraje_salida INTEGER CHECK(kilometraje_salida BETWEEN 0 AND 9999999),
    pedido_cliente TEXT CHECK(pedido_cliente IS NULL OR length(pedido_cliente)<=500),
    resumen_publico TEXT CHECK(resumen_publico IS NULL OR length(trim(resumen_publico)) BETWEEN 1 AND 1000),
    nota_interna TEXT CHECK(nota_interna IS NULL OR length(nota_interna)<=1000),
    intervalo_aceite_km INTEGER CHECK(intervalo_aceite_km IN (5000,10000)),
    km_proximo INTEGER GENERATED ALWAYS AS (CASE WHEN intervalo_aceite_km IS NOT NULL THEN kilometraje_salida+intervalo_aceite_km END) STORED,
    corregido INTEGER NOT NULL DEFAULT 0 CHECK(corregido IN (0,1)),
    tipo_aceite TEXT CHECK(tipo_aceite IN ('MINERAL','SINTETICO')),
    fecha_ingreso TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_ingreso IS NULL OR (length(fecha_ingreso)=19 AND datetime(fecha_ingreso, '+0 seconds') IS fecha_ingreso)),
    fecha_inicio TEXT CHECK (fecha_inicio IS NULL OR (length(fecha_inicio)=19 AND datetime(fecha_inicio, '+0 seconds') IS fecha_inicio)),
    fecha_fin TEXT CHECK (fecha_fin IS NULL OR (length(fecha_fin)=19 AND datetime(fecha_fin, '+0 seconds') IS fecha_fin)),
    fecha_cancelacion TEXT CHECK (fecha_cancelacion IS NULL OR (length(fecha_cancelacion)=19 AND datetime(fecha_cancelacion, '+0 seconds') IS fecha_cancelacion)),
    motivo_cancelacion TEXT,
    id_usuario_cancelacion INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    CHECK((tipo_aceite IS NULL AND intervalo_aceite_km IS NULL) OR (tipo_aceite IS NOT NULL AND tipo_aceite='MINERAL' AND intervalo_aceite_km IS NOT NULL AND intervalo_aceite_km=5000) OR (tipo_aceite IS NOT NULL AND tipo_aceite='SINTETICO' AND intervalo_aceite_km IS NOT NULL AND intervalo_aceite_km=10000)),
    CHECK(fecha_inicio IS NULL OR fecha_inicio>=fecha_ingreso),
    CHECK(fecha_fin IS NULL OR fecha_fin>=fecha_inicio),
    CHECK(fecha_cancelacion IS NULL OR fecha_cancelacion>=fecha_ingreso),
    CHECK(estado<>'EN_PROCESO' OR (id_mecanico IS NOT NULL AND id_zanja_ejecucion IS NOT NULL AND fecha_inicio IS NOT NULL)),
    CHECK(estado<>'FINALIZADO' OR (fecha_inicio IS NOT NULL AND fecha_fin IS NOT NULL AND kilometraje_salida IS NOT NULL AND id_mecanico_terminal IS NOT NULL AND id_zanja_ejecucion IS NOT NULL AND resumen_publico IS NOT NULL AND length(trim(resumen_publico)) BETWEEN 1 AND 1000)),
    CHECK(estado<>'CANCELADO' OR (fecha_cancelacion IS NOT NULL AND id_usuario_cancelacion IS NOT NULL AND (motivo_cancelacion IS NOT NULL AND length(trim(motivo_cancelacion)) BETWEEN 5 AND 250)))
) STRICT;

CREATE UNIQUE INDEX uq_ot_abierta_vehiculo ON ordenes_trabajo(id_vehiculo) WHERE estado IN ('EN_ESPERA','EN_PROCESO');

CREATE UNIQUE INDEX uq_zanja_en_proceso ON ordenes_trabajo(id_zanja_ejecucion) WHERE estado='EN_PROCESO';

CREATE TABLE detalle_ot_servicios (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_ot INTEGER NOT NULL REFERENCES ordenes_trabajo(id_ot) ON DELETE RESTRICT,
    id_servicio INTEGER NOT NULL REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    codigo_snapshot TEXT NOT NULL,
    nombre_snapshot TEXT NOT NULL,
    cantidad INTEGER NOT NULL CHECK(cantidad BETWEEN 1 AND 99),
    precio_unitario_cent INTEGER NOT NULL CHECK(precio_unitario_cent BETWEEN 1 AND 99999999),
    es_cambio_aceite INTEGER NOT NULL CHECK(es_cambio_aceite IN (0,1)),
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    fecha_registro TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_registro IS NULL OR (length(fecha_registro)=19 AND datetime(fecha_registro, '+0 seconds') IS fecha_registro)),
    UNIQUE(id_ot,id_servicio)
) STRICT;

CREATE TABLE detalle_ot_productos (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_ot INTEGER NOT NULL REFERENCES ordenes_trabajo(id_ot) ON DELETE RESTRICT,
    id_producto INTEGER NOT NULL REFERENCES productos(id_producto) ON DELETE RESTRICT,
    codigo_snapshot TEXT NOT NULL,
    nombre_snapshot TEXT NOT NULL,
    unidad_medida TEXT NOT NULL CHECK(unidad_medida IN ('UND','ML')),
    cantidad INTEGER NOT NULL CHECK(cantidad BETWEEN 1 AND 9999999999),
    precio_unitario_cent INTEGER NOT NULL CHECK(precio_unitario_cent BETWEEN 1 AND 99999999),
    costo_unitario_micro INTEGER NOT NULL CHECK(costo_unitario_micro BETWEEN 1 AND 999999999999),
    valor_consumido_micro INTEGER NOT NULL CHECK(valor_consumido_micro BETWEEN 0 AND 9000000000000000),
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    fecha_consumo TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_consumo IS NULL OR (length(fecha_consumo)=19 AND datetime(fecha_consumo, '+0 seconds') IS fecha_consumo))
) STRICT;

CREATE TABLE caja_chica (
    id_caja INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    codigo_caja TEXT NOT NULL UNIQUE,
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    ambiente TEXT NOT NULL CHECK(ambiente IN ('PRUEBA','PRODUCCION')),
    monto_apertura_cent INTEGER NOT NULL CHECK(monto_apertura_cent BETWEEN 0 AND 1000000),
    estado TEXT NOT NULL DEFAULT 'ABIERTA' CHECK(estado IN ('ABIERTA','CERRADA')),
    por_conciliar INTEGER NOT NULL DEFAULT 0 CHECK(por_conciliar IN (0,1)),
    fecha_apertura TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_apertura IS NULL OR (length(fecha_apertura)=19 AND datetime(fecha_apertura, '+0 seconds') IS fecha_apertura)),
    fecha_cierre TEXT CHECK (fecha_cierre IS NULL OR (length(fecha_cierre)=19 AND datetime(fecha_cierre, '+0 seconds') IS fecha_cierre)),
    id_operacion_cierre TEXT UNIQUE,
    id_usuario_cierre INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    tipo_cierre TEXT CHECK(tipo_cierre IN ('NORMAL','AUSENCIA','RECUPERACION')),
    motivo_cierre TEXT CHECK(motivo_cierre IS NULL OR length(trim(motivo_cierre)) BETWEEN 5 AND 250),
    efectivo_esperado_cent INTEGER CHECK(efectivo_esperado_cent BETWEEN 0 AND 9999999999),
    efectivo_contado_cent INTEGER CHECK(efectivo_contado_cent BETWEEN 0 AND 9999999999),
    diferencia_efectivo_cent INTEGER GENERATED ALWAYS AS (efectivo_contado_cent-efectivo_esperado_cent) STORED,
    tarjeta_neto_cent INTEGER CHECK(tarjeta_neto_cent BETWEEN -9999999999 AND 9999999999),
    yape_neto_cent INTEGER CHECK(yape_neto_cent BETWEEN -9999999999 AND 9999999999),
    plin_neto_cent INTEGER CHECK(plin_neto_cent BETWEEN -9999999999 AND 9999999999),
    transferencia_neto_cent INTEGER CHECK(transferencia_neto_cent BETWEEN -9999999999 AND 9999999999),
    aclaracion TEXT CHECK(aclaracion IS NULL OR length(aclaracion)<=500),
    id_usuario_aclaracion INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    fecha_aclaracion TEXT CHECK(fecha_aclaracion IS NULL OR (length(fecha_aclaracion)=19 AND datetime(fecha_aclaracion,'+0 seconds') IS fecha_aclaracion)),
    fondo_siguiente_cent INTEGER CHECK(fondo_siguiente_cent BETWEEN 0 AND 1000000),
    retiro_cierre_cent INTEGER CHECK(retiro_cierre_cent>=0),
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    CHECK(estado<>'CERRADA' OR (fecha_cierre IS NOT NULL AND fecha_cierre>=fecha_apertura AND id_operacion_cierre IS NOT NULL AND id_usuario_cierre IS NOT NULL AND tipo_cierre IS NOT NULL AND efectivo_esperado_cent IS NOT NULL AND efectivo_contado_cent IS NOT NULL AND tarjeta_neto_cent IS NOT NULL AND yape_neto_cent IS NOT NULL AND plin_neto_cent IS NOT NULL AND transferencia_neto_cent IS NOT NULL AND fondo_siguiente_cent IS NOT NULL AND retiro_cierre_cent IS NOT NULL)),
    CHECK(tipo_cierre IS NULL OR tipo_cierre='NORMAL' OR (motivo_cierre IS NOT NULL AND length(trim(motivo_cierre)) BETWEEN 5 AND 250))
) STRICT;

CREATE UNIQUE INDEX uq_caja_abierta_instalacion ON caja_chica(estado) WHERE estado='ABIERTA';

CREATE TABLE ventas (
    id_venta INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_ot INTEGER UNIQUE REFERENCES ordenes_trabajo(id_ot) ON DELETE RESTRICT,
    id_cliente INTEGER NOT NULL REFERENCES clientes(id_cliente) ON DELETE RESTRICT,
    receptor_snapshot TEXT NOT NULL CHECK(json_valid(receptor_snapshot)),
    id_emisor INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_autorizante INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_caja INTEGER NOT NULL REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    ambiente TEXT NOT NULL CHECK(ambiente IN ('PRUEBA','PRODUCCION')),
    tipo_original TEXT NOT NULL CHECK(tipo_original IN ('BOLETA','FACTURA')),
    estado_registro TEXT NOT NULL DEFAULT 'PREPARANDO' CHECK(estado_registro IN ('PREPARANDO','CONFIRMADA')),
    descuento_bp INTEGER NOT NULL DEFAULT 0 CHECK(descuento_bp BETWEEN 0 AND 9999),
    motivo_descuento TEXT CHECK(motivo_descuento IS NULL OR length(trim(motivo_descuento)) BETWEEN 5 AND 250),
    bruto_cent INTEGER NOT NULL CHECK(bruto_cent BETWEEN 1 AND 9999999999),
    descuento_cent INTEGER NOT NULL CHECK(descuento_cent>=0 AND descuento_cent<bruto_cent),
    base_cent INTEGER NOT NULL CHECK(base_cent>=0),
    igv_cent INTEGER NOT NULL CHECK(igv_cent>=0),
    total_cent INTEGER NOT NULL CHECK(total_cent=bruto_cent-descuento_cent AND total_cent=base_cent+igv_cent),
    fecha_venta TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_venta IS NULL OR (length(fecha_venta)=19 AND datetime(fecha_venta, '+0 seconds') IS fecha_venta))
) STRICT;

CREATE TABLE detalle_venta (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_venta INTEGER NOT NULL REFERENCES ventas(id_venta) ON DELETE RESTRICT,
    id_producto INTEGER REFERENCES productos(id_producto) ON DELETE RESTRICT,
    id_servicio INTEGER REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    id_consumo_ot INTEGER REFERENCES detalle_ot_productos(id_detalle) ON DELETE RESTRICT,
    id_servicio_ot INTEGER REFERENCES detalle_ot_servicios(id_detalle) ON DELETE RESTRICT,
    codigo_snapshot TEXT NOT NULL,
    nombre_snapshot TEXT NOT NULL,
    unidad_medida TEXT NOT NULL CHECK(unidad_medida IN ('UND','ML')),
    cantidad INTEGER NOT NULL CHECK(cantidad BETWEEN 1 AND 9999999999),
    precio_unitario_cent INTEGER NOT NULL CHECK(precio_unitario_cent BETWEEN 1 AND 99999999),
    bruto_cent INTEGER NOT NULL CHECK(bruto_cent>=0),
    descuento_cent INTEGER NOT NULL CHECK(descuento_cent BETWEEN 0 AND bruto_cent),
    base_cent INTEGER NOT NULL CHECK(base_cent>=0),
    igv_cent INTEGER NOT NULL CHECK(igv_cent>=0),
    igv_bp INTEGER NOT NULL CHECK(igv_bp=1800),
    total_cent INTEGER NOT NULL CHECK(total_cent>0 AND total_cent=bruto_cent-descuento_cent AND total_cent=base_cent+igv_cent),
    costo_unitario_micro INTEGER CHECK(costo_unitario_micro BETWEEN 1 AND 999999999999),
    costo_total_micro INTEGER CHECK(costo_total_micro BETWEEN 0 AND 9000000000000000),
    CHECK((id_producto IS NOT NULL AND id_servicio IS NULL AND costo_unitario_micro IS NOT NULL AND costo_total_micro IS NOT NULL) OR (id_producto IS NULL AND id_servicio IS NOT NULL AND unidad_medida='UND' AND costo_unitario_micro IS NULL AND costo_total_micro IS NULL)),
    CHECK(id_consumo_ot IS NULL OR (id_producto IS NOT NULL AND id_servicio_ot IS NULL)),
    CHECK(unidad_medida='ML' OR cantidad<=9999999),
    CHECK(id_servicio_ot IS NULL OR (id_servicio IS NOT NULL AND id_consumo_ot IS NULL))
) STRICT;

CREATE UNIQUE INDEX uq_linea_venta_consumo ON detalle_venta(id_consumo_ot) WHERE id_consumo_ot IS NOT NULL;

CREATE UNIQUE INDEX uq_linea_venta_servicio_ot ON detalle_venta(id_servicio_ot) WHERE id_servicio_ot IS NOT NULL;

CREATE TABLE devoluciones (
    id_devolucion INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_venta INTEGER NOT NULL REFERENCES ventas(id_venta) ON DELETE RESTRICT,
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_autorizante INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    id_caja INTEGER NOT NULL REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    id_documento_original INTEGER NOT NULL REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT,
    motivo TEXT NOT NULL CHECK(length(trim(motivo)) BETWEEN 5 AND 250),
    estado_registro TEXT NOT NULL DEFAULT 'PREPARANDO' CHECK(estado_registro IN ('PREPARANDO','CONFIRMADA')),
    base_cent INTEGER NOT NULL CHECK(base_cent>=0),
    igv_cent INTEGER NOT NULL CHECK(igv_cent>=0),
    total_cent INTEGER NOT NULL CHECK(total_cent BETWEEN 1 AND 9999999999 AND total_cent=base_cent+igv_cent),
    fecha_operacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_operacion IS NULL OR (length(fecha_operacion)=19 AND datetime(fecha_operacion, '+0 seconds') IS fecha_operacion))
) STRICT;

CREATE TABLE detalle_devolucion (
    id_detalle INTEGER PRIMARY KEY AUTOINCREMENT,
    id_devolucion INTEGER NOT NULL REFERENCES devoluciones(id_devolucion) ON DELETE RESTRICT,
    id_detalle_venta INTEGER NOT NULL REFERENCES detalle_venta(id_detalle) ON DELETE RESTRICT,
    cantidad INTEGER NOT NULL CHECK(cantidad BETWEEN 1 AND 9999999999),
    cantidad_recuperada INTEGER NOT NULL DEFAULT 0 CHECK(cantidad_recuperada BETWEEN 0 AND cantidad),
    base_cent INTEGER NOT NULL CHECK(base_cent>=0),
    igv_cent INTEGER NOT NULL CHECK(igv_cent>=0),
    total_cent INTEGER NOT NULL CHECK(total_cent=base_cent+igv_cent),
    costo_revertido_micro INTEGER NOT NULL DEFAULT 0 CHECK(costo_revertido_micro BETWEEN 0 AND 9000000000000000),
    UNIQUE(id_devolucion,id_detalle_venta),
    CHECK(cantidad_recuperada>0 OR costo_revertido_micro=0)
) STRICT;

CREATE TABLE movimientos_caja (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_caja INTEGER NOT NULL REFERENCES caja_chica(id_caja) ON DELETE RESTRICT,
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    tipo TEXT NOT NULL CHECK(tipo IN ('COBRO','REINTEGRO','INGRESO_MANUAL','EGRESO_MANUAL','REVERSO_MANUAL')),
    medio_pago TEXT NOT NULL CHECK(medio_pago IN ('EFECTIVO','TARJETA','YAPE','PLIN','TRANSFERENCIA')),
    monto_cent INTEGER NOT NULL CHECK(monto_cent BETWEEN 1 AND 9999999999),
    signo INTEGER NOT NULL CHECK(signo IN (-1,1)),
    id_venta INTEGER REFERENCES ventas(id_venta) ON DELETE RESTRICT,
    id_devolucion INTEGER REFERENCES devoluciones(id_devolucion) ON DELETE RESTRICT,
    id_movimiento_original INTEGER UNIQUE REFERENCES movimientos_caja(id_movimiento) ON DELETE RESTRICT,
    recibido_cent INTEGER CHECK(recibido_cent>=0),
    vuelto_cent INTEGER CHECK(vuelto_cent>=0),
    concepto TEXT NOT NULL CHECK(length(trim(concepto)) BETWEEN 3 AND 120),
    referencia TEXT CHECK(referencia IS NULL OR length(referencia)<=100),
    categoria TEXT,
    motivo TEXT,
    fecha_operacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_operacion IS NULL OR (length(fecha_operacion)=19 AND datetime(fecha_operacion, '+0 seconds') IS fecha_operacion)),
    CHECK((tipo='COBRO' AND signo=1 AND id_venta IS NOT NULL AND id_devolucion IS NULL AND id_movimiento_original IS NULL) OR (tipo='REINTEGRO' AND signo=-1 AND id_venta IS NULL AND id_devolucion IS NOT NULL AND id_movimiento_original IS NULL) OR (tipo IN ('INGRESO_MANUAL','EGRESO_MANUAL') AND id_venta IS NULL AND id_devolucion IS NULL AND id_movimiento_original IS NULL AND medio_pago='EFECTIVO' AND monto_cent<=1000000 AND signo=CASE tipo WHEN 'INGRESO_MANUAL' THEN 1 ELSE -1 END) OR (tipo='REVERSO_MANUAL' AND medio_pago='EFECTIVO' AND id_venta IS NULL AND id_devolucion IS NULL AND id_movimiento_original IS NOT NULL AND motivo IS NOT NULL AND length(trim(motivo)) BETWEEN 5 AND 250)),
    CHECK((tipo='COBRO' AND medio_pago='EFECTIVO' AND recibido_cent IS NOT NULL AND vuelto_cent IS NOT NULL AND recibido_cent-vuelto_cent=monto_cent) OR ((tipo<>'COBRO' OR medio_pago<>'EFECTIVO') AND recibido_cent IS NULL AND vuelto_cent IS NULL))
) STRICT;

CREATE UNIQUE INDEX uq_cobro_medio ON movimientos_caja(id_venta,medio_pago) WHERE tipo='COBRO';

CREATE UNIQUE INDEX uq_reintegro_medio ON movimientos_caja(id_devolucion,medio_pago) WHERE tipo='REINTEGRO';

CREATE TABLE series_comprobante (
    id_serie INTEGER PRIMARY KEY AUTOINCREMENT,
    ambiente TEXT NOT NULL CHECK(ambiente IN ('PRUEBA','PRODUCCION')),
    tipo_comprobante TEXT NOT NULL CHECK(tipo_comprobante IN ('BOLETA','FACTURA','NC_BOLETA','NC_FACTURA')),
    serie TEXT NOT NULL CHECK(length(serie)=4 AND serie=upper(serie) AND serie NOT GLOB '*[^A-Z0-9]*'),
    correlativo_actual INTEGER NOT NULL DEFAULT 0 CHECK(correlativo_actual BETWEEN 0 AND 99999999),
    mayor_previo INTEGER NOT NULL DEFAULT 0 CHECK(mayor_previo BETWEEN 0 AND 99999999),
    ultimo_real_comprobado INTEGER NOT NULL DEFAULT 0 CHECK(ultimo_real_comprobado BETWEEN 0 AND 99999999),
    referencia_verificacion TEXT CHECK(referencia_verificacion IS NULL OR length(referencia_verificacion) BETWEEN 1 AND 250),
    fecha_verificacion TEXT CHECK(fecha_verificacion IS NULL OR (length(fecha_verificacion)=19 AND datetime(fecha_verificacion,'+0 seconds') IS fecha_verificacion)),
    activa INTEGER NOT NULL DEFAULT 0 CHECK(activa IN (0,1)),
    requiere_conciliacion INTEGER NOT NULL DEFAULT 0 CHECK(requiere_conciliacion IN (0,1)),
    UNIQUE(ambiente,tipo_comprobante,serie),
    UNIQUE(id_serie,ambiente,tipo_comprobante),
    CHECK((tipo_comprobante IN ('BOLETA','NC_BOLETA') AND substr(serie,1,1)='B') OR (tipo_comprobante IN ('FACTURA','NC_FACTURA') AND substr(serie,1,1)='F'))
) STRICT;

CREATE UNIQUE INDEX uq_serie_activa ON series_comprobante(ambiente,tipo_comprobante) WHERE activa=1;

CREATE TABLE comprobantes (
    id_comprobante INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_venta INTEGER NOT NULL REFERENCES ventas(id_venta) ON DELETE RESTRICT,
    id_devolucion INTEGER REFERENCES devoluciones(id_devolucion) ON DELETE RESTRICT,
    id_documento_anterior INTEGER REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT,
    id_documento_referenciado INTEGER REFERENCES comprobantes(id_comprobante) ON DELETE RESTRICT,
    relacion TEXT NOT NULL CHECK(relacion IN ('ORIGINAL','REEMISION','NOTA_CREDITO','REEMISION_NC')),
    tipo_comprobante TEXT NOT NULL CHECK(tipo_comprobante IN ('BOLETA','FACTURA','NC_BOLETA','NC_FACTURA')),
    ambiente TEXT NOT NULL CHECK(ambiente IN ('PRUEBA','PRODUCCION')),
    id_serie INTEGER NOT NULL,
    correlativo INTEGER NOT NULL CHECK(correlativo BETWEEN 1 AND 99999999),
    empresa_snapshot TEXT NOT NULL CHECK(json_valid(empresa_snapshot)),
    receptor_snapshot TEXT NOT NULL CHECK(json_valid(receptor_snapshot)),
    payload_json TEXT NOT NULL CHECK(json_valid(payload_json)),
    payload_sha256 TEXT NOT NULL CHECK(length(payload_sha256)=64 AND payload_sha256 NOT GLOB '*[^0-9a-f]*'),
    base_cent INTEGER NOT NULL CHECK(base_cent>=0),
    igv_cent INTEGER NOT NULL CHECK(igv_cent>=0),
    total_cent INTEGER NOT NULL CHECK(total_cent=base_cent+igv_cent),
    fecha_emision TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_emision IS NULL OR (length(fecha_emision)=19 AND datetime(fecha_emision, '+0 seconds') IS fecha_emision)),
    estado_envio TEXT NOT NULL DEFAULT 'PENDIENTE' CHECK(estado_envio IN ('PENDIENTE','ENVIANDO','ERROR','ACEPTADO','RECHAZADO')),
    intento_iniciado INTEGER NOT NULL DEFAULT 0 CHECK(intento_iniciado IN (0,1)),
    requiere_consulta INTEGER NOT NULL DEFAULT 0 CHECK(requiere_consulta IN (0,1)),
    resultado_incierto INTEGER NOT NULL DEFAULT 0 CHECK(resultado_incierto IN (0,1)),
    fecha_ultimo_intento TEXT CHECK(fecha_ultimo_intento IS NULL OR (length(fecha_ultimo_intento)=19 AND datetime(fecha_ultimo_intento,'+0 seconds') IS fecha_ultimo_intento)),
    fecha_ultima_consulta TEXT CHECK(fecha_ultima_consulta IS NULL OR (length(fecha_ultima_consulta)=19 AND datetime(fecha_ultima_consulta,'+0 seconds') IS fecha_ultima_consulta)),
    fecha_confirmacion TEXT CHECK(fecha_confirmacion IS NULL OR (length(fecha_confirmacion)=19 AND datetime(fecha_confirmacion,'+0 seconds') IS fecha_confirmacion)),
    codigo_respuesta TEXT,
    respuesta_resumida TEXT CHECK(respuesta_resumida IS NULL OR length(respuesta_resumida)<=500),
    advertencias_json TEXT CHECK(advertencias_json IS NULL OR json_valid(advertencias_json)),
    ruta_pdf TEXT,
    ruta_xml TEXT,
    ruta_cdr TEXT,
    version INTEGER NOT NULL DEFAULT 0 CHECK(version>=0),
    FOREIGN KEY(id_serie,ambiente,tipo_comprobante) REFERENCES series_comprobante(id_serie,ambiente,tipo_comprobante) ON DELETE RESTRICT,
    UNIQUE(id_serie,correlativo),
    CHECK(estado_envio NOT IN ('ACEPTADO','RECHAZADO') OR (resultado_incierto=0 AND requiere_consulta=0 AND intento_iniciado=0 AND fecha_confirmacion IS NOT NULL)),
    CHECK(estado_envio<>'ENVIANDO' OR (intento_iniciado=1 AND requiere_consulta=1 AND fecha_ultimo_intento IS NOT NULL)),
    CHECK(resultado_incierto=0 OR requiere_consulta=1),
    CHECK((relacion IN ('ORIGINAL','REEMISION') AND tipo_comprobante IN ('BOLETA','FACTURA') AND id_devolucion IS NULL) OR (relacion IN ('NOTA_CREDITO','REEMISION_NC') AND tipo_comprobante IN ('NC_BOLETA','NC_FACTURA') AND id_devolucion IS NOT NULL AND id_documento_referenciado IS NOT NULL)),
    CHECK((relacion IN ('REEMISION','REEMISION_NC') AND id_documento_anterior IS NOT NULL) OR (relacion NOT IN ('REEMISION','REEMISION_NC') AND id_documento_anterior IS NULL))
) STRICT;

CREATE UNIQUE INDEX uq_documento_original ON comprobantes(id_venta) WHERE relacion='ORIGINAL';

CREATE UNIQUE INDEX uq_reemision_por_documento ON comprobantes(id_documento_anterior) WHERE relacion IN ('REEMISION','REEMISION_NC');

CREATE UNIQUE INDEX uq_correctivo_devolucion ON comprobantes(id_devolucion) WHERE relacion='NOTA_CREDITO';

CREATE TABLE movimientos_inventario (
    id_movimiento INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_producto INTEGER NOT NULL REFERENCES productos(id_producto) ON DELETE RESTRICT,
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    tipo_movimiento TEXT NOT NULL CHECK(tipo_movimiento IN ('STOCK_INICIAL','COMPRA','ANULACION_INGRESO','CONSUMO_OT','REPOSICION_OT','VENTA_DIRECTA','DEVOLUCION_VENTA','AJUSTE_ENTRADA','AJUSTE_SALIDA')),
    id_detalle_ingreso INTEGER REFERENCES detalle_ingreso(id_detalle) ON DELETE RESTRICT,
    id_consumo_ot INTEGER REFERENCES detalle_ot_productos(id_detalle) ON DELETE RESTRICT,
    id_detalle_venta INTEGER REFERENCES detalle_venta(id_detalle) ON DELETE RESTRICT,
    id_detalle_devolucion INTEGER REFERENCES detalle_devolucion(id_detalle) ON DELETE RESTRICT,
    unidad_medida TEXT NOT NULL CHECK(unidad_medida IN ('UND','ML')),
    stock_anterior INTEGER NOT NULL CHECK(stock_anterior BETWEEN 0 AND 9999999999),
    diferencia INTEGER NOT NULL CHECK(diferencia<>0 AND diferencia BETWEEN -9999999999 AND 9999999999),
    stock_resultante INTEGER NOT NULL CHECK(stock_resultante BETWEEN 0 AND 9999999999 AND stock_resultante=stock_anterior+diferencia),
    valor_anterior_micro INTEGER NOT NULL CHECK(valor_anterior_micro BETWEEN 0 AND 9000000000000000),
    valor_delta_micro INTEGER NOT NULL CHECK(valor_delta_micro BETWEEN -9000000000000000 AND 9000000000000000),
    valor_resultante_micro INTEGER NOT NULL CHECK(valor_resultante_micro BETWEEN 0 AND 9000000000000000 AND valor_resultante_micro=valor_anterior_micro+valor_delta_micro),
    costo_unitario_micro INTEGER NOT NULL CHECK(costo_unitario_micro BETWEEN 1 AND 999999999999),
    motivo TEXT,
    fecha_movimiento TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_movimiento IS NULL OR (length(fecha_movimiento)=19 AND datetime(fecha_movimiento, '+0 seconds') IS fecha_movimiento)),
    CHECK((stock_resultante=0 AND valor_resultante_micro=0) OR (stock_resultante>0 AND valor_resultante_micro>0)),
    CHECK(unidad_medida='ML' OR (stock_anterior<=9999999 AND stock_resultante<=9999999)),
    CHECK((diferencia>0 AND valor_delta_micro>=0) OR (diferencia<0 AND valor_delta_micro<=0)),
    CHECK((tipo_movimiento IN ('STOCK_INICIAL','COMPRA','ANULACION_INGRESO') AND id_detalle_ingreso IS NOT NULL AND id_consumo_ot IS NULL AND id_detalle_venta IS NULL AND id_detalle_devolucion IS NULL) OR (tipo_movimiento IN ('CONSUMO_OT','REPOSICION_OT') AND id_consumo_ot IS NOT NULL AND id_detalle_ingreso IS NULL AND id_detalle_venta IS NULL AND id_detalle_devolucion IS NULL) OR (tipo_movimiento='VENTA_DIRECTA' AND id_detalle_venta IS NOT NULL AND id_consumo_ot IS NULL AND id_detalle_ingreso IS NULL AND id_detalle_devolucion IS NULL) OR (tipo_movimiento='DEVOLUCION_VENTA' AND id_detalle_devolucion IS NOT NULL AND id_consumo_ot IS NULL AND id_detalle_ingreso IS NULL AND id_detalle_venta IS NULL) OR (tipo_movimiento IN ('AJUSTE_ENTRADA','AJUSTE_SALIDA') AND id_detalle_ingreso IS NULL AND id_consumo_ot IS NULL AND id_detalle_venta IS NULL AND id_detalle_devolucion IS NULL AND length(trim(motivo)) BETWEEN 5 AND 250)),
    CHECK((tipo_movimiento IN ('STOCK_INICIAL','COMPRA','REPOSICION_OT','DEVOLUCION_VENTA') AND diferencia>0) OR (tipo_movimiento IN ('ANULACION_INGRESO','CONSUMO_OT','VENTA_DIRECTA') AND diferencia<0) OR (tipo_movimiento='AJUSTE_ENTRADA' AND diferencia>0) OR (tipo_movimiento='AJUSTE_SALIDA' AND diferencia<0))
) STRICT;

CREATE UNIQUE INDEX uq_kardex_id_detalle_ingreso ON movimientos_inventario(id_detalle_ingreso,tipo_movimiento) WHERE tipo_movimiento IN ('STOCK_INICIAL','COMPRA','ANULACION_INGRESO');

CREATE UNIQUE INDEX uq_kardex_id_consumo_ot ON movimientos_inventario(id_consumo_ot,tipo_movimiento) WHERE tipo_movimiento IN ('CONSUMO_OT');

CREATE UNIQUE INDEX uq_kardex_id_detalle_venta ON movimientos_inventario(id_detalle_venta,tipo_movimiento) WHERE tipo_movimiento IN ('VENTA_DIRECTA');

CREATE UNIQUE INDEX uq_kardex_id_detalle_devolucion ON movimientos_inventario(id_detalle_devolucion,tipo_movimiento) WHERE tipo_movimiento IN ('DEVOLUCION_VENTA');

CREATE TABLE backups (
    id_backup INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    nombre_archivo TEXT NOT NULL,
    ruta_archivo TEXT NOT NULL UNIQUE,
    tipo TEXT NOT NULL CHECK(tipo IN ('AUTOMATICO','MANUAL','IMPORTADO','PREVIA')),
    estado_local TEXT NOT NULL CHECK(estado_local IN ('EN_CURSO','VALIDO','FALLIDO','ELIMINADO','DANADO','INCOMPATIBLE')),
    estado_externo TEXT NOT NULL DEFAULT 'NO_SOLICITADO' CHECK(estado_externo IN ('NO_SOLICITADO','COPIADO','FALLIDO')),
    sha256 TEXT CHECK(sha256 IS NULL OR length(sha256)=64),
    tamano_bytes INTEGER CHECK(tamano_bytes>=0),
    ambiente_origen TEXT CHECK(ambiente_origen IN ('PRUEBA','PRODUCCION')),
    ruta_externa TEXT,
    error_externo TEXT,
    ruc_origen TEXT,
    version_aplicacion TEXT,
    version_esquema INTEGER,
    resumen_json TEXT CHECK(resumen_json IS NULL OR json_valid(resumen_json)),
    id_usuario INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    fecha_creacion TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_creacion IS NULL OR (length(fecha_creacion)=19 AND datetime(fecha_creacion, '+0 seconds') IS fecha_creacion)),
    fecha_datos TEXT CHECK (fecha_datos IS NULL OR (length(fecha_datos)=19 AND datetime(fecha_datos, '+0 seconds') IS fecha_datos)),
    ultima_verificacion TEXT CHECK (ultima_verificacion IS NULL OR (length(ultima_verificacion)=19 AND datetime(ultima_verificacion, '+0 seconds') IS ultima_verificacion)),
    ultima_copia_externa TEXT CHECK (ultima_copia_externa IS NULL OR (length(ultima_copia_externa)=19 AND datetime(ultima_copia_externa, '+0 seconds') IS ultima_copia_externa)),
    fecha_eliminacion TEXT CHECK (fecha_eliminacion IS NULL OR (length(fecha_eliminacion)=19 AND datetime(fecha_eliminacion, '+0 seconds') IS fecha_eliminacion)),
    ultimo_error TEXT,
    CHECK(estado_local<>'VALIDO' OR (sha256 IS NOT NULL AND tamano_bytes>0 AND ultima_verificacion IS NOT NULL AND version_esquema IS NOT NULL))
) STRICT;

CREATE TABLE restauraciones (
    id_restauracion INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_backup INTEGER REFERENCES backups(id_backup) ON DELETE RESTRICT,
    id_backup_previa INTEGER REFERENCES backups(id_backup) ON DELETE RESTRICT,
    evento_externo_id TEXT NOT NULL UNIQUE,
    actor_snapshot TEXT NOT NULL CHECK(length(trim(actor_snapshot))>0),
    nombre_copia_snapshot TEXT NOT NULL,
    ambiente TEXT NOT NULL CHECK(ambiente IN ('PRUEBA','PRODUCCION')),
    sha256_copia TEXT NOT NULL CHECK(length(sha256_copia)=64),
    id_usuario INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    estado TEXT NOT NULL CHECK(estado IN ('CONTROLES_PENDIENTES','COMPLETA')),
    fecha_inicio TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_inicio IS NULL OR (length(fecha_inicio)=19 AND datetime(fecha_inicio, '+0 seconds') IS fecha_inicio)),
    fecha_fin TEXT CHECK (fecha_fin IS NULL OR (length(fecha_fin)=19 AND datetime(fecha_fin, '+0 seconds') IS fecha_fin))
) STRICT;

CREATE TABLE conciliaciones_series (
    id_conciliacion INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT NOT NULL UNIQUE,
    id_restauracion INTEGER REFERENCES restauraciones(id_restauracion) ON DELETE RESTRICT,
    id_serie INTEGER NOT NULL REFERENCES series_comprobante(id_serie) ON DELETE RESTRICT,
    mayor_local INTEGER NOT NULL CHECK(mayor_local>=0),
    mayor_previo INTEGER NOT NULL CHECK(mayor_previo>=0),
    mayor_declarado_remoto INTEGER NOT NULL CHECK(mayor_declarado_remoto>=0),
    correlativo_conservado INTEGER NOT NULL CHECK(correlativo_conservado>=max(mayor_local,mayor_previo,mayor_declarado_remoto)),
    evidencia TEXT NOT NULL CHECK(length(trim(evidencia)) BETWEEN 10 AND 500),
    id_usuario INTEGER NOT NULL REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    fecha_registro TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_registro IS NULL OR (length(fecha_registro)=19 AND datetime(fecha_registro, '+0 seconds') IS fecha_registro))
) STRICT;

CREATE TABLE auditoria (
    id_auditoria INTEGER PRIMARY KEY AUTOINCREMENT,
    id_operacion TEXT,
    id_usuario INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    actor TEXT NOT NULL CHECK(actor IN ('USUARIO','SISTEMA','NO_IDENTIFICADO')),
    username_snapshot TEXT,
    rol_snapshot TEXT CHECK(rol_snapshot IN ('ADMINISTRADOR','MECANICO','CAJERO')),
    id_autorizante INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    modulo TEXT NOT NULL CHECK(modulo IN ('LOGIN','DASHBOARD','FICHA_VEHICULAR','OT','INVENTARIO','POS','CAJA','BACKUP','ADMIN','REPORTES')),
    accion TEXT NOT NULL,
    entidad TEXT,
    id_entidad TEXT,
    antes_json TEXT CHECK(antes_json IS NULL OR json_valid(antes_json)),
    despues_json TEXT CHECK(despues_json IS NULL OR json_valid(despues_json)),
    motivo TEXT,
    resultado TEXT NOT NULL CHECK(resultado IN ('OK','DENEGADO','ERROR')),
    fecha_evento TEXT NOT NULL DEFAULT (datetime('now', '-5 hours')) CHECK (fecha_evento IS NULL OR (length(fecha_evento)=19 AND datetime(fecha_evento, '+0 seconds') IS fecha_evento)),
    CHECK((actor='USUARIO' AND id_usuario IS NOT NULL AND username_snapshot IS NOT NULL AND rol_snapshot IS NOT NULL) OR (actor<>'USUARIO' AND id_usuario IS NULL))
) STRICT;

CREATE INDEX idx_vehiculos_1 ON vehiculos(id_cliente);

CREATE INDEX idx_vehiculos_2 ON vehiculos(activo,placa);

CREATE INDEX idx_productos_1 ON productos(activo,categoria,nombre);

CREATE INDEX idx_ingresos_stock_1 ON ingresos_stock(fecha_registro,id_ingreso);

CREATE INDEX idx_detalle_ingreso_1 ON detalle_ingreso(id_producto);

CREATE INDEX idx_ordenes_trabajo_1 ON ordenes_trabajo(id_mecanico,estado);

CREATE INDEX idx_ordenes_trabajo_2 ON ordenes_trabajo(fecha_fin);

CREATE INDEX idx_ordenes_trabajo_3 ON ordenes_trabajo(fecha_cancelacion);

CREATE INDEX idx_detalle_ot_servicios_1 ON detalle_ot_servicios(id_ot);

CREATE INDEX idx_detalle_ot_productos_1 ON detalle_ot_productos(id_ot);

CREATE INDEX idx_detalle_ot_productos_2 ON detalle_ot_productos(id_producto);

CREATE INDEX idx_caja_chica_1 ON caja_chica(id_usuario,fecha_cierre);

CREATE INDEX idx_caja_chica_2 ON caja_chica(ambiente,fecha_cierre);

CREATE INDEX idx_ventas_1 ON ventas(ambiente,fecha_venta);

CREATE INDEX idx_ventas_2 ON ventas(id_emisor,fecha_venta);

CREATE INDEX idx_ventas_3 ON ventas(id_caja);

CREATE INDEX idx_detalle_venta_1 ON detalle_venta(id_venta);

CREATE INDEX idx_detalle_venta_2 ON detalle_venta(id_producto);

CREATE INDEX idx_detalle_venta_3 ON detalle_venta(id_servicio);

CREATE INDEX idx_devoluciones_1 ON devoluciones(id_venta);

CREATE INDEX idx_devoluciones_2 ON devoluciones(fecha_operacion);

CREATE INDEX idx_detalle_devolucion_1 ON detalle_devolucion(id_detalle_venta);

CREATE INDEX idx_movimientos_caja_1 ON movimientos_caja(id_caja,medio_pago,fecha_operacion);

CREATE INDEX idx_movimientos_caja_2 ON movimientos_caja(id_usuario,fecha_operacion);

CREATE INDEX idx_comprobantes_1 ON comprobantes(id_venta);

CREATE INDEX idx_comprobantes_2 ON comprobantes(ambiente,fecha_emision);

CREATE INDEX idx_movimientos_inventario_1 ON movimientos_inventario(id_producto,fecha_movimiento,id_movimiento);

CREATE INDEX idx_movimientos_inventario_2 ON movimientos_inventario(id_consumo_ot);

CREATE INDEX idx_backups_1 ON backups(estado_local,fecha_creacion);

CREATE INDEX idx_auditoria_1 ON auditoria(fecha_evento,id_auditoria);

CREATE INDEX idx_auditoria_2 ON auditoria(id_usuario,fecha_evento);

CREATE INDEX idx_auditoria_3 ON auditoria(modulo,resultado,fecha_evento);

CREATE INDEX idx_auditoria_4 ON auditoria(id_operacion);

CREATE TRIGGER trg_ordenes_trabajo_id_mecanico_insert BEFORE INSERT ON ordenes_trabajo
BEGIN
    SELECT CASE WHEN NEW.id_mecanico IS NOT NULL AND NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_mecanico AND activo=1 AND rol IN ('MECANICO')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_ordenes_trabajo_id_mecanico_update BEFORE UPDATE ON ordenes_trabajo
BEGIN
SELECT CASE WHEN NEW.estado IN ('EN_ESPERA','EN_PROCESO') AND NEW.id_mecanico IS NOT NULL AND NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_mecanico AND activo=1 AND rol='MECANICO') THEN RAISE(ABORT, 'Mecanico inactivo o rol invalido') END;
END;

CREATE TRIGGER trg_ordenes_trabajo_id_creador_insert BEFORE INSERT ON ordenes_trabajo
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_creador AND activo=1 AND rol IN ('ADMINISTRADOR','CAJERO')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_caja_chica_id_usuario_insert BEFORE INSERT ON caja_chica
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND activo=1 AND rol IN ('ADMINISTRADOR','CAJERO')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_ingresos_stock_id_usuario_insert BEFORE INSERT ON ingresos_stock
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND activo=1 AND rol IN ('ADMINISTRADOR')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_devoluciones_id_usuario_insert BEFORE INSERT ON devoluciones
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND activo=1 AND rol IN ('ADMINISTRADOR','CAJERO')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_conciliaciones_series_id_usuario_insert BEFORE INSERT ON conciliaciones_series
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND activo=1 AND rol IN ('ADMINISTRADOR')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_ventas_id_emisor_insert BEFORE INSERT ON ventas
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_emisor AND activo=1 AND rol IN ('ADMINISTRADOR','CAJERO')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_ventas_id_autorizante_insert BEFORE INSERT ON ventas
BEGIN
    SELECT CASE WHEN NEW.id_autorizante IS NOT NULL AND NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_autorizante AND activo=1 AND rol IN ('ADMINISTRADOR')) THEN RAISE(ABORT, 'Actor inactivo o rol no autorizado') END;
END;

CREATE TRIGGER trg_vehiculo_identificado BEFORE INSERT ON vehiculos
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM clientes WHERE id_cliente=NEW.id_cliente AND es_varios=0) THEN RAISE(ABORT, 'Vehiculo requiere cliente identificado') END;
END;

CREATE TRIGGER trg_cliente_varios_fijo BEFORE UPDATE ON clientes
BEGIN
    SELECT CASE WHEN OLD.es_varios=1 THEN RAISE(ABORT, 'Cliente varios es fijo') END;
END;

CREATE TRIGGER trg_ultimo_admin BEFORE UPDATE ON usuarios
BEGIN
    SELECT CASE WHEN OLD.activo=1 AND OLD.rol='ADMINISTRADOR' AND (NEW.activo=0 OR NEW.rol<>'ADMINISTRADOR') AND (SELECT count(*) FROM usuarios WHERE activo=1 AND rol='ADMINISTRADOR')=1 THEN RAISE(ABORT, 'No se desactiva el ultimo administrador') END;
END;

CREATE TRIGGER trg_producto_nuevo_sin_stock BEFORE INSERT ON productos
BEGIN
    SELECT CASE WHEN NEW.stock_actual<>0 OR NEW.valor_stock_micro<>0 THEN RAISE(ABORT, 'Registrar stock mediante ingreso y kardex') END;
END;

CREATE TRIGGER trg_unidad_codigo_historia BEFORE UPDATE ON productos
BEGIN
SELECT CASE WHEN (NEW.unidad_medida IS NOT OLD.unidad_medida OR NEW.codigo IS NOT OLD.codigo) AND (EXISTS(SELECT 1 FROM movimientos_inventario WHERE id_producto=OLD.id_producto) OR EXISTS(SELECT 1 FROM detalle_ot_productos WHERE id_producto=OLD.id_producto) OR EXISTS(SELECT 1 FROM detalle_venta WHERE id_producto=OLD.id_producto)) THEN RAISE(ABORT, 'Codigo y unidad usados son inmutables') END;
END;

CREATE TRIGGER trg_ot_nace_espera BEFORE INSERT ON ordenes_trabajo
BEGIN
SELECT CASE WHEN NEW.estado<>'EN_ESPERA' OR NEW.fecha_inicio IS NOT NULL OR NEW.id_zanja_ejecucion IS NOT NULL OR NEW.fecha_fin IS NOT NULL OR NEW.fecha_cancelacion IS NOT NULL OR NEW.tipo_aceite IS NOT NULL OR NEW.intervalo_aceite_km IS NOT NULL OR NEW.corregido<>0 THEN RAISE(ABORT, 'OT nace en espera libre o asignada, sin ejecucion') END;
END;

CREATE TRIGGER trg_ot_transicion BEFORE UPDATE ON ordenes_trabajo
BEGIN
SELECT CASE WHEN OLD.estado='CANCELADO' THEN RAISE(ABORT, 'OT cancelada inmutable') END;
    SELECT CASE WHEN OLD.estado='FINALIZADO' AND (NEW.id_ot IS NOT OLD.id_ot OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.codigo_ot IS NOT OLD.codigo_ot OR NEW.id_vehiculo IS NOT OLD.id_vehiculo OR NEW.id_cliente_atendido IS NOT OLD.id_cliente_atendido OR NEW.placa_snapshot IS NOT OLD.placa_snapshot OR NEW.cliente_snapshot IS NOT OLD.cliente_snapshot OR NEW.id_creador IS NOT OLD.id_creador OR NEW.id_mecanico IS NOT OLD.id_mecanico OR NEW.id_mecanico_terminal IS NOT OLD.id_mecanico_terminal OR NEW.id_zanja_ejecucion IS NOT OLD.id_zanja_ejecucion OR NEW.estado IS NOT OLD.estado OR NEW.kilometraje_ingreso IS NOT OLD.kilometraje_ingreso OR NEW.pedido_cliente IS NOT OLD.pedido_cliente OR NEW.nota_interna IS NOT OLD.nota_interna OR NEW.fecha_ingreso IS NOT OLD.fecha_ingreso OR NEW.fecha_inicio IS NOT OLD.fecha_inicio OR NEW.fecha_fin IS NOT OLD.fecha_fin OR NEW.fecha_cancelacion IS NOT OLD.fecha_cancelacion OR NEW.motivo_cancelacion IS NOT OLD.motivo_cancelacion OR NEW.id_usuario_cancelacion IS NOT OLD.id_usuario_cancelacion) THEN RAISE(ABORT, 'Solo correccion publica en OT finalizada') END;
    SELECT CASE WHEN OLD.estado='FINALIZADO' AND NEW.corregido<>1 THEN RAISE(ABORT, 'Correccion publica debe quedar marcada') END;
    SELECT CASE WHEN NEW.estado<>OLD.estado AND NOT ((OLD.estado='EN_ESPERA' AND NEW.estado IN ('EN_PROCESO','CANCELADO')) OR (OLD.estado='EN_PROCESO' AND NEW.estado IN ('FINALIZADO','CANCELADO'))) THEN RAISE(ABORT, 'Transicion OT no permitida') END;
    SELECT CASE WHEN NEW.estado='EN_PROCESO' AND NOT EXISTS(SELECT 1 FROM zanjas WHERE id_zanja=NEW.id_zanja_ejecucion AND activa=1) THEN RAISE(ABORT, 'Zanja no habilitada') END;
    SELECT CASE WHEN OLD.fecha_inicio IS NOT NULL AND (NEW.fecha_inicio IS NOT OLD.fecha_inicio OR NEW.id_zanja_ejecucion IS NOT OLD.id_zanja_ejecucion) THEN RAISE(ABORT, 'Inicio y zanja historicos') END;
    SELECT CASE WHEN NEW.estado='FINALIZADO' AND NEW.id_mecanico_terminal IS NOT NEW.id_mecanico THEN RAISE(ABORT, 'Conservar mecanico terminal') END;
    SELECT CASE WHEN NEW.estado='FINALIZADO' AND (NOT EXISTS(SELECT 1 FROM detalle_ot_servicios WHERE id_ot=NEW.id_ot) OR (EXISTS(SELECT 1 FROM detalle_ot_servicios WHERE id_ot=NEW.id_ot AND es_cambio_aceite=1) AND NEW.tipo_aceite IS NULL)) THEN RAISE(ABORT, 'Finalizacion requiere servicios y aceite aplicable') END;
    SELECT CASE WHEN NEW.estado='FINALIZADO' AND NEW.kilometraje_salida IS NULL THEN RAISE(ABORT, 'Finalizacion requiere kilometraje') END;
END;

CREATE TRIGGER trg_detalle_ot_productos_ot_estado BEFORE INSERT ON detalle_ot_productos
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=NEW.id_ot AND estado ='EN_PROCESO') THEN RAISE(ABORT, 'OT no admite esa linea') END;
END;

CREATE TRIGGER trg_detalle_ot_servicios_ot_estado BEFORE INSERT ON detalle_ot_servicios
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=NEW.id_ot AND estado IN ('EN_ESPERA','EN_PROCESO')) THEN RAISE(ABORT, 'OT no admite esa linea') END;
END;

CREATE TRIGGER trg_detalle_ot_productos_update BEFORE UPDATE ON detalle_ot_productos
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_detalle_ot_productos_delete BEFORE DELETE ON detalle_ot_productos
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_caja_nueva BEFORE INSERT ON caja_chica
BEGIN
    SELECT CASE WHEN NEW.estado<>'ABIERTA' THEN RAISE(ABORT, 'Caja debe abrirse antes de cerrar') END;
END;

CREATE TRIGGER trg_movimiento_caja_valido BEFORE INSERT ON movimientos_caja
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM caja_chica c JOIN usuarios u ON u.id_usuario=NEW.id_usuario WHERE c.id_caja=NEW.id_caja AND c.estado='ABIERTA' AND c.por_conciliar=0 AND c.id_usuario=NEW.id_usuario AND u.activo=1 AND u.rol IN ('ADMINISTRADOR','CAJERO')) THEN RAISE(ABORT, 'Movimiento exige caja propia abierta operativa') END;
    SELECT CASE WHEN NEW.tipo='COBRO' AND NOT EXISTS(SELECT 1 FROM ventas v JOIN caja_chica c ON c.id_caja=v.id_caja WHERE v.id_venta=NEW.id_venta AND v.estado_registro='PREPARANDO' AND v.id_caja=NEW.id_caja AND v.id_emisor=NEW.id_usuario AND v.ambiente=c.ambiente) THEN RAISE(ABORT, 'Cobro no corresponde a venta en preparacion') END;
    SELECT CASE WHEN NEW.tipo='REINTEGRO' AND NOT EXISTS(SELECT 1 FROM devoluciones d WHERE d.id_devolucion=NEW.id_devolucion AND d.estado_registro='PREPARANDO' AND d.id_caja=NEW.id_caja AND d.id_usuario=NEW.id_usuario) THEN RAISE(ABORT, 'Reintegro no corresponde a devolucion en preparacion') END;
    SELECT CASE WHEN NEW.tipo='REVERSO_MANUAL' AND NOT EXISTS(SELECT 1 FROM movimientos_caja m WHERE m.id_movimiento=NEW.id_movimiento_original AND m.id_caja=NEW.id_caja AND m.tipo IN ('INGRESO_MANUAL','EGRESO_MANUAL') AND m.monto_cent=NEW.monto_cent AND m.signo=-NEW.signo) THEN RAISE(ABORT, 'Reverso manual no coincide con original') END;
    SELECT CASE WHEN NEW.tipo IN ('INGRESO_MANUAL','EGRESO_MANUAL') AND (NEW.categoria IS NULL) THEN RAISE(ABORT, 'Manual requiere categoria') END;
    SELECT CASE WHEN NEW.medio_pago='EFECTIVO' AND NEW.signo=-1 AND NEW.monto_cent>(SELECT monto_apertura_cent FROM caja_chica WHERE id_caja=NEW.id_caja)+(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='EFECTIVO') THEN RAISE(ABORT, 'Efectivo registrado insuficiente') END;
END;

CREATE TRIGGER trg_movimiento_version_caja AFTER INSERT ON movimientos_caja
BEGIN
    UPDATE caja_chica SET version=version+1 WHERE id_caja=NEW.id_caja;
END;

CREATE TRIGGER trg_caja_confirmar_cierre BEFORE UPDATE ON caja_chica
WHEN OLD.estado='ABIERTA' AND NEW.estado='CERRADA'
BEGIN
SELECT CASE WHEN NEW.efectivo_esperado_cent IS NOT (NEW.monto_apertura_cent+(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='EFECTIVO')) THEN RAISE(ABORT, 'Efectivo esperado no concilia') END;
    SELECT CASE WHEN NEW.tarjeta_neto_cent IS NOT (SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='TARJETA') THEN RAISE(ABORT, 'Resumen digital no concilia') END;
    SELECT CASE WHEN NEW.yape_neto_cent IS NOT (SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='YAPE') THEN RAISE(ABORT, 'Resumen digital no concilia') END;
    SELECT CASE WHEN NEW.plin_neto_cent IS NOT (SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='PLIN') THEN RAISE(ABORT, 'Resumen digital no concilia') END;
    SELECT CASE WHEN NEW.transferencia_neto_cent IS NOT (SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja WHERE id_caja=NEW.id_caja AND medio_pago='TRANSFERENCIA') THEN RAISE(ABORT, 'Resumen digital no concilia') END;
    SELECT CASE WHEN NEW.efectivo_contado_cent IS NULL OR NEW.fondo_siguiente_cent IS NULL OR NEW.retiro_cierre_cent IS NULL OR NEW.fondo_siguiente_cent+NEW.retiro_cierre_cent<>NEW.efectivo_contado_cent THEN RAISE(ABORT, 'Fondo y retiro no concilian') END;
    SELECT CASE WHEN (NEW.efectivo_contado_cent<>NEW.efectivo_esperado_cent OR NEW.tipo_cierre IN ('AUSENCIA','RECUPERACION')) AND (NEW.motivo_cierre IS NULL OR length(trim(NEW.motivo_cierre)) NOT BETWEEN 5 AND 250) THEN RAISE(ABORT, 'Causa o diferencia exige una explicacion') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios u WHERE u.id_usuario=NEW.id_usuario_cierre AND u.activo=1 AND ((NEW.tipo_cierre='NORMAL' AND NEW.id_usuario_cierre=NEW.id_usuario AND u.rol IN ('ADMINISTRADOR','CAJERO')) OR (NEW.tipo_cierre IN ('AUSENCIA','RECUPERACION') AND u.rol='ADMINISTRADOR'))) THEN RAISE(ABORT, 'Actor de cierre no autorizado') END;
    SELECT CASE WHEN (OLD.por_conciliar=1 AND NEW.tipo_cierre<>'RECUPERACION') OR (OLD.por_conciliar=0 AND NEW.tipo_cierre='RECUPERACION') OR (NEW.tipo_cierre='AUSENCIA' AND NEW.id_usuario_cierre=NEW.id_usuario) THEN RAISE(ABORT, 'Tipo de cierre incompatible con turno') END;
    SELECT CASE WHEN EXISTS(SELECT 1 FROM ventas WHERE id_caja=NEW.id_caja AND estado_registro='PREPARANDO') OR EXISTS(SELECT 1 FROM devoluciones WHERE id_caja=NEW.id_caja AND estado_registro='PREPARANDO') THEN RAISE(ABORT, 'Operacion comercial incompleta') END;
END;

CREATE TRIGGER trg_caja_cerrada_fija BEFORE UPDATE ON caja_chica
WHEN OLD.estado='CERRADA'
BEGIN
SELECT CASE WHEN NEW.estado IS NOT OLD.estado OR NEW.id_usuario IS NOT OLD.id_usuario OR NEW.monto_apertura_cent IS NOT OLD.monto_apertura_cent OR NEW.fecha_apertura IS NOT OLD.fecha_apertura OR NEW.fecha_cierre IS NOT OLD.fecha_cierre OR NEW.tipo_cierre IS NOT OLD.tipo_cierre OR NEW.id_usuario_cierre IS NOT OLD.id_usuario_cierre OR NEW.fondo_siguiente_cent IS NOT OLD.fondo_siguiente_cent OR NEW.retiro_cierre_cent IS NOT OLD.retiro_cierre_cent OR NEW.ambiente IS NOT OLD.ambiente OR NEW.motivo_cierre IS NOT OLD.motivo_cierre OR NEW.id_operacion_cierre IS NOT OLD.id_operacion_cierre OR NEW.efectivo_esperado_cent IS NOT OLD.efectivo_esperado_cent OR NEW.efectivo_contado_cent IS NOT OLD.efectivo_contado_cent OR NEW.tarjeta_neto_cent IS NOT OLD.tarjeta_neto_cent OR NEW.yape_neto_cent IS NOT OLD.yape_neto_cent OR NEW.plin_neto_cent IS NOT OLD.plin_neto_cent OR NEW.transferencia_neto_cent IS NOT OLD.transferencia_neto_cent OR NEW.por_conciliar IS NOT OLD.por_conciliar THEN RAISE(ABORT, 'Cierre original inmutable') END;
END;

CREATE TRIGGER trg_venta_insert BEFORE INSERT ON ventas
BEGIN
SELECT CASE WHEN NEW.estado_registro<>'PREPARANDO' THEN RAISE(ABORT, 'Venta nace preparando') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM caja_chica WHERE id_caja=NEW.id_caja AND id_usuario=NEW.id_emisor AND estado='ABIERTA' AND por_conciliar=0 AND ambiente=NEW.ambiente) THEN RAISE(ABORT, 'Venta exige caja propia operativa') END;
    SELECT CASE WHEN NEW.id_ot IS NOT NULL AND NOT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=NEW.id_ot AND estado='FINALIZADO') THEN RAISE(ABORT, 'OT no finalizada') END;
    SELECT CASE WHEN NEW.descuento_cent<>(NEW.bruto_cent*NEW.descuento_bp+5000)/10000 THEN RAISE(ABORT, 'Descuento no corresponde al porcentaje') END;
    SELECT CASE WHEN NEW.descuento_bp>500 AND (NEW.motivo_descuento IS NULL OR length(trim(NEW.motivo_descuento)) NOT BETWEEN 5 AND 250) THEN RAISE(ABORT, 'Descuento mayor de 5 requiere motivo') END;
    SELECT CASE WHEN NEW.descuento_bp>500 AND EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_emisor AND rol='CAJERO') AND NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_autorizante AND rol='ADMINISTRADOR' AND activo=1 AND debe_cambiar_clave=0 AND (bloqueado_hasta IS NULL OR bloqueado_hasta<=datetime('now','-5 hours'))) THEN RAISE(ABORT, 'Descuento requiere autorizante administrativo') END;
END;

CREATE TRIGGER trg_linea_venta_insert BEFORE INSERT ON detalle_venta
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ventas WHERE id_venta=NEW.id_venta AND estado_registro='PREPARANDO') THEN RAISE(ABORT, 'Venta ya confirmada') END;
    SELECT CASE WHEN NEW.id_consumo_ot IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ot_productos p JOIN ventas v ON v.id_ot=p.id_ot WHERE v.id_venta=NEW.id_venta AND p.id_detalle=NEW.id_consumo_ot AND p.id_producto=NEW.id_producto) THEN RAISE(ABORT, 'Consumo OT no pertenece a venta') END;
    SELECT CASE WHEN NEW.id_servicio_ot IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ot_servicios s JOIN ventas v ON v.id_ot=s.id_ot WHERE v.id_venta=NEW.id_venta AND s.id_detalle=NEW.id_servicio_ot AND s.id_servicio=NEW.id_servicio) THEN RAISE(ABORT, 'Servicio OT no pertenece a venta') END;
END;

CREATE TRIGGER trg_confirmar_venta BEFORE UPDATE ON ventas
WHEN OLD.estado_registro='PREPARANDO'
BEGIN
    SELECT CASE WHEN NEW.estado_registro='CONFIRMADA' AND ((SELECT count(*) FROM detalle_venta WHERE id_venta=NEW.id_venta)=0 OR NEW.total_cent<>(SELECT coalesce(sum(total_cent),0) FROM detalle_venta WHERE id_venta=NEW.id_venta) OR NEW.bruto_cent<>(SELECT coalesce(sum(bruto_cent),0) FROM detalle_venta WHERE id_venta=NEW.id_venta) OR NEW.descuento_cent<>(SELECT coalesce(sum(descuento_cent),0) FROM detalle_venta WHERE id_venta=NEW.id_venta) OR NEW.base_cent<>(SELECT coalesce(sum(base_cent),0) FROM detalle_venta WHERE id_venta=NEW.id_venta) OR NEW.igv_cent<>(SELECT coalesce(sum(igv_cent),0) FROM detalle_venta WHERE id_venta=NEW.id_venta) OR NEW.total_cent<>(SELECT coalesce(sum(monto_cent),0) FROM movimientos_caja WHERE id_venta=NEW.id_venta AND tipo='COBRO')) THEN RAISE(ABORT, 'Venta no concilia lineas y pagos') END;
END;

CREATE TRIGGER trg_ventas_update BEFORE UPDATE ON ventas
WHEN OLD.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_ventas_delete BEFORE DELETE ON ventas
WHEN OLD.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_devolucion_insert BEFORE INSERT ON devoluciones
BEGIN
    SELECT CASE WHEN NEW.estado_registro<>'PREPARANDO' OR NOT EXISTS(SELECT 1 FROM ventas v JOIN caja_chica c ON c.id_caja=NEW.id_caja WHERE v.id_venta=NEW.id_venta AND v.estado_registro='CONFIRMADA' AND c.estado='ABIERTA' AND c.por_conciliar=0 AND c.id_usuario=NEW.id_usuario AND c.ambiente=v.ambiente) THEN RAISE(ABORT, 'Devolucion exige venta y caja propia') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM comprobantes WHERE id_comprobante=NEW.id_documento_original AND id_venta=NEW.id_venta AND relacion IN ('ORIGINAL','REEMISION')) THEN RAISE(ABORT, 'Documento original no corresponde a venta') END;
END;

CREATE TRIGGER trg_detalle_devolucion BEFORE INSERT ON detalle_devolucion
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM devoluciones d JOIN detalle_venta v ON v.id_venta=d.id_venta WHERE d.id_devolucion=NEW.id_devolucion AND d.estado_registro='PREPARANDO' AND v.id_detalle=NEW.id_detalle_venta) THEN RAISE(ABORT, 'Linea no pertenece a venta original') END;
    SELECT CASE WHEN NEW.cantidad+(SELECT coalesce(sum(cantidad),0) FROM detalle_devolucion WHERE id_detalle_venta=NEW.id_detalle_venta)>(SELECT cantidad FROM detalle_venta WHERE id_detalle=NEW.id_detalle_venta) THEN RAISE(ABORT, 'Cantidad devuelta supera original') END;
    SELECT CASE WHEN NEW.total_cent+(SELECT coalesce(sum(total_cent),0) FROM detalle_devolucion WHERE id_detalle_venta=NEW.id_detalle_venta)>(SELECT total_cent FROM detalle_venta WHERE id_detalle=NEW.id_detalle_venta) THEN RAISE(ABORT, 'Reintegro supera importe original') END;
    SELECT CASE WHEN NEW.costo_revertido_micro+(SELECT coalesce(sum(costo_revertido_micro),0) FROM detalle_devolucion WHERE id_detalle_venta=NEW.id_detalle_venta)>(SELECT coalesce(costo_total_micro,0) FROM detalle_venta WHERE id_detalle=NEW.id_detalle_venta) THEN RAISE(ABORT, 'Costo revertido supera costo original') END;
    SELECT CASE WHEN NEW.cantidad_recuperada>0 AND EXISTS(SELECT 1 FROM detalle_venta WHERE id_detalle=NEW.id_detalle_venta AND id_servicio IS NOT NULL) THEN RAISE(ABORT, 'Servicio no retorna stock') END;
END;

CREATE TRIGGER trg_devolucion_confirmar BEFORE UPDATE ON devoluciones
WHEN OLD.estado_registro='PREPARANDO'
BEGIN
    SELECT CASE WHEN NEW.estado_registro='CONFIRMADA' AND ((SELECT count(*) FROM detalle_devolucion WHERE id_devolucion=NEW.id_devolucion)=0 OR NEW.total_cent<>(SELECT coalesce(sum(total_cent),0) FROM detalle_devolucion WHERE id_devolucion=NEW.id_devolucion) OR NEW.base_cent<>(SELECT coalesce(sum(base_cent),0) FROM detalle_devolucion WHERE id_devolucion=NEW.id_devolucion) OR NEW.igv_cent<>(SELECT coalesce(sum(igv_cent),0) FROM detalle_devolucion WHERE id_devolucion=NEW.id_devolucion) OR NEW.total_cent<>(SELECT coalesce(sum(monto_cent),0) FROM movimientos_caja WHERE id_devolucion=NEW.id_devolucion AND tipo='REINTEGRO')) THEN RAISE(ABORT, 'Devolucion no concilia lineas y reintegros') END;
END;

CREATE TRIGGER trg_devoluciones_update BEFORE UPDATE ON devoluciones
WHEN OLD.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_devoluciones_delete BEFORE DELETE ON devoluciones
WHEN OLD.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_serie_no_baja BEFORE UPDATE ON series_comprobante
BEGIN
    SELECT CASE WHEN NEW.correlativo_actual<OLD.correlativo_actual THEN RAISE(ABORT, 'Correlativo no puede disminuir') END;
    SELECT CASE WHEN EXISTS(SELECT 1 FROM comprobantes WHERE id_serie=OLD.id_serie) AND (NEW.ambiente IS NOT OLD.ambiente OR NEW.serie IS NOT OLD.serie OR NEW.tipo_comprobante IS NOT OLD.tipo_comprobante) THEN RAISE(ABORT, 'Serie utilizada inmutable') END;
END;

CREATE TRIGGER trg_kardex_valida BEFORE INSERT ON movimientos_inventario
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM productos WHERE id_producto=NEW.id_producto AND unidad_medida=NEW.unidad_medida AND stock_actual=NEW.stock_anterior AND valor_stock_micro=NEW.valor_anterior_micro) THEN RAISE(ABORT, 'Saldo de inventario desactualizado') END;
    SELECT CASE WHEN NEW.tipo_movimiento IN ('STOCK_INICIAL','COMPRA','CONSUMO_OT','VENTA_DIRECTA') AND EXISTS(SELECT 1 FROM productos WHERE id_producto=NEW.id_producto AND activo=0) THEN RAISE(ABORT, 'Producto inactivo') END;
    SELECT CASE WHEN NEW.id_detalle_ingreso IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ingreso di WHERE di.id_detalle=NEW.id_detalle_ingreso AND di.id_producto=NEW.id_producto AND di.cantidad=abs(NEW.diferencia) AND di.valor_aporte_micro=abs(NEW.valor_delta_micro)) THEN RAISE(ABORT, 'Movimiento no coincide con ingreso') END;
    SELECT CASE WHEN NEW.tipo_movimiento='CONSUMO_OT' AND NOT EXISTS(SELECT 1 FROM detalle_ot_productos p JOIN ordenes_trabajo o ON o.id_ot=p.id_ot WHERE p.id_detalle=NEW.id_consumo_ot AND p.id_producto=NEW.id_producto AND p.cantidad=-NEW.diferencia AND p.valor_consumido_micro=-NEW.valor_delta_micro AND o.estado='EN_PROCESO') THEN RAISE(ABORT, 'Movimiento no coincide con consumo OT') END;
    SELECT CASE WHEN NEW.tipo_movimiento='REPOSICION_OT' AND NOT EXISTS(SELECT 1 FROM detalle_ot_productos p JOIN ordenes_trabajo o ON o.id_ot=p.id_ot WHERE p.id_detalle=NEW.id_consumo_ot AND p.id_producto=NEW.id_producto AND o.estado IN ('EN_ESPERA','EN_PROCESO')) THEN RAISE(ABORT, 'Consumo no recuperable o OT terminal finalizada') END;
    SELECT CASE WHEN NEW.tipo_movimiento='REPOSICION_OT' AND (NEW.diferencia+(SELECT coalesce(sum(diferencia),0) FROM movimientos_inventario WHERE id_consumo_ot=NEW.id_consumo_ot AND tipo_movimiento='REPOSICION_OT')>(SELECT cantidad FROM detalle_ot_productos WHERE id_detalle=NEW.id_consumo_ot) OR NEW.valor_delta_micro+(SELECT coalesce(sum(valor_delta_micro),0) FROM movimientos_inventario WHERE id_consumo_ot=NEW.id_consumo_ot AND tipo_movimiento='REPOSICION_OT')>(SELECT valor_consumido_micro FROM detalle_ot_productos WHERE id_detalle=NEW.id_consumo_ot)) THEN RAISE(ABORT, 'Retorno supera consumo original') END;
    SELECT CASE WHEN NEW.tipo_movimiento='VENTA_DIRECTA' AND NOT EXISTS(SELECT 1 FROM detalle_venta l JOIN ventas v ON v.id_venta=l.id_venta WHERE l.id_detalle=NEW.id_detalle_venta AND l.id_producto=NEW.id_producto AND l.id_consumo_ot IS NULL AND l.cantidad=-NEW.diferencia AND l.costo_total_micro=-NEW.valor_delta_micro AND v.estado_registro='PREPARANDO') THEN RAISE(ABORT, 'No descontar por segunda vez consumo de OT') END;
    SELECT CASE WHEN NEW.tipo_movimiento='DEVOLUCION_VENTA' AND NOT EXISTS(SELECT 1 FROM detalle_devolucion r JOIN detalle_venta l ON l.id_detalle=r.id_detalle_venta JOIN devoluciones d ON d.id_devolucion=r.id_devolucion WHERE r.id_detalle=NEW.id_detalle_devolucion AND l.id_producto=NEW.id_producto AND r.cantidad_recuperada=NEW.diferencia AND r.costo_revertido_micro=NEW.valor_delta_micro AND d.estado_registro='PREPARANDO') THEN RAISE(ABORT, 'Retorno no coincide con devolucion') END;
END;

CREATE TRIGGER trg_movimientos_inventario_update BEFORE UPDATE ON movimientos_inventario
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_movimientos_inventario_delete BEFORE DELETE ON movimientos_inventario
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_movimientos_caja_update BEFORE UPDATE ON movimientos_caja
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_movimientos_caja_delete BEFORE DELETE ON movimientos_caja
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_comprobantes_update BEFORE UPDATE ON comprobantes
BEGIN
SELECT CASE WHEN NEW.id_comprobante IS NOT OLD.id_comprobante OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.id_venta IS NOT OLD.id_venta OR NEW.id_devolucion IS NOT OLD.id_devolucion OR NEW.id_documento_anterior IS NOT OLD.id_documento_anterior OR NEW.id_documento_referenciado IS NOT OLD.id_documento_referenciado OR NEW.relacion IS NOT OLD.relacion OR NEW.tipo_comprobante IS NOT OLD.tipo_comprobante OR NEW.ambiente IS NOT OLD.ambiente OR NEW.id_serie IS NOT OLD.id_serie OR NEW.correlativo IS NOT OLD.correlativo OR NEW.empresa_snapshot IS NOT OLD.empresa_snapshot OR NEW.receptor_snapshot IS NOT OLD.receptor_snapshot OR NEW.payload_json IS NOT OLD.payload_json OR NEW.payload_sha256 IS NOT OLD.payload_sha256 OR NEW.base_cent IS NOT OLD.base_cent OR NEW.igv_cent IS NOT OLD.igv_cent OR NEW.total_cent IS NOT OLD.total_cent OR NEW.fecha_emision IS NOT OLD.fecha_emision THEN RAISE(ABORT, 'Contenido fiscal confirmado inmutable') END;
    SELECT CASE WHEN OLD.estado_envio IN ('ACEPTADO','RECHAZADO') AND NEW.estado_envio<>OLD.estado_envio THEN RAISE(ABORT, 'Estado oficial terminal inmutable') END;
    SELECT CASE WHEN NEW.estado_envio='ENVIANDO' AND OLD.estado_envio<>'ENVIANDO' AND (OLD.requiere_consulta=1 OR OLD.resultado_incierto=1 OR OLD.intento_iniciado=1) THEN RAISE(ABORT, 'Consultar identidad y confirmar ausencia antes de reenviar') END;
END;

CREATE TRIGGER trg_comprobantes_delete BEFORE DELETE ON comprobantes
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_conciliaciones_series_update BEFORE UPDATE ON conciliaciones_series
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_conciliaciones_series_delete BEFORE DELETE ON conciliaciones_series
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_auditoria_update BEFORE UPDATE ON auditoria
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_auditoria_delete BEFORE DELETE ON auditoria
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_usuarios_no_delete BEFORE DELETE ON usuarios
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_clientes_no_delete BEFORE DELETE ON clientes
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_vehiculos_no_delete BEFORE DELETE ON vehiculos
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_proveedores_no_delete BEFORE DELETE ON proveedores
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_productos_no_delete BEFORE DELETE ON productos
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_servicios_no_delete BEFORE DELETE ON servicios
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_ingresos_stock_no_delete BEFORE DELETE ON ingresos_stock
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_detalle_ingreso_no_delete BEFORE DELETE ON detalle_ingreso
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_ordenes_trabajo_no_delete BEFORE DELETE ON ordenes_trabajo
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_caja_chica_no_delete BEFORE DELETE ON caja_chica
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_series_comprobante_no_delete BEFORE DELETE ON series_comprobante
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_backups_no_delete BEFORE DELETE ON backups
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_restauraciones_no_delete BEFORE DELETE ON restauraciones
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_configuracion_no_delete BEFORE DELETE ON configuracion
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_zanjas_no_delete BEFORE DELETE ON zanjas
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'No eliminar registros de negocio; usar correccion autorizada') END;
END;

CREATE TRIGGER trg_detalle_ingreso_update BEFORE UPDATE ON detalle_ingreso
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Registro historico inmutable') END;
END;

CREATE TRIGGER trg_detalle_venta_snapshot_update BEFORE UPDATE ON detalle_venta
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Linea inmutable; revertir la transaccion incompleta') END;
END;

CREATE TRIGGER trg_detalle_venta_snapshot_delete BEFORE DELETE ON detalle_venta
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Linea inmutable; revertir la transaccion incompleta') END;
END;

CREATE TRIGGER trg_detalle_devolucion_snapshot_update BEFORE UPDATE ON detalle_devolucion
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Linea inmutable; revertir la transaccion incompleta') END;
END;

CREATE TRIGGER trg_detalle_devolucion_snapshot_delete BEFORE DELETE ON detalle_devolucion
BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Linea inmutable; revertir la transaccion incompleta') END;
END;

CREATE TRIGGER trg_ot_vehiculo_activo BEFORE INSERT ON ordenes_trabajo
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM vehiculos v JOIN clientes c ON c.id_cliente=v.id_cliente WHERE v.id_vehiculo=NEW.id_vehiculo AND v.activo=1 AND c.es_varios=0 AND v.id_cliente=NEW.id_cliente_atendido AND v.kilometraje_actual<=NEW.kilometraje_ingreso) THEN RAISE(ABORT, 'Vehiculo o kilometraje no apto para OT') END;
END;

CREATE TRIGGER trg_ot_aceite_no_inventado BEFORE UPDATE ON ordenes_trabajo
BEGIN
SELECT CASE WHEN NEW.tipo_aceite IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ot_servicios WHERE id_ot=NEW.id_ot AND es_cambio_aceite=1) THEN RAISE(ABORT, 'Servicio no es cambio de aceite') END;
END;

CREATE TRIGGER trg_ingreso_actor_estado BEFORE INSERT ON ingresos_stock
BEGIN
SELECT CASE WHEN NEW.estado<>'PREPARANDO' THEN RAISE(ABORT, 'Ingreso nace preparando') END;
END;

CREATE TRIGGER trg_ingreso_transicion BEFORE UPDATE ON ingresos_stock
BEGIN
    SELECT CASE WHEN OLD.estado='ANULADO' OR NOT ((OLD.estado='PREPARANDO' AND NEW.estado='REGISTRADO') OR (OLD.estado='REGISTRADO' AND NEW.estado='ANULADO')) THEN RAISE(ABORT, 'Estado de ingreso no permitido') END;
    SELECT CASE WHEN NEW.total_costo_micro IS NOT OLD.total_costo_micro OR NEW.id_proveedor IS NOT OLD.id_proveedor OR NEW.fecha_registro IS NOT OLD.fecha_registro OR NEW.numero_documento IS NOT OLD.numero_documento OR NEW.proveedor_snapshot IS NOT OLD.proveedor_snapshot THEN RAISE(ABORT, 'Original de ingreso inmutable') END;
    SELECT CASE WHEN NEW.estado='REGISTRADO' AND ((SELECT count(*) FROM detalle_ingreso WHERE id_ingreso=NEW.id_ingreso) NOT BETWEEN 1 AND 100 OR NEW.total_costo_micro<>(SELECT sum(valor_aporte_micro) FROM detalle_ingreso WHERE id_ingreso=NEW.id_ingreso) OR EXISTS(SELECT 1 FROM detalle_ingreso d WHERE d.id_ingreso=NEW.id_ingreso AND NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_detalle_ingreso=d.id_detalle AND m.tipo_movimiento IN ('STOCK_INICIAL','COMPRA')))) THEN RAISE(ABORT, 'Ingreso no concilia lineas y kardex') END;
    SELECT CASE WHEN NEW.estado='ANULADO' AND EXISTS(SELECT 1 FROM detalle_ingreso d WHERE d.id_ingreso=NEW.id_ingreso AND NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_detalle_ingreso=d.id_detalle AND m.tipo_movimiento='ANULACION_INGRESO')) THEN RAISE(ABORT, 'Anulacion incompleta') END;
END;

CREATE TRIGGER trg_detalle_ingreso_nuevo BEFORE INSERT ON detalle_ingreso
BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ingresos_stock WHERE id_ingreso=NEW.id_ingreso AND estado='PREPARANDO') THEN RAISE(ABORT, 'Ingreso ya registrado') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM productos WHERE id_producto=NEW.id_producto AND activo=1 AND unidad_medida=NEW.unidad_medida) THEN RAISE(ABORT, 'Producto o unidad incompatible') END;
END;

CREATE TRIGGER trg_kardex_ingreso_estado BEFORE INSERT ON movimientos_inventario
BEGIN
    SELECT CASE WHEN NEW.tipo_movimiento IN ('STOCK_INICIAL','COMPRA') AND NOT EXISTS(SELECT 1 FROM detalle_ingreso d JOIN ingresos_stock i ON i.id_ingreso=d.id_ingreso WHERE d.id_detalle=NEW.id_detalle_ingreso AND i.estado='PREPARANDO') THEN RAISE(ABORT, 'Ingreso fuera de transaccion') END;
    SELECT CASE WHEN NEW.tipo_movimiento='ANULACION_INGRESO' AND NOT EXISTS(SELECT 1 FROM detalle_ingreso d JOIN ingresos_stock i ON i.id_ingreso=d.id_ingreso JOIN movimientos_inventario m ON m.id_detalle_ingreso=d.id_detalle AND m.tipo_movimiento IN ('STOCK_INICIAL','COMPRA') WHERE d.id_detalle=NEW.id_detalle_ingreso AND i.estado='REGISTRADO') THEN RAISE(ABORT, 'Anulacion sin ingreso registrado') END;
END;

CREATE TRIGGER trg_cobro_sin_revisiones_pendientes BEFORE INSERT ON ventas
BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM configuracion WHERE recuperacion_pendiente=1 OR revision_caja_pendiente=1 OR revision_token_pendiente=1 OR revision_series_pendiente=1) THEN RAISE(ABORT, 'Instalacion requiere conciliacion') END;
END;

CREATE TRIGGER trg_venta_kardex_completo BEFORE UPDATE ON ventas
WHEN OLD.estado_registro='PREPARANDO' AND NEW.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM detalle_venta l WHERE l.id_venta=NEW.id_venta AND l.id_producto IS NOT NULL AND l.id_consumo_ot IS NULL AND NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_detalle_venta=l.id_detalle AND m.tipo_movimiento='VENTA_DIRECTA')) THEN RAISE(ABORT, 'Venta sin salida directa de stock') END;
END;

CREATE TRIGGER trg_costo_linea_ot BEFORE INSERT ON detalle_venta
BEGIN
    SELECT CASE WHEN NEW.id_consumo_ot IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ot_productos p WHERE p.id_detalle=NEW.id_consumo_ot AND NEW.cantidad=p.cantidad-(SELECT coalesce(sum(m.diferencia),0) FROM movimientos_inventario m WHERE m.id_consumo_ot=p.id_detalle AND m.tipo_movimiento='REPOSICION_OT') AND NEW.costo_total_micro=p.valor_consumido_micro-(SELECT coalesce(sum(m.valor_delta_micro),0) FROM movimientos_inventario m WHERE m.id_consumo_ot=p.id_detalle AND m.tipo_movimiento='REPOSICION_OT') AND NEW.precio_unitario_cent=p.precio_unitario_cent AND NEW.costo_unitario_micro=p.costo_unitario_micro) THEN RAISE(ABORT, 'Linea OT debe heredar cantidad precio y costo') END;
END;

CREATE TRIGGER trg_devolucion_stock_completo BEFORE UPDATE ON devoluciones
WHEN OLD.estado_registro='PREPARANDO' AND NEW.estado_registro='CONFIRMADA'
BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM detalle_devolucion r WHERE r.id_devolucion=NEW.id_devolucion AND r.cantidad_recuperada>0 AND NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_detalle_devolucion=r.id_detalle AND m.tipo_movimiento='DEVOLUCION_VENTA')) THEN RAISE(ABORT, 'Devolucion sin retorno de stock') END;
END;

CREATE TRIGGER trg_config_ruc_usado BEFORE UPDATE ON configuracion
BEGIN
    SELECT CASE WHEN NEW.ruc IS NOT OLD.ruc AND EXISTS(SELECT 1 FROM comprobantes WHERE ambiente='PRODUCCION') THEN RAISE(ABORT, 'RUC utilizado no puede cambiar') END;
END;

CREATE TRIGGER trg_ventas_identidad_snapshot BEFORE UPDATE ON ventas
BEGIN
    SELECT CASE WHEN NEW.id_venta IS NOT OLD.id_venta OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.id_ot IS NOT OLD.id_ot OR NEW.id_cliente IS NOT OLD.id_cliente OR NEW.receptor_snapshot IS NOT OLD.receptor_snapshot OR NEW.id_emisor IS NOT OLD.id_emisor OR NEW.id_autorizante IS NOT OLD.id_autorizante OR NEW.id_caja IS NOT OLD.id_caja OR NEW.ambiente IS NOT OLD.ambiente OR NEW.tipo_original IS NOT OLD.tipo_original OR NEW.motivo_descuento IS NOT OLD.motivo_descuento OR NEW.bruto_cent IS NOT OLD.bruto_cent OR NEW.descuento_cent IS NOT OLD.descuento_cent OR NEW.base_cent IS NOT OLD.base_cent OR NEW.igv_cent IS NOT OLD.igv_cent OR NEW.total_cent IS NOT OLD.total_cent OR NEW.fecha_venta IS NOT OLD.fecha_venta THEN RAISE(ABORT, 'Identidad y valores originales inmutables') END;
END;

CREATE TRIGGER trg_devoluciones_identidad_snapshot BEFORE UPDATE ON devoluciones
BEGIN
    SELECT CASE WHEN NEW.id_devolucion IS NOT OLD.id_devolucion OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.id_venta IS NOT OLD.id_venta OR NEW.id_usuario IS NOT OLD.id_usuario OR NEW.id_autorizante IS NOT OLD.id_autorizante OR NEW.id_caja IS NOT OLD.id_caja OR NEW.id_documento_original IS NOT OLD.id_documento_original OR NEW.motivo IS NOT OLD.motivo OR NEW.base_cent IS NOT OLD.base_cent OR NEW.igv_cent IS NOT OLD.igv_cent OR NEW.total_cent IS NOT OLD.total_cent OR NEW.fecha_operacion IS NOT OLD.fecha_operacion THEN RAISE(ABORT, 'Identidad y valores originales inmutables') END;
END;

CREATE TRIGGER trg_ingresos_stock_identidad_snapshot BEFORE UPDATE ON ingresos_stock
BEGIN
    SELECT CASE WHEN NEW.id_ingreso IS NOT OLD.id_ingreso OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.codigo IS NOT OLD.codigo OR NEW.origen IS NOT OLD.origen OR NEW.id_proveedor IS NOT OLD.id_proveedor OR NEW.proveedor_snapshot IS NOT OLD.proveedor_snapshot OR NEW.tipo_documento IS NOT OLD.tipo_documento OR NEW.numero_documento IS NOT OLD.numero_documento OR NEW.fecha_documento IS NOT OLD.fecha_documento OR NEW.referencia_inicial IS NOT OLD.referencia_inicial OR NEW.total_costo_micro IS NOT OLD.total_costo_micro OR NEW.id_usuario IS NOT OLD.id_usuario OR NEW.observacion IS NOT OLD.observacion OR NEW.fecha_registro IS NOT OLD.fecha_registro THEN RAISE(ABORT, 'Identidad y valores originales inmutables') END;
END;

CREATE TRIGGER trg_caja_chica_identidad_snapshot BEFORE UPDATE ON caja_chica
BEGIN
SELECT CASE WHEN NEW.id_caja IS NOT OLD.id_caja OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.codigo_caja IS NOT OLD.codigo_caja OR NEW.id_usuario IS NOT OLD.id_usuario OR NEW.ambiente IS NOT OLD.ambiente OR NEW.monto_apertura_cent IS NOT OLD.monto_apertura_cent OR NEW.fecha_apertura IS NOT OLD.fecha_apertura THEN RAISE(ABORT, 'Identidad del turno inmutable') END;
END;

CREATE TRIGGER trg_usuario_trabajo_abierto BEFORE UPDATE ON usuarios
BEGIN
    SELECT CASE WHEN (NEW.activo=0 OR NEW.rol IS NOT OLD.rol) AND (EXISTS(SELECT 1 FROM caja_chica WHERE id_usuario=OLD.id_usuario AND estado='ABIERTA') OR EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_mecanico=OLD.id_usuario AND estado IN ('EN_ESPERA','EN_PROCESO'))) THEN RAISE(ABORT, 'Resolver caja o trabajo antes de cambiar cuenta') END;
END;

CREATE TRIGGER trg_baja_zanja_ocupada BEFORE UPDATE ON zanjas
BEGIN
    SELECT CASE WHEN NEW.activa=0 AND EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_zanja_ejecucion=OLD.id_zanja AND estado='EN_PROCESO') THEN RAISE(ABORT, 'Zanja ocupada no se deshabilita') END;
END;

CREATE TRIGGER trg_producto_baja_consumo_abierto BEFORE UPDATE ON productos
BEGIN
    SELECT CASE WHEN OLD.activo=1 AND NEW.activo=0 AND EXISTS(SELECT 1 FROM detalle_ot_productos p JOIN ordenes_trabajo o ON o.id_ot=p.id_ot WHERE p.id_producto=OLD.id_producto AND o.estado IN ('EN_ESPERA','EN_PROCESO') AND p.cantidad>(SELECT coalesce(sum(diferencia),0) FROM movimientos_inventario m WHERE m.id_consumo_ot=p.id_detalle AND m.tipo_movimiento='REPOSICION_OT')) THEN RAISE(ABORT, 'Resolver consumo en OT abierta antes de desactivar') END;
END;

CREATE TRIGGER trg_stock_directo_no_bypass BEFORE UPDATE ON productos
BEGIN
    SELECT CASE WHEN (NEW.stock_actual IS NOT OLD.stock_actual OR NEW.valor_stock_micro IS NOT OLD.valor_stock_micro) AND NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_producto=OLD.id_producto AND m.id_movimiento=(SELECT max(id_movimiento) FROM movimientos_inventario WHERE id_producto=OLD.id_producto) AND m.stock_anterior=OLD.stock_actual AND m.valor_anterior_micro=OLD.valor_stock_micro AND m.stock_resultante=NEW.stock_actual AND m.valor_resultante_micro=NEW.valor_stock_micro) THEN RAISE(ABORT, 'Actualizar saldo solo mediante kardex') END;
END;

CREATE TRIGGER trg_ot_motivos_insert BEFORE INSERT ON ordenes_trabajo
BEGIN
    SELECT CASE WHEN NEW.estado='CANCELADO' AND NEW.motivo_cancelacion IS NULL THEN RAISE(ABORT, 'Cancelacion exige motivo') END;
END;

CREATE TRIGGER trg_ot_motivos_update BEFORE UPDATE ON ordenes_trabajo
BEGIN
    SELECT CASE WHEN NEW.estado='CANCELADO' AND NEW.motivo_cancelacion IS NULL THEN RAISE(ABORT, 'Cancelacion exige motivo') END;
END;

CREATE TRIGGER trg_producto_stock_y_valor_completos BEFORE INSERT ON movimientos_inventario
BEGIN
    SELECT CASE WHEN NEW.tipo_movimiento IN ('AJUSTE_ENTRADA','AJUSTE_SALIDA','REPOSICION_OT','ANULACION_INGRESO') AND (NEW.motivo IS NULL OR length(trim(NEW.motivo)) NOT BETWEEN 5 AND 250) THEN RAISE(ABORT, 'Movimiento sensible requiere motivo') END;
END;

CREATE TRIGGER trg_vehiculo_anio_insert BEFORE INSERT ON vehiculos
BEGIN
    SELECT CASE WHEN NEW.anio IS NOT NULL AND NEW.anio>CAST(strftime('%Y','now','-5 hours') AS INTEGER)+1 THEN RAISE(ABORT, 'Anio del vehiculo posterior al permitido') END;
END;

CREATE TRIGGER trg_vehiculo_anio_update BEFORE UPDATE ON vehiculos
BEGIN
    SELECT CASE WHEN NEW.anio IS NOT NULL AND NEW.anio>CAST(strftime('%Y','now','-5 hours') AS INTEGER)+1 THEN RAISE(ABORT, 'Anio del vehiculo posterior al permitido') END;
END;

CREATE TRIGGER trg_servicio_codigo_usado BEFORE UPDATE ON servicios
BEGIN
SELECT CASE WHEN NEW.codigo IS NOT OLD.codigo AND (EXISTS(SELECT 1 FROM detalle_ot_servicios WHERE id_servicio=OLD.id_servicio) OR EXISTS(SELECT 1 FROM detalle_venta WHERE id_servicio=OLD.id_servicio)) THEN RAISE(ABORT, 'Codigo de servicio usado inmutable') END;
END;

CREATE TRIGGER trg_manual_categoria BEFORE INSERT ON movimientos_caja
BEGIN
    SELECT CASE WHEN (NEW.tipo='INGRESO_MANUAL' AND NEW.categoria NOT IN ('APORTE_FONDO','OTRO')) OR (NEW.tipo='EGRESO_MANUAL' AND NEW.categoria NOT IN ('GASTO_MENOR','RETIRO_PARCIAL','OTRO')) THEN RAISE(ABORT, 'Categoria incompatible con movimiento') END;
END;

CREATE TABLE solicitudes_operacion (
    id_operacion TEXT PRIMARY KEY CHECK(length(id_operacion) BETWEEN 16 AND 100),
    comando TEXT NOT NULL CHECK(length(comando) BETWEEN 1 AND 100),
    solicitud_sha256 TEXT NOT NULL CHECK(length(solicitud_sha256)=64 AND solicitud_sha256 NOT GLOB '*[^0-9a-f]*'),
    id_usuario INTEGER REFERENCES usuarios(id_usuario) ON DELETE RESTRICT,
    actor_snapshot TEXT NOT NULL,
    resultado_json TEXT NOT NULL CHECK(json_valid(resultado_json)),
    fecha_confirmacion TEXT NOT NULL DEFAULT (datetime('now','-5 hours')) CHECK(length(fecha_confirmacion)=19 AND datetime(fecha_confirmacion,'+0 seconds') IS fecha_confirmacion)
) STRICT;

CREATE INDEX idx_solicitudes_fecha ON solicitudes_operacion(fecha_confirmacion);

CREATE TRIGGER trg_solicitud_update BEFORE UPDATE ON solicitudes_operacion

BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Resultado idempotente inmutable') END;
END;

CREATE TRIGGER trg_solicitud_delete BEFORE DELETE ON solicitudes_operacion

BEGIN
    SELECT CASE WHEN 1 THEN RAISE(ABORT, 'Resultado idempotente inmutable') END;
END;

CREATE TRIGGER trg_ruc_clientes_insert BEFORE INSERT ON clientes

BEGIN
    SELECT CASE WHEN NEW.tipo_documento='RUC' AND NOT (length(NEW.numero_documento)=11 AND NEW.numero_documento NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.numero_documento,11,1) AS INTEGER)=((11-((CAST(substr(NEW.numero_documento,1,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,2,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,3,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,4,1) AS INTEGER)*2+CAST(substr(NEW.numero_documento,5,1) AS INTEGER)*7+CAST(substr(NEW.numero_documento,6,1) AS INTEGER)*6+CAST(substr(NEW.numero_documento,7,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,8,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,9,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_ruc_clientes_update BEFORE UPDATE ON clientes

BEGIN
    SELECT CASE WHEN NEW.tipo_documento='RUC' AND NOT (length(NEW.numero_documento)=11 AND NEW.numero_documento NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.numero_documento,11,1) AS INTEGER)=((11-((CAST(substr(NEW.numero_documento,1,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,2,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,3,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,4,1) AS INTEGER)*2+CAST(substr(NEW.numero_documento,5,1) AS INTEGER)*7+CAST(substr(NEW.numero_documento,6,1) AS INTEGER)*6+CAST(substr(NEW.numero_documento,7,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,8,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,9,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_ruc_proveedores_insert BEFORE INSERT ON proveedores

BEGIN
    SELECT CASE WHEN NEW.tipo_documento='RUC' AND NOT (length(NEW.numero_documento)=11 AND NEW.numero_documento NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.numero_documento,11,1) AS INTEGER)=((11-((CAST(substr(NEW.numero_documento,1,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,2,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,3,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,4,1) AS INTEGER)*2+CAST(substr(NEW.numero_documento,5,1) AS INTEGER)*7+CAST(substr(NEW.numero_documento,6,1) AS INTEGER)*6+CAST(substr(NEW.numero_documento,7,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,8,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,9,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_ruc_proveedores_update BEFORE UPDATE ON proveedores

BEGIN
    SELECT CASE WHEN NEW.tipo_documento='RUC' AND NOT (length(NEW.numero_documento)=11 AND NEW.numero_documento NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.numero_documento,11,1) AS INTEGER)=((11-((CAST(substr(NEW.numero_documento,1,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,2,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,3,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,4,1) AS INTEGER)*2+CAST(substr(NEW.numero_documento,5,1) AS INTEGER)*7+CAST(substr(NEW.numero_documento,6,1) AS INTEGER)*6+CAST(substr(NEW.numero_documento,7,1) AS INTEGER)*5+CAST(substr(NEW.numero_documento,8,1) AS INTEGER)*4+CAST(substr(NEW.numero_documento,9,1) AS INTEGER)*3+CAST(substr(NEW.numero_documento,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_ruc_configuracion_insert BEFORE INSERT ON configuracion

BEGIN
    SELECT CASE WHEN NEW.ruc IS NOT NULL AND NOT (length(NEW.ruc)=11 AND NEW.ruc NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.ruc,11,1) AS INTEGER)=((11-((CAST(substr(NEW.ruc,1,1) AS INTEGER)*5+CAST(substr(NEW.ruc,2,1) AS INTEGER)*4+CAST(substr(NEW.ruc,3,1) AS INTEGER)*3+CAST(substr(NEW.ruc,4,1) AS INTEGER)*2+CAST(substr(NEW.ruc,5,1) AS INTEGER)*7+CAST(substr(NEW.ruc,6,1) AS INTEGER)*6+CAST(substr(NEW.ruc,7,1) AS INTEGER)*5+CAST(substr(NEW.ruc,8,1) AS INTEGER)*4+CAST(substr(NEW.ruc,9,1) AS INTEGER)*3+CAST(substr(NEW.ruc,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_ruc_configuracion_update BEFORE UPDATE ON configuracion

BEGIN
    SELECT CASE WHEN NEW.ruc IS NOT NULL AND NOT (length(NEW.ruc)=11 AND NEW.ruc NOT GLOB '*[^0-9]*' AND CAST(substr(NEW.ruc,11,1) AS INTEGER)=((11-((CAST(substr(NEW.ruc,1,1) AS INTEGER)*5+CAST(substr(NEW.ruc,2,1) AS INTEGER)*4+CAST(substr(NEW.ruc,3,1) AS INTEGER)*3+CAST(substr(NEW.ruc,4,1) AS INTEGER)*2+CAST(substr(NEW.ruc,5,1) AS INTEGER)*7+CAST(substr(NEW.ruc,6,1) AS INTEGER)*6+CAST(substr(NEW.ruc,7,1) AS INTEGER)*5+CAST(substr(NEW.ruc,8,1) AS INTEGER)*4+CAST(substr(NEW.ruc,9,1) AS INTEGER)*3+CAST(substr(NEW.ruc,10,1) AS INTEGER)*2)%11))%10)) THEN RAISE(ABORT, 'RUC con digito de control invalido') END;
END;

CREATE TRIGGER trg_username_fijo BEFORE UPDATE ON usuarios

BEGIN
    SELECT CASE WHEN NEW.username IS NOT OLD.username THEN RAISE(ABORT, 'Login inmutable') END;
END;

CREATE TRIGGER trg_ambiente_fijo BEFORE UPDATE ON configuracion

BEGIN
    SELECT CASE WHEN NEW.ambiente_actual IS NOT OLD.ambiente_actual THEN RAISE(ABORT, 'Cambiar ambiente abriendo otra base, no editando esta fila') END;
END;

CREATE TRIGGER trg_ambiente_ventas BEFORE INSERT ON ventas

BEGIN
    SELECT CASE WHEN NEW.ambiente IS NOT (SELECT ambiente_actual FROM configuracion WHERE id_config=1) THEN RAISE(ABORT, 'Registro de otro ambiente') END;
END;

CREATE TRIGGER trg_ambiente_caja_chica BEFORE INSERT ON caja_chica

BEGIN
    SELECT CASE WHEN NEW.ambiente IS NOT (SELECT ambiente_actual FROM configuracion WHERE id_config=1) THEN RAISE(ABORT, 'Registro de otro ambiente') END;
END;

CREATE TRIGGER trg_ambiente_series_comprobante BEFORE INSERT ON series_comprobante

BEGIN
    SELECT CASE WHEN NEW.ambiente IS NOT (SELECT ambiente_actual FROM configuracion WHERE id_config=1) THEN RAISE(ABORT, 'Registro de otro ambiente') END;
END;

CREATE TRIGGER trg_ambiente_comprobantes BEFORE INSERT ON comprobantes

BEGIN
    SELECT CASE WHEN NEW.ambiente IS NOT (SELECT ambiente_actual FROM configuracion WHERE id_config=1) THEN RAISE(ABORT, 'Registro de otro ambiente') END;
END;

CREATE TRIGGER trg_ot_final_km BEFORE UPDATE ON ordenes_trabajo
BEGIN
SELECT CASE WHEN OLD.estado<>'FINALIZADO' AND NEW.estado='FINALIZADO' AND (NEW.kilometraje_salida<NEW.kilometraje_ingreso OR NEW.kilometraje_salida<(SELECT kilometraje_actual FROM vehiculos WHERE id_vehiculo=NEW.id_vehiculo)) THEN RAISE(ABORT, 'Km final menor al vigente') END;
END;

CREATE TRIGGER trg_ot_servicio_limite BEFORE INSERT ON detalle_ot_servicios

BEGIN
    SELECT CASE WHEN (SELECT count(*) FROM detalle_ot_servicios WHERE id_ot=NEW.id_ot)>=50 THEN RAISE(ABORT, 'Maximo 50 servicios vigentes') END;
END;

CREATE TRIGGER trg_vehiculo_relacion BEFORE UPDATE ON vehiculos
BEGIN
SELECT CASE WHEN (NEW.placa IS NOT OLD.placa OR NEW.id_cliente IS NOT OLD.id_cliente OR NEW.activo=0) AND EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_vehiculo=OLD.id_vehiculo AND estado IN ('EN_ESPERA','EN_PROCESO')) THEN RAISE(ABORT, 'Vehiculo con OT abierta') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM clientes WHERE id_cliente=NEW.id_cliente AND es_varios=0) THEN RAISE(ABORT, 'Destino requiere cliente identificado') END;
END;

CREATE TRIGGER trg_ingreso_fecha BEFORE INSERT ON ingresos_stock

BEGIN
    SELECT CASE WHEN NEW.fecha_documento>date('now','-5 hours') THEN RAISE(ABORT, 'Fecha documental futura') END;
END;

CREATE TRIGGER trg_stock_inicial_unico BEFORE INSERT ON detalle_ingreso

BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM ingresos_stock WHERE id_ingreso=NEW.id_ingreso AND origen='INICIAL') AND EXISTS(SELECT 1 FROM movimientos_inventario WHERE id_producto=NEW.id_producto) THEN RAISE(ABORT, 'Stock inicial solo antes del primer movimiento') END;
END;

CREATE TRIGGER trg_kardex_origen_ingreso BEFORE INSERT ON movimientos_inventario

BEGIN
    SELECT CASE WHEN NEW.tipo_movimiento IN ('STOCK_INICIAL','COMPRA') AND NOT EXISTS(SELECT 1 FROM detalle_ingreso d JOIN ingresos_stock i ON i.id_ingreso=d.id_ingreso WHERE d.id_detalle=NEW.id_detalle_ingreso AND ((NEW.tipo_movimiento='STOCK_INICIAL' AND i.origen='INICIAL') OR (NEW.tipo_movimiento='COMPRA' AND i.origen='COMPRA'))) THEN RAISE(ABORT, 'Tipo de entrada distinto del origen') END;
END;

CREATE TRIGGER trg_consumo_actor BEFORE INSERT ON detalle_ot_productos

BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios u JOIN ordenes_trabajo o ON o.id_ot=NEW.id_ot WHERE u.id_usuario=NEW.id_usuario AND u.activo=1 AND (u.rol='ADMINISTRADOR' OR (u.rol='MECANICO' AND o.id_mecanico=u.id_usuario))) THEN RAISE(ABORT, 'Consumo de OT sin actor permitido') END;
END;

CREATE TRIGGER trg_kardex_actor BEFORE INSERT ON movimientos_inventario

BEGIN
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND activo=1 AND rol IN ('ADMINISTRADOR','MECANICO','CAJERO')) THEN RAISE(ABORT, 'Actor de inventario no valido') END;
    SELECT CASE WHEN NEW.tipo_movimiento IN ('STOCK_INICIAL','COMPRA','ANULACION_INGRESO','AJUSTE_ENTRADA','AJUSTE_SALIDA','REPOSICION_OT') AND NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND rol='ADMINISTRADOR') THEN RAISE(ABORT, 'Movimiento administrativo') END;
    SELECT CASE WHEN NEW.tipo_movimiento='CONSUMO_OT' AND NOT EXISTS(SELECT 1 FROM detalle_ot_productos WHERE id_detalle=NEW.id_consumo_ot AND id_usuario=NEW.id_usuario) THEN RAISE(ABORT, 'Actor distinto del consumo') END;
    SELECT CASE WHEN NEW.tipo_movimiento='VENTA_DIRECTA' AND NOT EXISTS(SELECT 1 FROM detalle_venta l JOIN ventas v ON v.id_venta=l.id_venta WHERE l.id_detalle=NEW.id_detalle_venta AND v.id_emisor=NEW.id_usuario) THEN RAISE(ABORT, 'Actor distinto de venta') END;
    SELECT CASE WHEN NEW.tipo_movimiento='DEVOLUCION_VENTA' AND NOT EXISTS(SELECT 1 FROM detalle_devolucion l JOIN devoluciones v ON v.id_devolucion=l.id_devolucion WHERE l.id_detalle=NEW.id_detalle_devolucion AND v.id_usuario=NEW.id_usuario) THEN RAISE(ABORT, 'Actor distinto de devolucion') END;
END;

CREATE TRIGGER trg_servicio_ot_venta_snapshot BEFORE INSERT ON detalle_venta

BEGIN
    SELECT CASE WHEN NEW.id_servicio_ot IS NOT NULL AND NOT EXISTS(SELECT 1 FROM detalle_ot_servicios s WHERE s.id_detalle=NEW.id_servicio_ot AND s.cantidad=NEW.cantidad AND s.precio_unitario_cent=NEW.precio_unitario_cent) THEN RAISE(ABORT, 'Servicio OT conserva cantidad y precio') END;
END;

CREATE TRIGGER trg_ot_venta_completa BEFORE UPDATE ON ventas
WHEN OLD.estado_registro='PREPARANDO' AND NEW.estado_registro='CONFIRMADA' AND NEW.id_ot IS NOT NULL
BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM detalle_ot_servicios s WHERE s.id_ot=NEW.id_ot AND NOT EXISTS(SELECT 1 FROM detalle_venta l WHERE l.id_venta=NEW.id_venta AND l.id_servicio_ot=s.id_detalle)) OR EXISTS(SELECT 1 FROM detalle_ot_productos p WHERE p.id_ot=NEW.id_ot AND p.cantidad>(SELECT coalesce(sum(diferencia),0) FROM movimientos_inventario WHERE id_consumo_ot=p.id_detalle AND tipo_movimiento='REPOSICION_OT') AND NOT EXISTS(SELECT 1 FROM detalle_venta l WHERE l.id_venta=NEW.id_venta AND l.id_consumo_ot=p.id_detalle)) THEN RAISE(ABORT, 'Faltan lineas de OT en venta') END;
END;

CREATE TRIGGER trg_devolucion_original_vigente BEFORE INSERT ON devoluciones

BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM comprobantes WHERE id_documento_anterior=NEW.id_documento_original) THEN RAISE(ABORT, 'Usar documento vigente tras reemision') END;
END;

CREATE TRIGGER trg_devolucion_autorizante BEFORE INSERT ON devoluciones

BEGIN
    SELECT CASE WHEN EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario AND rol='CAJERO') AND (NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_autorizante AND rol='ADMINISTRADOR' AND activo=1 AND debe_cambiar_clave=0 AND (bloqueado_hasta IS NULL OR bloqueado_hasta<=datetime('now','-5 hours'))) OR NEW.id_usuario IS NOT (SELECT id_emisor FROM ventas WHERE id_venta=NEW.id_venta)) THEN RAISE(ABORT, 'Cajero requiere venta propia y autorizante administrativo') END;
END;

CREATE TRIGGER trg_devolucion_documento_resuelto BEFORE INSERT ON devoluciones
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM comprobantes WHERE id_comprobante=NEW.id_documento_original AND estado_envio='ACEPTADO' AND requiere_consulta=0 AND resultado_incierto=0) THEN RAISE(ABORT, 'Original debe estar aceptado') END;
    SELECT CASE WHEN EXISTS(SELECT 1 FROM comprobantes n WHERE n.id_venta=NEW.id_venta AND n.relacion IN ('NOTA_CREDITO','REEMISION_NC') AND NOT EXISTS(SELECT 1 FROM comprobantes r WHERE r.id_documento_anterior=n.id_comprobante) AND n.estado_envio<>'ACEPTADO') THEN RAISE(ABORT, 'Aceptar notas anteriores antes de devolver') END;
END;

CREATE TRIGGER trg_comprobante_reglas BEFORE INSERT ON comprobantes

BEGIN
    SELECT CASE WHEN NEW.estado_envio<>'PENDIENTE' OR NEW.intento_iniciado<>0 OR NEW.requiere_consulta<>0 OR NEW.resultado_incierto<>0 THEN RAISE(ABORT, 'Documento nace pendiente sin envio') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ventas WHERE id_venta=NEW.id_venta AND estado_registro='CONFIRMADA' AND ambiente=NEW.ambiente) THEN RAISE(ABORT, 'Documento requiere venta confirmada') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM series_comprobante WHERE id_serie=NEW.id_serie AND activa=1 AND requiere_conciliacion=0 AND correlativo_actual=NEW.correlativo AND correlativo_actual>=max(mayor_previo,ultimo_real_comprobado)) THEN RAISE(ABORT, 'Serie no activa conciliada o numero no reservado') END;
    SELECT CASE WHEN NEW.relacion IN ('ORIGINAL','REEMISION') AND NOT EXISTS(SELECT 1 FROM ventas v WHERE v.id_venta=NEW.id_venta AND v.total_cent=NEW.total_cent AND v.base_cent=NEW.base_cent AND v.igv_cent=NEW.igv_cent) THEN RAISE(ABORT, 'Documento cambia importes de venta') END;
    SELECT CASE WHEN NEW.relacion IN ('REEMISION','REEMISION_NC') AND NOT EXISTS(SELECT 1 FROM comprobantes a WHERE a.id_comprobante=NEW.id_documento_anterior AND a.id_venta=NEW.id_venta AND a.tipo_comprobante=NEW.tipo_comprobante AND a.estado_envio='RECHAZADO' AND a.resultado_incierto=0 AND ((NEW.relacion='REEMISION' AND a.relacion IN ('ORIGINAL','REEMISION') AND NOT EXISTS(SELECT 1 FROM devoluciones WHERE id_venta=NEW.id_venta AND estado_registro='CONFIRMADA')) OR (NEW.relacion='REEMISION_NC' AND a.id_devolucion=NEW.id_devolucion AND a.relacion IN ('NOTA_CREDITO','REEMISION_NC')))) THEN RAISE(ABORT, 'Reemision sin rechazo o venta ya revertida') END;
    SELECT CASE WHEN NEW.relacion IN ('NOTA_CREDITO','REEMISION_NC') AND NOT EXISTS(SELECT 1 FROM devoluciones d JOIN comprobantes a ON a.id_comprobante=d.id_documento_original WHERE d.id_devolucion=NEW.id_devolucion AND d.estado_registro='CONFIRMADA' AND d.id_venta=NEW.id_venta AND a.id_comprobante=NEW.id_documento_referenciado AND d.total_cent=NEW.total_cent AND d.base_cent=NEW.base_cent AND d.igv_cent=NEW.igv_cent AND ((a.tipo_comprobante='BOLETA' AND NEW.tipo_comprobante='NC_BOLETA') OR (a.tipo_comprobante='FACTURA' AND NEW.tipo_comprobante='NC_FACTURA'))) THEN RAISE(ABORT, 'Nota no corresponde a devolucion') END;
END;

CREATE TRIGGER trg_serie_no_retroceso_maximos BEFORE UPDATE ON series_comprobante

BEGIN
    SELECT CASE WHEN NEW.mayor_previo<OLD.mayor_previo OR NEW.ultimo_real_comprobado<OLD.ultimo_real_comprobado THEN RAISE(ABORT, 'Maximos conocidos no disminuyen') END;
END;

CREATE TRIGGER trg_recuperacion_caja_chica BEFORE INSERT ON caja_chica

BEGIN
    SELECT CASE WHEN (SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1)=1 THEN RAISE(ABORT, 'Recuperacion pendiente bloquea nuevas operaciones') END;
END;

CREATE TRIGGER trg_recuperacion_ordenes_trabajo BEFORE INSERT ON ordenes_trabajo

BEGIN
    SELECT CASE WHEN (SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1)=1 THEN RAISE(ABORT, 'Recuperacion pendiente bloquea nuevas operaciones') END;
END;

CREATE TRIGGER trg_recuperacion_ingresos_stock BEFORE INSERT ON ingresos_stock

BEGIN
    SELECT CASE WHEN (SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1)=1 THEN RAISE(ABORT, 'Recuperacion pendiente bloquea nuevas operaciones') END;
END;

CREATE TRIGGER trg_recuperacion_movimientos_inventario BEFORE INSERT ON movimientos_inventario

BEGIN
    SELECT CASE WHEN (SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1)=1 THEN RAISE(ABORT, 'Recuperacion pendiente bloquea nuevas operaciones') END;
END;

CREATE INDEX idx_ot_fecha_ingreso ON ordenes_trabajo(fecha_ingreso,id_ot);

CREATE INDEX idx_documento_tipo_fecha ON comprobantes(tipo_comprobante,fecha_emision);

CREATE INDEX idx_auditoria_restauracion ON auditoria(entidad,id_entidad);

CREATE TRIGGER trg_servicio_ot_editar BEFORE UPDATE ON detalle_ot_servicios
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=OLD.id_ot AND estado IN ('EN_ESPERA','EN_PROCESO')) THEN RAISE(ABORT, 'OT terminal no editable') END;
    SELECT CASE WHEN NEW.id_detalle IS NOT OLD.id_detalle OR NEW.id_ot IS NOT OLD.id_ot OR NEW.id_servicio IS NOT OLD.id_servicio OR NEW.codigo_snapshot IS NOT OLD.codigo_snapshot OR NEW.nombre_snapshot IS NOT OLD.nombre_snapshot OR NEW.precio_unitario_cent IS NOT OLD.precio_unitario_cent OR NEW.es_cambio_aceite IS NOT OLD.es_cambio_aceite OR NEW.fecha_registro IS NOT OLD.fecha_registro OR NEW.id_operacion IS NOT OLD.id_operacion THEN RAISE(ABORT, 'Servicio conserva origen y precio') END;
    SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios u JOIN ordenes_trabajo o ON o.id_ot=NEW.id_ot WHERE u.id_usuario=NEW.id_usuario AND u.activo=1 AND (u.rol='ADMINISTRADOR' OR (u.rol='MECANICO' AND u.id_usuario=o.id_mecanico))) THEN RAISE(ABORT, 'Edicion de servicio sin actor permitido') END;
END;

CREATE TRIGGER trg_servicio_ot_retirar BEFORE DELETE ON detalle_ot_servicios
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=OLD.id_ot AND estado IN ('EN_ESPERA','EN_PROCESO')) OR (SELECT count(*) FROM detalle_ot_servicios WHERE id_ot=OLD.id_ot)<=1 THEN RAISE(ABORT, 'Conservar al menos un servicio; terminal inmutable') END;
END;

CREATE TRIGGER trg_medios_limitados BEFORE INSERT ON movimientos_caja
BEGIN
SELECT CASE WHEN NEW.tipo='COBRO' AND (SELECT count(*) FROM movimientos_caja WHERE id_venta=NEW.id_venta AND tipo='COBRO')>=2 THEN RAISE(ABORT, 'Maximo dos medios por venta') END;
    SELECT CASE WHEN NEW.tipo='REINTEGRO' AND EXISTS(SELECT 1 FROM movimientos_caja WHERE id_devolucion=NEW.id_devolucion AND tipo='REINTEGRO') THEN RAISE(ABORT, 'Un medio por devolucion') END;
END;

CREATE TRIGGER trg_caja_aclaracion BEFORE UPDATE ON caja_chica
WHEN NEW.aclaracion IS NOT OLD.aclaracion
BEGIN
SELECT CASE WHEN NEW.estado<>'CERRADA' OR NEW.id_usuario_aclaracion IS NULL OR NEW.fecha_aclaracion IS NULL OR NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario_aclaracion AND activo=1 AND rol='ADMINISTRADOR') THEN RAISE(ABORT, 'Aclaracion requiere cierre y administrador') END;
END;

CREATE TRIGGER trg_ingreso_anular_ultimo BEFORE INSERT ON movimientos_inventario
BEGIN
SELECT CASE WHEN NEW.tipo_movimiento='ANULACION_INGRESO' AND EXISTS(SELECT 1 FROM detalle_ingreso d JOIN detalle_ingreso origen ON origen.id_ingreso=d.id_ingreso JOIN movimientos_inventario entrada ON entrada.id_detalle_ingreso=origen.id_detalle AND entrada.tipo_movimiento IN ('STOCK_INICIAL','COMPRA') JOIN movimientos_inventario posterior ON posterior.id_producto=origen.id_producto AND posterior.id_movimiento>entrada.id_movimiento WHERE d.id_detalle=NEW.id_detalle_ingreso AND NOT (posterior.tipo_movimiento='ANULACION_INGRESO' AND posterior.id_detalle_ingreso=origen.id_detalle)) THEN RAISE(ABORT, 'Entrada con movimientos posteriores: no anulable') END;
END;

CREATE TRIGGER trg_servicio_ot_actor_insert BEFORE INSERT ON detalle_ot_servicios
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios u JOIN ordenes_trabajo o ON o.id_ot=NEW.id_ot WHERE u.id_usuario=NEW.id_usuario AND u.activo=1 AND (u.rol='ADMINISTRADOR' OR (u.rol='MECANICO' AND u.id_usuario=o.id_mecanico) OR (u.rol='CAJERO' AND o.id_creador=u.id_usuario AND o.estado='EN_ESPERA'))) THEN RAISE(ABORT, 'Servicio OT sin actor autorizado') END;
END;

CREATE TRIGGER trg_venta_porcentaje_fijo BEFORE UPDATE ON ventas
BEGIN
SELECT CASE WHEN NEW.descuento_bp IS NOT OLD.descuento_bp THEN RAISE(ABORT, 'Porcentaje de venta inmutable') END;
END;

CREATE TRIGGER trg_venta_confirmar_medios BEFORE UPDATE ON ventas
WHEN OLD.estado_registro='PREPARANDO' AND NEW.estado_registro='CONFIRMADA'
BEGIN
SELECT CASE WHEN (SELECT count(*) FROM movimientos_caja WHERE id_venta=NEW.id_venta AND tipo='COBRO') NOT BETWEEN 1 AND 2 THEN RAISE(ABORT, 'Venta exige uno o dos medios') END;
END;

CREATE TRIGGER trg_devolucion_un_reintegro BEFORE UPDATE ON devoluciones
WHEN OLD.estado_registro='PREPARANDO' AND NEW.estado_registro='CONFIRMADA'
BEGIN
SELECT CASE WHEN (SELECT count(*) FROM movimientos_caja WHERE id_devolucion=NEW.id_devolucion AND tipo='REINTEGRO')<>1 THEN RAISE(ABORT, 'Devolucion exige un reintegro') END;
END;

CREATE TRIGGER trg_costo_ingreso_captura BEFORE INSERT ON detalle_ingreso
BEGIN
SELECT CASE WHEN NEW.costo_efectivo_micro%10000<>0 OR NEW.costo_efectivo_micro>999999990000 THEN RAISE(ABORT, 'Costo capturado admite dos decimales y limite comercial') END;
    SELECT CASE WHEN (NEW.unidad_medida='UND' AND (NEW.cantidad>9000000000000000/NEW.costo_efectivo_micro OR NEW.valor_aporte_micro<>NEW.cantidad*NEW.costo_efectivo_micro)) OR (NEW.unidad_medida='ML' AND (NEW.cantidad>(NEW.valor_aporte_micro*1000+499)/NEW.costo_efectivo_micro OR NEW.cantidad<(NEW.valor_aporte_micro*1000-500+NEW.costo_efectivo_micro-1)/NEW.costo_efectivo_micro)) THEN RAISE(ABORT, 'Valor de entrada incompatible con cantidad y costo') END;
END;

CREATE TRIGGER trg_caja_nueva_sin_cierre BEFORE INSERT ON caja_chica
BEGIN
SELECT CASE WHEN NEW.fecha_cierre IS NOT NULL OR NEW.tipo_cierre IS NOT NULL OR NEW.efectivo_contado_cent IS NOT NULL OR NEW.efectivo_esperado_cent IS NOT NULL OR NEW.aclaracion IS NOT NULL OR NEW.por_conciliar<>0 THEN RAISE(ABORT, 'Apertura sin campos de cierre o recuperacion') END;
END;

CREATE TRIGGER trg_serie_maximos_actuales BEFORE UPDATE ON series_comprobante
BEGIN
SELECT CASE WHEN NEW.requiere_conciliacion=0 AND NEW.correlativo_actual<max(NEW.mayor_previo,NEW.ultimo_real_comprobado) THEN RAISE(ABORT, 'Correlativo menor al maximo conocido') END;
END;

CREATE TRIGGER trg_comprobante_tipo_venta BEFORE INSERT ON comprobantes
BEGIN
SELECT CASE WHEN NEW.relacion IN ('ORIGINAL','REEMISION') AND NEW.tipo_comprobante IS NOT (SELECT tipo_original FROM ventas WHERE id_venta=NEW.id_venta) THEN RAISE(ABORT, 'Tipo documental de venta inmutable') END;
END;

CREATE TRIGGER trg_devolucion_recuperacion BEFORE INSERT ON devoluciones
BEGIN
SELECT CASE WHEN EXISTS(SELECT 1 FROM configuracion WHERE recuperacion_pendiente=1 OR revision_caja_pendiente=1 OR revision_token_pendiente=1 OR revision_series_pendiente=1) THEN RAISE(ABORT, 'Completar recuperacion antes de devolver') END;
END;

CREATE TRIGGER trg_ot_cancelacion_admin BEFORE UPDATE ON ordenes_trabajo
WHEN NEW.estado='CANCELADO' AND OLD.estado<>'CANCELADO'
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario_cancelacion AND activo=1 AND rol='ADMINISTRADOR') THEN RAISE(ABORT, 'Cancelacion requiere administrador') END;
END;

CREATE TRIGGER trg_ingreso_anulacion_admin BEFORE UPDATE ON ingresos_stock
WHEN NEW.estado='ANULADO' AND OLD.estado<>'ANULADO'
BEGIN
SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=NEW.id_usuario_anulacion AND activo=1 AND rol='ADMINISTRADOR') THEN RAISE(ABORT, 'Anulacion requiere administrador') END;
END;

CREATE INDEX idx_documento_pendiente ON comprobantes(estado_envio,fecha_emision,id_comprobante);

CREATE INDEX idx_ot_ultimo_aceite ON ordenes_trabajo(id_vehiculo,fecha_fin DESC,id_ot DESC) WHERE estado='FINALIZADO' AND tipo_aceite IS NOT NULL;

CREATE INDEX idx_kardex_producto_ultimo ON movimientos_inventario(id_producto,id_movimiento DESC);

CREATE INDEX idx_servicio_catalogo ON servicios(activo,nombre,codigo);


CREATE VIEW v_historial_vigente AS
SELECT o.id_ot AS id_historial,o.id_ot,o.id_vehiculo,o.id_cliente_atendido,o.cliente_snapshot,o.placa_snapshot,
 o.fecha_fin AS fecha_servicio,o.kilometraje_salida,o.resumen_publico,o.tipo_aceite,o.intervalo_aceite_km,o.km_proximo,o.corregido,
 CASE WHEN o.tipo_aceite IS NULL THEN 0 ELSE 1 END AS es_cambio_aceite,
 o.id_mecanico_terminal,o.id_zanja_ejecucion
FROM ordenes_trabajo o WHERE o.estado='FINALIZADO';

CREATE VIEW v_proximos_cambios AS
SELECT v.id_vehiculo,v.placa,v.kilometraje_actual,c.id_cliente,c.nombre_razon_social,c.telefono,
 h.id_ot,h.fecha_servicio,h.tipo_aceite,h.intervalo_aceite_km,h.km_proximo,
 CASE WHEN v.kilometraje_actual>=h.km_proximo-500 AND date('now','-5 hours')>=date(h.fecha_servicio,'+180 days') THEN 'AMBOS'
 WHEN v.kilometraje_actual>=h.km_proximo-500 THEN 'POR_KM' ELSE 'POR_FECHA' END AS causa
FROM vehiculos v JOIN clientes c ON c.id_cliente=v.id_cliente JOIN v_historial_vigente h ON h.id_ot=(
 SELECT o.id_ot FROM ordenes_trabajo o WHERE o.id_vehiculo=v.id_vehiculo AND o.estado='FINALIZADO' AND o.tipo_aceite IS NOT NULL ORDER BY o.fecha_fin DESC,o.id_ot DESC LIMIT 1)
WHERE v.activo=1 AND (v.kilometraje_actual>=h.km_proximo-500 OR date('now','-5 hours')>=date(h.fecha_servicio,'+180 days'));

CREATE VIEW v_operaciones_comerciales AS
SELECT v.id_venta,NULL AS id_devolucion,v.ambiente,v.fecha_venta AS fecha_operacion,v.id_emisor AS id_usuario,
 'VENTA' AS tipo,v.tipo_original,v.bruto_cent,v.descuento_cent,v.base_cent,v.igv_cent,v.total_cent
FROM ventas v WHERE v.estado_registro='CONFIRMADA'
UNION ALL
SELECT v.id_venta,d.id_devolucion,v.ambiente,d.fecha_operacion,d.id_usuario,'DEVOLUCION',v.tipo_original,
 0,0,-d.base_cent,-d.igv_cent,-d.total_cent
FROM devoluciones d JOIN ventas v ON v.id_venta=d.id_venta WHERE d.estado_registro='CONFIRMADA';

CREATE VIEW v_estado_comercial_venta AS
SELECT v.id_venta,CASE
 WHEN coalesce((SELECT sum(d.total_cent) FROM devoluciones d WHERE d.id_venta=v.id_venta AND d.estado_registro='CONFIRMADA'),0)=v.total_cent THEN 'DEVOLUCION_TOTAL'
 WHEN EXISTS(SELECT 1 FROM devoluciones d WHERE d.id_venta=v.id_venta AND d.estado_registro='CONFIRMADA') THEN 'DEVOLUCION_PARCIAL'
 ELSE 'VIGENTE' END AS situacion FROM ventas v WHERE v.estado_registro='CONFIRMADA';

CREATE VIEW v_caja_diferencias AS
SELECT id_caja,efectivo_esperado_cent,efectivo_contado_cent,diferencia_efectivo_cent,
 tarjeta_neto_cent,yape_neto_cent,plin_neto_cent,transferencia_neto_cent,motivo_cierre,aclaracion
FROM caja_chica WHERE estado='CERRADA';

CREATE VIEW v_productos_criticos AS
SELECT id_producto,codigo,nombre,categoria,unidad_medida,stock_actual,stock_minimo,
 max(stock_minimo-stock_actual,0) AS faltante FROM productos WHERE activo=1 AND stock_actual<=stock_minimo;

CREATE TRIGGER trg_ot_identidad_fija BEFORE UPDATE ON ordenes_trabajo BEGIN
    SELECT CASE WHEN NEW.id_ot IS NOT OLD.id_ot OR NEW.id_operacion IS NOT OLD.id_operacion OR NEW.codigo_ot IS NOT OLD.codigo_ot OR NEW.id_vehiculo IS NOT OLD.id_vehiculo OR NEW.id_cliente_atendido IS NOT OLD.id_cliente_atendido OR NEW.placa_snapshot IS NOT OLD.placa_snapshot OR NEW.cliente_snapshot IS NOT OLD.cliente_snapshot OR NEW.id_creador IS NOT OLD.id_creador OR NEW.kilometraje_ingreso IS NOT OLD.kilometraje_ingreso OR NEW.pedido_cliente IS NOT OLD.pedido_cliente OR NEW.fecha_ingreso IS NOT OLD.fecha_ingreso THEN RAISE(ABORT, 'Recepcion historica inmutable') END;
END;

-- CONTRATOS QUE EXIGEN SERVICIO RUST (no los sustituye este DDL):
-- * Motivo 5..250 + antes/después en corrección pública, placa/propietario/km menor,
--   reasignación, estado de catálogos/cuentas; cambios ordinarios no piden motivo.
-- * Corregir atención antigua no actualiza km_actual. Finalizar sí lo actualiza.
-- * Registrar al menos un servicio en el mismo comando de creación OT; retirar
--   servicio guarda auditoría y no borra una línea ya facturada/terminal.
-- * Reparto determinista de céntimos de descuento/impuesto; devolución proporcional
--   al original con residuales en última cantidad; retornos al costo original.
-- * Cada saldo final debe coincidir con último kardex ANTES del COMMIT.
-- * Venta confirmada debe tener original; devolución confirmada su nota, creados
--   en esa misma transacción después de confirmar internamente las cabeceras.
-- * Clientes varios solo boleta <=S/700; factura exige RUC/nombre/dirección y
--   validación del payload oficial. RUC con dígito correcto no prueba estado SUNAT.
-- * Backend nunca entrega costos/notas privadas/contactos/secretos a rol indebido.
-- * Cambios de sesión no cierran caja; cierre no genera otro egreso por retiro.
-- * Permisos/actores de servicios: SQL permite recepción del creador cajero;
--   Rust restringe esa inserción al comando original, no a una edición posterior.
-- * Solicitudes idempotentes+auditoría se escriben con el negocio; no secretos.
-- * Al restaurar mantener journal/máximos fuera de BD y marcar documentos inciertos.
-- * No registrar lecturas/filtros normales en auditoría; sí exportaciones y cambios.
PRAGMA user_version = 700;

-- ============================================================================
-- RESUMEN EXACTO DE LA REVISION V6.1 -> V7.0
-- Adjunto revisado: 37 tablas de aplicación (no 38). Resultado: 27.
-- 10 tablas retiradas del NUEVO esquema; 0 tablas nuevas. Sin borrar datos reales.
-- proformas / detalle_proforma: funcionalidad eliminada del POS.
-- historial_vehicular: sustituida por vista v_historial_vigente sobre OT finalizadas.
-- correcciones_historial: valores vigentes en OT; anterior/nuevo en auditoria.
-- seguimientos_cambio_aceite: no hay registro de llamadas ni ocultamiento 30 dias.
-- notas_ot: campo único nota_interna en ordenes_trabajo.
-- cierre_caja_medios: un contado de efectivo y resúmenes digitales en caja_chica.
-- revisiones_caja: sin aprobación posterior; aclaracion editable y auditoria común.
-- cola_envio_sunat: estado, contenido y resultado en comprobantes.
-- intentos_envio: transiciones/acciones en auditoria; sin agenda de reintentos.
--
-- COLUMNAS NUEVAS/RETIRADAS (también se adaptaron CHECK, triggers y vistas):
-- backups:
--   Retiradas: nota.
-- caja_chica:
--   Retiradas: observacion_apertura, observacion_cierre, revision, umbral_revision_cent.
--   Incorporadas: aclaracion, diferencia_efectivo_cent, efectivo_contado_cent, efectivo_esperado_cent, fecha_aclaracion, id_usuario_aclaracion, plin_neto_cent, tarjeta_neto_cent, transferencia_neto_cent, yape_neto_cent.
-- clientes:
--   Retiradas: activo.
-- comprobantes:
--   Incorporadas: advertencias_json, codigo_respuesta, estado_envio, fecha_confirmacion, fecha_ultima_consulta, fecha_ultimo_intento, intento_iniciado, requiere_consulta, respuesta_resumida, resultado_incierto, ruta_cdr, ruta_pdf, ruta_xml, version.
-- configuracion:
--   Retiradas: consulta_conocida_json, destino_backup_externo, envio_pausado, fecha_consulta_nubefact, umbral_diferencia_cent, umbral_egreso_cent.
--   Incorporadas: retencion_backups.
-- detalle_ot_servicios:
--   Retiradas: motivo, reemplaza_id, vigente.
-- devoluciones:
--   Retiradas: tipo, via_documental.
-- ingresos_stock:
--   Retiradas: motivo.
-- movimientos_caja:
--   Retiradas: umbral_motivo_cent.
-- ordenes_trabajo:
--   Retiradas: id_zanja_prevista.
--   Incorporadas: corregido, intervalo_aceite_km, km_proximo, nota_interna.
-- proveedores:
--   Retiradas: activo.
-- restauraciones:
--   Retiradas: id_backup_retorno, motivo.
--   Incorporadas: id_backup_previa.
-- ventas:
--   Retiradas: id_proforma.
--   Incorporadas: descuento_bp.
-- Se conservan conciliaciones_series y restauraciones para evidencias de
-- numeración/recuperación; no son nuevas pantallas ni una agenda de aprobación.
-- descuento_bp: 100 puntos básicos = 1%; 500=5%; 9999=99,99%.
-- id_backup_retorno fue renombrado a id_backup_previa.
-- Cambios de estados: caja NORMAL/AUSENCIA/RECUPERACION; backup RETORNO -> PREVIA,
-- externo NO_SOLICITADO/COPIADO/FALLIDO; comprobante PENDIENTE/ENVIANDO/ERROR/
-- ACEPTADO/RECHAZADO. Eliminar tipo/via_documental de devolución evita la vía de
-- anulación local; devolución parcial/total se calcula a partir de sus importes.
--
-- TABLAS FINALES (27):
-- 01. auditoria
-- 02. backups
-- 03. caja_chica
-- 04. clientes
-- 05. comprobantes
-- 06. conciliaciones_series
-- 07. configuracion
-- 08. detalle_devolucion
-- 09. detalle_ingreso
-- 10. detalle_ot_productos
-- 11. detalle_ot_servicios
-- 12. detalle_venta
-- 13. devoluciones
-- 14. ingresos_stock
-- 15. movimientos_caja
-- 16. movimientos_inventario
-- 17. ordenes_trabajo
-- 18. productos
-- 19. proveedores
-- 20. restauraciones
-- 21. series_comprobante
-- 22. servicios
-- 23. solicitudes_operacion
-- 24. usuarios
-- 25. vehiculos
-- 26. ventas
-- 27. zanjas
--
-- VERIFICACIÓN REALIZADA EN SQLITE DE PRUEBA (15-10-2026):
-- 27 casos reproducibles aprobados; creación en memoria y compilación de todos
-- los INSERT/UPDATE/DELETE y consultas de las 6 vistas; integrity_check=ok y
-- foreign_key_check sin errores. Casos: alta inicial y rollback, entrada sin
-- documento, anulación simple/multiproducto y bloqueo tras movimiento compensado,
-- recepción asignada/libre, servicios editables, corrección pública, OT->POS sin
-- segundo stock, venta mixta/tercer medio, descuento 5%, devoluciones 40+60,
-- una vía/medio de reintegro, documentos inciertos y reemisión sin segundo cobro,
-- cierre efectivo/diferencia S/0,01, gasto S/50,01, turno recuperado, catálogo y
-- formatos/tipos, reloj Lima con TZ Asia/Tokyo y cruce de medianoche UTC/Lima.
-- No equivalen a pruebas del código Tauri/Rust/React ni de la API NubeFact real.
-- La integración externa, seguridad de sesión y archivos requieren implementación
-- y sus pruebas en el proyecto. Este archivo no modifica la BD ya instalada.

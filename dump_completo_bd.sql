-- DUMP COMPLETO DE BASE DE DATOS LUBRICENTRO

-- ==========================================
-- TABLA: usuarios (3 registros)
-- ==========================================
INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES (2, 'Carlos Administrador', 'admin', '$2b$12$qIeUtZIP8F9Lav885SclpOqq.R1WzOOrPVnut2kVA8Q8CACq84F8W', 'ADMINISTRADOR', 1);
INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES (3, 'Juan Mecánico', 'mecanico', '$2b$12$qIeUtZIP8F9Lav885SclpOqq.R1WzOOrPVnut2kVA8Q8CACq84F8W', 'MECANICO', 1);
INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) VALUES (4, 'Maria Cajera', 'cajero', '$2b$12$qIeUtZIP8F9Lav885SclpOqq.R1WzOOrPVnut2kVA8Q8CACq84F8W', 'CAJERO', 1);

-- ==========================================
-- TABLA: permisos_rol (18 registros)
-- ==========================================
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (1, 'ADMINISTRADOR', 'FICHA_VEHICULAR', 1, 1, 1, 1);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (2, 'MECANICO', 'FICHA_VEHICULAR', 1, 0, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (3, 'CAJERO', 'FICHA_VEHICULAR', 1, 1, 1, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (4, 'ADMINISTRADOR', 'OT', 1, 1, 1, 1);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (5, 'MECANICO', 'OT', 1, 0, 1, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (6, 'CAJERO', 'OT', 1, 1, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (7, 'ADMINISTRADOR', 'INVENTARIO', 1, 1, 1, 1);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (8, 'MECANICO', 'INVENTARIO', 1, 0, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (9, 'CAJERO', 'INVENTARIO', 1, 0, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (10, 'ADMINISTRADOR', 'POS', 1, 1, 1, 1);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (11, 'CAJERO', 'POS', 1, 1, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (12, 'ADMINISTRADOR', 'CAJA', 1, 1, 1, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (13, 'CAJERO', 'CAJA', 1, 1, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (14, 'ADMINISTRADOR', 'FIDELIZACION', 1, 0, 1, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (15, 'CAJERO', 'FIDELIZACION', 1, 0, 1, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (16, 'ADMINISTRADOR', 'COLA_SUNAT', 1, 0, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (17, 'CAJERO', 'COLA_SUNAT', 1, 0, 0, 0);
INSERT INTO permisos_rol (id_permiso, rol, modulo, puede_ver, puede_crear, puede_editar, puede_eliminar) VALUES (18, 'ADMINISTRADOR', 'BACKUP', 1, 1, 0, 1);

-- ==========================================
-- TABLA: proveedores (20 registros)
-- ==========================================
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (1, '20100053451', 'PRIMAX S.A.', '012117800', 'Gerson Torres', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (2, '20100132831', 'DISTRIBUIDORA DERCO PERU S.A.', '017135000', 'Lucía Mendoza', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (3, '20504892301', 'SOCIEDAD DE COMERCIO EXTERIOR LUBRICANTES PERU S.A.C.', '014221190', 'Carlos Ramos', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (4, '20100051246', 'REPSOL COMERCIAL SAC', '012158000', 'Mariana Chavez', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (5, '20305829101', 'IMPORTADORA MOBIL PERU S.A.C.', '013192000', 'Jorge Alarcón', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (6, '20491823091', 'DISTRIBUIDORA VALVOLINE DEL PERU S.A.', '014412233', 'Hernán Silva', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (7, '20512938401', 'FILTROS LYS S.A.', '015132000', 'Patricia Quispe', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (8, '20101928374', 'BOSCH PERU S.A.', '012085000', 'Fernando Castro', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (9, '20601928301', 'DISTRIBUIDORA MOTUL PERU E.I.R.L.', '014567890', 'Diego Morales', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (10, '20521092831', 'LIQUI MOLY PERU S.A.C.', '012219988', 'Andrés Delgado', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (11, '20401928371', 'TOTALENERGIES MARKETING PERU S.A.U.', '016149000', 'Sofia Benavides', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (12, '20519283011', 'DISTRIBUIDORA DE REPUESTOS JAPAN S.A.C.', '014271010', 'Manuel Rojas', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (13, '20349281029', 'FILTROS MANN FILTER PERU S.A.', '013451122', 'Rocío Huamán', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (14, '20502918302', 'LUBRICANTES Y ADITIVOS CASTROL PERU S.A.C.', '012349000', 'Gonzalo Vargas', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (15, '20603928104', 'COMMERCE AUTO PARTS SUR E.I.R.L.', '056231144', 'Víctor Chumpitaz', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (16, '20459281039', 'PRESTONE AUTOMOTIVE PERU S.A.', '014529900', 'Claudia Paredes', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (17, '20582910293', 'IMPORTADORA DE LUBRICANTES DEL SUR S.A.C.', '056502010', 'Efraín Yáñez', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (18, '20192830192', 'TOYOTA DEL PERU S.A.', '016173000', 'Alberto Coronado', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (19, '20381920391', 'NISSAN PERU S.A.', '016148000', 'Renato Cabrera', 1);
INSERT INTO proveedores (id_proveedor, ruc, razon_social, telefono, contacto_asesor, activo) VALUES (20, '20510293841', 'REPUESTOS Y FILTROS GENERALES ICA S.A.C.', '056215588', 'Sonia Palomino', 1);

-- ==========================================
-- TABLA: categorias (15 registros)
-- ==========================================
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (1, 'Aceites Sintéticos', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (2, 'Aceites Semi-Sintéticos', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (3, 'Aceites Minerales', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (4, 'Aceites Diésel y Gran Tonelaje', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (5, 'Aceites para Motores 2T y 4T', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (6, 'Filtros de Aceite', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (7, 'Filtros de Aire de Motor', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (8, 'Filtros de Aire Acondicionado / Cabina', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (9, 'Filtros de Combustible / Trampas Diésel', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (10, 'Refrigerantes y Enfriamiento (Coolant)', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (11, 'Fluidos de Transmisión Manual y Automática (ATF/CVT)', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (12, 'Aceites de Transmisión y Diferencial / Corona (75W90 / 80W90)', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (13, 'Líquidos de Frenos (DOT3 / DOT4)', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (14, 'Aditivos, Limpiadores y Tratamientos', 1);
INSERT INTO categorias (id_categoria, nombre_categoria, activo) VALUES (15, 'Grasas Chasis, Multipropósito y Mantenimiento', 1);

-- ==========================================
-- TABLA: productos (125 registros)
-- ==========================================
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (1, 1, '775000100001', 'Aceite Shell Helix Ultra 5W-30 Sintético (1 Galón)', 'GALON', 25, 5, 11500, 16000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (2, 1, '775000100002', 'Aceite Shell Helix Ultra 5W-30 Sintético (1 Litro)', 'LITRO', 40, 8, 3200, 4500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (3, 1, '775000100003', 'Aceite Shell Helix Ultra 5W-40 Sintético (1 Galón)', 'GALON', 18, 4, 11800, 16500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (4, 1, '775000100004', 'Aceite Castrol EDGE 5W-30 Advanced Full Synthetic (1 Galón)', 'GALON', 20, 5, 12000, 17000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (5, 1, '775000100005', 'Aceite Castrol EDGE 5W-30 Advanced Full Synthetic (1 Litro)', 'LITRO', 30, 6, 3400, 4800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (6, 1, '775000100006', 'Aceite Castrol Magnatec Synthetic 5W-30 (1 Galón)', 'GALON', 22, 5, 10500, 14800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (7, 1, '775000100007', 'Aceite Mobil 1 ESP 5W-30 Full Synthetic (1 Galón)', 'GALON', 15, 4, 13000, 18000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (8, 1, '775000100008', 'Aceite Mobil 1 FS 0W-40 Full Synthetic (1 Litro)', 'LITRO', 25, 5, 3800, 5200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (9, 1, '775000100009', 'Aceite Mobil Super 3000 XE 5W-30 (1 Galón)', 'GALON', 18, 4, 10800, 15000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (10, 1, '775000100010', 'Aceite Motul 8100 X-cess 5W-40 Full Synthetic (1 Galón)', 'GALON', 12, 3, 13500, 18500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (11, 1, '775000100011', 'Aceite Motul 8100 Eco-lite 0W-20 Full Synthetic (1 Galón)', 'GALON', 10, 3, 14000, 19000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (12, 1, '775000100012', 'Aceite Liqui Moly Leichtlauf High Tech 5W-40 (1 Galón)', 'GALON', 14, 3, 14500, 19800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (13, 1, '775000100013', 'Aceite Liqui Moly Special Tec AA 5W-30 (1 Galón)', 'GALON', 16, 4, 14200, 19500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (14, 1, '775000100014', 'Aceite TotalEnergies Quartz 9000 NFC 5W-30 (1 Galón)', 'GALON', 15, 4, 10200, 14500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (15, 1, '775000100015', 'Aceite Repsol Elite Evolution 5W-30 (1 Galón)', 'GALON', 14, 4, 9800, 14000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (16, 2, '775000200001', 'Aceite Shell HX7 10W-40 Semi-Sintético (1 Galón)', 'GALON', 35, 8, 7500, 10800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (17, 2, '775000200002', 'Aceite Shell HX7 10W-40 Semi-Sintético (1 Litro)', 'LITRO', 45, 10, 2100, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (18, 2, '775000200003', 'Aceite Shell HX7 5W-30 Semi-Sintético (1 Galón)', 'GALON', 20, 5, 7800, 11000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (19, 2, '775000200004', 'Aceite Castrol Magnatec 10W-40 Part Synthetic (1 Galón)', 'GALON', 30, 6, 7600, 11000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (20, 2, '775000200005', 'Aceite Castrol GTX Ultraclean 10W-40 (1 Galón)', 'GALON', 25, 6, 7200, 10200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (21, 2, '775000200006', 'Aceite Mobil Super 2000 X1 10W-40 (1 Galón)', 'GALON', 28, 6, 7400, 10600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (22, 2, '775000200007', 'Aceite Mobil Super 2000 5W-30 (1 Galón)', 'GALON', 18, 4, 7700, 10900, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (23, 2, '775000200008', 'Aceite Valvoline MaxLife Semi-Synthetic 10W-40 (1 Galón)', 'GALON', 22, 5, 7900, 11200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (24, 2, '775000200009', 'Aceite TotalEnergies Quartz 7000 10W-40 (1 Galón)', 'GALON', 20, 5, 7000, 10000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (25, 2, '775000200010', 'Aceite Repsol Elite Multivalvulas 10W-40 (1 Galón)', 'GALON', 18, 4, 6800, 9800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (26, 2, '775000200011', 'Aceite Motul 6100 Synergia+ 10W-40 (1 Galón)', 'GALON', 15, 4, 8500, 12000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (27, 2, '775000200012', 'Aceite Liqui Moly Super Leichtlauf 10W-40 (1 Galón)', 'GALON', 12, 3, 9500, 13500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (28, 3, '775000300001', 'Aceite Shell HX5 20W-50 Mineral (1 Galón)', 'GALON', 40, 10, 5800, 8500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (29, 3, '775000300002', 'Aceite Shell HX5 20W-50 Mineral (1 Litro)', 'LITRO', 50, 12, 1600, 2400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (30, 3, '775000300003', 'Aceite Shell HX3 20W-50 Alto Kilometraje (1 Galón)', 'GALON', 30, 8, 5200, 7800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (31, 3, '775000300004', 'Aceite Castrol GTX 20W-50 Mineral (1 Galón)', 'GALON', 45, 10, 6000, 8800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (32, 3, '775000300005', 'Aceite Castrol GTX 20W-50 Mineral (1 Litro)', 'LITRO', 60, 15, 1700, 2500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (33, 3, '775000300006', 'Aceite Castrol GTX High Mileage 20W-50 (1 Galón)', 'GALON', 25, 6, 6300, 9200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (34, 3, '775000300007', 'Aceite Mobil Super 1000 20W-50 Mineral (1 Galón)', 'GALON', 35, 8, 5900, 8600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (35, 3, '775000300008', 'Aceite Valvoline Premium Protection 20W-50 (1 Galón)', 'GALON', 20, 5, 5800, 8500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (36, 3, '775000300009', 'Aceite Repsol Leader 20W-50 Mineral (1 Galón)', 'GALON', 25, 5, 5400, 8000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (37, 3, '775000300010', 'Aceite TotalEnergies Quartz 5000 20W-50 (1 Galón)', 'GALON', 20, 5, 5500, 8200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (38, 4, '775000400001', 'Aceite Shell Rimula R4 X 15W-40 Ci-4 (1 Galón)', 'GALON', 30, 8, 6500, 9500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (39, 4, '775000400002', 'Aceite Shell Rimula R4 X 15W-40 Ci-4 (1 Barril 55 Gal)', 'BARRIL', 2, 1, 320000, 420000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (40, 4, '775000400003', 'Aceite Shell Rimula R6 LM 10W-40 Sintético (1 Galón)', 'GALON', 12, 3, 11000, 15500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (41, 4, '775000400004', 'Aceite Mobil Delvac MX 15W-40 (1 Galón)', 'GALON', 35, 8, 6600, 9600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (42, 4, '775000400005', 'Aceite Mobil Delvac Modern 15W-40 Super Defense (1 Galón)', 'GALON', 20, 5, 6800, 9800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (43, 4, '775000400006', 'Aceite Castrol CRB Multi 15W-40 (1 Galón)', 'GALON', 25, 6, 6400, 9400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (44, 4, '775000400007', 'Aceite Valvoline All-Fleet Extra 15W-40 (1 Galón)', 'GALON', 18, 4, 6300, 9200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (45, 4, '775000400008', 'Aceite TotalEnergies Rubia TIR 7400 15W-40 (1 Galón)', 'GALON', 22, 5, 6100, 9000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (46, 4, '775000400009', 'Aceite Repsol Giant 300 15W-40 (1 Galón)', 'GALON', 15, 4, 6000, 8800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (47, 5, '775000500001', 'Aceite Motul 5100 4T 10W-40 Semi-Sintético (1 Litro)', 'LITRO', 30, 6, 2800, 4200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (48, 5, '775000500002', 'Aceite Motul 3000 4T 20W-50 Mineral (1 Litro)', 'LITRO', 40, 8, 2000, 3200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (49, 5, '775000500003', 'Aceite Shell Advance 4T AX7 10W-40 (1 Litro)', 'LITRO', 35, 8, 2200, 3400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (50, 5, '775000500004', 'Aceite Castrol Power 1 4T 10W-40 (1 Litro)', 'LITRO', 25, 5, 2600, 4000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (51, 5, '775000500005', 'Aceite Repsol Moto Rider 4T 20W-50 (1 Litro)', 'LITRO', 30, 6, 1800, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (52, 5, '775000500006', 'Aceite Motul 2T Super Stihl / Moto (1 Litro)', 'LITRO', 20, 4, 1900, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (53, 6, '775000600001', 'Filtro de Aceite Bosch O-0112 / Toyota Yaris, Corolla, Etios', 'UNIDAD', 50, 10, 1100, 2200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (54, 6, '775000600002', 'Filtro de Aceite Bosch O-0018 / Hyundai Accent, Elantra, Kia Rio', 'UNIDAD', 45, 10, 1100, 2200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (55, 6, '775000600003', 'Filtro de Aceite Bosch O-0118 / Nissan Sentra, Versa, Tiida', 'UNIDAD', 40, 8, 1200, 2400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (56, 6, '775000600004', 'Filtro de Aceite Lys FO-4521 / Toyota Hilux, Fortuner 1KD/2KD', 'UNIDAD', 35, 8, 1400, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (57, 6, '775000600005', 'Filtro de Aceite Lys FO-1012 / Nissan Frontier, NP300', 'UNIDAD', 30, 6, 1500, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (58, 6, '775000600006', 'Filtro de Aceite Lys FO-3021 / Suzuki Swift, Alto, Dzire', 'UNIDAD', 25, 5, 1000, 2000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (59, 6, '775000600007', 'Filtro de Aceite Mann Filter W 68/3 / Chevrolet Sail, Spark', 'UNIDAD', 30, 6, 1300, 2500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (60, 6, '775000600008', 'Filtro de Aceite Toyota Original 90915-YZZD2 / Hilux', 'UNIDAD', 20, 5, 2200, 4000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (61, 6, '775000600009', 'Filtro de Aceite Toyota Original 90915-YZZN2 / Yaris', 'UNIDAD', 25, 5, 2000, 3800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (62, 6, '775000600010', 'Filtro de Aceite Hyundai Original 26300-35505', 'UNIDAD', 20, 5, 1900, 3500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (63, 6, '775000600011', 'Filtro de Aceite Japan Parts FO-120S / Honda Civic, CR-V', 'UNIDAD', 15, 4, 1200, 2400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (64, 6, '775000600012', 'Filtro de Aceite Japan Parts FO-503S / Mitsubishi L200', 'UNIDAD', 18, 4, 1600, 3200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (65, 7, '775000700001', 'Filtro de Aire Lys FA-8812 / Toyota Yaris 2014-2020', 'UNIDAD', 25, 5, 1600, 3200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (66, 7, '775000700002', 'Filtro de Aire Lys FA-9102 / Toyota Hilux 1GD/2GD', 'UNIDAD', 20, 5, 2500, 4800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (67, 7, '775000700003', 'Filtro de Aire Lys FA-4410 / Nissan Versa, March, Note', 'UNIDAD', 22, 5, 1700, 3400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (68, 7, '775000700004', 'Filtro de Aire Lys FA-5511 / Hyundai Accent, Kia Rio 2012-2017', 'UNIDAD', 30, 6, 1800, 3500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (69, 7, '775000700005', 'Filtro de Aire Lys FA-5520 / Hyundai Accent, Kia Rio 2018-2023', 'UNIDAD', 25, 5, 1900, 3600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (70, 7, '775000700006', 'Filtro de Aire Lys FA-2101 / Kia Sportage, Hyundai Tucson 2.0', 'UNIDAD', 18, 4, 2200, 4200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (71, 7, '775000700007', 'Filtro de Aire Bosch C-24011 / Chevrolet Sail 1.5', 'UNIDAD', 20, 5, 1800, 3500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (72, 7, '775000700008', 'Filtro de Aire Mann Filter C 25 014 / Volkswagen Gol, Voyage', 'UNIDAD', 15, 4, 2000, 3800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (73, 7, '775000700009', 'Filtro de Aire Japan Parts FA-288S / Suzuki Swift 1.2', 'UNIDAD', 15, 4, 1700, 3200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (74, 7, '775000700010', 'Filtro de Aire Japan Parts FA-512S / Mitsubishi L200 Triton', 'UNIDAD', 12, 3, 2800, 5200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (75, 8, '775000800001', 'Filtro de Cabina Lys FC-101 / Toyota Yaris, Corolla, RAV4', 'UNIDAD', 20, 5, 1400, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (76, 8, '775000800002', 'Filtro de Cabina Lys FC-202 / Nissan Versa, Sentra, Kicks', 'UNIDAD', 18, 4, 1500, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (77, 8, '775000800003', 'Filtro de Cabina Lys FC-303 / Hyundai Accent, Tucson, Kia Rio', 'UNIDAD', 25, 5, 1400, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (78, 8, '775000800004', 'Filtro de Cabina Bosch Carbón Activado / Toyota Hilux', 'UNIDAD', 12, 3, 2500, 4800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (79, 8, '775000800005', 'Filtro de Cabina Japan Parts FAA-KI19 / Kia Picanto, Soluto', 'UNIDAD', 15, 4, 1300, 2500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (80, 9, '775000900001', 'Filtro de Combustible Lys FC-8801 / Trampa Diésel Hilux 1GD', 'UNIDAD', 20, 5, 3200, 6000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (81, 9, '775000900002', 'Filtro de Combustible Lys FC-1102 / Nissan Frontier NP300', 'UNIDAD', 15, 4, 3500, 6500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (82, 9, '775000900003', 'Filtro de Combustible Bosch en Línea Universal Gasolina', 'UNIDAD', 30, 8, 800, 1800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (83, 9, '775000900004', 'Filtro de Combustible Mann Filter WK 820/17 / Mitsubishi L200', 'UNIDAD', 10, 3, 4200, 7800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (84, 9, '775000900005', 'Filtro de Combustible Original Toyota 23390-0L070 / Hilux', 'UNIDAD', 12, 3, 4800, 8500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (85, 10, '775001000001', 'Refrigerante Prestone Coolant Rojo 33% Listo para usar (1 Galón)', 'GALON', 25, 5, 2800, 4500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (86, 10, '775001000002', 'Refrigerante Prestone Coolant Verde 33% Listo para usar (1 Galón)', 'GALON', 20, 5, 2800, 4500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (87, 10, '775001000003', 'Refrigerante Prestone Coolant Concentrado 100% (1 Galón)', 'GALON', 10, 3, 4200, 6500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (88, 10, '775001000004', 'Refrigerante Shell LubeMatch Coolant Rojo 50% (1 Galón)', 'GALON', 18, 4, 3200, 5000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (89, 10, '775001000005', 'Refrigerante Peak Long Life 50/50 (1 Galón)', 'GALON', 15, 4, 2600, 4200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (90, 10, '775001000006', 'Refrigerante Liqui Moly RAF 12+ (1 Litro)', 'LITRO', 12, 3, 2200, 3800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (91, 10, '775001000007', 'Agua Desmineralizada para Batería / Radiador (1 Galón)', 'GALON', 40, 10, 400, 1000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (92, 11, '775001100001', 'Aceite Transmisión Shell Spirax S2 ATF AX (1 Litro)', 'LITRO', 30, 6, 1800, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (93, 11, '775001100002', 'Aceite Transmisión Castrol Transmax ATF Dex/Merc (1 Litro)', 'LITRO', 25, 5, 2000, 3200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (94, 11, '775001100003', 'Aceite Transmisión Castrol Transmax CVT (1 Litro)', 'LITRO', 20, 4, 3200, 4800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (95, 11, '775001100004', 'Aceite Transmisión Mobil ATF 320 (1 Litro)', 'LITRO', 25, 5, 1900, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (96, 11, '775001100005', 'Aceite Transmisión Motul Multi ATF (1 Litro)', 'LITRO', 15, 4, 3500, 5200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (97, 11, '775001100006', 'Aceite Transmisión Motul CVTF Multi (1 Litro)', 'LITRO', 15, 4, 3800, 5500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (98, 11, '775001100007', 'Aceite Transmisión Toyota Genuine ATF WS (1 Litro)', 'LITRO', 20, 5, 3800, 5800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (99, 11, '775001100008', 'Aceite Transmisión Nissan Genuine Fluid CVT NS-3 (1 Litro)', 'LITRO', 18, 4, 4200, 6200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (100, 12, '775001200001', 'Aceite Corona Shell Spirax S2 A 80W-90 (1 Litro)', 'LITRO', 35, 8, 1600, 2500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (101, 12, '775001200002', 'Aceite Corona Shell Spirax S2 A 80W-90 (1 Galón)', 'GALON', 15, 4, 5800, 8800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (102, 12, '775001200003', 'Aceite Transmisión Shell Spirax S4 CX 85W-140 (1 Galón)', 'GALON', 12, 3, 6200, 9200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (103, 12, '775001200004', 'Aceite Caja Castrol Manual EP 80W-90 (1 Litro)', 'LITRO', 25, 5, 1800, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (104, 12, '775001200005', 'Aceite Caja Mobilube HD 80W-90 (1 Litro)', 'LITRO', 30, 6, 1700, 2600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (105, 12, '775001200006', 'Aceite Caja Sintético Motul Motylgear 75W-90 (1 Litro)', 'LITRO', 15, 4, 3600, 5400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (106, 12, '775001200007', 'Aceite Diferencial Liqui Moly Hypoid GL5 80W-90 (1 Litro)', 'LITRO', 10, 3, 2800, 4200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (107, 13, '775001300001', 'Líquido de Frenos Varga DOT 4 (250 ml)', 'UNIDAD', 40, 10, 900, 1600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (108, 13, '775001300002', 'Líquido de Frenos Varga DOT 4 (500 ml)', 'UNIDAD', 30, 8, 1500, 2600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (109, 13, '775001300003', 'Líquido de Frenos Bosch DOT 4 High Performance (500 ml)', 'UNIDAD', 25, 5, 1800, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (110, 13, '775001300004', 'Líquido de Frenos Castrol Brake Fluid DOT 4 (500 ml)', 'UNIDAD', 20, 5, 1700, 2900, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (111, 13, '775001300005', 'Líquido de Frenos Motul DOT 3&4 (500 ml)', 'UNIDAD', 15, 4, 2000, 3400, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (112, 13, '775001300006', 'Líquido de Frenos Prestone DOT 3 (354 ml)', 'UNIDAD', 30, 6, 1100, 2000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (113, 14, '775001400001', 'Aditivo Limpiador de Inyectores Liqui Moly Injection Cleaner (300 ml)', 'UNIDAD', 20, 5, 2800, 4500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (114, 14, '775001400002', 'Aditivo Tratamiento de Motor Liqui Moly Cera Tec (300 ml)', 'UNIDAD', 12, 3, 6500, 9800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (115, 14, '775001400003', 'Aditivo Limpiador de Motor Liqui Moly Engine Flush (300 ml)', 'UNIDAD', 15, 4, 2500, 4000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (116, 14, '775001400004', 'Aditivo Limpiador de Inyectores Prestone Gasolina (355 ml)', 'UNIDAD', 25, 5, 1400, 2500, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (117, 14, '775001400005', 'Aditivo Elevador de Octanaje STO Oktan Booster (250 ml)', 'UNIDAD', 20, 5, 1500, 2600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (118, 14, '775001400006', 'Limpiador de Carburador / Cuerpo de Aceleración ABRO (10 oz)', 'UNIDAD', 35, 8, 1200, 2200, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (119, 14, '775001400007', 'Desengrasante de Motor ABRO Foam Spray (450 ml)', 'UNIDAD', 25, 6, 1100, 2000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (120, 14, '775001400008', 'Silicona Protectora de Tableros y Vinilo Simoniz (300 ml)', 'UNIDAD', 30, 6, 1000, 1800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (121, 15, '775001500001', 'Grasa Chasis Varga Roja Multipropósito (1 Kilo)', 'UNIDAD', 20, 5, 1600, 2800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (122, 15, '775001500002', 'Grasa Rodajes Shell Gadus S2 V220 2 (1 Kilo)', 'UNIDAD', 15, 4, 2200, 3600, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (123, 15, '775001500003', 'Grasa Mobilgrease XHP 222 Azul (1 Kilo)', 'UNIDAD', 15, 4, 2400, 3800, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (124, 15, '775001500004', 'Aflojatodo / Lubricante Multiuso WD-40 (11 oz)', 'UNIDAD', 40, 10, 1800, 3000, 1);
INSERT INTO productos (id_producto, id_categoria, codigo_barras, descripcion, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta, activo) VALUES (125, 15, '775001500005', 'Aflojatodo / Lubricante Multiuso WD-40 (8 oz)', 'UNIDAD', 30, 8, 1400, 2400, 1);

-- ==========================================
-- TABLA: servicios (20 registros)
-- ==========================================
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (1, 'Cambio de Aceite de Motor y Filtro (Mano de Obra)', 2000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (2, 'Engrase General de Chasis, Suspensión y Transmisión', 2500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (3, 'Cambio de Aceite de Caja Mecánica', 3000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (4, 'Cambio de Aceite de Caja Automática / CVT', 4500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (5, 'Cambio de Aceite de Diferencial / Corona Posterior o 4x4', 3500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (6, 'Cambio y Purga de Líquido de Frenos por Vacío', 4000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (7, 'Limpieza e Inspección de Filtro de Aire y Soplado', 1000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (8, 'Cambio de Filtro de Combustible en Línea / Tanque', 2500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (9, 'Cambio de Filtro de Aire Acondicionado / Cabina', 1500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (10, 'Aplicación Directa de Aditivo Limpiador de Inyectores', 1500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (11, 'Cambio de Refrigerante, Purgado y Lavado Básico de Radiador', 3500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (12, 'Engrase de Rodajes y Bocinas de Ruedas', 5000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (13, 'Revision General Preventiva de Niveles y Fugas (Chequeo 15 Puntos)', 0, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (14, 'Lavado de Motor en Seco / Desengrasante', 3500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (15, 'Mantenimiento Preventivo Sistema de Frenos (Limpieza de Pastillas)', 4500, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (16, 'Inspección y Regulación de Presión de Neumáticos', 0, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (17, 'Scanner Automotriz Lectura y Borrado de Códigos de Error', 5000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (18, 'Mantenimiento Preventivo de Batería y Bornes', 1000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (19, 'Cambio de Bujías de Encendido (Juego)', 3000, 1);
INSERT INTO servicios (id_servicio, descripcion, precio_referencial, activo) VALUES (20, 'Cambio de Faja de Accesorios / Alternador', 3500, 1);

-- ==========================================
-- TABLA: compras (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_compra (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: ordenes_trabajo (8 registros)
-- ==========================================
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (1, 'OT-000001', 'ABC-123', 3, 1, 'FINALIZADO', 62000, 67000, 'Cambio de aceite sintético Shell Helix 5W-30 y filtro de aceite. Vehículo en excelente estado.', '2026-06-10 09:30:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (2, 'OT-000002', 'XYZ-789', 3, 2, 'FINALIZADO', 27000, 32000, 'Mantenimiento de los 25k km. Cambio de aceite Castrol 10W-40, filtro de aire y líquido de frenos.', '2026-07-15 11:00:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (3, 'OT-000003', 'V1B-456', 3, 1, 'FINALIZADO', 105000, 110000, 'Cambio de aceite mineral 20W-50, filtro de aceite y engrase general de chasis.', '2026-08-01 15:20:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (4, 'OT-000004', 'F5T-892', 3, 2, 'FINALIZADO', 84000, 89000, 'Mantenimiento de flota. Cambio de aceite diésel Shell Rimula 15W-40 y filtro trampa de combustible.', '2026-08-20 08:45:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (5, 'OT-000005', 'B3W-102', 3, 1, 'FINALIZADO', 49000, 54000, 'Cambio de aceite semi-sintético 10W-40, filtro de aire de cabina y aditivo de inyectores.', '2026-09-05 10:10:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (6, 'OT-000006', 'C2X-819', 3, 2, 'FINALIZADO', 93000, 98000, 'Cambio de aceite de motor y refrigerante Prestone 33%. Purgado completo del sistema.', '2026-09-18 14:00:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (7, 'OT-000007', 'ABC-123', 3, 1, 'EN_PROCESO', 68500, 73500, 'Ingreso por mantenimiento periódico de 68k km. En proceso de cambio de aceite y filtro de aire.', '2026-10-02 16:30:00');
INSERT INTO ordenes_trabajo (id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso) VALUES (8, 'OT-000008', 'F5T-892', 3, 2, 'EN_ESPERA', 89400, 94400, 'Pendiente de inicio. Inspección de suspensión y cambio de fluido de transmisión.', '2026-10-02 18:00:00');

-- ==========================================
-- TABLA: detalle_ot_productos (16 registros)
-- ==========================================
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (1, 1, 1, 1, 16000, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (2, 1, 6, 1, 2200, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (3, 2, 19, 1, 11000, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (4, 2, 38, 1, 3400, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (5, 2, 60, 1, 1600, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (6, 3, 30, 1, 8500, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (7, 3, 37, 1, 2200, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (8, 4, 39, 2, 9500, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (9, 4, 50, 1, 6000, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (10, 5, 18, 1, 10800, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (11, 5, 47, 1, 2800, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (12, 5, 66, 1, 4500, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (13, 6, 22, 1, 10200, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (14, 6, 53, 1, 4500, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (15, 7, 1, 1, 16000, 0);
INSERT INTO detalle_ot_productos (id_detalle, id_ot, id_producto, cantidad, precio_aplicado, descuento) VALUES (16, 7, 36, 1, 3200, 0);

-- ==========================================
-- TABLA: detalle_ot_servicios (12 registros)
-- ==========================================
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (1, 1, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (2, 2, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (3, 2, 6, 3, 4000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (4, 3, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (5, 3, 2, 3, 2500, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (6, 4, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (7, 4, 8, 3, 2500, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (8, 5, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (9, 5, 10, 3, 1500, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (10, 6, 1, 3, 2000, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (11, 6, 11, 3, 3500, 0);
INSERT INTO detalle_ot_servicios (id_detalle, id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento) VALUES (12, 7, 1, 3, 2000, 0);

-- ==========================================
-- TABLA: caja_chica (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: series_comprobante (4 registros)
-- ==========================================
INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) VALUES (1, 'BOLETA', 'B001', 0, 1);
INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) VALUES (2, 'BOLETA', 'B002', 0, 1);
INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) VALUES (3, 'FACTURA', 'F001', 0, 1);
INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) VALUES (4, 'FACTURA', 'F002', 0, 1);

-- ==========================================
-- TABLA: detalle_comprobante (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: pagos_comprobante (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: movimientos_caja (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: movimientos_inventario (14 registros)
-- ==========================================
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (1, 1, 2, NULL, NULL, NULL, 'AJUSTE', 0, 25, 25, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (2, 6, 2, NULL, NULL, NULL, 'AJUSTE', 0, 50, 50, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (3, 18, 2, NULL, NULL, NULL, 'AJUSTE', 0, 35, 35, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (4, 19, 2, NULL, NULL, NULL, 'AJUSTE', 0, 30, 30, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (5, 30, 2, NULL, NULL, NULL, 'AJUSTE', 0, 40, 40, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (6, 36, 2, NULL, NULL, NULL, 'AJUSTE', 0, 25, 25, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (7, 37, 2, NULL, NULL, NULL, 'AJUSTE', 0, 45, 45, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (8, 38, 2, NULL, NULL, NULL, 'AJUSTE', 0, 22, 22, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (9, 39, 2, NULL, NULL, NULL, 'AJUSTE', 0, 30, 30, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (10, 47, 2, NULL, NULL, NULL, 'AJUSTE', 0, 25, 25, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (11, 50, 2, NULL, NULL, NULL, 'AJUSTE', 0, 20, 20, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (12, 53, 2, NULL, NULL, NULL, 'AJUSTE', 0, 25, 25, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (13, 60, 2, NULL, NULL, NULL, 'AJUSTE', 0, 40, 40, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');
INSERT INTO movimientos_inventario (id_movimiento, id_producto, id_usuario, id_detalle_compra, id_detalle_ot, id_detalle_comprobante, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento) VALUES (14, 66, 2, NULL, NULL, NULL, 'AJUSTE', 0, 20, 20, 'Carga inicial de inventario masivo', '2026-06-01 08:00:00');

-- ==========================================
-- TABLA: seguimientos_fidelizacion (5 registros)
-- ==========================================
INSERT INTO seguimientos_fidelizacion (id_seguimiento, placa, fecha_programada, fecha_contacto, contactado, observacion, id_usuario) VALUES (1, 'ABC-123', '2026-11-10', NULL, 0, 'Llamar para recordatorio de cambio de aceite de los 67,000 km', 2);
INSERT INTO seguimientos_fidelizacion (id_seguimiento, placa, fecha_programada, fecha_contacto, contactado, observacion, id_usuario) VALUES (2, 'XYZ-789', '2026-12-15', NULL, 0, 'Recordatorio de mantenimiento preventivo de los 32,000 km', 2);
INSERT INTO seguimientos_fidelizacion (id_seguimiento, placa, fecha_programada, fecha_contacto, contactado, observacion, id_usuario) VALUES (3, 'V1B-456', '2026-11-01', NULL, 0, 'Seguimiento por engrase de chasis y revisión de kilometraje', 2);
INSERT INTO seguimientos_fidelizacion (id_seguimiento, placa, fecha_programada, fecha_contacto, contactado, observacion, id_usuario) VALUES (4, 'F5T-892', '2026-10-20', NULL, 0, 'Mantenimiento de flota bimestral', 2);
INSERT INTO seguimientos_fidelizacion (id_seguimiento, placa, fecha_programada, fecha_contacto, contactado, observacion, id_usuario) VALUES (5, 'B3W-102', '2027-01-05', NULL, 0, 'Recordatorio de cambio de aceite y filtro de cabina', 2);

-- ==========================================
-- TABLA: cola_envio_sunat (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: backups (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: auditoria (15 registros)
-- ==========================================
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (1, 2, 'vehiculos', 'INSERT', 'Registro de vehículo placa: ERS-963, cliente ID: 105', '2026-10-05 04:19:01');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (2, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ERS-963', '2026-10-05 04:19:20');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (3, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ERS-963', '2026-10-05 04:20:11');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (4, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:20:32');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (5, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:20:40');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (6, 2, 'vehiculos', 'DELETE', 'Eliminación lógica de vehículo placa: ERS-963', '2026-10-05 04:27:49');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (7, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: QWK-964', '2026-10-05 04:35:18');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (8, 2, 'vehiculos', 'DELETE', 'Eliminación lógica de vehículo placa: QWK-964', '2026-10-05 04:35:28');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (9, 2, 'vehiculos', 'UPDATE', 'Reasignación/Venta de vehículo placa: ABC-123 a cliente ID: 101', '2026-10-05 04:52:59');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (10, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:55:08');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (11, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:55:17');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (12, 2, 'vehiculos', 'INSERT', 'Registro de vehículo placa: DEF-902, cliente ID: 101', '2026-10-05 04:57:07');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (13, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:58:11');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (14, 2, 'vehiculos', 'UPDATE', 'Actualización de vehículo placa: ABC-123', '2026-10-05 04:58:16');
INSERT INTO auditoria (id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha) VALUES (15, 2, 'vehiculos', 'INSERT', 'Registro de vehículo placa: CVD-589, cliente ID: 1', '2026-10-05 04:58:59');

-- ==========================================
-- TABLA: clientes (105 registros)
-- ==========================================
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (1, 'DNI', '45892138', 'Juan Pedro Morales Quispe', '987654321', 'Av. San Martín 432, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (2, 'DNI', '71234567', 'Rosa Maria Flores Benavides', '912345678', 'Calle Los Naranjos 122, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (3, 'DNI', '10423981', 'Jorge Luis Benitez Silva', '955112233', 'Av. Benavides 890, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (4, 'RUC', '20601234567', 'TRANSPORTES Y SERVICIOS CHINCHA S.A.C.', '944332211', 'Av. Panamericana Sur Km 198, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (5, 'RUC', '20559876543', 'INVERSIONES AVALOS & HIJOS E.I.R.L.', '988776655', 'Calle Lima 450, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (6, 'DNI', '42109283', 'Carlos Alberto Mendoza Ruiz', '956123489', 'Av. Grau 312, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (7, 'DNI', '73910293', 'Ana Lucia Palomino Torres', '981230948', 'Calle Italia 145, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (8, 'DNI', '10928301', 'Miguel Angel Ramos Castilla', '952341092', 'Av. Arenales 520, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (9, 'DNI', '48201923', 'Sofia Elizabeth Gutierrez Vega', '971029384', 'Calle Comercio 230, Pueblo Nuevo', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (10, 'DNI', '70192830', 'Luis Fernando Castro Herrera', '961029384', 'Av. Municipalidad 110, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (11, 'DNI', '41029381', 'Roberto Carlos Diaz Sanchez', '941029381', 'Calle Junín 450, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (12, 'DNI', '72019283', 'Patricia Elena Vasquez Garcia', '931029382', 'Calle Sucre 120, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (13, 'DNI', '10293841', 'Hector Mario Fernandez Prado', '921029383', 'Av. Cutervo 890, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (14, 'DNI', '43019283', 'Carmen Rosa Salazar Huaman', '911029384', 'Calle Callao 340, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (15, 'DNI', '74019283', 'Gonzalo Javier Chumpitaz Solis', '951029385', 'Calle Primavera 210, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (16, 'DNI', '45019283', 'Diego Armando Guerrero Neira', '961029386', 'Av. Ayacucho 560, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (17, 'DNI', '75019283', 'Monica Beatriz Paredes Lujan', '971029387', 'Calle Arequipa 180, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (18, 'DNI', '10519283', 'Victor Manuel Yañez Cordero', '981029388', 'Av. Conde de Nieva 430, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (19, 'DNI', '46019283', 'Raul Enrique Farfan Quispe', '991029389', 'Calle San José 290, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (20, 'DNI', '76019283', 'Silvia Veronica Lujan Coronado', '952029380', 'Av. Centenario 150, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (21, 'RUC', '20192830192', 'DISTRIBUIDORA AGRICOLA DEL SUR S.A.C.', '056231092', 'Panamericana Sur Km 300, Subtanjalla, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (22, 'RUC', '20381920391', 'CONSTRUCTORA E INMOBILIARIA ICA S.R.L.', '056214589', 'Av. Los Maestros B-12, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (23, 'RUC', '20510293841', 'SERVICIOS GENERALES Y LOGISTICA PERU E.I.R.L.', '056221045', 'Calle Prolongación Ayacucho 450, Chincha', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (24, 'RUC', '20491029381', 'EMPRESA DE TRANSPORTES SEÑOR DE LUREN S.A.', '056234901', 'Av. Tupac Amaru 120, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (25, 'RUC', '20602918231', 'AGROINDUSTRIAS SUNAMPE S.A.C.', '056219082', 'Calle Principal 890, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (26, 'DNI', '47102938', 'Fernando Jose Alarcon Bendezú', '943019283', 'Calle Paita 112, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (27, 'DNI', '77102938', 'Gabriela Mercedes Rios Sotomayor', '953019284', 'Av. Los Incas 340, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (28, 'DNI', '10610293', 'Cesar Augusto Valenzuela Cabrera', '963019285', 'Calle Tacna 230, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (29, 'DNI', '48102938', 'Vanessa Vanessa Espinoza Carrillo', '973019286', 'Av. Bolivar 670, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (30, 'DNI', '78102938', 'Guillermo Esteban Navarrete Pecho', '983019287', 'Calle Pisco 190, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (31, 'DNI', '49102938', 'Lourdes Maria Cardenas Ponce', '993019288', 'Calle Libertad 410, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (32, 'DNI', '79102938', 'Renzo Paolo Guerrero Alva', '954019289', 'Av. La Marina 220, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (33, 'DNI', '10710293', 'Julio Cesar Farfán Rejas', '964019290', 'Calle Chiclayo 150, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (34, 'DNI', '40201928', 'Teresa De Jesus Orellana Soria', '974019291', 'Av. Progreso 380, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (35, 'DNI', '70201928', 'Andres Avelino Caceres Nieto', '984019292', 'Calle Condorcanqui 120, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (36, 'DNI', '41201928', 'Milagros Del Pilar Loyola Soto', '994019293', 'Av. Peru 540, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (37, 'DNI', '71201928', 'Oswaldo Javier Benavides Rivas', '955019294', 'Calle Trujillo 280, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (38, 'DNI', '10820192', 'Fabiola Andrea Zevallos Prada', '965019295', 'Calle Piura 190, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (39, 'DNI', '42201928', 'Santamaria Martin Vizcarra Cornejo', '975019296', 'Av. Brasil 430, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (40, 'DNI', '72201928', 'Rocio Del Carmen Villanueva Milla', '985019297', 'Calle Cusco 310, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (41, 'RUC', '20592810293', 'COMERCIALIZADORA Y DISTRIBUIDORA VALLE S.A.C.', '056239102', 'Av. Industrial 120, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (42, 'RUC', '20481029381', 'MUNICIPALIDAD DISTRITAL DE SUNAMPE', '056219010', 'Plaza de Armas s/n, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (43, 'RUC', '20603918201', 'SERVICIOS MULTIPLES SAN FRANCISCO E.I.R.L.', '056228192', 'Calle Benavides 450, Chincha', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (44, 'RUC', '20510928371', 'CORPORACION LOGISTICA DEL PERU S.A.', '056230192', 'Panamericana Sur Km 195, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (45, 'RUC', '20410293841', 'FLOTA DE TAXIS CHINCHA EXPRESS S.A.C.', '056218902', 'Av. San Idelfonso 230, Pueblo Nuevo', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (46, 'DNI', '43201928', 'Evelyn Pamela Huaman Vilca', '956019298', 'Calle Huancavelica 140, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (47, 'DNI', '73201928', 'Alex Alejandro Sanchez Paredes', '966019299', 'Av. Panamericana 890, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (48, 'DNI', '10920192', 'Gisela Yessenia Valenzuela Rivas', '976019300', 'Calle Moquegua 210, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (49, 'DNI', '44201928', 'Sandro Paolo De La Cruz Torres', '986019301', 'Calle Loreto 320, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (50, 'DNI', '74201928', 'Claudia Fiorella Barrientos Cayo', '996019302', 'Av. Municipalidad 450, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (51, 'DNI', '45201928', 'Marco Antonio Ormeño Pecho', '957019303', 'Calle Colon 120, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (52, 'DNI', '75201928', 'Karla Viviana Magallanes Ramos', '967019304', 'Av. Los Angeles 230, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (53, 'DNI', '10130192', 'Jaime Eduardo Levano Castillo', '977019305', 'Calle Victoria 340, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (54, 'DNI', '46201928', 'Jessica Paola Almora Sotelo', '987019306', 'Av. Primavera 560, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (55, 'DNI', '76201928', 'Daniel Francisco Tasayco Munayco', '997019307', 'Calle Rosario 180, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (56, 'DNI', '47201928', 'Alicia Maria Pachas Mateo', '958019308', 'Calle Alfonso Ugarte 290, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (57, 'DNI', '77201928', 'Enrique Nicolas Saravia Yataco', '968019309', 'Av. Bolognesi 410, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (58, 'DNI', '10230192', 'Lucia Milagros Siguas Garibay', '978019310', 'Calle San Martin 150, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (59, 'DNI', '48201928', 'Bruno Alexander Carbone Mesias', '988019311', 'Av. San Juan 670, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (60, 'DNI', '78201928', 'Diana Carolina Mesias Sotelo', '998019312', 'Calle Olaya 230, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (61, 'RUC', '20501928371', 'TRANSPORTE Y CARGA ICA E.I.R.L.', '056231000', 'Av. Los Moches 112, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (62, 'RUC', '20601928471', 'AGRICOLA Y GANADERA DEL VALLE S.A.', '056214500', 'Fundo San José s/n, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (63, 'RUC', '20491827361', 'TEXTIL Y CONFECCIONES DEL SUR S.A.C.', '056228900', 'Av. Prolongación Grau 560, Chincha', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (64, 'RUC', '20581920381', 'SERVICIOS AMBIENTALES Y LIMPIEZA PERU S.A.', '056230011', 'Calle Los Jazmines 120, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (65, 'RUC', '20602938471', 'DISTRIBUIDORA LUBRICANTES Y REPUESTOS LUREN E.I.R.L.', '056219900', 'Av. Cutervo 340, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (66, 'DNI', '49201928', 'Joel Omar Ramos Tasayco', '959019313', 'Calle Tarapaca 120, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (67, 'DNI', '79201928', 'Brenda Stephanie Yataco Almeyda', '969019314', 'Av. Centenario 340, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (68, 'DNI', '10330192', 'Sonia Maria Almeyda Castilla', '979019315', 'Calle Espinar 210, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (69, 'DNI', '40301928', 'Gaston Alberto Mateo Levano', '989019316', 'Av. Los Alamos 450, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (70, 'DNI', '70301928', 'Fiorella Beatriz Castillon Solis', '999019317', 'Calle Galvez 180, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (71, 'DNI', '41301928', 'Christian Paul Solis Vilca', '950119318', 'Av. Grau 290, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (72, 'DNI', '71301928', 'Katia Yuliana Vilca Chumpitaz', '960119319', 'Calle Prado 410, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (73, 'DNI', '10430192', 'Erick Manuel Chumpitaz Najar', '970119320', 'Av. Cutervo 150, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (74, 'DNI', '42301928', 'Yessica Maria Najar Lujan', '980119321', 'Calle Italia 670, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (75, 'DNI', '72301928', 'Felipe Santiago Lujan Cordero', '990119322', 'Av. San Idelfonso 230, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (76, 'DNI', '43301928', 'Maritza Isabel Cordero Farfan', '951219323', 'Calle Callao 140, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (77, 'DNI', '73301928', 'Oscar David Farfan Quispe', '961219324', 'Av. Arequipa 380, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (78, 'DNI', '10530192', 'Nelly Victoria Quispe Siguas', '971219325', 'Calle Lima 210, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (79, 'DNI', '44301928', 'Walter Jesus Siguas Carbone', '981219326', 'Av. Benavides 540, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (80, 'DNI', '74301928', 'Yuliana Beatriz Carbone Mesias', '991219327', 'Calle Paita 190, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (81, 'RUC', '20512930491', 'CONSTRUCTORA SAN MARTIN DE PORRES S.A.C.', '056238910', 'Calle Comercio 450, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (82, 'RUC', '20601920391', 'IMPORTADORA Y EXPORTADORA PERU DEL SUR E.I.R.L.', '056214980', 'Av. Panamericana Sur Km 201, Chincha', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (83, 'RUC', '20491028371', 'SERVICIOS TURISTICOS Y HOTELES ICA S.A.', '056221099', 'Av. Los Leones 120, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (84, 'RUC', '20581928371', 'AGROPECUARIA Y RIEGO TECNIFICADO S.A.C.', '056230055', 'Calle Principal 340, Subtanjalla', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (85, 'RUC', '20602938101', 'EMPRESA DE TRANSPORTES RAPIDO SUNAMPE S.A.', '056219088', 'Av. Municipalidad 110, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (86, 'DNI', '45301928', 'Edgar Antonio Mesias Ramos', '952319328', 'Calle Ayacucho 430, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (87, 'DNI', '75301928', 'Cynthia Vanessa Ramos Yataco', '962319329', 'Av. Brasil 310, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (88, 'DNI', '10630192', 'Guillermo Hugo Yataco Castilla', '972319330', 'Calle Sucre 140, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (89, 'DNI', '46301928', 'Miriam Roxana Castilla Mateo', '982319331', 'Av. Progreso 230, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (90, 'DNI', '76301928', 'Jose Luis Mateo Solis', '992319332', 'Calle Junin 340, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (91, 'DNI', '47301928', 'Gladys Elena Solis Najar', '953419333', 'Av. Centenario 560, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (92, 'DNI', '77301928', 'Ruben Dario Najar Farfan', '963419334', 'Calle Olaya 180, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (93, 'DNI', '10730192', 'Hilda Elizabeth Farfan Siguas', '973419335', 'Av. Los Incas 290, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (94, 'DNI', '48301928', 'Mario Alberto Siguas Mesias', '983419336', 'Calle Chiclayo 410, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (95, 'DNI', '78301928', 'Luz Marina Mesias Yataco', '993419337', 'Av. San Martin 150, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (96, 'DNI', '49301928', 'Cesar Augusto Yataco Mateo', '954519338', 'Calle Arequipa 670, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (97, 'DNI', '79301928', 'Rocio Elizabeth Mateo Najar', '964519339', 'Av. Cutervo 230, Sunampe', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (98, 'DNI', '10830192', 'Jorge Eduardo Najar Siguas', '974519340', 'Calle Trujillo 140, Ica', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (99, 'DNI', '40401928', 'Teresa De Jesus Siguas Mesias', '984519341', 'Av. Benavides 380, Chincha Alta', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (100, 'DNI', '70401928', 'Victor Manuel Mesias Yataco', '994519342', 'Calle Pisco 210, Grocio Prado', 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (101, 'DNI', '72017471', 'JESÚS LOZA YATACO', '977860423', NULL, 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (102, 'DNI', '78456396', 'JSADJDSJADSDSASDA', '956323631', NULL, 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (103, 'DNI', '72017478', 'PRUEBAAAS', '956231456', NULL, 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (104, 'DNI', '56253614', 'PRUEBA', '956236145', NULL, 1);
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo) VALUES (105, 'DNI', '58964552', 'PRUEBA 100', '956895452', 'ASDDSADAS', 1);

-- ==========================================
-- TABLA: vehiculos (42 registros)
-- ==========================================
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('ABC-123', 101, 'Hiunday', 'i20', 2025, 'lslad', 68500, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('XYZ-789', 2, 'Nissan', 'Versa', 2021, '1.6L HR16DE', 32100, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V1B-456', 3, 'Hyundai', 'Accent', 2016, '1.4L Kappa', 112000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('F5T-892', 4, 'Toyota', 'Hilux', 2020, '2.8L Diesel 1GD', 89400, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('B3W-102', 5, 'Kia', 'Rio', 2019, '1.4L Gamma', 54200, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('C2X-819', 6, 'Chevrolet', 'Sail', 2017, '1.5L STEC', 98000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('D4Y-901', 7, 'Kia', 'Picanto', 2022, '1.0L Kappa', 18500, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('E1Z-234', 8, 'Toyota', 'Corolla', 2015, '1.8L 2ZR-FE', 135000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('F3A-567', 9, 'Suzuki', 'Swift', 2020, '1.2L K12M', 41200, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('G2B-890', 10, 'Hyundai', 'Tucson', 2018, '2.0L Nu MPI', 76000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('H5C-123', 11, 'Nissan', 'Sentra', 2019, '1.8L MRA8DE', 62400, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('I1D-456', 12, 'Volkswagen', 'Gol', 2016, '1.6L MSI', 105000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('J4E-789', 13, 'Kia', 'Sportage', 2021, '2.0L Nu', 29800, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('K2F-012', 14, 'Toyota', 'Agya', 2022, '1.2L 3NR-VE', 15400, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('L5G-345', 15, 'Nissan', 'Frontier NP300', 2018, '2.5L Diesel YD25', 118000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('M1H-678', 16, 'Honda', 'Civic', 2017, '2.0L K20C2', 82000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('N4I-901', 17, 'Hyundai', 'Creta', 2020, '1.6L Gamma', 45000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('O2J-234', 18, 'Mitsubishi', 'L200 Triton', 2019, '2.4L Diesel 4N15', 92000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('P5K-567', 19, 'Toyota', 'Etios', 2018, '1.5L 2NR-FBE', 87000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('Q1L-890', 20, 'Kia', 'Soluto', 2021, '1.4L Kappa', 31000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V2M-111', 21, 'Toyota', 'Hilux 4x4', 2021, '2.8L Diesel 1GD', 64000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V3N-222', 22, 'Hyundai', 'H-1 Van', 2017, '2.5L Diesel A2', 145000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V4O-333', 23, 'Isuzu', 'D-Max', 2019, '3.0L Diesel 4JJ1', 108000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V5P-444', 24, 'Toyota', 'Coaster', 2016, '4.0L Diesel N04C', 195000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V6Q-555', 25, 'Kia', 'Grand Carnival', 2020, '2.2L Diesel CRDi', 51000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('R2R-123', 26, 'Suzuki', 'Alto K10', 2018, '1.0L K10B', 58000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('S5S-456', 27, 'Chevrolet', 'Tracker', 2021, '1.2L Turbo', 27500, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('T1T-789', 28, 'Nissan', 'Kicks', 2020, '1.6L HR16DE', 39000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('U4U-012', 29, 'Toyota', 'RAV4', 2019, '2.0L M20A-FKS', 56000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('V2V-345', 30, 'Hyundai', 'Elantra', 2017, '1.6L Gamma', 91000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('W5W-678', 31, 'Kia', 'Seltos', 2022, '1.6L Gamma', 19000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('X1X-901', 32, 'Mazda', 'CX-5', 2018, '2.0L SkyActiv-G', 73000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('Y4Y-234', 33, 'Nissan', 'Tiida', 2014, '1.6L HR16DE', 162000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('Z2Z-567', 34, 'Toyota', 'Yaris Hatchback', 2019, '1.5L 2NR-FE', 48000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('A1A-890', 35, 'Ford', 'Ranger', 2020, '3.2L Diesel Duratorq', 79000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('TJG-526', 101, 'Hyundai', 'i10', 2025, '1.5 GOR', 20000, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('QWE-458', 102, 'ASDDSDS', 'ADSDAS', 2021, 'SDADSDS', 56200, 0);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('PLO-895', 103, 'SDSA', 'DSADAS', 2010, 'DSDAS', 25630, 0);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('QWK-964', 104, 'Luxuris', 'Pols', 2025, '2.8LSA TA', 55800, 0);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('ERS-963', 105, 'LO8', 'SDDAKS', 2018, 'DSSAASÑPÑ', 55441, 0);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('DEF-902', 101, 'ASDASAS', 'ADSDSA', 2018, 'DSDSA', 58963, 1);
INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo) VALUES ('CVD-589', 1, 'Lol', 'Po', 2018, 'LSLSD', 58693, 1);

-- ==========================================
-- TABLA: comprobantes (0 registros)
-- ==========================================
-- (Tabla vacía)


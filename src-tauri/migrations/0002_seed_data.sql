-- 0002_seed_data.sql

-- 1. Usuarios de prueba (Uno por cada rol, con hashes ficticios por ahora)
INSERT INTO usuarios (nombre_completo, usuario, password_hash, rol, activo) VALUES 
('Admin Principal', 'admin', 'hash_admin_123', 'ADMINISTRADOR', 1),
('Juan Mecánico', 'mecanico1', 'hash_mecanico_123', 'MECANICO', 1),
('María Cajera', 'cajera1', 'hash_cajera_123', 'CAJERO', 1);

-- 2. Clientes de prueba (5 variados entre DNI, RUC y CE)
INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, direccion) VALUES 
('DNI', '11111111', 'Carlos Sanchez', '999111222', 'Av. Central 123'),
('DNI', '22222222', 'Luis Gomez', '999333444', 'Calle Las Flores 456'),
('RUC', '20123456789', 'Transportes del Sur SAC', '01-4445555', 'Av. Industrial 789'),
('RUC', '10987654321', 'Logística Express EIRL', '987654321', 'Mz. A Lote 5 Zona Ind.'),
('CE', '000123456', 'John Doe', '912345678', 'Residencial El Sol 101');

-- 3. Categorías básicas
INSERT INTO categorias (nombre_categoria) VALUES 
('Aceites de Motor'),
('Filtros'),
('Refrigerantes'),
('Grasas y Lubricantes');

-- 4. Productos de prueba (Diferentes unidades de medida y precisión en reales)
INSERT INTO productos (id_categoria, codigo_barras, unidad_medida, stock_actual, stock_minimo, precio_compra, precio_venta) VALUES 
(1, '775000000001', 'LITROS', 150.5, 20.0, 15.00, 25.00),
(1, '775000000002', 'GALONES', 30.0, 5.0, 55.00, 85.00),
(2, '775000000003', 'UNIDADES', 45.0, 10.0, 12.00, 22.50),
(3, '775000000004', 'LITROS', 80.0, 15.0, 8.00, 16.00),
(4, '775000000005', 'UNIDADES', 25.0, 5.0, 25.00, 40.00);
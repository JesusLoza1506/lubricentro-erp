-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=320,valor_stock_micro=3456000000,version=15 WHERE id_producto=57;
UPDATE productos SET stock_actual=321,valor_stock_micro=4044600000,version=14 WHERE id_producto=58;
UPDATE productos SET stock_actual=321,valor_stock_micro=4622400000,version=14 WHERE id_producto=59;
UPDATE productos SET stock_actual=321,valor_stock_micro=5200200000,version=14 WHERE id_producto=60;
UPDATE productos SET stock_actual=321,valor_stock_micro=5778000000,version=14 WHERE id_producto=61;

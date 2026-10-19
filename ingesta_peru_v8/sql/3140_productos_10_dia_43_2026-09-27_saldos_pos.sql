-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=323,valor_stock_micro=3488400000,version=13 WHERE id_producto=57;
UPDATE productos SET stock_actual=323,valor_stock_micro=4069800000,version=13 WHERE id_producto=58;
UPDATE productos SET stock_actual=323,valor_stock_micro=4651200000,version=13 WHERE id_producto=59;
UPDATE productos SET stock_actual=324,valor_stock_micro=5248800000,version=12 WHERE id_producto=60;
UPDATE productos SET stock_actual=324,valor_stock_micro=5832000000,version=12 WHERE id_producto=61;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=323,valor_stock_micro=3488400000,version=13 WHERE id_producto=47;
UPDATE productos SET stock_actual=324,valor_stock_micro=4082400000,version=12 WHERE id_producto=48;
UPDATE productos SET stock_actual=324,valor_stock_micro=4665600000,version=12 WHERE id_producto=49;
UPDATE productos SET stock_actual=324,valor_stock_micro=5248800000,version=12 WHERE id_producto=50;
UPDATE productos SET stock_actual=324,valor_stock_micro=5832000000,version=12 WHERE id_producto=51;

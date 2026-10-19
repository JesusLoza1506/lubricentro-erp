-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=3639600000,version=4 WHERE id_producto=47;
UPDATE productos SET stock_actual=338,valor_stock_micro=4258800000,version=3 WHERE id_producto=48;
UPDATE productos SET stock_actual=338,valor_stock_micro=4867200000,version=3 WHERE id_producto=49;
UPDATE productos SET stock_actual=338,valor_stock_micro=5475600000,version=3 WHERE id_producto=50;
UPDATE productos SET stock_actual=338,valor_stock_micro=6084000000,version=3 WHERE id_producto=51;

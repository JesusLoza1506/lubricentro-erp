-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=3639600000,version=4 WHERE id_producto=57;
UPDATE productos SET stock_actual=337,valor_stock_micro=4246200000,version=4 WHERE id_producto=58;
UPDATE productos SET stock_actual=337,valor_stock_micro=4852800000,version=4 WHERE id_producto=59;
UPDATE productos SET stock_actual=338,valor_stock_micro=5475600000,version=3 WHERE id_producto=60;
UPDATE productos SET stock_actual=338,valor_stock_micro=6084000000,version=3 WHERE id_producto=61;

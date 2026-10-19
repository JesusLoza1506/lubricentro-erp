-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=6672600000,version=4 WHERE id_producto=72;
UPDATE productos SET stock_actual=338,valor_stock_micro=7300800000,version=3 WHERE id_producto=73;
UPDATE productos SET stock_actual=338,valor_stock_micro=7909200000,version=3 WHERE id_producto=74;
UPDATE productos SET stock_actual=338,valor_stock_micro=8517600000,version=3 WHERE id_producto=75;
UPDATE productos SET stock_actual=338,valor_stock_micro=9126000000,version=3 WHERE id_producto=76;

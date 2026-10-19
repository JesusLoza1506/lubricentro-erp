-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1176000,valor_stock_micro=17640000000,version=8 WHERE id_producto=113;
UPDATE productos SET stock_actual=338,valor_stock_micro=3650400000,version=4 WHERE id_producto=37;
UPDATE productos SET stock_actual=1176000,valor_stock_micro=31752000000,version=8 WHERE id_producto=114;
UPDATE productos SET stock_actual=338,valor_stock_micro=4258800000,version=4 WHERE id_producto=38;

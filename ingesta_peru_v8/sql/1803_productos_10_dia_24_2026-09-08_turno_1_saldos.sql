-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1104000,valor_stock_micro=16560000000,version=26 WHERE id_producto=113;
UPDATE productos SET stock_actual=331,valor_stock_micro=7149600000,version=9 WHERE id_producto=43;
UPDATE productos SET stock_actual=1104000,valor_stock_micro=29808000000,version=26 WHERE id_producto=114;
UPDATE productos SET stock_actual=331,valor_stock_micro=7745400000,version=9 WHERE id_producto=44;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1000000,valor_stock_micro=15000000000,version=52 WHERE id_producto=113;
UPDATE productos SET stock_actual=318,valor_stock_micro=8013600000,version=18 WHERE id_producto=45;
UPDATE productos SET stock_actual=1000000,valor_stock_micro=27000000000,version=52 WHERE id_producto=114;
UPDATE productos SET stock_actual=318,valor_stock_micro=8586000000,version=18 WHERE id_producto=46;

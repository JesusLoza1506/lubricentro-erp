-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1004000,valor_stock_micro=15060000000,version=51 WHERE id_producto=113;
UPDATE productos SET stock_actual=318,valor_stock_micro=6868800000,version=18 WHERE id_producto=43;
UPDATE productos SET stock_actual=1004000,valor_stock_micro=27108000000,version=51 WHERE id_producto=114;
UPDATE productos SET stock_actual=318,valor_stock_micro=7441200000,version=18 WHERE id_producto=44;

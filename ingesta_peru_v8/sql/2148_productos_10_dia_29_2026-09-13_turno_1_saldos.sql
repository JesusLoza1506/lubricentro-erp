-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1084000,valor_stock_micro=16260000000,version=31 WHERE id_producto=113;
UPDATE productos SET stock_actual=328,valor_stock_micro=7084800000,version=11 WHERE id_producto=43;
UPDATE productos SET stock_actual=1084000,valor_stock_micro=29268000000,version=31 WHERE id_producto=114;
UPDATE productos SET stock_actual=328,valor_stock_micro=7675200000,version=11 WHERE id_producto=44;

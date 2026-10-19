-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1180000,valor_stock_micro=17700000000,version=7 WHERE id_producto=113;
UPDATE productos SET stock_actual=339,valor_stock_micro=8542800000,version=3 WHERE id_producto=45;
UPDATE productos SET stock_actual=1180000,valor_stock_micro=31860000000,version=7 WHERE id_producto=114;
UPDATE productos SET stock_actual=339,valor_stock_micro=9153000000,version=3 WHERE id_producto=46;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1160000,valor_stock_micro=17400000000,version=12 WHERE id_producto=113;
UPDATE productos SET stock_actual=336,valor_stock_micro=8467200000,version=5 WHERE id_producto=45;
UPDATE productos SET stock_actual=1160000,valor_stock_micro=31320000000,version=12 WHERE id_producto=114;
UPDATE productos SET stock_actual=336,valor_stock_micro=9072000000,version=5 WHERE id_producto=46;

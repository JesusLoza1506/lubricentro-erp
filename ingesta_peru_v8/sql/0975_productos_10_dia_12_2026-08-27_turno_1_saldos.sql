-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1152000,valor_stock_micro=17280000000,version=14 WHERE id_producto=113;
UPDATE productos SET stock_actual=335,valor_stock_micro=4824000000,version=6 WHERE id_producto=39;
UPDATE productos SET stock_actual=1152000,valor_stock_micro=31104000000,version=14 WHERE id_producto=114;
UPDATE productos SET stock_actual=335,valor_stock_micro=5427000000,version=6 WHERE id_producto=40;

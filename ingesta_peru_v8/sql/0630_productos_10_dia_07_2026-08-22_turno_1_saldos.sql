-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1172000,valor_stock_micro=17580000000,version=9 WHERE id_producto=113;
UPDATE productos SET stock_actual=338,valor_stock_micro=4867200000,version=4 WHERE id_producto=39;
UPDATE productos SET stock_actual=1172000,valor_stock_micro=31644000000,version=9 WHERE id_producto=114;
UPDATE productos SET stock_actual=338,valor_stock_micro=5475600000,version=4 WHERE id_producto=40;

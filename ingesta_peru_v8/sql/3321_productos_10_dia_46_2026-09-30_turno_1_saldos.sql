-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1016000,valor_stock_micro=15240000000,version=48 WHERE id_producto=113;
UPDATE productos SET stock_actual=320,valor_stock_micro=3456000000,version=17 WHERE id_producto=37;
UPDATE productos SET stock_actual=1016000,valor_stock_micro=27432000000,version=48 WHERE id_producto=114;
UPDATE productos SET stock_actual=320,valor_stock_micro=4032000000,version=17 WHERE id_producto=38;

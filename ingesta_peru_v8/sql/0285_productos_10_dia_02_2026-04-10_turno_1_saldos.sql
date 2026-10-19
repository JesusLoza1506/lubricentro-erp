-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1192000,valor_stock_micro=17880000000,version=4 WHERE id_producto=113;
UPDATE productos SET stock_actual=339,valor_stock_micro=4881600000,version=3 WHERE id_producto=39;
UPDATE productos SET stock_actual=1192000,valor_stock_micro=32184000000,version=4 WHERE id_producto=114;
UPDATE productos SET stock_actual=339,valor_stock_micro=5491800000,version=3 WHERE id_producto=40;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1012000,valor_stock_micro=15180000000,version=49 WHERE id_producto=113;
UPDATE productos SET stock_actual=320,valor_stock_micro=4608000000,version=17 WHERE id_producto=39;
UPDATE productos SET stock_actual=1012000,valor_stock_micro=27324000000,version=49 WHERE id_producto=114;
UPDATE productos SET stock_actual=320,valor_stock_micro=5184000000,version=17 WHERE id_producto=40;

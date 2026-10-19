-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=333,valor_stock_micro=3596400000,version=7 WHERE id_producto=37;
UPDATE productos SET stock_actual=333,valor_stock_micro=4195800000,version=7 WHERE id_producto=38;
UPDATE productos SET stock_actual=333,valor_stock_micro=4795200000,version=7 WHERE id_producto=39;
UPDATE productos SET stock_actual=333,valor_stock_micro=5394600000,version=7 WHERE id_producto=40;
UPDATE productos SET stock_actual=333,valor_stock_micro=5994000000,version=7 WHERE id_producto=41;

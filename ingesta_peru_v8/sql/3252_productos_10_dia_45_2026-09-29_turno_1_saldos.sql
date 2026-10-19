-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1020000,valor_stock_micro=15300000000,version=47 WHERE id_producto=113;
UPDATE productos SET stock_actual=321,valor_stock_micro=8089200000,version=16 WHERE id_producto=45;
UPDATE productos SET stock_actual=1020000,valor_stock_micro=27540000000,version=47 WHERE id_producto=114;
UPDATE productos SET stock_actual=321,valor_stock_micro=8667000000,version=16 WHERE id_producto=46;

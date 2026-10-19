-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1060000,valor_stock_micro=15900000000,version=37 WHERE id_producto=113;
UPDATE productos SET stock_actual=325,valor_stock_micro=8190000000,version=13 WHERE id_producto=45;
UPDATE productos SET stock_actual=1060000,valor_stock_micro=28620000000,version=37 WHERE id_producto=114;
UPDATE productos SET stock_actual=325,valor_stock_micro=8775000000,version=13 WHERE id_producto=46;

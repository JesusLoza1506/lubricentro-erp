-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1120000,valor_stock_micro=16800000000,version=22 WHERE id_producto=113;
UPDATE productos SET stock_actual=332,valor_stock_micro=8366400000,version=8 WHERE id_producto=45;
UPDATE productos SET stock_actual=1120000,valor_stock_micro=30240000000,version=22 WHERE id_producto=114;
UPDATE productos SET stock_actual=332,valor_stock_micro=8964000000,version=8 WHERE id_producto=46;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1124000,valor_stock_micro=16860000000,version=21 WHERE id_producto=113;
UPDATE productos SET stock_actual=332,valor_stock_micro=7171200000,version=8 WHERE id_producto=43;
UPDATE productos SET stock_actual=1124000,valor_stock_micro=30348000000,version=21 WHERE id_producto=114;
UPDATE productos SET stock_actual=332,valor_stock_micro=7768800000,version=8 WHERE id_producto=44;

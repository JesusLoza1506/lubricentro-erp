-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=960000,valor_stock_micro=14400000000,version=62 WHERE id_producto=113;
UPDATE productos SET stock_actual=314,valor_stock_micro=7912800000,version=21 WHERE id_producto=45;
UPDATE productos SET stock_actual=960000,valor_stock_micro=25920000000,version=62 WHERE id_producto=114;
UPDATE productos SET stock_actual=314,valor_stock_micro=8478000000,version=21 WHERE id_producto=46;

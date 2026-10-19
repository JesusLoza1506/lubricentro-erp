-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1080000,valor_stock_micro=16200000000,version=32 WHERE id_producto=113;
UPDATE productos SET stock_actual=328,valor_stock_micro=8265600000,version=11 WHERE id_producto=45;
UPDATE productos SET stock_actual=1080000,valor_stock_micro=29160000000,version=32 WHERE id_producto=114;
UPDATE productos SET stock_actual=328,valor_stock_micro=8856000000,version=11 WHERE id_producto=46;

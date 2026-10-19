-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1024000,valor_stock_micro=15360000000,version=46 WHERE id_producto=113;
UPDATE productos SET stock_actual=321,valor_stock_micro=6933600000,version=16 WHERE id_producto=43;
UPDATE productos SET stock_actual=1024000,valor_stock_micro=27648000000,version=46 WHERE id_producto=114;
UPDATE productos SET stock_actual=321,valor_stock_micro=7511400000,version=16 WHERE id_producto=44;

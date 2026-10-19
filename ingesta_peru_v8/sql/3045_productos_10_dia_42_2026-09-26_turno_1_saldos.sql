-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1032000,valor_stock_micro=15480000000,version=44 WHERE id_producto=113;
UPDATE productos SET stock_actual=321,valor_stock_micro=4622400000,version=16 WHERE id_producto=39;
UPDATE productos SET stock_actual=1032000,valor_stock_micro=27864000000,version=44 WHERE id_producto=114;
UPDATE productos SET stock_actual=321,valor_stock_micro=5200200000,version=16 WHERE id_producto=40;

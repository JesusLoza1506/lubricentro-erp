-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1112000,valor_stock_micro=16680000000,version=24 WHERE id_producto=113;
UPDATE productos SET stock_actual=331,valor_stock_micro=4766400000,version=9 WHERE id_producto=39;
UPDATE productos SET stock_actual=1112000,valor_stock_micro=30024000000,version=24 WHERE id_producto=114;
UPDATE productos SET stock_actual=331,valor_stock_micro=5362200000,version=9 WHERE id_producto=40;

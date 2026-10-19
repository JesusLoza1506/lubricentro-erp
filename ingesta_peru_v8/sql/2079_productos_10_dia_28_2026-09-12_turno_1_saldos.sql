-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1088000,valor_stock_micro=16320000000,version=30 WHERE id_producto=113;
UPDATE productos SET stock_actual=328,valor_stock_micro=5904000000,version=11 WHERE id_producto=41;
UPDATE productos SET stock_actual=1088000,valor_stock_micro=29376000000,version=30 WHERE id_producto=114;
UPDATE productos SET stock_actual=328,valor_stock_micro=6494400000,version=11 WHERE id_producto=42;

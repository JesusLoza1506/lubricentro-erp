-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1048000,valor_stock_micro=15720000000,version=40 WHERE id_producto=113;
UPDATE productos SET stock_actual=324,valor_stock_micro=5832000000,version=14 WHERE id_producto=41;
UPDATE productos SET stock_actual=1048000,valor_stock_micro=28296000000,version=40 WHERE id_producto=114;
UPDATE productos SET stock_actual=324,valor_stock_micro=6415200000,version=14 WHERE id_producto=42;

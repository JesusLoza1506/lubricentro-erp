-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1068000,valor_stock_micro=16020000000,version=35 WHERE id_producto=113;
UPDATE productos SET stock_actual=325,valor_stock_micro=5850000000,version=13 WHERE id_producto=41;
UPDATE productos SET stock_actual=1068000,valor_stock_micro=28836000000,version=35 WHERE id_producto=114;
UPDATE productos SET stock_actual=325,valor_stock_micro=6435000000,version=13 WHERE id_producto=42;

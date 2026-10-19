-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1148000,valor_stock_micro=17220000000,version=15 WHERE id_producto=113;
UPDATE productos SET stock_actual=335,valor_stock_micro=6030000000,version=6 WHERE id_producto=41;
UPDATE productos SET stock_actual=1148000,valor_stock_micro=30996000000,version=15 WHERE id_producto=114;
UPDATE productos SET stock_actual=335,valor_stock_micro=6633000000,version=6 WHERE id_producto=42;

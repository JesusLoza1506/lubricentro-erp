-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=996000,valor_stock_micro=14940000000,version=53 WHERE id_producto=113;
UPDATE productos SET stock_actual=317,valor_stock_micro=3423600000,version=19 WHERE id_producto=37;
UPDATE productos SET stock_actual=996000,valor_stock_micro=26892000000,version=53 WHERE id_producto=114;
UPDATE productos SET stock_actual=317,valor_stock_micro=3994200000,version=19 WHERE id_producto=38;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1056000,valor_stock_micro=15840000000,version=38 WHERE id_producto=113;
UPDATE productos SET stock_actual=324,valor_stock_micro=3499200000,version=14 WHERE id_producto=37;
UPDATE productos SET stock_actual=1056000,valor_stock_micro=28512000000,version=38 WHERE id_producto=114;
UPDATE productos SET stock_actual=324,valor_stock_micro=4082400000,version=14 WHERE id_producto=38;

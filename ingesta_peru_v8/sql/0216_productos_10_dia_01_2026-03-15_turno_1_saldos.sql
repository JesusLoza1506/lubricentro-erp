-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1196000,valor_stock_micro=17940000000,version=3 WHERE id_producto=113;
UPDATE productos SET stock_actual=339,valor_stock_micro=3661200000,version=3 WHERE id_producto=37;
UPDATE productos SET stock_actual=1196000,valor_stock_micro=32292000000,version=3 WHERE id_producto=114;
UPDATE productos SET stock_actual=339,valor_stock_micro=4271400000,version=3 WHERE id_producto=38;

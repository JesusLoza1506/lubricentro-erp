-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1044000,valor_stock_micro=15660000000,version=41 WHERE id_producto=113;
UPDATE productos SET stock_actual=324,valor_stock_micro=6998400000,version=14 WHERE id_producto=43;
UPDATE productos SET stock_actual=1044000,valor_stock_micro=28188000000,version=41 WHERE id_producto=114;
UPDATE productos SET stock_actual=324,valor_stock_micro=7581600000,version=14 WHERE id_producto=44;

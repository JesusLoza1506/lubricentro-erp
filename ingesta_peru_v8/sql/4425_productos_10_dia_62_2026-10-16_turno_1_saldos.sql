-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=952000,valor_stock_micro=14280000000,version=64 WHERE id_producto=113;
UPDATE productos SET stock_actual=313,valor_stock_micro=4507200000,version=22 WHERE id_producto=39;
UPDATE productos SET stock_actual=952000,valor_stock_micro=25704000000,version=64 WHERE id_producto=114;
UPDATE productos SET stock_actual=313,valor_stock_micro=5070600000,version=22 WHERE id_producto=40;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=984000,valor_stock_micro=14760000000,version=56 WHERE id_producto=113;
UPDATE productos SET stock_actual=317,valor_stock_micro=6847200000,version=19 WHERE id_producto=43;
UPDATE productos SET stock_actual=984000,valor_stock_micro=26568000000,version=56 WHERE id_producto=114;
UPDATE productos SET stock_actual=317,valor_stock_micro=7417800000,version=19 WHERE id_producto=44;

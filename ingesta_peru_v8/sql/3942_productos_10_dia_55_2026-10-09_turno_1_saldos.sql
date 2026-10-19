-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=980000,valor_stock_micro=14700000000,version=57 WHERE id_producto=113;
UPDATE productos SET stock_actual=317,valor_stock_micro=7988400000,version=19 WHERE id_producto=45;
UPDATE productos SET stock_actual=980000,valor_stock_micro=26460000000,version=57 WHERE id_producto=114;
UPDATE productos SET stock_actual=317,valor_stock_micro=8559000000,version=19 WHERE id_producto=46;

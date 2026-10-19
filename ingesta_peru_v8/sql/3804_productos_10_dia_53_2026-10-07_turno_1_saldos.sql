-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=988000,valor_stock_micro=14820000000,version=55 WHERE id_producto=113;
UPDATE productos SET stock_actual=317,valor_stock_micro=5706000000,version=19 WHERE id_producto=41;
UPDATE productos SET stock_actual=988000,valor_stock_micro=26676000000,version=55 WHERE id_producto=114;
UPDATE productos SET stock_actual=317,valor_stock_micro=6276600000,version=19 WHERE id_producto=42;

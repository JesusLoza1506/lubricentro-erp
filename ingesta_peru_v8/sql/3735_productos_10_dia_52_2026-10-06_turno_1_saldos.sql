-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=992000,valor_stock_micro=14880000000,version=54 WHERE id_producto=113;
UPDATE productos SET stock_actual=317,valor_stock_micro=4564800000,version=19 WHERE id_producto=39;
UPDATE productos SET stock_actual=992000,valor_stock_micro=26784000000,version=54 WHERE id_producto=114;
UPDATE productos SET stock_actual=317,valor_stock_micro=5135400000,version=19 WHERE id_producto=40;

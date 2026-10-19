-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=948000,valor_stock_micro=14220000000,version=65 WHERE id_producto=113;
UPDATE productos SET stock_actual=313,valor_stock_micro=5634000000,version=22 WHERE id_producto=41;
UPDATE productos SET stock_actual=948000,valor_stock_micro=25596000000,version=65 WHERE id_producto=114;
UPDATE productos SET stock_actual=313,valor_stock_micro=6197400000,version=22 WHERE id_producto=42;

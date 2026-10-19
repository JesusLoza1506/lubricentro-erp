-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1008000,valor_stock_micro=15120000000,version=50 WHERE id_producto=113;
UPDATE productos SET stock_actual=318,valor_stock_micro=5724000000,version=18 WHERE id_producto=41;
UPDATE productos SET stock_actual=1008000,valor_stock_micro=27216000000,version=50 WHERE id_producto=114;
UPDATE productos SET stock_actual=320,valor_stock_micro=6336000000,version=17 WHERE id_producto=42;

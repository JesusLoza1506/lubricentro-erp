-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1108000,valor_stock_micro=16620000000,version=25 WHERE id_producto=113;
UPDATE productos SET stock_actual=331,valor_stock_micro=5958000000,version=9 WHERE id_producto=41;
UPDATE productos SET stock_actual=1108000,valor_stock_micro=29916000000,version=25 WHERE id_producto=114;
UPDATE productos SET stock_actual=331,valor_stock_micro=6553800000,version=9 WHERE id_producto=42;

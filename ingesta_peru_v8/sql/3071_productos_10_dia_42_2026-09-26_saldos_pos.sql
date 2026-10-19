-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=324,valor_stock_micro=6415200000,version=12 WHERE id_producto=52;
UPDATE productos SET stock_actual=324,valor_stock_micro=6998400000,version=12 WHERE id_producto=53;
UPDATE productos SET stock_actual=324,valor_stock_micro=7581600000,version=12 WHERE id_producto=54;
UPDATE productos SET stock_actual=324,valor_stock_micro=8164800000,version=12 WHERE id_producto=55;
UPDATE productos SET stock_actual=324,valor_stock_micro=8748000000,version=12 WHERE id_producto=56;

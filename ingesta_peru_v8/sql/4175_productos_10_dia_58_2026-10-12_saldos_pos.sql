-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=318,valor_stock_micro=6296400000,version=16 WHERE id_producto=52;
UPDATE productos SET stock_actual=318,valor_stock_micro=6868800000,version=16 WHERE id_producto=53;
UPDATE productos SET stock_actual=318,valor_stock_micro=7441200000,version=16 WHERE id_producto=54;
UPDATE productos SET stock_actual=319,valor_stock_micro=8038800000,version=15 WHERE id_producto=55;
UPDATE productos SET stock_actual=319,valor_stock_micro=8613000000,version=15 WHERE id_producto=56;

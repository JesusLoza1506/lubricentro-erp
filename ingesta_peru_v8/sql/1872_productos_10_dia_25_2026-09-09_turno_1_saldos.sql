-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1100000,valor_stock_micro=16500000000,version=27 WHERE id_producto=113;
UPDATE productos SET stock_actual=329,valor_stock_micro=8290800000,version=10 WHERE id_producto=45;
UPDATE productos SET stock_actual=1100000,valor_stock_micro=29700000000,version=27 WHERE id_producto=114;
UPDATE productos SET stock_actual=329,valor_stock_micro=8883000000,version=10 WHERE id_producto=46;

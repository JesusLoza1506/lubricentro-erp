-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1132000,valor_stock_micro=16980000000,version=19 WHERE id_producto=113;
UPDATE productos SET stock_actual=332,valor_stock_micro=4780800000,version=8 WHERE id_producto=39;
UPDATE productos SET stock_actual=1132000,valor_stock_micro=30564000000,version=19 WHERE id_producto=114;
UPDATE productos SET stock_actual=332,valor_stock_micro=5378400000,version=8 WHERE id_producto=40;

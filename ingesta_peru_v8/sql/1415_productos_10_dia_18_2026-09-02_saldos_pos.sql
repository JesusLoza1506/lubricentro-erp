-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=332,valor_stock_micro=6573600000,version=7 WHERE id_producto=52;
UPDATE productos SET stock_actual=332,valor_stock_micro=7171200000,version=7 WHERE id_producto=53;
UPDATE productos SET stock_actual=332,valor_stock_micro=7768800000,version=7 WHERE id_producto=54;
UPDATE productos SET stock_actual=333,valor_stock_micro=8391600000,version=6 WHERE id_producto=55;
UPDATE productos SET stock_actual=333,valor_stock_micro=8991000000,version=6 WHERE id_producto=56;

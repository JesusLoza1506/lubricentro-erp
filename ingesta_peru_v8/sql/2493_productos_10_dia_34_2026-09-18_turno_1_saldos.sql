-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1064000,valor_stock_micro=15960000000,version=36 WHERE id_producto=113;
UPDATE productos SET stock_actual=325,valor_stock_micro=7020000000,version=13 WHERE id_producto=43;
UPDATE productos SET stock_actual=1064000,valor_stock_micro=28728000000,version=36 WHERE id_producto=114;
UPDATE productos SET stock_actual=325,valor_stock_micro=7605000000,version=13 WHERE id_producto=44;

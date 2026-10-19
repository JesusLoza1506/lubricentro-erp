-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1036000,valor_stock_micro=15540000000,version=43 WHERE id_producto=113;
UPDATE productos SET stock_actual=321,valor_stock_micro=3466800000,version=16 WHERE id_producto=37;
UPDATE productos SET stock_actual=1036000,valor_stock_micro=27972000000,version=43 WHERE id_producto=114;
UPDATE productos SET stock_actual=321,valor_stock_micro=4044600000,version=16 WHERE id_producto=38;

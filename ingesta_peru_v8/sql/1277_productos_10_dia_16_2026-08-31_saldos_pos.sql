-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=333,valor_stock_micro=6593400000,version=7 WHERE id_producto=42;
UPDATE productos SET stock_actual=333,valor_stock_micro=7192800000,version=7 WHERE id_producto=43;
UPDATE productos SET stock_actual=333,valor_stock_micro=7792200000,version=7 WHERE id_producto=44;
UPDATE productos SET stock_actual=333,valor_stock_micro=8391600000,version=7 WHERE id_producto=45;
UPDATE productos SET stock_actual=333,valor_stock_micro=8991000000,version=7 WHERE id_producto=46;

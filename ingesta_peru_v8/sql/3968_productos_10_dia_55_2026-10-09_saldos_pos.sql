-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=315,valor_stock_micro=3402000000,version=20 WHERE id_producto=37;
UPDATE productos SET stock_actual=315,valor_stock_micro=3969000000,version=20 WHERE id_producto=38;
UPDATE productos SET stock_actual=315,valor_stock_micro=4536000000,version=20 WHERE id_producto=39;
UPDATE productos SET stock_actual=315,valor_stock_micro=5103000000,version=20 WHERE id_producto=40;
UPDATE productos SET stock_actual=315,valor_stock_micro=5670000000,version=20 WHERE id_producto=41;

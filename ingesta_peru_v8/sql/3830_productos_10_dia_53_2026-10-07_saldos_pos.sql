-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=320,valor_stock_micro=3456000000,version=15 WHERE id_producto=67;
UPDATE productos SET stock_actual=320,valor_stock_micro=4032000000,version=15 WHERE id_producto=68;
UPDATE productos SET stock_actual=320,valor_stock_micro=4608000000,version=15 WHERE id_producto=69;
UPDATE productos SET stock_actual=321,valor_stock_micro=5200200000,version=14 WHERE id_producto=70;
UPDATE productos SET stock_actual=321,valor_stock_micro=5778000000,version=14 WHERE id_producto=71;

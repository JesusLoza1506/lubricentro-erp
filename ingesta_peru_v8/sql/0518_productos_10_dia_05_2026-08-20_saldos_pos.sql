-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=3639600000,version=4 WHERE id_producto=67;
UPDATE productos SET stock_actual=337,valor_stock_micro=4246200000,version=4 WHERE id_producto=68;
UPDATE productos SET stock_actual=337,valor_stock_micro=4852800000,version=4 WHERE id_producto=69;
UPDATE productos SET stock_actual=337,valor_stock_micro=5459400000,version=4 WHERE id_producto=70;
UPDATE productos SET stock_actual=337,valor_stock_micro=6066000000,version=4 WHERE id_producto=71;

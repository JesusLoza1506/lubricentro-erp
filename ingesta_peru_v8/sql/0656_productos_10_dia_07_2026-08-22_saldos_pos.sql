-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=336,valor_stock_micro=3628800000,version=5 WHERE id_producto=37;
UPDATE productos SET stock_actual=336,valor_stock_micro=4233600000,version=5 WHERE id_producto=38;
UPDATE productos SET stock_actual=336,valor_stock_micro=4838400000,version=5 WHERE id_producto=39;
UPDATE productos SET stock_actual=336,valor_stock_micro=5443200000,version=5 WHERE id_producto=40;
UPDATE productos SET stock_actual=337,valor_stock_micro=6066000000,version=4 WHERE id_producto=41;

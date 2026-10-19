-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1168000,valor_stock_micro=17520000000,version=10 WHERE id_producto=113;
UPDATE productos SET stock_actual=336,valor_stock_micro=6048000000,version=5 WHERE id_producto=41;
UPDATE productos SET stock_actual=1168000,valor_stock_micro=31536000000,version=10 WHERE id_producto=114;
UPDATE productos SET stock_actual=338,valor_stock_micro=6692400000,version=4 WHERE id_producto=42;

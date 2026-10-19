-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1116000,valor_stock_micro=16740000000,version=23 WHERE id_producto=113;
UPDATE productos SET stock_actual=331,valor_stock_micro=3574800000,version=9 WHERE id_producto=37;
UPDATE productos SET stock_actual=1116000,valor_stock_micro=30132000000,version=23 WHERE id_producto=114;
UPDATE productos SET stock_actual=331,valor_stock_micro=4170600000,version=9 WHERE id_producto=38;

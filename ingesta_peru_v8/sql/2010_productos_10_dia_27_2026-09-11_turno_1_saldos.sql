-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1092000,valor_stock_micro=16380000000,version=29 WHERE id_producto=113;
UPDATE productos SET stock_actual=328,valor_stock_micro=4723200000,version=11 WHERE id_producto=39;
UPDATE productos SET stock_actual=1092000,valor_stock_micro=29484000000,version=29 WHERE id_producto=114;
UPDATE productos SET stock_actual=328,valor_stock_micro=5313600000,version=11 WHERE id_producto=40;

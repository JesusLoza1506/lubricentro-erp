-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1072000,valor_stock_micro=16080000000,version=34 WHERE id_producto=113;
UPDATE productos SET stock_actual=325,valor_stock_micro=4680000000,version=13 WHERE id_producto=39;
UPDATE productos SET stock_actual=1072000,valor_stock_micro=28944000000,version=34 WHERE id_producto=114;
UPDATE productos SET stock_actual=325,valor_stock_micro=5265000000,version=13 WHERE id_producto=40;

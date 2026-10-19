-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=329,valor_stock_micro=6514200000,version=9 WHERE id_producto=62;
UPDATE productos SET stock_actual=329,valor_stock_micro=7106400000,version=9 WHERE id_producto=63;
UPDATE productos SET stock_actual=329,valor_stock_micro=7698600000,version=9 WHERE id_producto=64;
UPDATE productos SET stock_actual=330,valor_stock_micro=8316000000,version=8 WHERE id_producto=65;
UPDATE productos SET stock_actual=330,valor_stock_micro=8910000000,version=8 WHERE id_producto=66;

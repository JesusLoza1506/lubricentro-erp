-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1028000,valor_stock_micro=15420000000,version=45 WHERE id_producto=113;
UPDATE productos SET stock_actual=321,valor_stock_micro=5778000000,version=16 WHERE id_producto=41;
UPDATE productos SET stock_actual=1028000,valor_stock_micro=27756000000,version=45 WHERE id_producto=114;
UPDATE productos SET stock_actual=321,valor_stock_micro=6355800000,version=16 WHERE id_producto=42;

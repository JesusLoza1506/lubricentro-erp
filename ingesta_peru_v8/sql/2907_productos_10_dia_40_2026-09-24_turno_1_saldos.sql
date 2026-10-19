-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1040000,valor_stock_micro=15600000000,version=42 WHERE id_producto=113;
UPDATE productos SET stock_actual=324,valor_stock_micro=8164800000,version=14 WHERE id_producto=45;
UPDATE productos SET stock_actual=1040000,valor_stock_micro=28080000000,version=42 WHERE id_producto=114;
UPDATE productos SET stock_actual=324,valor_stock_micro=8748000000,version=14 WHERE id_producto=46;

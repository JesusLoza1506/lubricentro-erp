-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=976000,valor_stock_micro=14640000000,version=58 WHERE id_producto=113;
UPDATE productos SET stock_actual=314,valor_stock_micro=3391200000,version=21 WHERE id_producto=37;
UPDATE productos SET stock_actual=976000,valor_stock_micro=26352000000,version=58 WHERE id_producto=114;
UPDATE productos SET stock_actual=314,valor_stock_micro=3956400000,version=21 WHERE id_producto=38;

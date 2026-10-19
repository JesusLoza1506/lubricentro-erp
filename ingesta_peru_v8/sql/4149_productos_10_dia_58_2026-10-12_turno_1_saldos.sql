-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=968000,valor_stock_micro=14520000000,version=60 WHERE id_producto=113;
UPDATE productos SET stock_actual=314,valor_stock_micro=5652000000,version=21 WHERE id_producto=41;
UPDATE productos SET stock_actual=968000,valor_stock_micro=26136000000,version=60 WHERE id_producto=114;
UPDATE productos SET stock_actual=314,valor_stock_micro=6217200000,version=21 WHERE id_producto=42;

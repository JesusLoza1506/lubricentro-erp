-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=323,valor_stock_micro=7558200000,version=13 WHERE id_producto=54;
UPDATE productos SET stock_actual=323,valor_stock_micro=7558200000,version=13 WHERE id_producto=64;

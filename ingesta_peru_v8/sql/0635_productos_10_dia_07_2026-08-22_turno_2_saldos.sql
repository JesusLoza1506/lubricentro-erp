-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=7279200000,version=4 WHERE id_producto=53;
UPDATE productos SET stock_actual=337,valor_stock_micro=7279200000,version=4 WHERE id_producto=63;

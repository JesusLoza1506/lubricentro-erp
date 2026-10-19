-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=337,valor_stock_micro=7885800000,version=4 WHERE id_producto=54;
UPDATE productos SET stock_actual=337,valor_stock_micro=7885800000,version=4 WHERE id_producto=64;

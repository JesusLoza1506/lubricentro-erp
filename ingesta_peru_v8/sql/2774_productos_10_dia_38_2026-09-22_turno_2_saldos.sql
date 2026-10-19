-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=7628400000,version=11 WHERE id_producto=54;
UPDATE productos SET stock_actual=326,valor_stock_micro=7628400000,version=11 WHERE id_producto=64;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=8215200000,version=11 WHERE id_producto=55;
UPDATE productos SET stock_actual=326,valor_stock_micro=8215200000,version=11 WHERE id_producto=65;

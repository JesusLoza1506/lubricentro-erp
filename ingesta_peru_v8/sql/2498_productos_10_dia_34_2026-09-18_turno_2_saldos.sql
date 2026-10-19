-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=5281200000,version=11 WHERE id_producto=50;
UPDATE productos SET stock_actual=328,valor_stock_micro=5313600000,version=10 WHERE id_producto=60;

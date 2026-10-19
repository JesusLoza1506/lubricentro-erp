-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=331,valor_stock_micro=5362200000,version=8 WHERE id_producto=50;
UPDATE productos SET stock_actual=331,valor_stock_micro=5362200000,version=8 WHERE id_producto=60;

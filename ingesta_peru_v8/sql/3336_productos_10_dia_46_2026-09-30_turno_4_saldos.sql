-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=294,valor_stock_micro=2822400000,version=48 WHERE id_producto=105;
UPDATE productos SET stock_actual=325,valor_stock_micro=6435000000,version=12 WHERE id_producto=72;

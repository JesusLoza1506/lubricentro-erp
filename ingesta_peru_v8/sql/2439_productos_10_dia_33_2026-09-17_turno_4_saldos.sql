-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=307,valor_stock_micro=2947200000,version=35 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=4723200000,version=10 WHERE id_producto=69;

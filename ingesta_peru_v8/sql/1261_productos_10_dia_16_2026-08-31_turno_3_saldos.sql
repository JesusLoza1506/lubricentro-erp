-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1120000,valor_stock_micro=10080000000,version=18 WHERE id_producto=119;
UPDATE productos SET stock_actual=1160000,valor_stock_micro=22272000000,version=18 WHERE id_producto=117;

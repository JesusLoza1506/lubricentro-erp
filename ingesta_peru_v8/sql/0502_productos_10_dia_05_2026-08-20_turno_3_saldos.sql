-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1175000,valor_stock_micro=10575000000,version=7 WHERE id_producto=119;
UPDATE productos SET stock_actual=1187500,valor_stock_micro=22800000000,version=7 WHERE id_producto=117;

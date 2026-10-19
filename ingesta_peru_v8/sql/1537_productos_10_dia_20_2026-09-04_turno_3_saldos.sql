-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1100000,valor_stock_micro=9900000000,version=22 WHERE id_producto=119;
UPDATE productos SET stock_actual=1150000,valor_stock_micro=22080000000,version=22 WHERE id_producto=117;

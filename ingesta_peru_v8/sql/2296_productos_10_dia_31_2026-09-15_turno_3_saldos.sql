-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1045000,valor_stock_micro=9405000000,version=33 WHERE id_producto=119;
UPDATE productos SET stock_actual=1122500,valor_stock_micro=21552000000,version=33 WHERE id_producto=117;

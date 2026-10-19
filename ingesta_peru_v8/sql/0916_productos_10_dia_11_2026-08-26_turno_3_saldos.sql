-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1145000,valor_stock_micro=10305000000,version=13 WHERE id_producto=119;
UPDATE productos SET stock_actual=1172500,valor_stock_micro=22512000000,version=13 WHERE id_producto=117;

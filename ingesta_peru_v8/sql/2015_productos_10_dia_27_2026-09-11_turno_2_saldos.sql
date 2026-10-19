-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=329,valor_stock_micro=7106400000,version=9 WHERE id_producto=53;
UPDATE productos SET stock_actual=331,valor_stock_micro=7149600000,version=8 WHERE id_producto=63;

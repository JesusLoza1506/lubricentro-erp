-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1135000,valor_stock_micro=10215000000,version=15 WHERE id_producto=119;
UPDATE productos SET stock_actual=1167500,valor_stock_micro=22416000000,version=15 WHERE id_producto=117;

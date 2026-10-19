-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=990000,valor_stock_micro=8910000000,version=44 WHERE id_producto=119;
UPDATE productos SET stock_actual=1095000,valor_stock_micro=21024000000,version=44 WHERE id_producto=117;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=895000,valor_stock_micro=8055000000,version=63 WHERE id_producto=119;
UPDATE productos SET stock_actual=1047500,valor_stock_micro=20112000000,version=63 WHERE id_producto=117;

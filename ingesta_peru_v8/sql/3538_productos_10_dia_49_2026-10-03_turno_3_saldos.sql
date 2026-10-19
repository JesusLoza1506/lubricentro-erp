-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=955000,valor_stock_micro=8595000000,version=51 WHERE id_producto=119;
UPDATE productos SET stock_actual=1077500,valor_stock_micro=20688000000,version=51 WHERE id_producto=117;

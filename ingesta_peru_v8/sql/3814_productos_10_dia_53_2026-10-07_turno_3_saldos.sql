-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=935000,valor_stock_micro=8415000000,version=55 WHERE id_producto=119;
UPDATE productos SET stock_actual=1067500,valor_stock_micro=20496000000,version=55 WHERE id_producto=117;

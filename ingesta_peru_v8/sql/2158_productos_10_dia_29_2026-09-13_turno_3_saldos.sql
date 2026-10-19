-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1055000,valor_stock_micro=9495000000,version=31 WHERE id_producto=119;
UPDATE productos SET stock_actual=1127500,valor_stock_micro=21648000000,version=31 WHERE id_producto=117;

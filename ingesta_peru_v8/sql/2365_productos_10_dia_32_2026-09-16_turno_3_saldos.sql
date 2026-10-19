-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1040000,valor_stock_micro=9360000000,version=34 WHERE id_producto=119;
UPDATE productos SET stock_actual=1120000,valor_stock_micro=21504000000,version=34 WHERE id_producto=117;

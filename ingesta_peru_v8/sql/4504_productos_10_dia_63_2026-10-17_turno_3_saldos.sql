-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=885000,valor_stock_micro=7965000000,version=65 WHERE id_producto=119;
UPDATE productos SET stock_actual=1042500,valor_stock_micro=20016000000,version=65 WHERE id_producto=117;

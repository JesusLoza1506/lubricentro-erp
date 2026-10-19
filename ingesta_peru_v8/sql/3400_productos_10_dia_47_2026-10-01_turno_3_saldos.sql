-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=965000,valor_stock_micro=8685000000,version=49 WHERE id_producto=119;
UPDATE productos SET stock_actual=1082500,valor_stock_micro=20784000000,version=49 WHERE id_producto=117;

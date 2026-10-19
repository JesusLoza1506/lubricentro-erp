-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=890000,valor_stock_micro=8010000000,version=64 WHERE id_producto=119;
UPDATE productos SET stock_actual=1045000,valor_stock_micro=20064000000,version=64 WHERE id_producto=117;

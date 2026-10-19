-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=920000,valor_stock_micro=8280000000,version=58 WHERE id_producto=119;
UPDATE productos SET stock_actual=1060000,valor_stock_micro=20352000000,version=58 WHERE id_producto=117;

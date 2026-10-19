-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=930000,valor_stock_micro=8370000000,version=56 WHERE id_producto=119;
UPDATE productos SET stock_actual=1065000,valor_stock_micro=20448000000,version=56 WHERE id_producto=117;

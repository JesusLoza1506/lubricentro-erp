-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1010000,valor_stock_micro=9090000000,version=40 WHERE id_producto=119;
UPDATE productos SET stock_actual=1105000,valor_stock_micro=21216000000,version=40 WHERE id_producto=117;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=308,valor_stock_micro=2956800000,version=34 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=4132800000,version=10 WHERE id_producto=68;

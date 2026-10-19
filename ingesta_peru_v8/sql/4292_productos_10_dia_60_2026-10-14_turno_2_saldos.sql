-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=318,valor_stock_micro=8586000000,version=16 WHERE id_producto=56;
UPDATE productos SET stock_actual=320,valor_stock_micro=8640000000,version=15 WHERE id_producto=66;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=318,valor_stock_micro=8013600000,version=16 WHERE id_producto=55;
UPDATE productos SET stock_actual=320,valor_stock_micro=8064000000,version=15 WHERE id_producto=65;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=288,valor_stock_micro=2764800000,version=54 WHERE id_producto=105;
UPDATE productos SET stock_actual=322,valor_stock_micro=4057200000,version=14 WHERE id_producto=68;

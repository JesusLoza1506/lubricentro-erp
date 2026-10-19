-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=334,valor_stock_micro=6012000000,version=6 WHERE id_producto=51;
UPDATE productos SET stock_actual=334,valor_stock_micro=6012000000,version=6 WHERE id_producto=61;

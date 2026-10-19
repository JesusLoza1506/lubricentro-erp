-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=299,valor_stock_micro=2870400000,version=43 WHERE id_producto=105;
UPDATE productos SET stock_actual=325,valor_stock_micro=3510000000,version=12 WHERE id_producto=67;

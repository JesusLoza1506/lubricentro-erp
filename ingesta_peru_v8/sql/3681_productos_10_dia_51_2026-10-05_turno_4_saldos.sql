-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=289,valor_stock_micro=2774400000,version=53 WHERE id_producto=105;
UPDATE productos SET stock_actual=322,valor_stock_micro=3477600000,version=14 WHERE id_producto=67;

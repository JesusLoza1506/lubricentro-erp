-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=287,valor_stock_micro=2755200000,version=55 WHERE id_producto=105;
UPDATE productos SET stock_actual=322,valor_stock_micro=4636800000,version=14 WHERE id_producto=69;

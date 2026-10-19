-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=339,valor_stock_micro=3254400000,version=3 WHERE id_producto=105;
UPDATE productos SET stock_actual=339,valor_stock_micro=3661200000,version=3 WHERE id_producto=67;

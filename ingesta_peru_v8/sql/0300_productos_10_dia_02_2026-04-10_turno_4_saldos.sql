-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=338,valor_stock_micro=3244800000,version=4 WHERE id_producto=105;
UPDATE productos SET stock_actual=339,valor_stock_micro=4271400000,version=3 WHERE id_producto=68;

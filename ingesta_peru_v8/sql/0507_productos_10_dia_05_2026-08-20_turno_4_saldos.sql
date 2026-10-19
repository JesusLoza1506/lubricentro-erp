-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=335,valor_stock_micro=3216000000,version=7 WHERE id_producto=105;
UPDATE productos SET stock_actual=339,valor_stock_micro=6102000000,version=3 WHERE id_producto=71;

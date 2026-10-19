-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=319,valor_stock_micro=3062400000,version=23 WHERE id_producto=105;
UPDATE productos SET stock_actual=333,valor_stock_micro=3596400000,version=7 WHERE id_producto=67;

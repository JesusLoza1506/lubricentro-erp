-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=279,valor_stock_micro=2678400000,version=63 WHERE id_producto=105;
UPDATE productos SET stock_actual=319,valor_stock_micro=3445200000,version=16 WHERE id_producto=67;

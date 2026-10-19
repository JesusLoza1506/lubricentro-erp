-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=305,valor_stock_micro=2928000000,version=37 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=5904000000,version=10 WHERE id_producto=71;

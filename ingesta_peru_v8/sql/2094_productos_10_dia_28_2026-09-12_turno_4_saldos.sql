-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=312,valor_stock_micro=2995200000,version=30 WHERE id_producto=105;
UPDATE productos SET stock_actual=331,valor_stock_micro=7745400000,version=8 WHERE id_producto=74;

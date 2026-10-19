-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=304,valor_stock_micro=2918400000,version=38 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=6494400000,version=10 WHERE id_producto=72;

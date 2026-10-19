-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=302,valor_stock_micro=2899200000,version=40 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=7675200000,version=10 WHERE id_producto=74;

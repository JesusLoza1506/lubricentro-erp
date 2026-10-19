-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=306,valor_stock_micro=2937600000,version=36 WHERE id_producto=105;
UPDATE productos SET stock_actual=328,valor_stock_micro=5313600000,version=10 WHERE id_producto=70;

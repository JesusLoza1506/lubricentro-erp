-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=334,valor_stock_micro=3607200000,version=6 WHERE id_producto=57;
UPDATE productos SET stock_actual=335,valor_stock_micro=4221000000,version=5 WHERE id_producto=58;
UPDATE productos SET stock_actual=335,valor_stock_micro=4824000000,version=5 WHERE id_producto=59;
UPDATE productos SET stock_actual=335,valor_stock_micro=5427000000,version=5 WHERE id_producto=60;
UPDATE productos SET stock_actual=335,valor_stock_micro=6030000000,version=5 WHERE id_producto=61;

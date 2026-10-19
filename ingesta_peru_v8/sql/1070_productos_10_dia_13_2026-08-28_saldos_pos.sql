-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=334,valor_stock_micro=3607200000,version=6 WHERE id_producto=67;
UPDATE productos SET stock_actual=334,valor_stock_micro=4208400000,version=6 WHERE id_producto=68;
UPDATE productos SET stock_actual=334,valor_stock_micro=4809600000,version=6 WHERE id_producto=69;
UPDATE productos SET stock_actual=335,valor_stock_micro=5427000000,version=5 WHERE id_producto=70;
UPDATE productos SET stock_actual=335,valor_stock_micro=6030000000,version=5 WHERE id_producto=71;

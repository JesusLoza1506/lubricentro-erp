-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1188000,valor_stock_micro=17820000000,version=5 WHERE id_producto=113;
UPDATE productos SET stock_actual=339,valor_stock_micro=6102000000,version=3 WHERE id_producto=41;
UPDATE productos SET stock_actual=1188000,valor_stock_micro=32076000000,version=5 WHERE id_producto=114;
UPDATE productos SET stock_actual=339,valor_stock_micro=6712200000,version=3 WHERE id_producto=42;

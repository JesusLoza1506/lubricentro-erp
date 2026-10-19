-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1144000,valor_stock_micro=17160000000,version=16 WHERE id_producto=113;
UPDATE productos SET stock_actual=335,valor_stock_micro=7236000000,version=6 WHERE id_producto=43;
UPDATE productos SET stock_actual=1144000,valor_stock_micro=30888000000,version=16 WHERE id_producto=114;
UPDATE productos SET stock_actual=335,valor_stock_micro=7839000000,version=6 WHERE id_producto=44;

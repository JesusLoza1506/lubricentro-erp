-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1140000,valor_stock_micro=17100000000,version=17 WHERE id_producto=113;
UPDATE productos SET stock_actual=335,valor_stock_micro=8442000000,version=6 WHERE id_producto=45;
UPDATE productos SET stock_actual=1140000,valor_stock_micro=30780000000,version=17 WHERE id_producto=114;
UPDATE productos SET stock_actual=335,valor_stock_micro=9045000000,version=6 WHERE id_producto=46;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=318,valor_stock_micro=6296400000,version=18 WHERE id_producto=42;
UPDATE productos SET stock_actual=319,valor_stock_micro=6890400000,version=17 WHERE id_producto=43;
UPDATE productos SET stock_actual=319,valor_stock_micro=7464600000,version=17 WHERE id_producto=44;
UPDATE productos SET stock_actual=319,valor_stock_micro=8038800000,version=17 WHERE id_producto=45;
UPDATE productos SET stock_actual=319,valor_stock_micro=8613000000,version=17 WHERE id_producto=46;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1184000,valor_stock_micro=17760000000,version=6 WHERE id_producto=113;
UPDATE productos SET stock_actual=339,valor_stock_micro=7322400000,version=3 WHERE id_producto=43;
UPDATE productos SET stock_actual=1184000,valor_stock_micro=31968000000,version=6 WHERE id_producto=114;
UPDATE productos SET stock_actual=339,valor_stock_micro=7932600000,version=3 WHERE id_producto=44;

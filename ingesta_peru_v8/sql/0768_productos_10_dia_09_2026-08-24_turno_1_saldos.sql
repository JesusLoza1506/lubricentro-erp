-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1164000,valor_stock_micro=17460000000,version=11 WHERE id_producto=113;
UPDATE productos SET stock_actual=336,valor_stock_micro=7257600000,version=5 WHERE id_producto=43;
UPDATE productos SET stock_actual=1164000,valor_stock_micro=31428000000,version=11 WHERE id_producto=114;
UPDATE productos SET stock_actual=336,valor_stock_micro=7862400000,version=5 WHERE id_producto=44;

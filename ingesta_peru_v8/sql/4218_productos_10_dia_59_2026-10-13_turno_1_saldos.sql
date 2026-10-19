-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=964000,valor_stock_micro=14460000000,version=61 WHERE id_producto=113;
UPDATE productos SET stock_actual=314,valor_stock_micro=6782400000,version=21 WHERE id_producto=43;
UPDATE productos SET stock_actual=964000,valor_stock_micro=26028000000,version=61 WHERE id_producto=114;
UPDATE productos SET stock_actual=314,valor_stock_micro=7347600000,version=21 WHERE id_producto=44;

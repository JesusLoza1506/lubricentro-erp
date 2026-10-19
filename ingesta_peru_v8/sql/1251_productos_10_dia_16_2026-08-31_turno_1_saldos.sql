-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1136000,valor_stock_micro=17040000000,version=18 WHERE id_producto=113;
UPDATE productos SET stock_actual=332,valor_stock_micro=3585600000,version=8 WHERE id_producto=37;
UPDATE productos SET stock_actual=1136000,valor_stock_micro=30672000000,version=18 WHERE id_producto=114;
UPDATE productos SET stock_actual=332,valor_stock_micro=4183200000,version=8 WHERE id_producto=38;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=331,valor_stock_micro=3574800000,version=8 WHERE id_producto=67;
UPDATE productos SET stock_actual=332,valor_stock_micro=4183200000,version=7 WHERE id_producto=68;
UPDATE productos SET stock_actual=332,valor_stock_micro=4780800000,version=7 WHERE id_producto=69;
UPDATE productos SET stock_actual=332,valor_stock_micro=5378400000,version=7 WHERE id_producto=70;
UPDATE productos SET stock_actual=332,valor_stock_micro=5976000000,version=7 WHERE id_producto=71;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1128000,valor_stock_micro=16920000000,version=20 WHERE id_producto=113;
UPDATE productos SET stock_actual=332,valor_stock_micro=5976000000,version=8 WHERE id_producto=41;
UPDATE productos SET stock_actual=1128000,valor_stock_micro=30456000000,version=20 WHERE id_producto=114;
UPDATE productos SET stock_actual=332,valor_stock_micro=6573600000,version=8 WHERE id_producto=42;

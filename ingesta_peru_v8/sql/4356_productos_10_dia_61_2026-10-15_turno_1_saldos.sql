-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=956000,valor_stock_micro=14340000000,version=63 WHERE id_producto=113;
UPDATE productos SET stock_actual=313,valor_stock_micro=3380400000,version=22 WHERE id_producto=37;
UPDATE productos SET stock_actual=956000,valor_stock_micro=25812000000,version=63 WHERE id_producto=114;
UPDATE productos SET stock_actual=313,valor_stock_micro=3943800000,version=22 WHERE id_producto=38;

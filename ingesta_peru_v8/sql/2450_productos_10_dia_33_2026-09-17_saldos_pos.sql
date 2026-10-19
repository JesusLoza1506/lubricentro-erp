-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=3520800000,version=11 WHERE id_producto=47;
UPDATE productos SET stock_actual=326,valor_stock_micro=4107600000,version=11 WHERE id_producto=48;
UPDATE productos SET stock_actual=326,valor_stock_micro=4694400000,version=11 WHERE id_producto=49;
UPDATE productos SET stock_actual=327,valor_stock_micro=5297400000,version=10 WHERE id_producto=50;
UPDATE productos SET stock_actual=327,valor_stock_micro=5886000000,version=10 WHERE id_producto=51;

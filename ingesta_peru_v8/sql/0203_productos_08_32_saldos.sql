-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1200000,valor_stock_micro=23040000000,ultimo_costo_micro=19200000,version=2 WHERE id_producto=117;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=27360000000,ultimo_costo_micro=22800000,version=2 WHERE id_producto=118;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=10800000000,ultimo_costo_micro=9000000,version=2 WHERE id_producto=119;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=12960000000,ultimo_costo_micro=10800000,version=2 WHERE id_producto=120;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1200000,valor_stock_micro=18000000000,ultimo_costo_micro=15000000,version=2 WHERE id_producto=113;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=32400000000,ultimo_costo_micro=27000000,version=2 WHERE id_producto=114;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=21600000000,ultimo_costo_micro=18000000,version=2 WHERE id_producto=115;
UPDATE productos SET stock_actual=1200000,valor_stock_micro=25200000000,ultimo_costo_micro=21000000,version=2 WHERE id_producto=116;

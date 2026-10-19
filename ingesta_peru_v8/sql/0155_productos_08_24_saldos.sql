-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=6120000000,ultimo_costo_micro=18000000,version=2 WHERE id_producto=85;
UPDATE productos SET stock_actual=340,valor_stock_micro=6936000000,ultimo_costo_micro=20400000,version=2 WHERE id_producto=86;
UPDATE productos SET stock_actual=340,valor_stock_micro=7752000000,ultimo_costo_micro=22800000,version=2 WHERE id_producto=87;
UPDATE productos SET stock_actual=340,valor_stock_micro=8568000000,ultimo_costo_micro=25200000,version=2 WHERE id_producto=88;

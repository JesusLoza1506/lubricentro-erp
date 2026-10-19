-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=7752000000,ultimo_costo_micro=22800000,version=2 WHERE id_producto=81;
UPDATE productos SET stock_actual=340,valor_stock_micro=8568000000,ultimo_costo_micro=25200000,version=2 WHERE id_producto=82;
UPDATE productos SET stock_actual=340,valor_stock_micro=4488000000,ultimo_costo_micro=13200000,version=2 WHERE id_producto=83;
UPDATE productos SET stock_actual=340,valor_stock_micro=5304000000,ultimo_costo_micro=15600000,version=2 WHERE id_producto=84;

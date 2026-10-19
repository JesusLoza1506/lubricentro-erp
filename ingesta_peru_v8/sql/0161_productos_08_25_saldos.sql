-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=4488000000,ultimo_costo_micro=13200000,version=2 WHERE id_producto=89;
UPDATE productos SET stock_actual=340,valor_stock_micro=5304000000,ultimo_costo_micro=15600000,version=2 WHERE id_producto=90;
UPDATE productos SET stock_actual=340,valor_stock_micro=6120000000,ultimo_costo_micro=18000000,version=2 WHERE id_producto=91;
UPDATE productos SET stock_actual=340,valor_stock_micro=6936000000,ultimo_costo_micro=20400000,version=2 WHERE id_producto=92;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=7752000000,ultimo_costo_micro=22800000,version=2 WHERE id_producto=5;
UPDATE productos SET stock_actual=340,valor_stock_micro=31008000000,ultimo_costo_micro=91200000,version=2 WHERE id_producto=6;
UPDATE productos SET stock_actual=340,valor_stock_micro=29376000000,ultimo_costo_micro=86400000,version=2 WHERE id_producto=7;
UPDATE productos SET stock_actual=340,valor_stock_micro=38760000000,ultimo_costo_micro=114000000,version=2 WHERE id_producto=8;

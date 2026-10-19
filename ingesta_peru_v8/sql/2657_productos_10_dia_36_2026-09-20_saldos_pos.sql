-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=6454800000,version=11 WHERE id_producto=62;
UPDATE productos SET stock_actual=327,valor_stock_micro=7063200000,version=10 WHERE id_producto=63;
UPDATE productos SET stock_actual=327,valor_stock_micro=7651800000,version=10 WHERE id_producto=64;
UPDATE productos SET stock_actual=327,valor_stock_micro=8240400000,version=10 WHERE id_producto=65;
UPDATE productos SET stock_actual=327,valor_stock_micro=8829000000,version=10 WHERE id_producto=66;

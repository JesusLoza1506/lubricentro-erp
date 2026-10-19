-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=11832000000,ultimo_costo_micro=34800000,version=2 WHERE id_producto=29;
UPDATE productos SET stock_actual=340,valor_stock_micro=47328000000,ultimo_costo_micro=139200000,version=2 WHERE id_producto=30;
UPDATE productos SET stock_actual=340,valor_stock_micro=44880000000,ultimo_costo_micro=132000000,version=2 WHERE id_producto=31;
UPDATE productos SET stock_actual=340,valor_stock_micro=59160000000,ultimo_costo_micro=174000000,version=2 WHERE id_producto=32;

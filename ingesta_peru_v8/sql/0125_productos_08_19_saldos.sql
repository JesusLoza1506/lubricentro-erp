-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=8568000000,ultimo_costo_micro=25200000,version=2 WHERE id_producto=65;
UPDATE productos SET stock_actual=340,valor_stock_micro=9180000000,ultimo_costo_micro=27000000,version=2 WHERE id_producto=66;
UPDATE productos SET stock_actual=340,valor_stock_micro=3672000000,ultimo_costo_micro=10800000,version=2 WHERE id_producto=67;
UPDATE productos SET stock_actual=340,valor_stock_micro=4284000000,ultimo_costo_micro=12600000,version=2 WHERE id_producto=68;

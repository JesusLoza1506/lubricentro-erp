-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=7344000000,ultimo_costo_micro=21600000,version=2 WHERE id_producto=73;
UPDATE productos SET stock_actual=340,valor_stock_micro=7956000000,ultimo_costo_micro=23400000,version=2 WHERE id_producto=74;
UPDATE productos SET stock_actual=340,valor_stock_micro=8568000000,ultimo_costo_micro=25200000,version=2 WHERE id_producto=75;
UPDATE productos SET stock_actual=340,valor_stock_micro=9180000000,ultimo_costo_micro=27000000,version=2 WHERE id_producto=76;

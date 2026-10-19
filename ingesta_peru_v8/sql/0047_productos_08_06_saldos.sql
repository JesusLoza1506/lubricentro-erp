-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=13260000000,ultimo_costo_micro=39000000,version=2 WHERE id_producto=13;
UPDATE productos SET stock_actual=340,valor_stock_micro=53040000000,ultimo_costo_micro=156000000,version=2 WHERE id_producto=14;
UPDATE productos SET stock_actual=340,valor_stock_micro=50184000000,ultimo_costo_micro=147600000,version=2 WHERE id_producto=15;
UPDATE productos SET stock_actual=340,valor_stock_micro=66300000000,ultimo_costo_micro=195000000,version=2 WHERE id_producto=16;

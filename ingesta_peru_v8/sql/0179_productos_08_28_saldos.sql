-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=5712000000,ultimo_costo_micro=16800000,version=2 WHERE id_producto=101;
UPDATE productos SET stock_actual=340,valor_stock_micro=6528000000,ultimo_costo_micro=19200000,version=2 WHERE id_producto=102;
UPDATE productos SET stock_actual=340,valor_stock_micro=7344000000,ultimo_costo_micro=21600000,version=2 WHERE id_producto=103;
UPDATE productos SET stock_actual=340,valor_stock_micro=2448000000,ultimo_costo_micro=7200000,version=2 WHERE id_producto=104;

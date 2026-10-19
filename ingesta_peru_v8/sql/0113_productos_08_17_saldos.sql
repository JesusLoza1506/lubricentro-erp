-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=3672000000,ultimo_costo_micro=10800000,version=2 WHERE id_producto=57;
UPDATE productos SET stock_actual=340,valor_stock_micro=4284000000,ultimo_costo_micro=12600000,version=2 WHERE id_producto=58;
UPDATE productos SET stock_actual=340,valor_stock_micro=4896000000,ultimo_costo_micro=14400000,version=2 WHERE id_producto=59;
UPDATE productos SET stock_actual=340,valor_stock_micro=5508000000,ultimo_costo_micro=16200000,version=2 WHERE id_producto=60;

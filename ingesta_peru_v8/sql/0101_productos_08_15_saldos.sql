-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=4896000000,ultimo_costo_micro=14400000,version=2 WHERE id_producto=49;
UPDATE productos SET stock_actual=340,valor_stock_micro=5508000000,ultimo_costo_micro=16200000,version=2 WHERE id_producto=50;
UPDATE productos SET stock_actual=340,valor_stock_micro=6120000000,ultimo_costo_micro=18000000,version=2 WHERE id_producto=51;
UPDATE productos SET stock_actual=340,valor_stock_micro=6732000000,ultimo_costo_micro=19800000,version=2 WHERE id_producto=52;

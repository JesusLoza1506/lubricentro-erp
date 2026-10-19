-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=12240000000,ultimo_costo_micro=36000000,version=2 WHERE id_producto=25;
UPDATE productos SET stock_actual=340,valor_stock_micro=48960000000,ultimo_costo_micro=144000000,version=2 WHERE id_producto=26;
UPDATE productos SET stock_actual=340,valor_stock_micro=46308000000,ultimo_costo_micro=136200000,version=2 WHERE id_producto=27;
UPDATE productos SET stock_actual=340,valor_stock_micro=61200000000,ultimo_costo_micro=180000000,version=2 WHERE id_producto=28;

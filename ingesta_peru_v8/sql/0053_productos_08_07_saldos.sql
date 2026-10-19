-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=15300000000,ultimo_costo_micro=45000000,version=2 WHERE id_producto=17;
UPDATE productos SET stock_actual=340,valor_stock_micro=61200000000,ultimo_costo_micro=180000000,version=2 WHERE id_producto=18;
UPDATE productos SET stock_actual=340,valor_stock_micro=57936000000,ultimo_costo_micro=170400000,version=2 WHERE id_producto=19;
UPDATE productos SET stock_actual=340,valor_stock_micro=76500000000,ultimo_costo_micro=225000000,version=2 WHERE id_producto=20;

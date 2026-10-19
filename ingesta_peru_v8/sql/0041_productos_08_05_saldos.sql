-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=8160000000,ultimo_costo_micro=24000000,version=2 WHERE id_producto=9;
UPDATE productos SET stock_actual=340,valor_stock_micro=32640000000,ultimo_costo_micro=96000000,version=2 WHERE id_producto=10;
UPDATE productos SET stock_actual=340,valor_stock_micro=30804000000,ultimo_costo_micro=90600000,version=2 WHERE id_producto=11;
UPDATE productos SET stock_actual=340,valor_stock_micro=40800000000,ultimo_costo_micro=120000000,version=2 WHERE id_producto=12;

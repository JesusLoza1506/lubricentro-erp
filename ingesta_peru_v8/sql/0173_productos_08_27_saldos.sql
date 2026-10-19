-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=2448000000,ultimo_costo_micro=7200000,version=2 WHERE id_producto=97;
UPDATE productos SET stock_actual=340,valor_stock_micro=3264000000,ultimo_costo_micro=9600000,version=2 WHERE id_producto=98;
UPDATE productos SET stock_actual=340,valor_stock_micro=4080000000,ultimo_costo_micro=12000000,version=2 WHERE id_producto=99;
UPDATE productos SET stock_actual=340,valor_stock_micro=4896000000,ultimo_costo_micro=14400000,version=2 WHERE id_producto=100;

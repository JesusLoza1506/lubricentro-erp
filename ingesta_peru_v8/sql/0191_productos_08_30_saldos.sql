-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=6528000000,ultimo_costo_micro=19200000,version=2 WHERE id_producto=109;
UPDATE productos SET stock_actual=4,valor_stock_micro=86400000,ultimo_costo_micro=21600000,version=2 WHERE id_producto=110;
UPDATE productos SET stock_actual=4,valor_stock_micro=28800000,ultimo_costo_micro=7200000,version=2 WHERE id_producto=111;
UPDATE productos SET stock_actual=4,valor_stock_micro=38400000,ultimo_costo_micro=9600000,version=2 WHERE id_producto=112;

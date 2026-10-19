-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1090000,valor_stock_micro=9810000000,version=24 WHERE id_producto=119;
UPDATE productos SET stock_actual=1145000,valor_stock_micro=21984000000,version=24 WHERE id_producto=117;

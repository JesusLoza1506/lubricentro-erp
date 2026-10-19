-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1125000,valor_stock_micro=10125000000,version=17 WHERE id_producto=119;
UPDATE productos SET stock_actual=1162500,valor_stock_micro=22320000000,version=17 WHERE id_producto=117;

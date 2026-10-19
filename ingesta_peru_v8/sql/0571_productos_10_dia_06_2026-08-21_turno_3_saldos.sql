-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1170000,valor_stock_micro=10530000000,version=8 WHERE id_producto=119;
UPDATE productos SET stock_actual=1185000,valor_stock_micro=22752000000,version=8 WHERE id_producto=117;

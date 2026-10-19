-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1110000,valor_stock_micro=9990000000,version=20 WHERE id_producto=119;
UPDATE productos SET stock_actual=1155000,valor_stock_micro=22176000000,version=20 WHERE id_producto=117;

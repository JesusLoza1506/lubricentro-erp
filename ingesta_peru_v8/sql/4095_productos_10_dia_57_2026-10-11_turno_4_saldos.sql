-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=283,valor_stock_micro=2716800000,version=59 WHERE id_producto=105;
UPDATE productos SET stock_actual=320,valor_stock_micro=6912000000,version=15 WHERE id_producto=73;

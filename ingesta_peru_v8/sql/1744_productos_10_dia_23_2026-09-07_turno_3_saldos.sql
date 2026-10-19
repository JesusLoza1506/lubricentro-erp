-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1085000,valor_stock_micro=9765000000,version=25 WHERE id_producto=119;
UPDATE productos SET stock_actual=1142500,valor_stock_micro=21936000000,version=25 WHERE id_producto=117;

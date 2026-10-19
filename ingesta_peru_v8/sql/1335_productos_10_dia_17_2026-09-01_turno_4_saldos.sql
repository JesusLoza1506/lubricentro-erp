-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=323,valor_stock_micro=3100800000,version=19 WHERE id_producto=105;
UPDATE productos SET stock_actual=334,valor_stock_micro=7214400000,version=6 WHERE id_producto=73;

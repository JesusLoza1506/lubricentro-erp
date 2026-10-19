-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=324,valor_stock_micro=3110400000,version=18 WHERE id_producto=105;
UPDATE productos SET stock_actual=334,valor_stock_micro=6613200000,version=6 WHERE id_producto=72;

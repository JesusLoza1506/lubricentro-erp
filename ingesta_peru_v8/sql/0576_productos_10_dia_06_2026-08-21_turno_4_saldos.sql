-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=334,valor_stock_micro=3206400000,version=8 WHERE id_producto=105;
UPDATE productos SET stock_actual=339,valor_stock_micro=6712200000,version=3 WHERE id_producto=72;

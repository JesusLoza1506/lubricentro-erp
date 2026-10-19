-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=313,valor_stock_micro=3004800000,version=29 WHERE id_producto=105;
UPDATE productos SET stock_actual=331,valor_stock_micro=7149600000,version=8 WHERE id_producto=73;

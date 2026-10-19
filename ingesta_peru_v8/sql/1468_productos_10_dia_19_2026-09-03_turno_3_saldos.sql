-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1105000,valor_stock_micro=9945000000,version=21 WHERE id_producto=119;
UPDATE productos SET stock_actual=1152500,valor_stock_micro=22128000000,version=21 WHERE id_producto=117;

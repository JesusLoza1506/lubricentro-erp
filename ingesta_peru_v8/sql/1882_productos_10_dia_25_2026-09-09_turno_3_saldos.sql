-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1075000,valor_stock_micro=9675000000,version=27 WHERE id_producto=119;
UPDATE productos SET stock_actual=1137500,valor_stock_micro=21840000000,version=27 WHERE id_producto=117;

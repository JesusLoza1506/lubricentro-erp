-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=945000,valor_stock_micro=8505000000,version=53 WHERE id_producto=119;
UPDATE productos SET stock_actual=1072500,valor_stock_micro=20592000000,version=53 WHERE id_producto=117;

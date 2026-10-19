-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=985000,valor_stock_micro=8865000000,version=45 WHERE id_producto=119;
UPDATE productos SET stock_actual=1092500,valor_stock_micro=20976000000,version=45 WHERE id_producto=117;

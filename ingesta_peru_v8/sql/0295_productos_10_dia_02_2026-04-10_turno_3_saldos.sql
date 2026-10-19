-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1190000,valor_stock_micro=10710000000,version=4 WHERE id_producto=119;
UPDATE productos SET stock_actual=1195000,valor_stock_micro=22944000000,version=4 WHERE id_producto=117;

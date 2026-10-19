-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=925000,valor_stock_micro=8325000000,version=57 WHERE id_producto=119;
UPDATE productos SET stock_actual=1062500,valor_stock_micro=20400000000,version=57 WHERE id_producto=117;

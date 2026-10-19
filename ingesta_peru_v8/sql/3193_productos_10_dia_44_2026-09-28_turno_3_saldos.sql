-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=980000,valor_stock_micro=8820000000,version=46 WHERE id_producto=119;
UPDATE productos SET stock_actual=1090000,valor_stock_micro=20928000000,version=46 WHERE id_producto=117;

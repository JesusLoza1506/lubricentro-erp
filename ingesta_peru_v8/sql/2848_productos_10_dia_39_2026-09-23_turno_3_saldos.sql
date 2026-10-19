-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1005000,valor_stock_micro=9045000000,version=41 WHERE id_producto=119;
UPDATE productos SET stock_actual=1102500,valor_stock_micro=21168000000,version=41 WHERE id_producto=117;

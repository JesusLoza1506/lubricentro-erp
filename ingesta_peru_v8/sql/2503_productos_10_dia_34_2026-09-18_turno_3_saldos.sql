-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1030000,valor_stock_micro=9270000000,version=36 WHERE id_producto=119;
UPDATE productos SET stock_actual=1115000,valor_stock_micro=21408000000,version=36 WHERE id_producto=117;

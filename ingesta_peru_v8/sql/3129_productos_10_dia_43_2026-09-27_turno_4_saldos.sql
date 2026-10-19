-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=297,valor_stock_micro=2851200000,version=45 WHERE id_producto=105;
UPDATE productos SET stock_actual=325,valor_stock_micro=4680000000,version=12 WHERE id_producto=69;

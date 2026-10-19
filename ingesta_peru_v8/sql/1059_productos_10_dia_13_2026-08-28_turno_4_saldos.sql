-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=327,valor_stock_micro=3139200000,version=15 WHERE id_producto=105;
UPDATE productos SET stock_actual=336,valor_stock_micro=4838400000,version=5 WHERE id_producto=69;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=320,valor_stock_micro=6336000000,version=15 WHERE id_producto=52;
UPDATE productos SET stock_actual=320,valor_stock_micro=6336000000,version=15 WHERE id_producto=62;

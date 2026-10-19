-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=336,valor_stock_micro=6652800000,version=5 WHERE id_producto=42;
UPDATE productos SET stock_actual=337,valor_stock_micro=7279200000,version=4 WHERE id_producto=43;
UPDATE productos SET stock_actual=337,valor_stock_micro=7885800000,version=4 WHERE id_producto=44;
UPDATE productos SET stock_actual=337,valor_stock_micro=8492400000,version=4 WHERE id_producto=45;
UPDATE productos SET stock_actual=337,valor_stock_micro=9099000000,version=4 WHERE id_producto=46;

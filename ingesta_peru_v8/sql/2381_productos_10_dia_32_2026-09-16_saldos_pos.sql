-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=6454800000,version=12 WHERE id_producto=42;
UPDATE productos SET stock_actual=326,valor_stock_micro=7041600000,version=12 WHERE id_producto=43;
UPDATE productos SET stock_actual=326,valor_stock_micro=7628400000,version=12 WHERE id_producto=44;
UPDATE productos SET stock_actual=326,valor_stock_micro=8215200000,version=12 WHERE id_producto=45;
UPDATE productos SET stock_actual=326,valor_stock_micro=8802000000,version=12 WHERE id_producto=46;

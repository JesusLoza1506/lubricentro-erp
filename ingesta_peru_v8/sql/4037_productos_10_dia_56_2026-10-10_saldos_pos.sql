-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=315,valor_stock_micro=6237000000,version=20 WHERE id_producto=42;
UPDATE productos SET stock_actual=315,valor_stock_micro=6804000000,version=20 WHERE id_producto=43;
UPDATE productos SET stock_actual=315,valor_stock_micro=7371000000,version=20 WHERE id_producto=44;
UPDATE productos SET stock_actual=315,valor_stock_micro=7938000000,version=20 WHERE id_producto=45;
UPDATE productos SET stock_actual=315,valor_stock_micro=8505000000,version=20 WHERE id_producto=46;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1070000,valor_stock_micro=9630000000,version=28 WHERE id_producto=119;
UPDATE productos SET stock_actual=1135000,valor_stock_micro=21792000000,version=28 WHERE id_producto=117;

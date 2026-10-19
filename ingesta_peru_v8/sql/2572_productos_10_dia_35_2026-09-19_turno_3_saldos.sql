-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1025000,valor_stock_micro=9225000000,version=37 WHERE id_producto=119;
UPDATE productos SET stock_actual=1112500,valor_stock_micro=21360000000,version=37 WHERE id_producto=117;

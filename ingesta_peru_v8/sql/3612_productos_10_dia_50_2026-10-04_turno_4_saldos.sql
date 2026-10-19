-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=290,valor_stock_micro=2784000000,version=52 WHERE id_producto=105;
UPDATE productos SET stock_actual=323,valor_stock_micro=8721000000,version=13 WHERE id_producto=76;

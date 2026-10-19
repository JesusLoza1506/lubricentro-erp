-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=286,valor_stock_micro=2745600000,version=56 WHERE id_producto=105;
UPDATE productos SET stock_actual=320,valor_stock_micro=5184000000,version=15 WHERE id_producto=70;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=298,valor_stock_micro=2860800000,version=44 WHERE id_producto=105;
UPDATE productos SET stock_actual=325,valor_stock_micro=4095000000,version=12 WHERE id_producto=68;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1076000,valor_stock_micro=16140000000,version=33 WHERE id_producto=113;
UPDATE productos SET stock_actual=327,valor_stock_micro=3531600000,version=12 WHERE id_producto=37;
UPDATE productos SET stock_actual=1076000,valor_stock_micro=29052000000,version=33 WHERE id_producto=114;
UPDATE productos SET stock_actual=327,valor_stock_micro=4120200000,version=12 WHERE id_producto=38;

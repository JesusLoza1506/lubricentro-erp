-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1052000,valor_stock_micro=15780000000,version=39 WHERE id_producto=113;
UPDATE productos SET stock_actual=324,valor_stock_micro=4665600000,version=14 WHERE id_producto=39;
UPDATE productos SET stock_actual=1052000,valor_stock_micro=28404000000,version=39 WHERE id_producto=114;
UPDATE productos SET stock_actual=324,valor_stock_micro=5248800000,version=14 WHERE id_producto=40;

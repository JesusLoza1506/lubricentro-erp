-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=972000,valor_stock_micro=14580000000,version=59 WHERE id_producto=113;
UPDATE productos SET stock_actual=314,valor_stock_micro=4521600000,version=21 WHERE id_producto=39;
UPDATE productos SET stock_actual=972000,valor_stock_micro=26244000000,version=59 WHERE id_producto=114;
UPDATE productos SET stock_actual=314,valor_stock_micro=5086800000,version=21 WHERE id_producto=40;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1156000,valor_stock_micro=17340000000,version=13 WHERE id_producto=113;
UPDATE productos SET stock_actual=335,valor_stock_micro=3618000000,version=6 WHERE id_producto=37;
UPDATE productos SET stock_actual=1156000,valor_stock_micro=31212000000,version=13 WHERE id_producto=114;
UPDATE productos SET stock_actual=335,valor_stock_micro=4221000000,version=6 WHERE id_producto=38;

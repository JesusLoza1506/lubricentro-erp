-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=317,valor_stock_micro=3423600000,version=17 WHERE id_producto=47;
UPDATE productos SET stock_actual=317,valor_stock_micro=3423600000,version=17 WHERE id_producto=57;

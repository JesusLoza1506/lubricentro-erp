-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=317,valor_stock_micro=4564800000,version=17 WHERE id_producto=49;
UPDATE productos SET stock_actual=317,valor_stock_micro=4564800000,version=17 WHERE id_producto=59;

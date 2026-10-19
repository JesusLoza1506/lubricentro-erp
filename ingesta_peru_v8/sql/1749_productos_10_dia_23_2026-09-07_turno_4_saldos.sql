-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=317,valor_stock_micro=3043200000,version=25 WHERE id_producto=105;
UPDATE productos SET stock_actual=331,valor_stock_micro=4766400000,version=8 WHERE id_producto=69;

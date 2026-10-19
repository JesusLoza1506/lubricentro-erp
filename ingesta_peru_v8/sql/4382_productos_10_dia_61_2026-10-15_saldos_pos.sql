-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=317,valor_stock_micro=3423600000,version=17 WHERE id_producto=67;
UPDATE productos SET stock_actual=318,valor_stock_micro=4006800000,version=16 WHERE id_producto=68;
UPDATE productos SET stock_actual=318,valor_stock_micro=4579200000,version=16 WHERE id_producto=69;
UPDATE productos SET stock_actual=318,valor_stock_micro=5151600000,version=16 WHERE id_producto=70;
UPDATE productos SET stock_actual=318,valor_stock_micro=5724000000,version=16 WHERE id_producto=71;

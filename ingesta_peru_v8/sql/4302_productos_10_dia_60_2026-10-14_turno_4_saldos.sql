-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=280,valor_stock_micro=2688000000,version=62 WHERE id_producto=105;
UPDATE productos SET stock_actual=320,valor_stock_micro=8640000000,version=15 WHERE id_producto=76;

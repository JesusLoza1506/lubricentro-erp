-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=329,valor_stock_micro=3158400000,version=13 WHERE id_producto=105;
UPDATE productos SET stock_actual=336,valor_stock_micro=3628800000,version=5 WHERE id_producto=67;

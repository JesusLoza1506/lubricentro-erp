-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=329,valor_stock_micro=8290800000,version=9 WHERE id_producto=55;
UPDATE productos SET stock_actual=329,valor_stock_micro=8290800000,version=9 WHERE id_producto=65;

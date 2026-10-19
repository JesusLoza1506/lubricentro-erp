-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1065000,valor_stock_micro=9585000000,version=29 WHERE id_producto=119;
UPDATE productos SET stock_actual=1132500,valor_stock_micro=21744000000,version=29 WHERE id_producto=117;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1060000,valor_stock_micro=9540000000,version=30 WHERE id_producto=119;
UPDATE productos SET stock_actual=1130000,valor_stock_micro=21696000000,version=30 WHERE id_producto=117;

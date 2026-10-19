-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=326,valor_stock_micro=3129600000,version=16 WHERE id_producto=105;
UPDATE productos SET stock_actual=334,valor_stock_micro=5410800000,version=6 WHERE id_producto=70;

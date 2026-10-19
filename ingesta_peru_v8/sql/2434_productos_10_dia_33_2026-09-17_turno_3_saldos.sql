-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1035000,valor_stock_micro=9315000000,version=35 WHERE id_producto=119;
UPDATE productos SET stock_actual=1117500,valor_stock_micro=21456000000,version=35 WHERE id_producto=117;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=1115000,valor_stock_micro=10035000000,version=19 WHERE id_producto=119;
UPDATE productos SET stock_actual=1157500,valor_stock_micro=22224000000,version=19 WHERE id_producto=117;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=905000,valor_stock_micro=8145000000,version=61 WHERE id_producto=119;
UPDATE productos SET stock_actual=1052500,valor_stock_micro=20208000000,version=61 WHERE id_producto=117;

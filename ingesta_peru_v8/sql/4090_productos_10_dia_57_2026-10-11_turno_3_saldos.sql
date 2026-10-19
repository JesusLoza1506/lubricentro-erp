-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=915000,valor_stock_micro=8235000000,version=59 WHERE id_producto=119;
UPDATE productos SET stock_actual=1057500,valor_stock_micro=20304000000,version=59 WHERE id_producto=117;

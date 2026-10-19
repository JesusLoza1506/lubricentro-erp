-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=322,valor_stock_micro=6375600000,version=15 WHERE id_producto=42;
UPDATE productos SET stock_actual=322,valor_stock_micro=6955200000,version=15 WHERE id_producto=43;
UPDATE productos SET stock_actual=322,valor_stock_micro=7534800000,version=15 WHERE id_producto=44;
UPDATE productos SET stock_actual=322,valor_stock_micro=8114400000,version=15 WHERE id_producto=45;
UPDATE productos SET stock_actual=322,valor_stock_micro=8694000000,version=15 WHERE id_producto=46;

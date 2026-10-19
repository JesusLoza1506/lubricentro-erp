-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=7140000000,ultimo_costo_micro=21000000,version=2 WHERE id_producto=1;
UPDATE productos SET stock_actual=340,valor_stock_micro=28560000000,ultimo_costo_micro=84000000,version=2 WHERE id_producto=2;
UPDATE productos SET stock_actual=340,valor_stock_micro=26928000000,ultimo_costo_micro=79200000,version=2 WHERE id_producto=3;
UPDATE productos SET stock_actual=340,valor_stock_micro=35700000000,ultimo_costo_micro=105000000,version=2 WHERE id_producto=4;

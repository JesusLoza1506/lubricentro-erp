-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=6120000000,ultimo_costo_micro=18000000,version=2 WHERE id_producto=61;
UPDATE productos SET stock_actual=340,valor_stock_micro=6732000000,ultimo_costo_micro=19800000,version=2 WHERE id_producto=62;
UPDATE productos SET stock_actual=340,valor_stock_micro=7344000000,ultimo_costo_micro=21600000,version=2 WHERE id_producto=63;
UPDATE productos SET stock_actual=340,valor_stock_micro=7956000000,ultimo_costo_micro=23400000,version=2 WHERE id_producto=64;

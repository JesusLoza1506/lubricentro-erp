-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=11220000000,ultimo_costo_micro=33000000,version=2 WHERE id_producto=21;
UPDATE productos SET stock_actual=340,valor_stock_micro=44880000000,ultimo_costo_micro=132000000,version=2 WHERE id_producto=22;
UPDATE productos SET stock_actual=340,valor_stock_micro=42432000000,ultimo_costo_micro=124800000,version=2 WHERE id_producto=23;
UPDATE productos SET stock_actual=340,valor_stock_micro=56100000000,ultimo_costo_micro=165000000,version=2 WHERE id_producto=24;

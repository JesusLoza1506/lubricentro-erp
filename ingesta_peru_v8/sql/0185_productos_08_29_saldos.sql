-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=3264000000,ultimo_costo_micro=9600000,version=2 WHERE id_producto=105;
UPDATE productos SET stock_actual=340,valor_stock_micro=4080000000,ultimo_costo_micro=12000000,version=2 WHERE id_producto=106;
UPDATE productos SET stock_actual=340,valor_stock_micro=4896000000,ultimo_costo_micro=14400000,version=2 WHERE id_producto=107;
UPDATE productos SET stock_actual=4,valor_stock_micro=67200000,ultimo_costo_micro=16800000,version=2 WHERE id_producto=108;

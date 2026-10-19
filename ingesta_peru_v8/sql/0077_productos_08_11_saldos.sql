-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET stock_actual=340,valor_stock_micro=12648000000,ultimo_costo_micro=37200000,version=2 WHERE id_producto=33;
UPDATE productos SET stock_actual=340,valor_stock_micro=50592000000,ultimo_costo_micro=148800000,version=2 WHERE id_producto=34;
UPDATE productos SET stock_actual=340,valor_stock_micro=47940000000,ultimo_costo_micro=141000000,version=2 WHERE id_producto=35;
UPDATE productos SET stock_actual=340,valor_stock_micro=63240000000,ultimo_costo_micro=186000000,version=2 WHERE id_producto=36;

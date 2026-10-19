-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (249,'lg-carga-20261018-kardex-249',119,11,'CONSUMO_OT','ML',1200000,-5000,1195000,10800000000,-45000000,10755000000,9000000,'2026-03-15 11:10:00',7);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (250,'lg-carga-20261018-kardex-250',117,7,'CONSUMO_OT','ML',1200000,-2500,1197500,23040000000,-48000000,22992000000,19200000,'2026-03-15 11:10:00',8);

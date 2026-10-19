-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1209,'lg-carga-20261018-kardex-1209',119,11,'CONSUMO_OT','ML',900000,-5000,895000,8100000000,-45000000,8055000000,9000000,'2026-10-15 11:10:00',667);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1210,'lg-carga-20261018-kardex-1210',117,7,'CONSUMO_OT','ML',1050000,-2500,1047500,20160000000,-48000000,20112000000,19200000,'2026-10-15 11:10:00',668);

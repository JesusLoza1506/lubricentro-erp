-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (345,'lg-carga-20261018-kardex-345',119,7,'CONSUMO_OT','ML',1170000,-5000,1165000,10530000000,-45000000,10485000000,9000000,'2026-08-22 11:10:00',73);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (346,'lg-carga-20261018-kardex-346',117,8,'CONSUMO_OT','ML',1185000,-2500,1182500,22752000000,-48000000,22704000000,19200000,'2026-08-22 11:10:00',74);

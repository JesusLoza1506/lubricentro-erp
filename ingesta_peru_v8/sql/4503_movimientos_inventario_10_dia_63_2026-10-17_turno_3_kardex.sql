-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1241,'lg-carga-20261018-kardex-1241',119,8,'CONSUMO_OT','ML',890000,-5000,885000,8010000000,-45000000,7965000000,9000000,'2026-10-17 11:10:00',689);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1242,'lg-carga-20261018-kardex-1242',117,9,'CONSUMO_OT','ML',1045000,-2500,1042500,20064000000,-48000000,20016000000,19200000,'2026-10-17 11:10:00',690);

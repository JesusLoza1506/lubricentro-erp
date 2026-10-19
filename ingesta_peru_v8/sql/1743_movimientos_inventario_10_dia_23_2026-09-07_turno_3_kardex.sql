-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (601,'lg-carga-20261018-kardex-601',119,8,'CONSUMO_OT','ML',1090000,-5000,1085000,9810000000,-45000000,9765000000,9000000,'2026-09-07 11:10:00',249);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (602,'lg-carga-20261018-kardex-602',117,9,'CONSUMO_OT','ML',1145000,-2500,1142500,21984000000,-48000000,21936000000,19200000,'2026-09-07 11:10:00',250);

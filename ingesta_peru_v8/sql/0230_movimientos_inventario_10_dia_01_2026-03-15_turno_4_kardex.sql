-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (251,'lg-carga-20261018-kardex-251',105,8,'CONSUMO_OT','UND',340,-1,339,3264000000,-9600000,3254400000,9600000,'2026-03-15 12:10:00',9);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (252,'lg-carga-20261018-kardex-252',67,9,'CONSUMO_OT','UND',340,-1,339,3672000000,-10800000,3661200000,10800000,'2026-03-15 12:10:00',10);

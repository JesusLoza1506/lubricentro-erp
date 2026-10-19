-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1003,'lg-carga-20261018-kardex-1003',105,10,'CONSUMO_OT','UND',293,-1,292,2812800000,-9600000,2803200000,9600000,'2026-10-02 12:10:00',526);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1004,'lg-carga-20261018-kardex-1004',74,11,'CONSUMO_OT','UND',324,-1,323,7581600000,-23400000,7558200000,23400000,'2026-10-02 12:10:00',527);

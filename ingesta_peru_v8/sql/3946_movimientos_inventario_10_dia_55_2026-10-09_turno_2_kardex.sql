-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1111,'lg-carga-20261018-kardex-1111',51,8,'CONSUMO_OT','UND',321,-1,320,5778000000,-18000000,5760000000,18000000,'2026-10-09 10:10:00',599);
INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (1112,'lg-carga-20261018-kardex-1112',61,9,'CONSUMO_OT','UND',321,-1,320,5778000000,-18000000,5760000000,18000000,'2026-10-09 10:10:00',600);

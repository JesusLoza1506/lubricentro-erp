-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,motivo) VALUES (242,'lg-carga-20261018-kardex-242',108,1,'AJUSTE_SALIDA','UND',6,-1,5,100800000,-16800000,84000000,16800000,'2026-03-01 11:00:00','Regularización por diferencia detectada en conteo físico');

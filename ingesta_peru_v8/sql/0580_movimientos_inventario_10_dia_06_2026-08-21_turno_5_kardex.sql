-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_inventario. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_inventario (id_movimiento,id_operacion,id_producto,id_usuario,tipo_movimiento,unidad_medida,stock_anterior,diferencia,stock_resultante,valor_anterior_micro,valor_delta_micro,valor_resultante_micro,costo_unitario_micro,fecha_movimiento,id_consumo_ot) VALUES (333,'lg-carga-20261018-kardex-333',109,11,'CONSUMO_OT','UND',335,-1,334,6432000000,-19200000,6412800000,19200000,'2026-08-21 13:10:00',66);

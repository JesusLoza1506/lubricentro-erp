-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_caja. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (876,'lg-carga-20261018-manual-876',47,3,'INGRESO_MANUAL','EFECTIVO',5000,1,'APORTE_FONDO','Reposición de fondo para cambio','2026-10-01 16:00:00');
INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (877,'lg-carga-20261018-manual-877',47,3,'EGRESO_MANUAL','EFECTIVO',1800,-1,'GASTO_MENOR','Compra de útiles de limpieza para el taller','2026-10-01 16:00:00');

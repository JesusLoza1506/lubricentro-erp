-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_caja. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (951,'lg-carga-20261018-manual-951',51,2,'INGRESO_MANUAL','EFECTIVO',5000,1,'APORTE_FONDO','Reposición de fondo para cambio','2026-10-05 16:00:00');
INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (952,'lg-carga-20261018-manual-952',51,2,'EGRESO_MANUAL','EFECTIVO',1800,-1,'GASTO_MENOR','Compra de útiles de limpieza para el taller','2026-10-05 16:00:00');

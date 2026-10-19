-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: movimientos_caja. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (727,'lg-carga-20261018-manual-727',39,5,'INGRESO_MANUAL','EFECTIVO',5000,1,'APORTE_FONDO','Reposición de fondo para cambio','2026-09-23 16:00:00');
INSERT INTO movimientos_caja (id_movimiento,id_operacion,id_caja,id_usuario,tipo,medio_pago,monto_cent,signo,categoria,concepto,fecha_operacion) VALUES (728,'lg-carga-20261018-manual-728',39,5,'EGRESO_MANUAL','EFECTIVO',1800,-1,'GASTO_MENOR','Compra de útiles de limpieza para el taller','2026-09-23 16:00:00');

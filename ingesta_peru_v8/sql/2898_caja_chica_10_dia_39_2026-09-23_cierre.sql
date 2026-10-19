-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-23 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-23',id_usuario_cierre=5,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=50400,efectivo_contado_cent=50400,tarjeta_neto_cent=17200,yape_neto_cent=29520,plin_neto_cent=34400,transferencia_neto_cent=14440,fondo_siguiente_cent=20000,retiro_cierre_cent=30400 WHERE id_caja=39;

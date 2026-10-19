-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-08-29 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-08-29',id_usuario_cierre=5,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=55650,efectivo_contado_cent=55650,tarjeta_neto_cent=11790,yape_neto_cent=41570,plin_neto_cent=30200,transferencia_neto_cent=16600,fondo_siguiente_cent=20000,retiro_cierre_cent=35650 WHERE id_caja=14;

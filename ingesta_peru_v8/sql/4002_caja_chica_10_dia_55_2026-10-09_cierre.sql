-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-10-09 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-10-09',id_usuario_cierre=6,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=48087,efectivo_contado_cent=48087,tarjeta_neto_cent=14800,yape_neto_cent=34988,plin_neto_cent=33725,transferencia_neto_cent=11400,fondo_siguiente_cent=20000,retiro_cierre_cent=28087 WHERE id_caja=55;

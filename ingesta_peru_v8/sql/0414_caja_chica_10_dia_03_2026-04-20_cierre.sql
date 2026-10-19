-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-04-20 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-04-20',id_usuario_cierre=4,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=49800,efectivo_contado_cent=49800,tarjeta_neto_cent=13600,yape_neto_cent=29100,plin_neto_cent=33590,transferencia_neto_cent=12800,fondo_siguiente_cent=20000,retiro_cierre_cent=29800 WHERE id_caja=3;

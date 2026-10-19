-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-14 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-14',id_usuario_cierre=6,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=51575,efectivo_contado_cent=51575,tarjeta_neto_cent=20410,yape_neto_cent=34475,plin_neto_cent=38000,transferencia_neto_cent=18400,fondo_siguiente_cent=20000,retiro_cierre_cent=31575 WHERE id_caja=30;

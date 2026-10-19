-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-16 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-16',id_usuario_cierre=3,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=55800,efectivo_contado_cent=55800,tarjeta_neto_cent=11400,yape_neto_cent=40570,plin_neto_cent=29000,transferencia_neto_cent=15245,fondo_siguiente_cent=20000,retiro_cierre_cent=35800 WHERE id_caja=32;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-21 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-21',id_usuario_cierre=3,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=47800,efectivo_contado_cent=47800,tarjeta_neto_cent=15760,yape_neto_cent=32250,plin_neto_cent=33200,transferencia_neto_cent=12000,fondo_siguiente_cent=20000,retiro_cierre_cent=27800 WHERE id_caja=37;

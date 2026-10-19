-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-08-21 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-08-21',id_usuario_cierre=2,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=51900,efectivo_contado_cent=51900,tarjeta_neto_cent=18400,yape_neto_cent=32400,plin_neto_cent=34445,transferencia_neto_cent=16790,fondo_siguiente_cent=20000,retiro_cierre_cent=31900 WHERE id_caja=6;

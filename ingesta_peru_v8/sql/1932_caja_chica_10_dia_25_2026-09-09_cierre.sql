-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-09 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-09',id_usuario_cierre=6,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=48150,efectivo_contado_cent=48150,tarjeta_neto_cent=14800,yape_neto_cent=34870,plin_neto_cent=35000,transferencia_neto_cent=11100,fondo_siguiente_cent=20000,retiro_cierre_cent=28150 WHERE id_caja=25;

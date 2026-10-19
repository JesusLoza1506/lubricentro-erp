-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-10-08 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-10-08',id_usuario_cierre=5,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=51615,efectivo_contado_cent=51615,tarjeta_neto_cent=16940,yape_neto_cent=34200,plin_neto_cent=37400,transferencia_neto_cent=16600,fondo_siguiente_cent=20000,retiro_cierre_cent=31615 WHERE id_caja=54;

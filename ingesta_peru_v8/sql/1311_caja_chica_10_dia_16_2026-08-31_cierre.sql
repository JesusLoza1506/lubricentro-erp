-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-08-31 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-08-31',id_usuario_cierre=2,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=51000,efectivo_contado_cent=51000,tarjeta_neto_cent=18010,yape_neto_cent=34850,plin_neto_cent=35600,transferencia_neto_cent=14700,fondo_siguiente_cent=20000,retiro_cierre_cent=31000 WHERE id_caja=16;

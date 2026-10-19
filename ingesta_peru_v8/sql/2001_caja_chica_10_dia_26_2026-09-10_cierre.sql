-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-10 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-10',id_usuario_cierre=2,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=55970,efectivo_contado_cent=55970,tarjeta_neto_cent=12600,yape_neto_cent=40755,plin_neto_cent=28400,transferencia_neto_cent=16900,fondo_siguiente_cent=20000,retiro_cierre_cent=35970 WHERE id_caja=26;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-26 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-26',id_usuario_cierre=3,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=51675,efectivo_contado_cent=51675,tarjeta_neto_cent=15820,yape_neto_cent=32005,plin_neto_cent=36200,transferencia_neto_cent=15500,fondo_siguiente_cent=20000,retiro_cierre_cent=31675 WHERE id_caja=42;

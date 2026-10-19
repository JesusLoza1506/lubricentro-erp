-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-08-25 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-08-25',id_usuario_cierre=6,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=50550,efectivo_contado_cent=50550,tarjeta_neto_cent=20800,yape_neto_cent=38050,plin_neto_cent=37640,transferencia_neto_cent=15900,fondo_siguiente_cent=20000,retiro_cierre_cent=30550 WHERE id_caja=10;

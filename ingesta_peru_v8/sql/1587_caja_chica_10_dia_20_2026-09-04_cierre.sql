-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-04 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-04',id_usuario_cierre=6,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=57000,efectivo_contado_cent=57000,tarjeta_neto_cent=13800,yape_neto_cent=43900,plin_neto_cent=29525,transferencia_neto_cent=18275,fondo_siguiente_cent=20000,retiro_cierre_cent=37000 WHERE id_caja=20;

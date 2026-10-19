-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-10-12 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-10-12',id_usuario_cierre=4,tipo_cierre='NORMAL',motivo_cierre=NULL,efectivo_esperado_cent=50900,efectivo_contado_cent=50900,tarjeta_neto_cent=19210,yape_neto_cent=35950,plin_neto_cent=36800,transferencia_neto_cent=15300,fondo_siguiente_cent=20000,retiro_cierre_cent=30900 WHERE id_caja=58;

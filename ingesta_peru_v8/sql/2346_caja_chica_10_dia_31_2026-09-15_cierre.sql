-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-09-15 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-09-15',id_usuario_cierre=2,tipo_cierre='NORMAL',motivo_cierre='Faltante de un sol detectado al realizar el arqueo',efectivo_esperado_cent=47300,efectivo_contado_cent=47200,tarjeta_neto_cent=12400,yape_neto_cent=32400,plin_neto_cent=32390,transferencia_neto_cent=10200,fondo_siguiente_cent=20000,retiro_cierre_cent=27200 WHERE id_caja=31;

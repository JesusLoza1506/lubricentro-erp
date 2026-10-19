-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: caja_chica. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE caja_chica SET estado='CERRADA',fecha_cierre='2026-10-05 17:00:00',id_operacion_cierre='lg-carga-20261018-cierre-2026-10-05',id_usuario_cierre=2,tipo_cierre='NORMAL',motivo_cierre='Faltante de un sol detectado al realizar el arqueo',efectivo_esperado_cent=50075,efectivo_contado_cent=49975,tarjeta_neto_cent=12160,yape_neto_cent=27575,plin_neto_cent=32600,transferencia_neto_cent=12200,fondo_siguiente_cent=20000,retiro_cierre_cent=29975 WHERE id_caja=51;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=1,fecha_inicio='2026-10-18 13:30:00' WHERE id_ot=631;
UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=2,fecha_inicio='2026-10-18 13:30:00' WHERE id_ot=632;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=656;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=657;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=658;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=659;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=660;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=661;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=662;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=663;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=664;
UPDATE ordenes_trabajo SET estado='CANCELADO',fecha_cancelacion='2026-10-18 13:35:00',id_usuario_cancelacion=1,motivo_cancelacion='Cliente reprograma el mantenimiento por disponibilidad de tiempo' WHERE id_ot=665;

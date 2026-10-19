-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=1,fecha_inicio='2026-09-20 12:00:00' WHERE id_ot=357;
UPDATE ordenes_trabajo SET estado='EN_PROCESO',id_zanja_ejecucion=2,fecha_inicio='2026-09-20 12:00:00' WHERE id_ot=358;

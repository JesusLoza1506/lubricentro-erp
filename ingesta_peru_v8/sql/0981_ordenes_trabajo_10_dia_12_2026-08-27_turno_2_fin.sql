-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=91822,resumen_publico='Se reemplazó el filtro de aire y se limpió su alojamiento.',id_mecanico_terminal=10,fecha_fin='2026-08-27 10:40:00' WHERE id_ot=113;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=152876,resumen_publico='Se reemplazó el filtro de combustible y se revisaron sus conexiones.',id_mecanico_terminal=11,fecha_fin='2026-08-27 10:40:00' WHERE id_ot=114;

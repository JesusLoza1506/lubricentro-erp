-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=115806,resumen_publico='Se reemplazó el filtro de aire y se limpió su alojamiento.',id_mecanico_terminal=9,fecha_fin='2026-09-10 10:40:00' WHERE id_ot=253;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=114672,resumen_publico='Se reemplazó el filtro de combustible y se revisaron sus conexiones.',id_mecanico_terminal=10,fecha_fin='2026-09-10 10:40:00' WHERE id_ot=254;

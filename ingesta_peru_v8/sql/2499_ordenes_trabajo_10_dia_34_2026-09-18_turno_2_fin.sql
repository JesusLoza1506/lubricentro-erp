-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=270130,resumen_publico='Se reemplazó el filtro de aire y se limpió su alojamiento.',id_mecanico_terminal=7,fecha_fin='2026-09-18 10:40:00' WHERE id_ot=333;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=70583,resumen_publico='Se reemplazó el filtro de combustible y se revisaron sus conexiones.',id_mecanico_terminal=8,fecha_fin='2026-09-18 10:40:00' WHERE id_ot=334;

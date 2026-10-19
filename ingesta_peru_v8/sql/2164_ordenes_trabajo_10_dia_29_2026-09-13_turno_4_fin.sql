-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=122705,resumen_publico='Se engrasaron los puntos de mantenimiento accesibles y se retiró el excedente.',id_mecanico_terminal=11,fecha_fin='2026-09-13 12:40:00' WHERE id_ot=287;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=58923,resumen_publico='Se reemplazó el filtro de cabina y se revisó el sentido de montaje.',id_mecanico_terminal=7,fecha_fin='2026-09-13 12:40:00' WHERE id_ot=288;

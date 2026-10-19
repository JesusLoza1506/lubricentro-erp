-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=144876,resumen_publico='Se revisaron niveles, presión de neumáticos y posibles fugas.',id_mecanico_terminal=9,fecha_fin='2026-09-19 13:40:00' WHERE id_ot=349;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=70356,resumen_publico='Se limpiaron bornes y terminales de batería, verificando su ajuste.',id_mecanico_terminal=10,fecha_fin='2026-09-19 13:40:00' WHERE id_ot=350;

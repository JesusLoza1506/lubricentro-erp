-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=58378,resumen_publico='Se revisaron niveles, presión de neumáticos y posibles fugas.',id_mecanico_terminal=7,fecha_fin='2026-10-12 13:40:00' WHERE id_ot=579;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=143435,resumen_publico='Se limpiaron bornes y terminales de batería, verificando su ajuste.',id_mecanico_terminal=8,fecha_fin='2026-10-12 13:40:00' WHERE id_ot=580;

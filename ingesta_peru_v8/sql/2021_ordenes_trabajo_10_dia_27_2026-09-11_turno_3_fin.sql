-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=112339,resumen_publico='Se renovó el refrigerante y se purgó el circuito de enfriamiento.',id_mecanico_terminal=7,fecha_fin='2026-09-11 11:40:00' WHERE id_ot=265;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=139909,resumen_publico='Se renovó el aceite de transmisión y se revisó el nivel de llenado.',id_mecanico_terminal=8,fecha_fin='2026-09-11 11:40:00' WHERE id_ot=266;

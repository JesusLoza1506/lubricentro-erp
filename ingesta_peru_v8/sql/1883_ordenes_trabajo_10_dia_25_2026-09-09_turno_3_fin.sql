-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=106689,resumen_publico='Se renovó el refrigerante y se purgó el circuito de enfriamiento.',id_mecanico_terminal=10,fecha_fin='2026-09-09 11:40:00' WHERE id_ot=245;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=99513,resumen_publico='Se renovó el aceite de transmisión y se revisó el nivel de llenado.',id_mecanico_terminal=11,fecha_fin='2026-09-09 11:40:00' WHERE id_ot=246;

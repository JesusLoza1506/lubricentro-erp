-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=125265,resumen_publico='Se renovó el refrigerante y se purgó el circuito de enfriamiento.',id_mecanico_terminal=9,fecha_fin='2026-10-13 11:40:00' WHERE id_ot=585;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=200786,resumen_publico='Se renovó el aceite de transmisión y se revisó el nivel de llenado.',id_mecanico_terminal=10,fecha_fin='2026-10-13 11:40:00' WHERE id_ot=586;

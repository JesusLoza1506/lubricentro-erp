-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=37191,resumen_publico='Cambio de aceite y filtro. Se verificó el nivel y la ausencia de fugas después del encendido.',id_mecanico_terminal=9,fecha_fin='2026-04-20 09:40:00',tipo_aceite='MINERAL',intervalo_aceite_km=5000 WHERE id_ot=21;
UPDATE ordenes_trabajo SET estado='FINALIZADO',kilometraje_salida=96003,resumen_publico='Cambio de aceite sintético y filtro. Se revisó el sello del tapón y el nivel final.',id_mecanico_terminal=10,fecha_fin='2026-04-20 09:40:00',tipo_aceite='SINTETICO',intervalo_aceite_km=10000 WHERE id_ot=22;

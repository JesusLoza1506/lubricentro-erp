-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ordenes_trabajo. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE ordenes_trabajo SET resumen_publico='Cambio de aceite y filtro. Se verificó el nivel y la ausencia de fugas después del encendido. Se confirmó la revisión del sello del tapón de drenaje.',corregido=1,version=1 WHERE id_ot=1;

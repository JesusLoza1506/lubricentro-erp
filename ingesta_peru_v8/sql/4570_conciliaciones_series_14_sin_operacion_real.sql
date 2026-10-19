-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: conciliaciones_series. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

SELECT 'Solo registrar tras verificacion real de series; no hay evidencia remota.' AS motivo_sin_datos WHERE (SELECT count(*) FROM conciliaciones_series)=0;

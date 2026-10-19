-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: restauraciones. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

SELECT 'Solo registrar cuando se restaure realmente una copia.' AS motivo_sin_datos WHERE (SELECT count(*) FROM restauraciones)=0;

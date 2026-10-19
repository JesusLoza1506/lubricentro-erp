-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: devoluciones. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

SELECT 'Requiere documento realmente aceptado; no se inventan aceptaciones SUNAT en PRODUCCION.' AS motivo_sin_datos WHERE (SELECT count(*) FROM devoluciones)=0;

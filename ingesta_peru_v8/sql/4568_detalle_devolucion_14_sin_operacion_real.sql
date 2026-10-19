-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: detalle_devolucion. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

SELECT 'Sin devolucion valida no corresponde insertar detalle.' AS motivo_sin_datos WHERE (SELECT count(*) FROM detalle_devolucion)=0;

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: auditoria. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO auditoria (id_operacion,actor,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado,fecha_evento) VALUES ('carga-lima-20261018-2959','SISTEMA','FICHA_VEHICULAR','IMPORTACION_CORRECCION_PUBLICA','ordenes_trabajo','1','{"resumen_publico":"Cambio de aceite y filtro. Se verificó el nivel y la ausencia de fugas después del encendido."}','{"resumen_publico":"Cambio de aceite y filtro. Se verificó el nivel y la ausencia de fugas después del encendido. Se confirmó la revisión del sello del tapón de drenaje."}','Precisar el alcance de la revisión efectuada al entregar el vehículo','OK','2026-10-18 13:15:00');

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: auditoria. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO auditoria (id_operacion,actor,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado,fecha_evento) VALUES ('carga-lima-20261018-1113','SISTEMA','INVENTARIO','IMPORTACION_INGRESO_STOCK','ingresos_stock','23',NULL,NULL,'Registro incorporado desde el historial de operaciones del establecimiento.','OK','2026-03-01 10:22:00');

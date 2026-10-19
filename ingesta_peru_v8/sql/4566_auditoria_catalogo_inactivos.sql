-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: auditoria. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO auditoria (id_operacion,actor,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado,fecha_evento) VALUES ('carga-lima-20261018-2995','SISTEMA','INVENTARIO','IMPORTACION_DESACTIVAR_PRODUCTO','productos','110','{"activo":1}','{"activo":0}','Artículo retirado del catálogo por renovación de presentaciones','OK','2026-10-18 13:40:00');
INSERT INTO auditoria (id_operacion,actor,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado,fecha_evento) VALUES ('carga-lima-20261018-2996','SISTEMA','INVENTARIO','IMPORTACION_DESACTIVAR_PRODUCTO','productos','111','{"activo":1}','{"activo":0}','Artículo retirado del catálogo por renovación de presentaciones','OK','2026-10-18 13:40:00');
INSERT INTO auditoria (id_operacion,actor,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado,fecha_evento) VALUES ('carga-lima-20261018-2997','SISTEMA','INVENTARIO','IMPORTACION_DESACTIVAR_PRODUCTO','productos','112','{"activo":1}','{"activo":0}','Artículo retirado del catálogo por renovación de presentaciones','OK','2026-10-18 13:40:00');

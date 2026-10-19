-- DUMP COMPLETO DE BASE DE DATOS LUBRICENTRO

-- ==========================================
-- TABLA: usuarios (1 registros)
-- ==========================================
INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo, debe_cambiar_clave, intentos_fallidos, bloqueado_hasta, fecha_creacion, fecha_cambio_clave, version) VALUES (1, 'Jesús Loza', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$Y5E4SjvF2RJ4cHgNoe+pZg$S2jWoKEBPVLGm+Z4ynb7gGMhJg+pR/eSDry8r7aEjiY', 'ADMINISTRADOR', 1, 0, 0, NULL, '2026-10-18 02:52:40', '2026-10-18 03:01:45', 1);

-- ==========================================
-- TABLA: configuracion (1 registros)
-- ==========================================
INSERT INTO configuracion (id_config, version_esquema, zona_horaria, ruc, razon_social, nombre_comercial, direccion, ubigeo, telefono, correo, ambiente_actual, igv_bp, descuento_cajero_bp, ciclo_mineral_km, ciclo_sintetico_km, margen_km, plazo_dias, hora_backup, retencion_backups, recuperacion_hash, fecha_codigo_recuperacion, recuperacion_fallos, recuperacion_bloqueada_hasta, recuperacion_pendiente, revision_token_pendiente, revision_caja_pendiente, nubefact_ruta_api, nubefact_token_ref, revision_series_pendiente, fecha_actualizacion, version) VALUES (1, 700, 'America/Lima', NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'PRODUCCION', 1800, 500, 5000, 10000, 500, 180, '23:00', 14, '$argon2id$v=19$m=19456,t=2,p=1$DLPdZMj/QaM57ebNc4LhPg$JTP8cucSIcWjbE5IbpLiSEzqTpZurDl4mMxZf/7ut74', '2026-10-18 02:52:40', 0, NULL, 0, 0, 0, NULL, NULL, 0, '2026-10-18 02:50:40', 0);

-- ==========================================
-- TABLA: zanjas (6 registros)
-- ==========================================
INSERT INTO zanjas (id_zanja, activa) VALUES (1, 1);
INSERT INTO zanjas (id_zanja, activa) VALUES (2, 1);
INSERT INTO zanjas (id_zanja, activa) VALUES (3, 0);
INSERT INTO zanjas (id_zanja, activa) VALUES (4, 0);
INSERT INTO zanjas (id_zanja, activa) VALUES (5, 0);
INSERT INTO zanjas (id_zanja, activa) VALUES (6, 0);

-- ==========================================
-- TABLA: clientes (1 registros)
-- ==========================================
INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, es_varios, fecha_creacion, version) VALUES (1, NULL, NULL, 'Cliente varios', NULL, NULL, NULL, 1, '2026-10-18 02:50:40', 0);

-- ==========================================
-- TABLA: proveedores (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: vehiculos (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: productos (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: servicios (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: ingresos_stock (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_ingreso (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: ordenes_trabajo (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_ot_servicios (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_ot_productos (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: caja_chica (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: ventas (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_venta (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: devoluciones (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: detalle_devolucion (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: movimientos_caja (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: series_comprobante (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: comprobantes (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: movimientos_inventario (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: backups (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: restauraciones (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: conciliaciones_series (0 registros)
-- ==========================================
-- (Tabla vacía)

-- ==========================================
-- TABLA: auditoria (10 registros)
-- ==========================================
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (1, 'aud_a90930345905404aba70d3325370565a', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'INSTALACION_INICIAL', 'usuarios', '1', NULL, NULL, 'Primer arranque del sistema', 'OK', '2026-10-18 02:52:40');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (2, 'aud_488e388169cb4079ab64177ef4fa58c9', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_OK', 'usuarios', '1', NULL, NULL, 'Inicio de sesión exitoso', 'OK', '2026-10-18 02:55:46');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (3, 'aud_88a1c7a905c741319f327f5bbb269fea', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_OK', 'usuarios', '1', NULL, NULL, 'Inicio de sesión exitoso', 'OK', '2026-10-18 02:59:52');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (4, 'aud_62b96f6d36b643b1928d33d8b5b42b16', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'CAMBIO_CLAVE_VOLUNTARIO', 'usuarios', '1', NULL, NULL, 'Actualización voluntaria de contraseña completada con éxito', 'OK', '2026-10-18 03:01:45');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (5, 'aud_e76997b9aee74d0da18c4abecd0a8f8b', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'CERRAR_SESION', 'usuarios', '1', NULL, NULL, 'Cierre voluntario de sesión', 'OK', '2026-10-18 03:01:54');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (6, 'aud_fb54cae8099b441381ca353035beab4c', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_OK', 'usuarios', '1', NULL, NULL, 'Inicio de sesión exitoso', 'OK', '2026-10-18 03:02:01');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (7, 'aud_0a66e486228c44a189c4c4aa5a49a9c0', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'CERRAR_SESION', 'usuarios', '1', NULL, NULL, 'Cierre voluntario de sesión', 'OK', '2026-10-18 03:02:03');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (8, 'aud_09a04c94eadf4c00afbf854ea692a0c7', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_DENEGADO', 'usuarios', '1', NULL, NULL, 'Contraseña incorrecta', 'DENEGADO', '2026-10-18 03:02:10');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (9, 'aud_fdd0136ac22d492b9a9c548ccbc7f7d6', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_OK', 'usuarios', '1', NULL, NULL, 'Inicio de sesión exitoso', 'OK', '2026-10-18 12:22:58');
INSERT INTO auditoria (id_auditoria, id_operacion, id_usuario, actor, username_snapshot, rol_snapshot, id_autorizante, modulo, accion, entidad, id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento) VALUES (10, 'aud_04a78fda3dc445959c0f5b7e84f3f501', 1, 'USUARIO', 'admin', 'ADMINISTRADOR', NULL, 'LOGIN', 'LOGIN_OK', 'usuarios', '1', NULL, NULL, 'Inicio de sesión exitoso', 'OK', '2026-10-18 13:52:24');

-- ==========================================
-- TABLA: solicitudes_operacion (0 registros)
-- ==========================================
-- (Tabla vacía)


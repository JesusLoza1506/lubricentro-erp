-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ingresos_stock. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO ingresos_stock (id_ingreso,id_operacion,codigo,origen,total_costo_micro,id_usuario,observacion,fecha_registro,id_proveedor,proveedor_snapshot,tipo_documento,numero_documento,fecha_documento) VALUES (24,'lg-carga-20261018-ingreso-23','ING-2026-0024','COMPRA',3456000000,1,'Recepción de mercadería y verificación de cantidades','2026-03-01 10:23:00',22,'{"id_proveedor":22,"tipo_documento":"RUC","numero_documento":"20248094177","nombre_razon_social":"Suministros Santa Beatriz E.I.R.L.","telefono":"958035189","correo":"suministros.santa.beatriz86@outlook.com","direccion":"Av. Los Incas N.° 329, Urb. Los Álamos, Parcona, Ica, Ica","contacto_asesor":"María Fernanda Cruz Medina","fecha_creacion":"2026-03-01 08:00:00","version":0}','FACTURA','F001-00039982','2026-03-01');

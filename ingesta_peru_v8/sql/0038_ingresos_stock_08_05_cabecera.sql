-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: ingresos_stock. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO ingresos_stock (id_ingreso,id_operacion,codigo,origen,total_costo_micro,id_usuario,observacion,fecha_registro,id_proveedor,proveedor_snapshot,tipo_documento,numero_documento,fecha_documento) VALUES (5,'lg-carga-20261018-ingreso-4','ING-2026-0005','COMPRA',13224000000,1,'Recepción de mercadería y verificación de cantidades','2026-03-01 10:04:00',3,'{"id_proveedor":3,"tipo_documento":"RUC","numero_documento":"20890059113","nombre_razon_social":"Filtros y Repuestos Santa Elena S.A.C.","telefono":"939822892","correo":"filtros.y.repuestos88@gmail.com","direccion":"Calle Las Palmeras N.° 230, Urb. San Isidro, Pisco, Pisco, Ica","contacto_asesor":"Teresa Victoria Morales Quispe","fecha_creacion":"2026-03-01 08:00:00","version":0}','FACTURA','F001-00087860','2026-03-01');

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: series_comprobante. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

INSERT INTO series_comprobante (id_serie,ambiente,tipo_comprobante,serie,activa) VALUES (1,'PRODUCCION','BOLETA','B001',1);
INSERT INTO series_comprobante (id_serie,ambiente,tipo_comprobante,serie,activa) VALUES (2,'PRODUCCION','FACTURA','F001',1);
INSERT INTO series_comprobante (id_serie,ambiente,tipo_comprobante,serie,activa) VALUES (3,'PRODUCCION','NC_BOLETA','BC01',1);
INSERT INTO series_comprobante (id_serie,ambiente,tipo_comprobante,serie,activa) VALUES (4,'PRODUCCION','NC_FACTURA','FC01',1);

-- LUBRICENTRO GLORIA | DATOS FICTICIOS | PRODUCCION LOCAL
-- Tabla: productos. Ejecutar en orden del plan, dentro de la transaccion global.
-- No ejecutar aisladamente ni desactivar triggers/foreign_keys.

UPDATE productos SET activo=0,version=3 WHERE id_producto=110;
UPDATE productos SET activo=0,version=3 WHERE id_producto=111;
UPDATE productos SET activo=0,version=3 WHERE id_producto=112;

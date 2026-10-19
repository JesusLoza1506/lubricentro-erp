-- No modifica datos. Ejecutar en la nueva base poblada.
PRAGMA integrity_check;
PRAGMA foreign_key_check;
SELECT name AS tabla FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name;
SELECT ambiente_actual,zona_horaria,version_esquema FROM configuracion;
SELECT rol,count(*) AS usuarios FROM usuarios GROUP BY rol;
SELECT estado,count(*) AS ordenes FROM ordenes_trabajo GROUP BY estado;
SELECT count(*) AS atenciones FROM v_historial_vigente;
SELECT id_vehiculo,count(*) AS atenciones FROM v_historial_vigente GROUP BY id_vehiculo ORDER BY atenciones DESC;
SELECT * FROM v_proximos_cambios;
SELECT * FROM v_productos_criticos;
SELECT * FROM v_caja_diferencias;
-- Debe devolver cero filas: saldos que no coincidan con el ultimo kardex.
SELECT p.codigo,p.stock_actual,p.valor_stock_micro,m.stock_resultante,m.valor_resultante_micro
FROM productos p LEFT JOIN movimientos_inventario m ON m.id_movimiento=(SELECT max(k.id_movimiento) FROM movimientos_inventario k WHERE k.id_producto=p.id_producto)
WHERE m.id_movimiento IS NULL OR p.stock_actual<>m.stock_resultante OR p.valor_stock_micro<>m.valor_resultante_micro;
-- Debe devolver cero filas: ventas sin coincidencia con cobros.
SELECT v.id_venta FROM ventas v WHERE total_cent<>(SELECT sum(monto_cent) FROM movimientos_caja m WHERE m.id_venta=v.id_venta AND tipo='COBRO');
SELECT estado_envio,count(*) AS documentos FROM comprobantes GROUP BY estado_envio;
SELECT nombre_archivo,estado_local,tamano_bytes,sha256,ruta_archivo FROM backups;

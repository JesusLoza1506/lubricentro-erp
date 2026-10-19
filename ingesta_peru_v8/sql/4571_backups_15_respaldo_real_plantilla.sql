-- Unica tabla: backups. El ejecutor vincula valores reales del archivo de respaldo PREVIO real.
-- Este archivo usa parametros; ejecutarlo mediante ejecutar_ingesta.cjs.
INSERT INTO backups(id_operacion,nombre_archivo,ruta_archivo,tipo,estado_local,sha256,tamano_bytes,ambiente_origen,version_aplicacion,version_esquema,resumen_json,id_usuario,fecha_creacion,fecha_datos,ultima_verificacion)
VALUES($operacion,$nombre,$ruta,'PREVIA','VALIDO',$sha256,$tamano,'PRODUCCION','demo-ingesta-v7',700,'{"datos_ficticios":true,"archivo_real":true}',$usuario,$fecha,$fecha,$fecha);

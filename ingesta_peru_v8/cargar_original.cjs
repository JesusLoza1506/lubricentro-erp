'use strict';
// Node.js >=22.13. Sin dependencias npm, sin red, sin llamadas a NubeFact.
const { DatabaseSync } = require('node:sqlite');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const ROOT = __dirname;
const sha = data => crypto.createHash('sha256').update(data).digest('hex');
const q = value => "'" + String(value).replaceAll("'", "''") + "'";
const assert = (test, msg) => { if (!test) throw new Error(msg); };
const all = (db, sql, ...args) => db.prepare(sql).all(...args);
const one = (db, sql, ...args) => db.prepare(sql).get(...args);
function args() {
 assert(process.argv.length===4 && process.argv[2]==='--bd', 'Uso: node cargar_original.cjs --bd RUTA_BD_ORIGINAL');
 return path.resolve(process.argv[3]);
}
function inventory(db) {
 return all(db,"SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").map(r=>r.name);
}
function verify(db) {
 assert(one(db,'PRAGMA integrity_check').integrity_check==='ok','Fallo integrity_check');
 assert(all(db,'PRAGMA foreign_key_check').length===0,'Referencias huerfanas');
 const tests = {
  stock_kardex: `SELECT count(*) n FROM productos p WHERE NOT EXISTS(SELECT 1 FROM movimientos_inventario m WHERE m.id_movimiento=(SELECT max(mm.id_movimiento) FROM movimientos_inventario mm WHERE mm.id_producto=p.id_producto) AND m.stock_resultante=p.stock_actual AND m.valor_resultante_micro=p.valor_stock_micro)`,
  ventas_lineas: `SELECT count(*) n FROM ventas v WHERE estado_registro<>'CONFIRMADA' OR total_cent<>(SELECT sum(total_cent) FROM detalle_venta d WHERE d.id_venta=v.id_venta) OR bruto_cent<>(SELECT sum(bruto_cent) FROM detalle_venta d WHERE d.id_venta=v.id_venta) OR descuento_cent<>(SELECT sum(descuento_cent) FROM detalle_venta d WHERE d.id_venta=v.id_venta) OR base_cent<>(SELECT sum(base_cent) FROM detalle_venta d WHERE d.id_venta=v.id_venta) OR igv_cent<>(SELECT sum(igv_cent) FROM detalle_venta d WHERE d.id_venta=v.id_venta)`,
  ventas_cobros: `SELECT count(*) n FROM ventas v WHERE total_cent<>(SELECT sum(monto_cent) FROM movimientos_caja m WHERE m.id_venta=v.id_venta AND tipo='COBRO')`,
  ingreso_lineas: `SELECT count(*) n FROM ingresos_stock i WHERE estado<>'REGISTRADO' OR total_costo_micro<>(SELECT sum(valor_aporte_micro) FROM detalle_ingreso d WHERE d.id_ingreso=i.id_ingreso)`,
  caja_efectivo: `SELECT count(*) n FROM caja_chica c WHERE estado<>'CERRADA' OR efectivo_esperado_cent<>monto_apertura_cent+(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja m WHERE m.id_caja=c.id_caja AND medio_pago='EFECTIVO') OR efectivo_contado_cent<>fondo_siguiente_cent+retiro_cierre_cent`,
  cajas_digitales: `SELECT count(*) n FROM caja_chica c WHERE tarjeta_neto_cent<>(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja m WHERE m.id_caja=c.id_caja AND medio_pago='TARJETA') OR yape_neto_cent<>(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja m WHERE m.id_caja=c.id_caja AND medio_pago='YAPE') OR plin_neto_cent<>(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja m WHERE m.id_caja=c.id_caja AND medio_pago='PLIN') OR transferencia_neto_cent<>(SELECT coalesce(sum(signo*monto_cent),0) FROM movimientos_caja m WHERE m.id_caja=c.id_caja AND medio_pago='TRANSFERENCIA')`,
  comprobantes_locales: `SELECT count(*) n FROM comprobantes WHERE estado_envio<>'PENDIENTE' OR json_extract(payload_json,'$.no_enviar')<>1 OR codigo_respuesta IS NOT NULL OR ruta_cdr IS NOT NULL`,
  venta_documento: `SELECT count(*) n FROM ventas v WHERE NOT EXISTS(SELECT 1 FROM comprobantes c WHERE c.id_venta=v.id_venta AND c.relacion='ORIGINAL' AND c.total_cent=v.total_cent AND c.base_cent=v.base_cent AND c.igv_cent=v.igv_cent)`,
  stock_sin_doble_salida: `SELECT count(*) n FROM movimientos_inventario m JOIN detalle_venta d ON d.id_detalle=m.id_detalle_venta WHERE m.tipo_movimiento='VENTA_DIRECTA' AND d.id_consumo_ot IS NOT NULL`,
  correlativos: `SELECT count(*) n FROM series_comprobante s WHERE correlativo_actual<>coalesce((SELECT max(correlativo) FROM comprobantes c WHERE c.id_serie=s.id_serie),0)`,
  consumos_completos: `SELECT count(*) n FROM detalle_ot_productos d WHERE cantidad<>-(SELECT sum(diferencia) FROM movimientos_inventario m WHERE m.id_consumo_ot=d.id_detalle AND m.tipo_movimiento='CONSUMO_OT')`,
  parametros_originales: `SELECT count(*) n FROM configuracion WHERE ambiente_actual<>'PRODUCCION' OR zona_horaria<>'America/Lima' OR nubefact_ruta_api IS NOT NULL OR nubefact_token_ref IS NOT NULL`,
  sin_fiscales_inventados: `SELECT (SELECT count(*) FROM devoluciones)+(SELECT count(*) FROM detalle_devolucion)+(SELECT count(*) FROM conciliaciones_series)+(SELECT count(*) FROM restauraciones) n`,
 };
 const results={};
 for (const [name,sql] of Object.entries(tests)) {const n=one(db,sql).n;assert(n===0,`Validacion ${name}: ${n} errores`);results[name]='OK';}
 for (const {name} of all(db,"SELECT name FROM sqlite_master WHERE type='view'")) all(db,'SELECT * FROM '+name);
 assert(one(db,"SELECT count(*) n FROM v_historial_vigente").n===630,'Historial esperado: 630');
 assert(one(db,"SELECT count(*) n FROM usuarios WHERE rol='CAJERO'").n===5,'Esperados 5 cajeros');
 assert(one(db,"SELECT count(*) n FROM usuarios WHERE rol='MECANICO'").n===5,'Esperados 5 mecanicos');
 const counts={};for(const t of inventory(db))counts[t]=one(db,'SELECT count(*) n FROM '+t).n;
 return {validaciones:results,conteos:counts};
}
let db, backupPath, transaction=false, success=false;
try {
 const dbPath=args();assert(fs.existsSync(dbPath),'No existe la BD indicada. No se creara una vacia.');
 const plan=JSON.parse(fs.readFileSync(path.join(ROOT,'plan.json'),'utf8'));
 const files=plan.archivos.map(f=>{
  assert(/^sql\/[A-Za-z0-9_\-.]+\.sql$/.test(f.archivo),'Ruta SQL no admitida');
  const sql=fs.readFileSync(path.join(ROOT,f.archivo),'utf8');assert(sha(sql)===f.sha256,'Archivo alterado: '+f.archivo);return {...f,sql};
 });
 db=new DatabaseSync(dbPath);db.exec('PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;');
 const preflight=()=>{
  assert(one(db,'PRAGMA integrity_check').integrity_check==='ok' && all(db,'PRAGMA foreign_key_check').length===0,'La BD tiene errores de integridad/referencias');
  assert(inventory(db).length===27,'Se requiere esquema v7 de 27 tablas');
  const cfg=one(db,'SELECT * FROM configuracion WHERE id_config=1');
  assert(cfg.version_esquema===700 && cfg.ambiente_actual==='PRODUCCION' && cfg.zona_horaria==='America/Lima','Se requiere esquema 700, PRODUCCION y America/Lima');
  assert(cfg.nubefact_ruta_api===null && cfg.nubefact_token_ref===null,'Carga ficticia: se requiere NubeFact sin configurar.');
  assert([cfg.recuperacion_pendiente,cfg.revision_token_pendiente,cfg.revision_caja_pendiente,cfg.revision_series_pendiente].every(v=>v===0),'Hay revisiones pendientes');
  const base=new Set(['usuarios','configuracion','zanjas','clientes','auditoria']);
  for(const t of inventory(db)) if(!base.has(t))assert(one(db,'SELECT count(*) n FROM '+t).n===0,'Ya hay datos en '+t+'. No se borraran ni se renumeraran datos cargados. Usa tu respaldo anterior a la ingesta.');
  assert(one(db,'SELECT count(*) n FROM usuarios').n===1 && one(db,'SELECT rol FROM usuarios WHERE id_usuario=1')?.rol==='ADMINISTRADOR' && one(db,'SELECT activo FROM usuarios WHERE id_usuario=1')?.activo===1,'Se requiere solo tu administrador activo con ID 1. No se borraran usuarios existentes.');
  assert(one(db,'SELECT count(*) n FROM clientes').n===1 && one(db,'SELECT es_varios FROM clientes WHERE id_cliente=1')?.es_varios===1,'Se requiere solo Cliente varios con ID 1');
  assert(one(db,'SELECT count(*) n FROM zanjas').n===6 && one(db,'SELECT count(*) n FROM zanjas WHERE id_zanja IN(1,2) AND activa=1').n===2,'Se requieren tus seis zanjas, 1 y 2 activas');
 };
 preflight();
 const stamp=new Date().toISOString().replaceAll(/[:.]/g,'-');const dir=path.join(path.dirname(dbPath),'respaldos_ingesta');fs.mkdirSync(dir,{recursive:true});
 backupPath=path.join(dir,'original_antes_ingesta_'+stamp+'_'+crypto.randomBytes(3).toString('hex')+'.db');
 db.exec('VACUUM INTO '+q(backupPath));
 const backup=new DatabaseSync(backupPath,{readOnly:true});assert(one(backup,'PRAGMA integrity_check').integrity_check==='ok','El respaldo previo no paso integridad');backup.close();
 const backupBytes=fs.readFileSync(backupPath);const backupSha=sha(backupBytes);
 const now=()=>new Date(Date.now()-5*3600*1000).toISOString().slice(0,19).replace('T',' ');
 db.exec('BEGIN IMMEDIATE');transaction=true;preflight();
 const admin=JSON.stringify(one(db,'SELECT * FROM usuarios WHERE id_usuario=1'));
 const config=JSON.stringify(one(db,'SELECT * FROM configuracion'));
 const zanjas=JSON.stringify(all(db,'SELECT * FROM zanjas ORDER BY id_zanja'));
 const varios=JSON.stringify(one(db,'SELECT * FROM clientes WHERE id_cliente=1'));
 const auditMax=one(db,'SELECT coalesce(max(id_auditoria),0) n FROM auditoria').n;
 const audit=JSON.stringify(all(db,'SELECT * FROM auditoria WHERE id_auditoria<=? ORDER BY id_auditoria',auditMax));
 let n=0;
 for(const f of files){if(f.modo==='respaldo_real')continue;try{db.exec(f.sql);}catch(e){throw new Error(f.archivo+': '+e.message)}if(++n%100===0)console.log(`Procesados ${n}/${files.length} SQL`);}
 const f=files.find(x=>x.modo==='respaldo_real');const date=now();
 db.prepare(f.sql).run({operacion:'lg-carga-20261018-respaldo-previo',nombre:path.basename(backupPath),ruta:backupPath,sha256:backupSha,tamano:backupBytes.length,usuario:1,fecha:date});
 db.prepare(`INSERT INTO auditoria(id_operacion,actor,modulo,accion,entidad,id_entidad,motivo,resultado,fecha_evento) VALUES('lg-carga-20261018-aud-respaldo','SISTEMA','BACKUP','INGESTA_RESPALDO_PREVIO_REAL','backups','1','Respaldo real de la base original ANTES de la carga, verificado con SHA256.','OK',?)`).run(date);
 assert(admin===JSON.stringify(one(db,'SELECT * FROM usuarios WHERE id_usuario=1')),'Administrador alterado');
 assert(config===JSON.stringify(one(db,'SELECT * FROM configuracion')),'Configuracion alterada');
 assert(zanjas===JSON.stringify(all(db,'SELECT * FROM zanjas ORDER BY id_zanja')),'Zanjas alteradas');
 assert(varios===JSON.stringify(one(db,'SELECT * FROM clientes WHERE id_cliente=1')),'Cliente varios alterado');
 assert(audit===JSON.stringify(all(db,'SELECT * FROM auditoria WHERE id_auditoria<=? ORDER BY id_auditoria',auditMax)),'Auditoria previa alterada');
 // Los rangos enteros del lote comienzan despues de los registros preexistentes.
 for(const t of inventory(db)){
  if(['configuracion','zanjas','auditoria','solicitudes_operacion'].includes(t))continue;
  const cols=all(db,'PRAGMA table_info('+t+')');const pk=cols.find(c=>c.pk).name;
  const r=one(db,'SELECT count(*) n,min('+pk+') lo,max('+pk+') hi FROM '+t);
  if(r.n)assert(r.lo===1 && r.hi===r.n,'IDs no consecutivos en '+t);
 }
 const result=verify(db);
 const expected=JSON.parse(fs.readFileSync(path.join(ROOT,'conteos_esperados.json'),'utf8'));for(const [t,count] of Object.entries(expected)){if(t==='auditoria')continue;assert(result.conteos[t]===count,'Conteo inesperado en '+t);}
 result.preservacion='Administrador, configuracion PRODUCCION, zanjas, Cliente varios y auditoria previa: SIN CAMBIOS';result.respaldo_previo={ruta:backupPath,sha256:backupSha,tamano_bytes:backupBytes.length};result.base_cargada=dbPath;result.ids='Consecutivos desde registros iniciales';
 db.exec('COMMIT');transaction=false;success=true;
 const report=dbPath+'.ingesta_peru_v8.validacion.json';fs.writeFileSync(report,JSON.stringify(result,null,2));db.close();db=null;
 console.log('\nCARGA COMPLETA EN TU BD ORIGINAL');console.log(dbPath);console.log('Respaldo previo real: '+backupPath);console.log('Validaciones: '+report);console.log('Usuarios 1-11; clientes 1-401; vehiculos 1-500. Configuracion PRODUCCION conservada.');
}catch(e){
 if(db){if(transaction)try{db.exec('ROLLBACK')}catch{}try{db.close()}catch{}}
 console.error('ERROR: '+e.message);console.error(success?'Los datos se confirmaron; fallo la escritura del informe posterior.':'No se confirmo la carga. Los cambios de la transaccion fueron revertidos.');if(backupPath)console.error('Respaldo previo: '+backupPath);process.exitCode=1;
}

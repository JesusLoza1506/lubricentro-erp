mod selector;
pub use selector::{elegir_guardado, elegir_ruta};
mod espacio;
mod paquete;
mod proteccion;
mod restauracion;
#[cfg(test)]
mod tests;

use crate::errors::AppError;
use chrono::{FixedOffset, NaiveDate, Timelike, Utc};
use paquete::{huella, identidad, Identidad, Manifiesto};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};
use uuid::Uuid;
use zeroize::Zeroizing;

pub type R<T> = Result<T, AppError>;
fn err(s: &str) -> AppError {
    AppError::Validation(s.into())
}
fn io<T>(v: std::io::Result<T>) -> R<T> {
    v.map_err(|e| {
        err(&format!(
            "No se pudo completar el acceso a archivos ({:?}).",
            e.kind()
        ))
    })
}
fn lima() -> chrono::DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap())
}
fn ahora() -> String {
    lima().format("%Y-%m-%d %H:%M:%S").to_string()
}
fn id() -> String {
    Uuid::new_v4().to_string()
}

fn reemplazar(path: &Path, bytes: &[u8]) -> R<()> {
    let dir = path.parent().ok_or_else(|| err("Ruta local inválida."))?;
    let mut tmp = io(tempfile::NamedTempFile::new_in(dir))?;
    io(tmp.write_all(bytes))?;
    io(tmp.as_file().sync_all())?;
    let tmp = tmp.into_temp_path();
    publicar(&tmp, path, true)?;
    if let Err(e) = sync_dir(dir) {
        tracing::error!("No se pudo sincronizar el directorio tras publicar: {e}");
    }
    Ok(())
}
#[cfg(not(windows))]
fn publicar(tmp: &Path, path: &Path, reemplazo: bool) -> R<()> {
    if !reemplazo {
        io(fs::hard_link(tmp, path))?;
        let _ = fs::remove_file(tmp);
        return Ok(());
    }
    io(fs::rename(tmp, path))
}
#[cfg(windows)]
fn publicar(tmp: &Path, path: &Path, reemplazo: bool) -> R<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(origen: *const u16, destino: *const u16, flags: u32) -> i32;
    }
    let a: Vec<_> = tmp.as_os_str().encode_wide().chain([0]).collect();
    let b: Vec<_> = path.as_os_str().encode_wide().chain([0]).collect();
    // WRITE_THROUGH; REPLACE_EXISTING solamente para punteros/configuración.
    let ok = unsafe { MoveFileExW(a.as_ptr(), b.as_ptr(), 8 | u32::from(reemplazo)) };
    if ok == 0 {
        return io(Err(std::io::Error::last_os_error()));
    }
    Ok(())
}
#[cfg(unix)]
fn sync_dir(path: &Path) -> R<()> {
    io(io(fs::File::open(path))?.sync_all())
}
#[cfg(not(unix))]
fn sync_dir(_path: &Path) -> R<()> {
    Ok(())
}

#[derive(Clone)]
pub struct Actor {
    pub id: Option<i64>,
    pub nombre: String,
}

pub struct BackupState {
    pub root: PathBuf,
    pub trabajo: Mutex<()>,
    pub envios: std::sync::Arc<tokio::sync::RwLock<()>>,
    seleccion: Mutex<Option<Seleccion>>,
    fase: std::sync::atomic::AtomicU8,
    cerrando: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    corte: std::sync::atomic::AtomicU8,
    _instancia: fs::File,
}
pub struct Trabajo<'a> {
    estado: &'a BackupState,
    _guardia: std::sync::MutexGuard<'a, ()>,
}
impl Trabajo<'_> {
    pub fn proteger_cierre(&self) {
        self.estado
            .cerrando
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
impl Drop for Trabajo<'_> {
    fn drop(&mut self) {
        self.estado.fase(0);
        self.estado
            .cerrando
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

struct Seleccion {
    id: String,
    archivo: PathBuf,
    temporal: tempfile::TempDir,
    manifiesto: Manifiesto,
    huella: (u64, String),
    actor: Option<i64>,
    sesion: Option<String>,
    backup: Option<i64>,
    creado: std::time::Instant,
}
#[derive(Default, Serialize, Deserialize)]
struct Diaria {
    intento: Option<String>,
    resultado: Option<String>,
}

pub fn ruta_activa(base: &Path) -> R<PathBuf> {
    let root = base.join("backup");
    let p = root.join("activo.json");
    match fs::read(&p) {
        Ok(raw) => {
            let registro: restauracion::Registro = serde_json::from_slice(&raw)
                .map_err(|_| err("Registro de restauración inválido. Se detuvo el arranque."))?;
            if registro.generacion == "ORIGINAL" && registro.fase == "ORIGINAL" {
                let p = base.join("lubricentro_secure.db");
                return if copia_disponible(&p) {
                    Ok(p)
                } else {
                    Err(err("Falta la base original registrada."))
                };
            }
            if Uuid::parse_str(&registro.generacion).is_err() {
                return Err(err("Generación de datos inválida."));
            }
            let dir = root.join("generaciones").join(registro.generacion);
            if !dir.join("base.sqlite").is_file() {
                return Err(err(
                    "Falta la base de la restauración activa. No se abrió una base vacía.",
                ));
            }
            Ok(dir.join("base.sqlite"))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let generaciones = root.join("generaciones");
            let hay_generaciones = if generaciones.is_dir() {
                io(fs::read_dir(generaciones))?
                    .filter_map(Result::ok)
                    .any(|e| Uuid::parse_str(&e.file_name().to_string_lossy()).is_ok())
            } else {
                false
            };
            if hay_generaciones
                || root.join("retorno.json").exists()
                || (root.join("instalacion.json").exists()
                    && !base.join("lubricentro_secure.db").is_file())
            {
                return Err(err(
                    "No se encontró el registro de activación de una instalación existente.",
                ));
            }
            Ok(base.join("lubricentro_secure.db"))
        }
        Err(e) => io(Err(e)),
    }
}
pub fn vacia(c: &Connection) -> R<bool> {
    let sql="SELECT NOT (EXISTS(SELECT 1 FROM usuarios) OR EXISTS(SELECT 1 FROM clientes WHERE es_varios=0)
      OR EXISTS(SELECT 1 FROM vehiculos) OR EXISTS(SELECT 1 FROM ordenes_trabajo) OR EXISTS(SELECT 1 FROM productos)
      OR EXISTS(SELECT 1 FROM proveedores) OR EXISTS(SELECT 1 FROM ventas) OR EXISTS(SELECT 1 FROM caja_chica)
      OR EXISTS(SELECT 1 FROM servicios) OR EXISTS(SELECT 1 FROM series_comprobante)
      OR EXISTS(SELECT 1 FROM configuracion WHERE ruc IS NOT NULL OR razon_social IS NOT NULL OR nubefact_token_ref IS NOT NULL OR recuperacion_hash IS NOT NULL))";
    Ok(c.query_row(sql, [], |r| r.get(0))?)
}
fn auditar(
    c: &Connection,
    actor: &Actor,
    accion: &str,
    entidad: &str,
    resultado: &str,
    datos: Value,
) -> R<()> {
    let (username, rol) = if let Some(uid) = actor.id {
        c.query_row(
            "SELECT username,rol FROM usuarios WHERE id_usuario=?1",
            [uid],
            |r| Ok((Some(r.get::<_, String>(0)?), Some(r.get::<_, String>(1)?))),
        )?
    } else {
        (None, None)
    };
    c.execute("INSERT INTO auditoria(id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,despues_json,resultado) VALUES(?1,?2,?3,?4,'BACKUP',?5,?6,?7,?8)",
       params![actor.id,if actor.id.is_some(){"USUARIO"}else{"SISTEMA"},username,rol,accion,entidad,datos.to_string(),resultado])?;
    Ok(())
}
fn copia_disponible(p: &Path) -> bool {
    fs::symlink_metadata(p).is_ok_and(|m| m.file_type().is_file())
}
fn archivo_regular(p: &Path) -> R<()> {
    if !io(fs::symlink_metadata(p))?.file_type().is_file() {
        return Err(err("Seleccione un archivo normal, no un enlace."));
    }
    Ok(())
}
fn es_portable(p: &Path) -> bool {
    p.extension()
        .and_then(|s| s.to_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("lgbackup"))
}
fn origen(c: &Connection) -> R<PathBuf> {
    Ok(PathBuf::from(
        c.path()
            .filter(|p| !p.is_empty())
            .ok_or_else(|| err("Base activa sin ruta en disco."))?,
    ))
}
pub fn ruta_comprobante_local(c: &Connection, referencia: &str) -> R<PathBuf> {
    let referencia = referencia.replace('\\', "/");
    if !paquete::segura(&referencia) {
        return Err(err("Ruta de comprobante local inválida."));
    }
    let base = origen(c)?;
    let dir = base
        .parent()
        .ok_or_else(|| err("No se encontró la carpeta activa."))?;
    let raiz = io(dir.join("comprobantes").canonicalize())?;
    if !raiz.starts_with(io(dir.canonicalize())?) {
        return Err(err(
            "La carpeta de comprobantes está fuera de los datos activos.",
        ));
    }
    let ruta = dir.join(referencia);
    archivo_regular(&ruta)?;
    let archivo = io(ruta.canonicalize())?;
    if !archivo.starts_with(&raiz) {
        return Err(err("Archivo fuera de la carpeta de comprobantes."));
    }
    Ok(archivo)
}
fn manifiesto(c: &Connection, a: &Actor, tipo: &str) -> R<Manifiesto> {
    Ok(Manifiesto {
        formato: 1,
        esquema: 700,
        app: env!("CARGO_PKG_VERSION").into(),
        identidad: identidad(c)?,
        fecha: ahora(),
        generado: ahora(),
        autor: a.nombre.clone(),
        tipo: tipo.into(),
        archivos: vec![],
    })
}

impl BackupState {
    pub fn new(base: &Path) -> R<Self> {
        let root = base.join("backup");
        io(fs::create_dir_all(root.join("copias")))?;
        io(fs::create_dir_all(root.join("generaciones")))?;
        let instancia = bloqueo_instancia(&root.join("instancia.lock"))?;
        let temporales = root.join("temporales");
        io(fs::create_dir_all(&temporales))?;
        // No se reutilizan temporales incompletos, ni se borran generaciones publicadas.
        for e in io(fs::read_dir(&temporales))?.flatten() {
            if e.file_name().to_string_lossy().starts_with(".tmp")
                && e.file_type().is_ok_and(|t| t.is_dir())
            {
                let _ = fs::remove_dir_all(e.path());
            }
        }
        Ok(Self {
            root,
            trabajo: Mutex::new(()),
            envios: std::sync::Arc::new(tokio::sync::RwLock::new(())),
            seleccion: Mutex::new(None),
            fase: std::sync::atomic::AtomicU8::new(0),
            cerrando: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            corte: std::sync::atomic::AtomicU8::new(0),
            _instancia: instancia,
        })
    }
    fn frase_path(&self) -> PathBuf {
        self.root.join("frase.dpapi")
    }
    fn tmp(&self) -> R<tempfile::TempDir> {
        io(tempfile::tempdir_in(self.root.join("temporales")))
    }
    #[cfg(test)]
    fn interrumpir_en(&self, punto: u8) -> R<()> {
        if self.corte.load(std::sync::atomic::Ordering::SeqCst) == punto {
            Err(err("Interrupción simulada de la restauración."))
        } else {
            Ok(())
        }
    }
    pub fn registrar_instalacion(&self) -> R<()> {
        let p = self.root.join("instalacion.json");
        if !p.is_file() {
            reemplazar(&p, b"{\"instalada\":true}")?;
        }
        Ok(())
    }
    pub fn ocupacion(&self) -> R<Trabajo<'_>> {
        let guardia = self
            .trabajo
            .try_lock()
            .map_err(|_| err("Hay una tarea de Backup en curso. Espere a que termine."))?;
        Ok(Trabajo {
            estado: self,
            _guardia: guardia,
        })
    }
    pub fn cierre_bloqueado(&self) -> bool {
        self.cerrando.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn fase_actual(&self) -> u8 {
        self.fase.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn fase(&self, fase: u8) {
        self.fase.store(fase, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn configurar(&self, c: &Connection, a: &Actor, frase: &str) -> R<Value> {
        let primera = !self.frase_path().exists();
        proteccion::guardar(&self.frase_path(), frase)?;
        if primera && !self.root.join("diaria.json").exists() {
            self.guardar_diaria(&Diaria {
                intento: Some(dia_programado().to_string()),
                resultado: None,
            })?;
        }
        auditar(c, a, "CONFIGURAR_PROTECCION", "backup", "OK", json!({}))?;
        Ok(json!({"configurada":true}))
    }
    fn diaria(&self) -> R<Diaria> {
        match fs::read(self.root.join("diaria.json")) {
            Ok(data) => serde_json::from_slice(&data)
                .map_err(|_| err("No se pudo leer el estado de la copia diaria.")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Diaria::default()),
            Err(e) => io(Err(e)),
        }
    }
    fn guardar_diaria(&self, diaria: &Diaria) -> R<()> {
        reemplazar(
            &self.root.join("diaria.json"),
            &serde_json::to_vec(diaria).map_err(|_| err("Estado de copia diaria inválido."))?,
        )
    }
    pub fn estado(&self, c: &Connection, tipo: Option<&str>, pagina: i64) -> R<Value> {
        let tipo = tipo.unwrap_or("");
        if !["", "MANUAL", "AUTOMATICO", "PREVIA", "IMPORTADO"].contains(&tipo) {
            return Err(err("Filtro inválido."));
        }
        let fecha = lima();
        let dia = fecha.date_naive();
        let proxima = if fecha.hour() < 23 {
            dia
        } else {
            dia.succ_opt().unwrap_or(dia)
        };
        let mut lista=crate::services::pos_service::consultas::filas(c,
            "SELECT json_object('id',id_backup,'nombre',nombre_archivo,'tipo',tipo,'estado',estado_local,'externo',estado_externo,'fecha',fecha_datos,'creado',fecha_creacion,'tamano',tamano_bytes,'verificado',ultima_verificacion,'error',ultimo_error,'rutaInterna',ruta_archivo) FROM backups
             WHERE estado_local<>'ELIMINADO' AND (?1='' OR tipo=?1) ORDER BY fecha_creacion DESC,id_backup DESC LIMIT 20 OFFSET ?2",&[&tipo,&((pagina.clamp(1,1_000_000)-1)*20)])?;
        for copia in &mut lista {
            let ruta = copia.as_object_mut().and_then(|o| o.remove("rutaInterna"));
            if copia["estado"] == "VALIDO"
                && !ruta
                    .as_ref()
                    .and_then(Value::as_str)
                    .is_some_and(|p| copia_disponible(Path::new(p)))
            {
                copia["estado"] = json!("AUSENTE");
            }
        }
        let total: i64 = c.query_row(
            "SELECT count(*) FROM backups WHERE estado_local<>'ELIMINADO' AND (?1='' OR tipo=?1)",
            [tipo],
            |r| r.get(0),
        )?;
        let mut q=c.prepare("SELECT ruta_archivo,fecha_datos FROM backups WHERE estado_local='VALIDO' ORDER BY fecha_datos DESC,id_backup DESC")?;
        let mut fechas = q.query([])?;
        let mut ultima = None::<String>;
        while let Some(r) = fechas.next()? {
            let p: String = r.get(0)?;
            if copia_disponible(Path::new(&p)) {
                ultima = r.get(1)?;
                break;
            }
        }
        let vencida = ultima
            .as_deref()
            .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
            .is_none_or(|s| {
                fecha.naive_local().signed_duration_since(s) > chrono::Duration::hours(24)
            });
        let ultima_rest=crate::services::pos_service::consultas::filas(c,
            "SELECT json_object('nombre',r.nombre_copia_snapshot,'estado',r.estado,'fecha',r.fecha_inicio,'previa',b.nombre_archivo) FROM restauraciones r LEFT JOIN backups b ON b.id_backup=r.id_backup_previa ORDER BY r.id_restauracion DESC LIMIT 1",&[])?;
        Ok(
            json!({"configurada":self.frase_path().is_file(),"ultima":ultima,"vencida":vencida,"proxima":format!("{proxima} 23:00"),
           "diaria":self.diaria()?.resultado,"copias":lista,"total":total,"ultima_restauracion":ultima_rest.first(),"recuperacion":self.resumen(c)?}),
        )
    }
    pub fn aviso(c: &Connection) -> R<bool> {
        let mut s=c.prepare("SELECT ruta_archivo FROM backups WHERE estado_local='VALIDO' AND fecha_datos>=datetime('now','-5 hours','-24 hours')")?;
        for p in s.query_map([], |r| r.get::<_, String>(0))? {
            if copia_disponible(Path::new(&p?)) {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn preparar_copia(&self, c: &Connection, a: &Actor, tipo: &str) -> R<(PathBuf, Manifiesto)> {
        Ok((origen(c)?, manifiesto(c, a, tipo)?))
    }
    fn producir(
        &self,
        base: &Path,
        m: Manifiesto,
        frase: &str,
    ) -> R<(PathBuf, Manifiesto, (u64, String))> {
        let tmp = self.tmp()?;
        let p = tmp.path().join("pendiente.lgbackup");
        self.fase(1);
        let meta = paquete::snapshot(base, tmp.path(), m, &p, frase)?;
        let prueba = self.tmp()?;
        self.fase(2);
        paquete::abrir(&p, prueba.path(), frase)?;
        let h = huella(&p)?;
        let nombre = format!("{}-{}.lgbackup", meta.fecha.replace([' ', ':'], "-"), id());
        let dest = self.root.join("copias").join(nombre);
        self.fase(3);
        publicar(&p, &dest, false)?;
        sync_dir(dest.parent().unwrap())?;
        Ok((dest, meta, h))
    }
    fn insertar(
        c: &Connection,
        p: &Path,
        m: &Manifiesto,
        h: &(u64, String),
        a: Option<i64>,
        tipo: &str,
    ) -> R<i64> {
        c.execute("INSERT INTO backups(id_operacion,nombre_archivo,ruta_archivo,tipo,estado_local,sha256,tamano_bytes,ambiente_origen,ruc_origen,version_aplicacion,version_esquema,resumen_json,id_usuario,fecha_datos,ultima_verificacion)
            VALUES(?1,?2,?3,?4,'VALIDO',?5,?6,?7,?8,?9,700,?10,?11,?12,datetime('now','-5 hours'))
            ON CONFLICT(ruta_archivo) DO UPDATE SET estado_local='VALIDO',sha256=excluded.sha256,tamano_bytes=excluded.tamano_bytes,ultima_verificacion=excluded.ultima_verificacion",
            params![id(),p.file_name().and_then(|s|s.to_str()),p.to_string_lossy().as_ref(),tipo,h.1,h.0 as i64,m.identidad.ambiente,m.identidad.ruc,m.app,serde_json::to_string(m).map_err(|_|err("Manifiesto inválido."))?,a,m.fecha])?;
        Ok(c.query_row(
            "SELECT id_backup FROM backups WHERE ruta_archivo=?1",
            [p.to_string_lossy().as_ref()],
            |r| r.get(0),
        )?)
    }
    pub fn generar(&self, db: &Mutex<Connection>, actor: &Actor, tipo: &str) -> R<Value> {
        let (base, m) = {
            let c = db.lock().map_err(|_| AppError::DbLock)?;
            self.preparar_copia(&c, actor, tipo)?
        };
        let frase = proteccion::leer(&self.frase_path())
            .map_err(|_| err("Configure la protección local antes de generar una copia."))?;
        let resultado = self.producir(&base, m.clone(), &frase);
        let mut c = db.lock().map_err(|_| AppError::DbLock)?;
        let (p, m, h) = match resultado {
            Ok(r) => r,
            Err(e) => {
                if let Ok(tx) = c.transaction() {
                    let key = id();
                    let nombre = format!("fallida-{key}.lgbackup");
                    let ruta = self
                        .root
                        .join("copias")
                        .join(&nombre)
                        .to_string_lossy()
                        .into_owned();
                    let registro = tx.execute(
                        "INSERT INTO backups(id_operacion,nombre_archivo,ruta_archivo,tipo,estado_local,ambiente_origen,ruc_origen,version_aplicacion,version_esquema,id_usuario,ultimo_error)
                        VALUES(?1,?2,?3,?4,'FALLIDO',?5,?6,?7,700,?8,?9)",
                        params![key,nombre,ruta,tipo,m.identidad.ambiente,m.identidad.ruc,m.app,actor.id,e.to_string()]);
                    if registro.is_ok()
                        && auditar(
                            &tx,
                            actor,
                            "GENERAR",
                            "backups",
                            "ERROR",
                            json!({"tipo":tipo}),
                        )
                        .is_ok()
                    {
                        let _ = tx.commit();
                    }
                }
                return Err(e);
            }
        };
        let tx = c.transaction()?;
        let key = Self::insertar(&tx, &p, &m, &h, actor.id, tipo)?;
        auditar(
            &tx,
            actor,
            "GENERAR",
            "backups",
            "OK",
            json!({"id":key,"tipo":tipo}),
        )?;
        tx.commit()?;
        if let Err(e) = self.retencion(&c) {
            tracing::warn!("No se completó la retención de copias: {e}");
        }
        Ok(json!({"id":key,"manifiesto":m,"tamano":h.0}))
    }
    fn gestionada(&self, p: &Path) -> bool {
        p.parent() == Some(self.root.join("copias").as_path()) && es_portable(p)
    }
    fn retencion(&self, c: &Connection) -> R<()> {
        let seleccion = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
        let mut q=c.prepare("SELECT id_backup,ruta_archivo FROM backups WHERE estado_local='VALIDO' AND tipo IN ('MANUAL','AUTOMATICO') ORDER BY fecha_datos DESC,id_backup DESC")?;
        let mut validas = 0;
        for x in q.query_map([], |r| {
            Ok((r.get::<_, i64>(0)?, PathBuf::from(r.get::<_, String>(1)?)))
        })? {
            let (id, p) = x?;
            if !self.gestionada(&p) || !p.is_file() {
                continue;
            }
            validas += 1;
            if validas <= 14 || seleccion.as_ref().is_some_and(|s| s.archivo == p) {
                continue;
            }
            if fs::remove_file(&p).is_ok() {
                c.execute("UPDATE backups SET estado_local='ELIMINADO',fecha_eliminacion=datetime('now','-5 hours') WHERE id_backup=?1",[id])?;
            }
        }
        Ok(())
    }
    pub fn automatico(&self, db: &Mutex<Connection>) -> R<()> {
        let _trabajo = self.ocupacion()?;
        {
            let mut s = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
            if s.as_ref()
                .is_some_and(|x| x.creado.elapsed().as_secs() >= 900)
            {
                *s = None;
            }
        }
        if !self.frase_path().is_file() {
            return Ok(());
        }
        {
            let c = db.lock().map_err(|_| AppError::DbLock)?;
            if vacia(&c)? {
                return Ok(());
            }
        }
        let dia = dia_programado();
        let mut diaria = self.diaria()?;
        let ultimo_intento = diaria
            .intento
            .as_deref()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
            .transpose()
            .map_err(|_| err("La fecha del último intento de copia diaria no es válida."))?;
        if ultimo_intento.is_some_and(|d| d >= dia) {
            return Ok(());
        }
        diaria.intento = Some(dia.to_string());
        diaria.resultado = Some("Interrumpida o en curso".into());
        self.guardar_diaria(&diaria)?;
        diaria.resultado = Some(
            if self
                .generar(
                    db,
                    &Actor {
                        id: None,
                        nombre: "Sistema".into(),
                    },
                    "AUTOMATICO",
                )
                .is_ok()
            {
                "Válida"
            } else {
                "Fallida"
            }
            .into(),
        );
        self.guardar_diaria(&diaria)
    }
    pub fn comprobar(
        &self,
        db: &Mutex<Connection>,
        actor: &Actor,
        sesion: Option<String>,
        local: Option<i64>,
        externa: Option<PathBuf>,
        frase: Option<&str>,
    ) -> R<Value> {
        let mut slot = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
        *slot = None;
        let p = if let Some(key) = local {
            if actor.id.is_none() {
                return Err(AppError::PermisoDenegado);
            }
            let c = db.lock().map_err(|_| AppError::DbLock)?;
            let p=PathBuf::from(c.query_row("SELECT ruta_archivo FROM backups WHERE id_backup=?1 AND estado_local IN ('VALIDO','DANADO','INCOMPATIBLE')",[key],|r|r.get::<_,String>(0))?);
            if !self.gestionada(&p) {
                return Err(err("La copia local no está en el directorio administrado."));
            }
            p
        } else {
            externa.ok_or_else(|| err("Seleccione el archivo de respaldo."))?
        };
        if !p.is_absolute() || !es_portable(&p) {
            return Err(err("Seleccione un archivo .lgbackup."));
        }
        if local.is_some() && !copia_disponible(&p) {
            return Err(err(
                "Archivo ausente. Actualice el historial y elija otra copia.",
            ));
        }
        archivo_regular(&p)?;
        self.fase(2);
        let tmp = self.tmp()?;
        // Se verifica una copia privada e inmutable, no la ruta original después de confirmar.
        let sellada = tmp.path().join("seleccion.lgbackup");
        let h = copiar(&p, &sellada)?;
        let frase = match frase {
            Some(s) => Zeroizing::new(s.to_owned()),
            None if local.is_some() => proteccion::leer(&self.frase_path())
                .map_err(|_| err("Ingrese la frase original de la copia."))?,
            None => return Err(err("Ingrese la frase original de la copia.")),
        };
        let m = paquete::abrir(&sellada, tmp.path(), &frase)?;
        let c = db.lock().map_err(|_| AppError::DbLock)?;
        let id_sel = id();
        let tiene_admin: bool = paquete::lectura(&tmp.path().join("base.sqlite"))?.query_row(
            "SELECT EXISTS(SELECT 1 FROM usuarios WHERE rol='ADMINISTRADOR' AND activo=1)",
            [],
            |r| r.get(0),
        )?;
        let compatible =
            tiene_admin && Self::compatible(&c, &m.identidad, actor.id.is_none()).is_ok();
        if let Some(key) = local {
            let n=c.execute("UPDATE backups SET ultima_verificacion=datetime('now','-5 hours'),estado_local='VALIDO' WHERE id_backup=?1 AND sha256=?2",params![key,h.1])?;
            if n != 1 {
                return Err(err(
                    "El archivo local ya no coincide con el respaldo registrado.",
                ));
            }
        }
        let abierto: bool = c.query_row(
            "SELECT EXISTS(SELECT 1 FROM caja_chica WHERE estado='ABIERTA')",
            [],
            |r| r.get(0),
        )?;
        let nombre = p
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or("Copia externa");
        let data = json!({"seleccion":id_sel,"manifiesto":m,"tamano":h.0,"compatible":compatible,"caja_abierta":abierto,"nombre":nombre});
        *slot = Some(Seleccion {
            id: id_sel,
            archivo: p,
            temporal: tmp,
            manifiesto: m,
            huella: h,
            actor: actor.id,
            sesion,
            backup: local,
            creado: std::time::Instant::now(),
        });
        Ok(data)
    }
    pub fn revalidar(
        &self,
        c: &Connection,
        actor: &Actor,
        sesion: Option<&str>,
        token: &str,
    ) -> R<Value> {
        let slot = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
        let Some(sel) = slot.as_ref() else {
            return Ok(json!({"vigente":false}));
        };
        if Self::validar_seleccion(sel, actor, sesion, token).is_err() {
            return Ok(json!({"vigente":false}));
        }
        let abierta: bool = c.query_row(
            "SELECT EXISTS(SELECT 1 FROM caja_chica WHERE estado='ABIERTA')",
            [],
            |r| r.get(0),
        )?;
        Ok(
            json!({"vigente":true,"caja_abierta":abierta,"compatible":Self::compatible(c, &sel.manifiesto.identidad, false).is_ok()}),
        )
    }
    fn validar_seleccion(
        sel: &Seleccion,
        actor: &Actor,
        sesion: Option<&str>,
        token: &str,
    ) -> R<()> {
        if sel.creado.elapsed().as_secs() >= 900
            || sel.id != token
            || sel.actor != actor.id
            || sel.sesion.as_deref() != sesion
        {
            return Err(err("Seleccione y verifique nuevamente la copia."));
        }
        if sel.backup.is_some() && !copia_disponible(&sel.archivo) {
            return Err(err(
                "El archivo de la copia local está ausente. Seleccione otra copia.",
            ));
        }
        Ok(())
    }
    fn compatible(c: &Connection, origen: &Identidad, inicial: bool) -> R<()> {
        let actual = identidad(c)?;
        if actual.ambiente != origen.ambiente
            || (!inicial && (actual.ruc.is_none() || actual.ruc != origen.ruc))
            || (inicial && !vacia(c)?)
        {
            return Err(err(
                "La copia no pertenece a la empresa y ambiente de esta instalación.",
            ));
        }
        Ok(())
    }
    pub fn exportar(
        &self,
        c: &Connection,
        a: &Actor,
        sesion: Option<&str>,
        token: &str,
        carpeta: &Path,
    ) -> R<Value> {
        let slot = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
        let s = slot.as_ref().ok_or_else(|| err("Seleccione una copia."))?;
        Self::validar_seleccion(s, a, sesion, token)?;
        if !carpeta.is_absolute() || !carpeta.is_dir() {
            return Err(err("Seleccione una carpeta de destino existente."));
        }
        let dir = io(carpeta.canonicalize())?;
        if dir.starts_with(io(self.root.canonicalize())?) {
            return Err(err("Elija un destino fuera de la carpeta administrada."));
        }
        let out = dir.join(format!("Lubricentro-{}.lgbackup", id()));
        let tmp = dir.join(format!(".lubricentro-{}.tmp", id()));
        self.fase(3);
        let resultado = (|| {
            if copiar(&s.temporal.path().join("seleccion.lgbackup"), &tmp)? != s.huella {
                return Err(err("La copia externa no coincide con el original."));
            }
            publicar(&tmp, &out, false)?;
            sync_dir(&dir)
        })();
        if resultado.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        if let Some(k) = s.backup {
            c.execute("UPDATE backups SET estado_externo=?2,ruta_externa=?3,error_externo=?4,ultima_copia_externa=CASE WHEN ?2='COPIADO' THEN datetime('now','-5 hours') ELSE ultima_copia_externa END WHERE id_backup=?1",
                params![k,if resultado.is_ok(){"COPIADO"}else{"FALLIDO"},if resultado.is_ok(){Some(out.to_string_lossy().into_owned())}else{None},resultado.as_ref().err().map(ToString::to_string)])?;
        }
        auditar(
            c,
            a,
            "GUARDAR_FUERA",
            "backups",
            if resultado.is_ok() { "OK" } else { "ERROR" },
            json!({"id":s.backup}),
        )?;
        resultado?;
        Ok(json!({"nombre":out.file_name().and_then(|s|s.to_str())}))
    }
    pub fn eliminar_previa(&self, c: &Connection, a: &Actor, key: i64) -> R<Value> {
        let p=PathBuf::from(c.query_row("SELECT ruta_archivo FROM backups WHERE id_backup=?1 AND tipo='PREVIA' AND estado_local='VALIDO' AND NOT EXISTS(SELECT 1 FROM restauraciones WHERE id_backup_previa=?1 AND id_restauracion=(SELECT max(id_restauracion) FROM restauraciones) AND estado='CONTROLES_PENDIENTES') AND id_backup<>(SELECT id_backup FROM backups WHERE estado_local='VALIDO' ORDER BY fecha_datos DESC,id_backup DESC LIMIT 1)",[key],|r|r.get::<_,String>(0)).optional()?.ok_or_else(||err("Esta copia previa está protegida o no existe."))?);
        if !self.gestionada(&p)
            || self
                .seleccion
                .lock()
                .map_err(|_| AppError::DbLock)?
                .as_ref()
                .is_some_and(|s| s.archivo == p)
        {
            return Err(err("La copia está en uso."));
        }
        io(fs::remove_file(&p))?;
        c.execute("UPDATE backups SET estado_local='ELIMINADO',fecha_eliminacion=datetime('now','-5 hours') WHERE id_backup=?1",[key])?;
        auditar(c, a, "ELIMINAR_PREVIA", "backups", "OK", json!({"id":key}))?;
        Ok(json!({"eliminada":true}))
    }
    pub fn limpiar_seleccion(&self) -> R<()> {
        *self.seleccion.lock().map_err(|_| AppError::DbLock)? = None;
        Ok(())
    }
}
fn copiar(p: &Path, destino: &Path) -> R<(u64, String)> {
    let orig = huella(p)?;
    let mut in_f = io(fs::File::open(p))?;
    let mut out = paquete::nuevo(destino)?;
    if io(std::io::copy(
        &mut std::io::Read::take(&mut in_f, orig.0 + 1),
        &mut out,
    ))? != orig.0
    {
        return Err(err("El archivo cambió durante la copia."));
    }
    io(out.sync_all())?;
    drop(out);
    if huella(destino)? != orig {
        return Err(err("La huella del destino no coincide."));
    }
    Ok(orig)
}

fn dia_programado() -> NaiveDate {
    dia_programado_en(lima())
}
fn dia_programado_en(hora: chrono::DateTime<FixedOffset>) -> NaiveDate {
    let dia = hora.date_naive();
    if hora.hour() >= 23 {
        dia
    } else {
        dia.pred_opt().unwrap_or(dia)
    }
}
fn bloqueo_instancia(path: &Path) -> R<fs::File> {
    let f = io(fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path))?;
    #[cfg(windows)]
    {
        use std::{ffi::c_void, os::windows::io::AsRawHandle};
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn LockFile(h: *mut c_void, lo: u32, hi: u32, len: u32, len_hi: u32) -> i32;
        }
        if unsafe { LockFile(f.as_raw_handle(), 0, 0, 1, 0) } == 0 {
            return Err(err("Ya hay otra instancia usando estos datos."));
        }
    }
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        unsafe extern "C" {
            fn flock(fd: i32, operation: i32) -> i32;
        }
        if unsafe { flock(f.as_raw_fd(), 2 | 4) } != 0 {
            return Err(err("Ya hay otra instancia usando estos datos."));
        }
    }
    Ok(f)
}

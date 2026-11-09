use super::{err, io, R};
use argon2::{Algorithm, Argon2, Params, Version};
use ring::{
    aead,
    rand::{SecureRandom, SystemRandom},
};
use rusqlite::{
    backup::{Backup, StepResult},
    Connection, OpenFlags,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use zeroize::Zeroizing;

const MAGIC: &[u8; 8] = b"LGBKP001";
const MAX: u64 = 2_000_000_000;
const CHUNK: usize = 1024 * 1024;
const HEADER: usize = 36;
const MANIFEST: usize = 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Identidad {
    pub ambiente: String,
    pub ruc: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Archivo {
    pub nombre: String,
    pub bytes: u64,
    pub sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifiesto {
    pub formato: u32,
    pub esquema: i64,
    pub app: String,
    pub identidad: Identidad,
    pub fecha: String,
    pub generado: String,
    pub autor: String,
    pub tipo: String,
    pub archivos: Vec<Archivo>,
}

pub fn identidad(c: &Connection) -> R<Identidad> {
    Ok(c.query_row(
        "SELECT ambiente_actual,ruc FROM configuracion WHERE id_config=1",
        [],
        |r| {
            Ok(Identidad {
                ambiente: r.get(0)?,
                ruc: r.get(1)?,
            })
        },
    )?)
}
pub fn lectura(p: &Path) -> R<Connection> {
    Ok(Connection::open_with_flags(
        p,
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?)
}
pub(super) fn segura(s: &str) -> bool {
    let Some(f) = s.strip_prefix("comprobantes/") else {
        return false;
    };
    let Some((nombre, ext)) = f.rsplit_once('.') else {
        return false;
    };
    let upper = nombre.split('.').next().unwrap_or("").to_ascii_uppercase();
    !nombre.is_empty()
        && !nombre.starts_with('.')
        && f.len() <= 140
        && f.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        && ["pdf", "xml", "cdr", "zip"].contains(&ext.to_ascii_lowercase().as_str())
        && ![
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ]
        .contains(&upper.as_str())
}
fn esquema(c: &Connection) -> R<Vec<(String, String, String)>> {
    let mut s=c.prepare("SELECT type,name,sql FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' AND sql IS NOT NULL ORDER BY type,name")?;
    let filas = s.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    let datos = filas.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(datos)
}
fn comprobar_db(c: &Connection, m: &Manifiesto) -> R<()> {
    let version: i64 = c.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if m.formato != 1 || m.esquema != 700 || version != 700 {
        return Err(err("Formato o esquema de respaldo incompatible."));
    }
    let patron = Connection::open_in_memory()?;
    patron.execute_batch(include_str!(
        "../../../migrations/0001_initial_schema_v7.sql"
    ))?;
    if esquema(c)? != esquema(&patron)? {
        return Err(err(
            "El esquema de la copia no coincide con esta aplicación.",
        ));
    }
    let integral: String = c.query_row("PRAGMA integrity_check", [], |r| r.get(0))?;
    if integral != "ok"
        || c.prepare("PRAGMA foreign_key_check")?.exists([])?
        || identidad(c)? != m.identidad
    {
        return Err(err(
            "Falló la integridad o la identidad de la base de la copia.",
        ));
    }
    Ok(())
}
pub(super) fn referencias(c: &Connection) -> R<Vec<String>> {
    let mut s=c.prepare("SELECT ruta_pdf FROM comprobantes WHERE ruta_pdf IS NOT NULL UNION SELECT ruta_xml FROM comprobantes WHERE ruta_xml IS NOT NULL UNION SELECT ruta_cdr FROM comprobantes WHERE ruta_cdr IS NOT NULL")?;
    let mut res = Vec::new();
    for r in s.query_map([], |r| r.get::<_, String>(0))? {
        let ruta = r?.replace('\\', "/");
        if ruta.starts_with("https://") {
            continue;
        }
        if !segura(&ruta) {
            return Err(err(
                "Un comprobante local no usa una ruta administrada de Backup.",
            ));
        }
        res.push(ruta);
    }
    res.sort_unstable();
    res.dedup();
    Ok(res)
}
pub fn huella(p: &Path) -> R<(u64, String)> {
    let mut f = io(File::open(p))?;
    let mut sha = Sha256::new();
    let mut n = 0;
    let mut buf = vec![0; CHUNK];
    loop {
        let l = io(f.read(&mut buf))?;
        if l == 0 {
            break;
        }
        n += l as u64;
        if n > MAX {
            return Err(err("El archivo supera el límite de 2 GB."));
        }
        sha.update(&buf[..l]);
    }
    let hexadecimal = sha.finalize().iter().map(|b| format!("{b:02x}")).collect();
    Ok((n, hexadecimal))
}
fn clave(frase: &str, sal: &[u8]) -> R<aead::LessSafeKey> {
    if frase.is_empty() || frase.len() > 512 {
        return Err(err("Frase de respaldo inválida."));
    }
    let params = Params::new(65_536, 3, 1, Some(32))
        .map_err(|_| err("Parámetros de protección inválidos."))?;
    let mut key = Zeroizing::new([0u8; 32]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(frase.as_bytes(), sal, &mut *key)
        .map_err(|_| err("No se pudo derivar la clave."))?;
    Ok(aead::LessSafeKey::new(
        aead::UnboundKey::new(&aead::AES_256_GCM, &*key)
            .map_err(|_| err("No se pudo proteger la copia."))?,
    ))
}
fn nonce(h: &[u8; HEADER], i: u64) -> aead::Nonce {
    let mut n = [0; 12];
    n[..4].copy_from_slice(&h[24..28]);
    n[4..].copy_from_slice(&i.to_be_bytes());
    aead::Nonce::assume_unique_for_key(n)
}
fn header(total: u64) -> R<[u8; HEADER]> {
    if total == 0 || total + HEADER as u64 + total.div_ceil(CHUNK as u64) * 16 > MAX {
        return Err(err("El paquete supera el límite de 2 GB."));
    }
    let mut h = [0; HEADER];
    h[..8].copy_from_slice(MAGIC);
    SystemRandom::new()
        .fill(&mut h[8..28])
        .map_err(|_| err("No se pudo obtener aleatoriedad."))?;
    h[28..].copy_from_slice(&total.to_be_bytes());
    Ok(h)
}
fn descriptor(nombre: &str, p: &Path) -> R<Archivo> {
    let (bytes, sha256) = huella(p)?;
    Ok(Archivo {
        nombre: nombre.into(),
        bytes,
        sha256,
    })
}
pub fn nuevo(p: &Path) -> R<File> {
    io(OpenOptions::new().write(true).create_new(true).open(p))
}
pub fn snapshot(
    origen: &Path,
    dir: &Path,
    mut m: Manifiesto,
    salida: &Path,
    frase: &str,
) -> R<Manifiesto> {
    let work = io(tempfile::tempdir_in(dir))?;
    let db = work.path().join("base.sqlite");
    let source = lectura(origen)?;
    let mut dest = Connection::open(&db)?;
    let begin = Instant::now();
    {
        let b = Backup::new(&source, &mut dest)?;
        loop {
            if begin.elapsed() > Duration::from_secs(90) {
                return Err(err("SQLite ocupado: no se pudo terminar la fotografía."));
            }
            match b.step(128)? {
                StepResult::Done => break,
                StepResult::Busy | StepResult::Locked => {
                    std::thread::sleep(Duration::from_millis(30))
                }
                _ => std::thread::sleep(Duration::from_millis(2)),
            }
        }
    }
    drop(source);
    m.fecha = super::ahora();
    m.identidad = identidad(&dest)?;
    comprobar_db(&dest, &m)?;
    let refs = referencias(&dest)?;
    dest.close().map_err(|(_, e)| e)?;
    m.archivos = vec![descriptor("base.sqlite", &db)?];
    let mut entradas = vec![db];
    let raiz = io(origen.parent().unwrap_or(dir).canonicalize())?;
    for ruta in refs {
        let p = raiz.join(&ruta);
        let real = io(p.canonicalize())?;
        if !real.starts_with(&raiz) || !real.is_file() {
            return Err(err(
                "Un archivo oficial está fuera del directorio administrado.",
            ));
        }
        m.archivos.push(descriptor(&ruta, &real)?);
        entradas.push(real);
    }
    let mut unicos = std::collections::HashSet::new();
    if m.archivos
        .iter()
        .any(|a| !unicos.insert(a.nombre.to_ascii_lowercase()))
    {
        return Err(err("La copia contiene nombres de archivos duplicados."));
    }
    let json = serde_json::to_vec(&m).map_err(|_| err("Manifiesto inválido."))?;
    if json.len() > MANIFEST {
        return Err(err("El manifiesto supera el límite."));
    }
    let total = 4 + json.len() as u64 + m.archivos.iter().map(|a| a.bytes).sum::<u64>();
    let h = header(total)?;
    let key = clave(frase, &h[8..24])?;
    let mut head = (json.len() as u32).to_be_bytes().to_vec();
    head.extend(json);
    let mut reader = Fuentes {
        actual: Box::new(std::io::Cursor::new(head)),
        rutas: entradas.into_iter(),
    };
    let mut f = nuevo(salida)?;
    io(f.write_all(&h))?;
    let mut restante = total;
    let mut i = 0;
    let mut chunk = Zeroizing::new(Vec::with_capacity(CHUNK + 16));
    while restante > 0 {
        let n = restante.min(CHUNK as u64) as usize;
        chunk.resize(n, 0);
        io(reader.read_exact(&mut chunk))?;
        key.seal_in_place_append_tag(nonce(&h, i), aead::Aad::from(h), &mut *chunk)
            .map_err(|_| err("No se pudo cifrar un bloque."))?;
        io(f.write_all(&chunk))?;
        restante -= n as u64;
        i += 1;
    }
    io(f.sync_all())?;
    Ok(m)
}

struct Fuentes {
    actual: Box<dyn Read>,
    rutas: std::vec::IntoIter<PathBuf>,
}
impl Read for Fuentes {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        if b.is_empty() {
            return Ok(0);
        }
        loop {
            let n = self.actual.read(b)?;
            if n != 0 {
                return Ok(n);
            }
            match self.rutas.next() {
                Some(p) => self.actual = Box::new(File::open(p)?),
                None => return Ok(0),
            }
        }
    }
}
struct Abierto {
    f: File,
    h: [u8; HEADER],
    key: aead::LessSafeKey,
    restante: u64,
    i: u64,
    chunk: Zeroizing<Vec<u8>>,
    pos: usize,
}
impl Read for Abierto {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        if self.pos == self.chunk.len() {
            if self.restante == 0 {
                return Ok(0);
            }
            let n = self.restante.min(CHUNK as u64) as usize;
            self.chunk.resize(n + 16, 0);
            self.f.read_exact(&mut self.chunk)?;
            self.key
                .open_in_place(
                    nonce(&self.h, self.i),
                    aead::Aad::from(self.h),
                    self.chunk.as_mut_slice(),
                )
                .map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "autenticación")
                })?;
            self.chunk.truncate(n);
            self.pos = 0;
            self.i += 1;
            self.restante -= n as u64;
        }
        let n = out.len().min(self.chunk.len() - self.pos);
        out[..n].copy_from_slice(&self.chunk[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}
fn abrir_error<T>(r: std::io::Result<T>) -> R<T> {
    r.map_err(|_| {
        err("No se pudo autenticar o leer la copia. Verifique la frase original y el archivo.")
    })
}
pub fn abrir(p: &Path, dest: &Path, frase: &str) -> R<Manifiesto> {
    let mut f = io(File::open(p))?;
    let size = io(f.metadata())?.len();
    let mut h = [0; HEADER];
    abrir_error(f.read_exact(&mut h))?;
    if &h[..8] != MAGIC {
        return Err(err("Formato de respaldo incompatible."));
    }
    let n = u64::from_be_bytes(h[28..].try_into().unwrap());
    if n == 0 || n > MAX || size > MAX || size != HEADER as u64 + n + n.div_ceil(CHUNK as u64) * 16
    {
        return Err(err("Tamaño de respaldo inválido."));
    }
    let mut input = Abierto {
        f,
        h,
        key: clave(frase, &h[8..24])?,
        restante: n,
        i: 0,
        chunk: Zeroizing::new(Vec::with_capacity(CHUNK + 16)),
        pos: 0,
    };
    let mut len = [0; 4];
    abrir_error(input.read_exact(&mut len))?;
    let len = u32::from_be_bytes(len) as usize;
    if len == 0 || len > MANIFEST {
        return Err(err("Tamaño de manifiesto inválido."));
    }
    let mut raw = vec![0; len];
    abrir_error(input.read_exact(&mut raw))?;
    let m: Manifiesto = serde_json::from_slice(&raw).map_err(|_| err("Manifiesto inválido."))?;
    if m.formato != 1
        || m.esquema != 700
        || m.archivos
            .first()
            .is_none_or(|a| a.nombre != "base.sqlite" || a.bytes == 0)
        || m.archivos
            .iter()
            .try_fold(4 + len as u64, |sum, a| sum.checked_add(a.bytes))
            != Some(n)
    {
        return Err(err("Contenido de respaldo incompatible."));
    }
    let mut unicos = std::collections::HashSet::new();
    for (i, a) in m.archivos.iter().enumerate() {
        if (i > 0 && !segura(&a.nombre)) || !unicos.insert(a.nombre.to_ascii_lowercase()) {
            return Err(err("Ruta de archivo inválida o duplicada."));
        }
        let p: PathBuf = dest.join(&a.nombre);
        io(fs::create_dir_all(p.parent().unwrap()))?;
        let mut file = nuevo(&p)?;
        let bytes = abrir_error(std::io::copy(&mut (&mut input).take(a.bytes), &mut file))?;
        io(file.sync_all())?;
        drop(file);
        if bytes != a.bytes || huella(&p)?.1 != a.sha256 {
            return Err(err("La huella de un archivo no coincide."));
        }
    }
    let c = lectura(&dest.join("base.sqlite"))?;
    comprobar_db(&c, &m)?;
    let mut adjuntos: Vec<String> = m
        .archivos
        .iter()
        .skip(1)
        .map(|a| a.nombre.clone())
        .collect();
    adjuntos.sort_unstable();
    if referencias(&c)? != adjuntos {
        return Err(err(
            "Los comprobantes locales no coinciden con el manifiesto.",
        ));
    }
    Ok(m)
}

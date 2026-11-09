use super::*;
use crate::services::admin_service::facturacion::{self, AlmacenLocal, ModoToken, Secretos};
use std::{collections::BTreeMap, io::Write};

#[derive(Default, Serialize, Deserialize)]
pub(super) struct Registro {
    pub generacion: String,
    anterior: String,
    identidad: Option<Identidad>,
    pub(super) fase: String,
    fecha: String,
    ejecutor: String,
    copia: String,
    previa: Option<String>,
    #[serde(default)]
    previa_sha: Option<String>,
    #[serde(default)]
    maximos: BTreeMap<String, i64>,
}
fn flag(c: &Connection) -> R<bool> {
    Ok(c.query_row(
        "SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?)
}
fn caja(c: &Connection) -> R<bool> {
    Ok(c.query_row(
        "SELECT EXISTS(SELECT 1 FROM caja_chica WHERE estado='ABIERTA')",
        [],
        |r| r.get(0),
    )?)
}
fn ref_token(c: &Connection) -> R<Option<String>> {
    Ok(c.query_row(
        "SELECT nubefact_token_ref FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?)
}
fn ruta_api(c: &Connection) -> R<Option<String>> {
    Ok(c.query_row(
        "SELECT nubefact_ruta_api FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?)
}
fn token_existe(ambiente: &str, r: Option<&str>) -> bool {
    r.is_some_and(|r| AlmacenLocal.existe(ambiente, r))
}
fn agregar_maximos(c: &Connection, result: &mut BTreeMap<String, i64>) -> R<()> {
    let mut stmt=c.prepare("SELECT s.tipo_comprobante||':'||s.serie, max(s.correlativo_actual,s.mayor_previo,s.ultimo_real_comprobado,
        coalesce((SELECT max(c.correlativo) FROM comprobantes c WHERE c.id_serie=s.id_serie),0))
        FROM series_comprobante s WHERE s.ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)")?;
    for r in stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))? {
        let (k, n) = r?;
        result
            .entry(k)
            .and_modify(|v| *v = (*v).max(n))
            .or_insert(n);
    }
    Ok(())
}
fn activa(c: &Connection) -> R<i64> {
    Ok(c.query_row("SELECT id_restauracion FROM restauraciones WHERE id_restauracion=(SELECT max(id_restauracion) FROM restauraciones) AND estado='CONTROLES_PENDIENTES'",[],|r|r.get(0))?)
}
fn aplicar_maximos(c: &Connection, maximos: &BTreeMap<String, i64>) -> R<()> {
    c.execute("UPDATE series_comprobante SET requiere_conciliacion=1 WHERE activa=1 AND ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)",[])?;
    for (k, n) in maximos {
        c.execute("UPDATE series_comprobante SET mayor_previo=max(mayor_previo,?2),correlativo_actual=CASE WHEN activa=0 THEN max(correlativo_actual,?2) ELSE correlativo_actual END
            WHERE tipo_comprobante||':'||serie=?1 AND ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)",params![k,n])?;
    }
    Ok(())
}
fn registrar_catalogo(old: &Connection, new: &Connection, root: &Path) -> R<()> {
    // No se confía en rutas o IDs numéricos que viajaron en la copia.
    new.execute("UPDATE backups SET estado_local='ELIMINADO',fecha_eliminacion=datetime('now','-5 hours') WHERE estado_local<>'ELIMINADO'",[])?;
    let mut q=old.prepare("SELECT nombre_archivo,ruta_archivo,tipo,sha256,tamano_bytes,ambiente_origen,ruc_origen,version_aplicacion,resumen_json,fecha_datos,ultima_verificacion FROM backups WHERE estado_local='VALIDO'")?;
    let mut filas = q.query([])?;
    while let Some(r) = filas.next()? {
        let p: String = r.get(1)?;
        let local = Path::new(&p);
        if local.parent() != Some(root.join("copias").as_path())
            || !es_portable(local)
            || !local.is_file()
        {
            continue;
        }
        new.execute("INSERT INTO backups(id_operacion,nombre_archivo,ruta_archivo,tipo,estado_local,sha256,tamano_bytes,ambiente_origen,ruc_origen,version_aplicacion,version_esquema,resumen_json,fecha_datos,ultima_verificacion)
            VALUES(?1,?2,?3,?4,'VALIDO',?5,?6,?7,?8,?9,700,?10,?11,?12)
            ON CONFLICT(ruta_archivo) DO UPDATE SET estado_local='VALIDO',fecha_eliminacion=NULL,tipo=excluded.tipo,
                nombre_archivo=excluded.nombre_archivo,sha256=excluded.sha256,tamano_bytes=excluded.tamano_bytes,ambiente_origen=excluded.ambiente_origen,ruc_origen=excluded.ruc_origen,
                version_aplicacion=excluded.version_aplicacion,version_esquema=700,resumen_json=excluded.resumen_json,fecha_datos=excluded.fecha_datos,ultima_verificacion=excluded.ultima_verificacion,id_usuario=NULL",
            params![id(),r.get::<_,String>(0)?,p,r.get::<_,String>(2)?,r.get::<_,Option<String>>(3)?,r.get::<_,Option<i64>>(4)?,
                r.get::<_,Option<String>>(5)?,r.get::<_,Option<String>>(6)?,r.get::<_,Option<String>>(7)?,r.get::<_,Option<String>>(8)?,r.get::<_,Option<String>>(9)?,r.get::<_,Option<String>>(10)?])?;
    }
    Ok(())
}
impl BackupState {
    pub fn restaurar(
        &self,
        c: &mut Connection,
        a: &Actor,
        sesion: Option<&str>,
        token: &str,
    ) -> R<Value> {
        self.restaurar_interno(c, a, sesion, token, None)
    }

    fn restaurar_interno(
        &self,
        c: &mut Connection,
        a: &Actor,
        sesion: Option<&str>,
        token: &str,
        retorno: Option<&Registro>,
    ) -> R<Value> {
        let mut slot = self.seleccion.lock().map_err(|_| AppError::DbLock)?;
        let s = slot.as_ref().ok_or_else(|| err("Seleccione una copia."))?;
        Self::validar_seleccion(s, a, sesion, token)?;
        if retorno.is_none() {
            Self::compatible(c, &s.manifiesto.identidad, a.id.is_none())?;
        }
        if caja(c)? {
            return Err(err("Cierre el turno de Caja actual antes de restaurar."));
        }
        let base = if let Some(r) = retorno {
            PathBuf::from(&r.anterior)
        } else {
            origen(c)?
        };
        let recuperado = s
            .manifiesto
            .archivos
            .iter()
            .fold(0u64, |n, f| n.saturating_add(f.bytes));
        let previa_bytes = if a.id.is_some() {
            let paginas: u64 = c.query_row("PRAGMA page_count", [], |r| r.get(0))?;
            let tamano: u64 = c.query_row("PRAGMA page_size", [], |r| r.get(0))?;
            let raiz = base
                .parent()
                .ok_or_else(|| err("No se encontró la carpeta activa."))?;
            let mut total = paginas.saturating_mul(tamano);
            for referencia in paquete::referencias(c)? {
                total = total.saturating_add(io(fs::metadata(raiz.join(referencia)))?.len());
            }
            total
        } else {
            0
        };
        espacio::exigir(
            &self.root,
            previa_bytes
                .saturating_mul(3)
                .saturating_add(recuperado.saturating_mul(3))
                .saturating_add(s.huella.0)
                .saturating_add(16 * 1024 * 1024),
        )?;
        let anterior = self.root.join("activo.json");
        let mut maximos = if let Some(r) = retorno {
            r.maximos.clone()
        } else {
            match fs::read(&anterior) {
                Ok(raw) => {
                    let previo: Registro = serde_json::from_slice(&raw)
                        .map_err(|_| err("Registro anterior inválido."))?;
                    if previo.identidad.as_ref() == Some(&s.manifiesto.identidad) {
                        previo.maximos
                    } else {
                        BTreeMap::new()
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
                Err(e) => return io(Err(e)),
            }
        };
        agregar_maximos(c, &mut maximos)?;
        if retorno.is_none() && !anterior.exists() {
            let registro = Registro {
                generacion: "ORIGINAL".into(),
                anterior: base.to_string_lossy().into_owned(),
                identidad: Some(identidad(c)?),
                fase: "ORIGINAL".into(),
                fecha: ahora(),
                ejecutor: a.nombre.clone(),
                copia: String::new(),
                previa: None,
                previa_sha: None,
                maximos: maximos.clone(),
            };
            let raw = serde_json::to_vec(&registro)
                .map_err(|_| err("Registro de activación inválido."))?;
            reemplazar(&anterior, &raw)?;
        }
        let previa = if a.id.is_some() {
            let frase = proteccion::leer(&self.frase_path()).map_err(|_| {
                err("Configure la protección para generar la copia previa obligatoria.")
            })?;
            let m = manifiesto(c, a, "PREVIA")?;
            let (p, m, h) = self.producir(&base, m, &frase)?;
            let tx = c.transaction()?;
            let key = Self::insertar(&tx, &p, &m, &h, a.id, "PREVIA")?;
            auditar(&tx, a, "GENERAR_PREVIA", "backups", "OK", json!({"id":key}))?;
            tx.commit()?;
            Some((p, h.1))
        } else {
            None
        };
        self.fase(4);
        let generacion = id();
        let path = self.root.join("generaciones").join(&generacion);
        let dir = io(tempfile::tempdir_in(self.root.join("generaciones")))?;
        for f in &s.manifiesto.archivos {
            let p = dir.path().join(&f.nombre);
            io(fs::create_dir_all(p.parent().unwrap()))?;
            if copiar(&s.temporal.path().join(&f.nombre), &p)? != (f.bytes, f.sha256.clone()) {
                return Err(err("Los archivos de restauración cambiaron."));
            }
        }
        let p = dir.path().join("base.sqlite");
        let mut nueva = Connection::open(&p)?;
        nueva.execute_batch(
            "PRAGMA foreign_keys=ON; PRAGMA journal_mode=DELETE; PRAGMA synchronous=FULL;",
        )?;
        let admin: bool = nueva.query_row(
            "SELECT EXISTS(SELECT 1 FROM usuarios WHERE rol='ADMINISTRADOR' AND activo=1)",
            [],
            |r| r.get(0),
        )?;
        if !admin {
            return Err(err("La copia no tiene un administrador activo."));
        }
        let amb = &s.manifiesto.identidad.ambiente;
        let ruta = ruta_api(&nueva)?;
        let trusted = if a.id.is_some() && ruta.is_some() && ruta_api(c)? == ruta {
            ref_token(c)?.filter(|r| token_existe(amb, Some(r)))
        } else {
            None
        };
        let exige_token = trusted.is_none()
            && (ruta.is_some()
                || nueva
                    .prepare("SELECT 1 FROM series_comprobante WHERE activa=1")?
                    .exists([])?);
        let nombre = s
            .archivo
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or("Copia externa");
        let tx = nueva.transaction()?;
        registrar_catalogo(c, &tx, &self.root)?;
        let destino_importado = if s.backup.is_none() {
            let p = self
                .root
                .join("copias")
                .join(format!("importada-{generacion}.lgbackup"));
            let tmp = self.tmp()?;
            let t = tmp.path().join("importada.lgbackup");
            if copiar(&s.temporal.path().join("seleccion.lgbackup"), &t)? != s.huella {
                return Err(err("Cambió el paquete verificado."));
            }
            publicar(&t, &p, false)?;
            sync_dir(p.parent().unwrap())?;
            Self::insertar(&tx, &p, &s.manifiesto, &s.huella, None, "IMPORTADO")?;
            p
        } else {
            s.archivo.clone()
        };
        let registro_backup: i64 = tx.query_row(
            "SELECT id_backup FROM backups WHERE ruta_archivo=?1",
            [destino_importado.to_string_lossy().as_ref()],
            |r| r.get(0),
        )?;
        let registro_previa: Option<i64> = previa
            .as_ref()
            .map(|(p, _)| {
                tx.query_row(
                    "SELECT id_backup FROM backups WHERE ruta_archivo=?1",
                    [p.to_string_lossy().as_ref()],
                    |r| r.get(0),
                )
            })
            .transpose()?;
        tx.execute("INSERT INTO restauraciones(id_operacion,id_backup,id_backup_previa,evento_externo_id,actor_snapshot,nombre_copia_snapshot,ambiente,sha256_copia,estado)
            VALUES(?1,?2,?3,?1,?4,?5,?6,?7,'CONTROLES_PENDIENTES')",params![generacion,registro_backup,registro_previa,a.nombre,nombre,amb,s.huella.1])?;
        tx.execute(
            "UPDATE caja_chica SET por_conciliar=1,version=version+1 WHERE estado='ABIERTA'",
            [],
        )?;
        tx.execute("UPDATE comprobantes SET requiere_consulta=1,resultado_incierto=1,version=version+1 WHERE estado_envio IN ('PENDIENTE','ENVIANDO','ERROR')",[])?;
        aplicar_maximos(&tx, &maximos)?;
        tx.execute("UPDATE configuracion SET recuperacion_pendiente=1,revision_caja_pendiente=?1,revision_token_pendiente=?2,
            revision_series_pendiente=EXISTS(SELECT 1 FROM series_comprobante WHERE activa=1 AND requiere_conciliacion=1),nubefact_token_ref=?3,
            fecha_actualizacion=datetime('now','-5 hours'),version=version+1 WHERE id_config=1",params![caja(&tx)?,exige_token,trusted])?;
        auditar(
            &tx,
            &Actor {
                id: None,
                nombre: "Sistema".into(),
            },
            "RESTAURAR",
            "restauraciones",
            "OK",
            json!({"ejecutor":a.nombre,"evento":generacion,"copia":nombre,"previa":registro_previa}),
        )?;
        if tx.prepare("PRAGMA foreign_key_check")?.exists([])? {
            return Err(err(
                "La preparación de recuperación no preservó las relaciones.",
            ));
        }
        tx.commit()?;
        agregar_maximos(&nueva, &mut maximos)?;
        nueva.close().map_err(|(_, e)| e)?;
        // Windows exige acceso de escritura para sincronizar el archivo.
        let archivo = io(fs::OpenOptions::new().write(true).open(&p))?;
        io(archivo.sync_all())?;
        drop(archivo);
        let registro = Registro {
            generacion: generacion.clone(),
            anterior: base.to_string_lossy().into_owned(),
            identidad: Some(s.manifiesto.identidad.clone()),
            fase: "ACTIVA_CON_CONTROLES".into(),
            fecha: ahora(),
            ejecutor: a.nombre.clone(),
            copia: nombre.into(),
            previa: previa
                .as_ref()
                .map(|(p, _)| p.to_string_lossy().into_owned())
                .or_else(|| retorno.and_then(|r| r.previa.clone())),
            previa_sha: previa
                .map(|(_, h)| h)
                .or_else(|| retorno.and_then(|r| r.previa_sha.clone())),
            maximos,
        };
        let raw =
            serde_json::to_vec(&registro).map_err(|_| err("Registro de restauración inválido."))?;
        let mut registro_f = paquete::nuevo(&dir.path().join("registro.json"))?;
        io(registro_f.write_all(&raw))?;
        io(registro_f.sync_all())?;
        drop(registro_f);
        if dir.path().join("comprobantes").exists() {
            sync_dir(&dir.path().join("comprobantes"))?;
        }
        sync_dir(dir.path())?;
        #[cfg(test)]
        self.interrumpir_en(1)?;
        publicar(dir.path(), &path, true)?;
        sync_dir(path.parent().unwrap())?;
        let conexion = crate::db::abrir_db(&path.join("base.sqlite"), false)?;
        // Este puntero único publica un conjunto completo. El anterior no se sobrescribe.
        self.fase(5);
        // Conserva la identidad, la huella de la previa y los máximos aun si el puntero se pierde.
        reemplazar(&self.root.join("retorno.json"), &raw)?;
        #[cfg(test)]
        self.interrumpir_en(2)?;
        reemplazar(&anterior, &raw)?;
        #[cfg(test)]
        self.interrumpir_en(3)?;
        *c = conexion;
        *slot = None;
        Ok(json!({"restaurada":true,"requiere_login":true}))
    }
    pub fn maximo_conservado(&self, c: &Connection, tipo: &str, codigo: &str) -> R<i64> {
        let raw = match fs::read(self.root.join("activo.json")) {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => return io(Err(e)),
        };
        let r: Registro = serde_json::from_slice(&raw)
            .map_err(|_| err("No se pudo leer el registro de máximos de Backup."))?;
        if r.identidad.as_ref() != Some(&identidad(c)?) {
            return Ok(0);
        }
        Ok(r.maximos
            .get(&format!("{tipo}:{codigo}"))
            .copied()
            .unwrap_or(0))
    }
    pub fn resumen(&self, c: &Connection) -> R<Value> {
        let pendiente = flag(c)?;
        let series = if pendiente {
            facturacion::series_recuperacion(c)?
        } else {
            Vec::new()
        };
        let token: bool = c.query_row(
            "SELECT revision_token_pendiente FROM configuracion WHERE id_config=1",
            [],
            |r| r.get(0),
        )?;
        let requiere_token = pendiente
            && (token
                || (ruta_api(c)?.is_some()
                    && !token_existe(&identidad(c)?.ambiente, ref_token(c)?.as_deref())));
        Ok(
            json!({"pendiente":pendiente,"caja":pendiente&&caja(c)?,"token":requiere_token,"series":series}),
        )
    }
    pub fn guardar_token(
        &self,
        c: &mut Connection,
        a: &Actor,
        valor: &str,
        ruta: Option<&str>,
    ) -> R<Value> {
        if !flag(c)? {
            return Err(err("No existe una recuperación pendiente."));
        }
        let ruta = ruta_api(c)?
            .or_else(|| ruta.map(str::to_owned))
            .ok_or_else(|| err("Indique la URL del API de NubeFact."))?;
        let actor = a.id.ok_or(AppError::PermisoDenegado)?;
        facturacion::guardar_token(
            c,
            actor,
            ModoToken::Recuperacion,
            &ruta,
            valor,
            &AlmacenLocal,
        )?;
        self.resumen(c)
    }
    pub fn conciliar(
        &self,
        c: &mut Connection,
        a: &Actor,
        serie: i64,
        remoto: i64,
        refe: &str,
    ) -> R<Value> {
        if !flag(c)? {
            return Err(err("No existe una recuperación pendiente."));
        }
        let tx = c.transaction()?;
        let restauracion = activa(&tx)?;
        let actor = a.id.ok_or(AppError::PermisoDenegado)?;
        let vista =
            facturacion::ajustar(&tx, actor, serie, remoto, refe, None, Some(restauracion))?;
        auditar(
            &tx,
            a,
            "CONCILIAR_SERIE",
            "restauraciones",
            "OK",
            json!({"serie":serie,"siguiente":vista["siguiente"]}),
        )?;
        tx.commit()?;
        self.resumen(c)
    }
    pub fn finalizar(&self, c: &mut Connection, a: &Actor) -> R<Value> {
        let info = self.resumen(c)?;
        if info["pendiente"] != true
            || info["caja"] == true
            || info["token"] == true
            || info["series"]
                .as_array()
                .is_some_and(|l| l.iter().any(|s| s["pendiente"] == 1))
        {
            return Err(err(
                "Complete Caja, credenciales y series antes de habilitar operaciones.",
            ));
        }
        let tx = c.transaction()?;
        let r = activa(&tx)?;
        tx.execute("UPDATE configuracion SET recuperacion_pendiente=0,revision_caja_pendiente=0,revision_token_pendiente=0,revision_series_pendiente=0,version=version+1 WHERE id_config=1",[])?;
        tx.execute("UPDATE restauraciones SET estado='COMPLETA',fecha_fin=datetime('now','-5 hours') WHERE id_restauracion=?1",[r])?;
        auditar(
            &tx,
            a,
            "FINALIZAR_RECUPERACION",
            "restauraciones",
            "OK",
            json!({"id":r}),
        )?;
        tx.commit()?;
        self.resumen(c)
    }
}

impl BackupState {
    fn registro_previa(&self, ambiente: &str) -> Option<(Registro, PathBuf)> {
        // No se infiere una activación a partir de una generación huérfana.
        for nombre in ["activo.json", "retorno.json"] {
            let Ok(raw) = fs::read(self.root.join(nombre)) else {
                continue;
            };
            let Ok(mut r) = serde_json::from_slice::<Registro>(&raw) else {
                continue;
            };
            let Some(ident) = r.identidad.as_ref() else {
                continue;
            };
            if ident.ambiente != ambiente || ident.ruc.is_none() {
                continue;
            }
            let Some(previa) = r.previa.as_deref().map(PathBuf::from) else {
                continue;
            };
            if !self.gestionada(&previa) || !copia_disponible(&previa) {
                continue;
            }
            // Los registros v7 anteriores no guardaban la huella fuera de SQLite.
            // Solo se admite la registrada por el conjunto anterior de la misma empresa.
            if r.previa_sha.is_none() {
                let anterior = Path::new(&r.anterior);
                let Some(base) = self.root.parent() else {
                    continue;
                };
                if anterior != base.join("lubricentro_secure.db")
                    && (!anterior.starts_with(self.root.join("generaciones"))
                        || anterior.file_name().and_then(|s| s.to_str()) != Some("base.sqlite"))
                {
                    continue;
                }
                let Ok(c) = paquete::lectura(anterior) else {
                    continue;
                };
                if identidad(&c).ok().as_ref() != Some(ident) {
                    continue;
                }
                r.previa_sha = c.query_row(
                    "SELECT sha256 FROM backups WHERE ruta_archivo=?1 AND tipo='PREVIA' AND estado_local='VALIDO'",
                    [previa.to_string_lossy().as_ref()], |row| row.get(0),
                ).ok();
            }
            if r.previa_sha
                .as_ref()
                .is_some_and(|s| s.len() == 64 && s.bytes().all(|x| x.is_ascii_hexdigit()))
            {
                return Some((r, previa));
            }
        }
        None
    }

    pub fn previa_disponible(&self, ambiente: &str) -> bool {
        self.registro_previa(ambiente).is_some()
    }

    pub fn recuperar_previa(
        &self,
        ambiente: &str,
        frase: &str,
        usuario: &str,
        clave: &str,
    ) -> R<Connection> {
        let (r, previa) = self
            .registro_previa(ambiente)
            .ok_or_else(|| err("No hay una copia previa registrada que se pueda verificar."))?;
        let mut c = Connection::open_in_memory()?;
        c.execute_batch(include_str!(
            "../../../migrations/0001_initial_schema_v7.sql"
        ))?;
        self.fase(2);
        let temporal = self.tmp()?;
        let sellada = temporal.path().join("seleccion.lgbackup");
        let huella = copiar(&previa, &sellada)?;
        if r.previa_sha.as_ref() != Some(&huella.1) {
            return Err(err("La copia previa no coincide con la huella registrada."));
        }
        let manifiesto = paquete::abrir(&sellada, temporal.path(), frase)?;
        if r.identidad.as_ref() != Some(&manifiesto.identidad) {
            return Err(err(
                "La copia previa corresponde a otra empresa o ambiente.",
            ));
        }
        let username = usuario.trim().to_ascii_lowercase();
        {
            let datos = paquete::lectura(&temporal.path().join("base.sqlite"))?;
            let hash: Option<String> = datos.query_row(
                "SELECT password_hash FROM usuarios WHERE username=?1 AND rol='ADMINISTRADOR' AND activo=1 AND (bloqueado_hasta IS NULL OR bloqueado_hasta <= datetime('now','-5 hours'))",
                [&username], |row| row.get(0),
            ).optional()?;
            if !hash.is_some_and(|h| {
                crate::services::auth_service::AuthService::verify_password(clave, &h)
            }) {
                return Err(err(
                    "No se pudo autenticar al administrador de la copia previa.",
                ));
            }
        }
        let actor = Actor {
            id: None,
            nombre: format!("Recuperación de arranque: {username}"),
        };
        let seleccion = id();
        *self.seleccion.lock().map_err(|_| AppError::DbLock)? = Some(Seleccion {
            id: seleccion.clone(),
            archivo: previa,
            temporal,
            manifiesto,
            huella,
            actor: None,
            sesion: None,
            backup: None,
            creado: std::time::Instant::now(),
        });
        let resultado = self.restaurar_interno(&mut c, &actor, None, &seleccion, Some(&r));
        if resultado.is_err() {
            let _ = self.limpiar_seleccion();
        }
        resultado?;
        Ok(c)
    }
}

impl BackupState {
    pub fn validar_generacion_activa(&self, c: &Connection, ruta: &Path) -> R<()> {
        let registro: Registro =
            serde_json::from_slice(&io(fs::read(self.root.join("activo.json")))?)
                .map_err(|_| err("Registro de activación ilegible."))?;
        let raiz = ruta
            .parent()
            .ok_or_else(|| err("Ruta de la generación inválida."))?;
        let integral: String = c.query_row("PRAGMA quick_check", [], |row| row.get(0))?;
        if integral != "ok" || registro.identidad.as_ref() != Some(&identidad(c)?) {
            return Err(err("La base activa no pasó la comprobación de arranque."));
        }
        if paquete::referencias(c)?
            .iter()
            .any(|f| !copia_disponible(&raiz.join(f)))
        {
            return Err(err("Faltan archivos referenciados en el conjunto activo."));
        }
        Ok(())
    }
}

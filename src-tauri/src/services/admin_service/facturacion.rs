use super::{ambiente, auditar, error, longitud, R};
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::Serialize;
use serde_json::{json, Value};
use zeroize::Zeroizing;

pub const LIMITE: i64 = 99_999_999;
pub fn url(ruta: &str) -> R<reqwest::Url> {
    let ruta = longitud(ruta, 9, 500, "Ruta NubeFact")?;
    let url = reqwest::Url::parse(&ruta).map_err(|_| error("La ruta de NubeFact no es válida."))?;
    if url.scheme() != "https"
        || url.host_str() != Some("api.nubefact.com")
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() == "/"
        || ruta.contains('@')
    {
        return Err(error(
            "Usa la ruta HTTPS de api.nubefact.com, sin credenciales ni parámetros.",
        ));
    }
    Ok(url)
}

pub fn credencial(ambiente: &str, referencia: &str) -> R<keyring::Entry> {
    keyring::Entry::new(&format!("lubricentro_erp_nubefact_{ambiente}"), referencia)
        .map_err(|_| error("No se pudo abrir el almacén de credenciales."))
}
pub trait Secretos {
    fn existe(&self, ambiente: &str, referencia: &str) -> bool;
    fn guardar(&self, ambiente: &str, referencia: &str, valor: &str) -> R<()>;
    fn borrar(&self, ambiente: &str, referencia: &str);
}
pub struct AlmacenLocal;
impl Secretos for AlmacenLocal {
    fn existe(&self, a: &str, r: &str) -> bool {
        credencial(a, r)
            .and_then(|e| e.get_password().map_err(|_| error("Token no disponible.")))
            .map(Zeroizing::new)
            .is_ok_and(|v| !v.trim().is_empty())
    }
    fn guardar(&self, a: &str, r: &str, valor: &str) -> R<()> {
        credencial(a, r)?
            .set_password(valor)
            .map_err(|_| error("No se pudo guardar el token protegido."))
    }
    fn borrar(&self, a: &str, r: &str) {
        if let Ok(e) = credencial(a, r) {
            let _ = e.delete_password();
        }
    }
}
#[derive(Clone, Copy)]
pub enum ModoToken {
    Administracion(i64),
    Recuperacion,
}

pub fn configuracion(c: &Connection, secretos: &impl Secretos) -> R<Value> {
    let (ruta, referencia, version): (Option<String>, Option<String>, i64) = c.query_row(
        "SELECT nubefact_ruta_api,nubefact_token_ref,version FROM configuracion WHERE id_config=1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )?;
    let a = ambiente(c)?;
    let disponible = referencia
        .as_deref()
        .is_some_and(|r| secretos.existe(&a, r));
    Ok(
        json!({"ruta":ruta,"version":version,"token_configurado":disponible,"configuracion_completa":disponible && ruta.as_deref().is_some_and(|r|url(r).is_ok()),"series":series(c)?}),
    )
}
pub fn guardar_token(
    c: &mut Connection,
    actor: i64,
    modo: ModoToken,
    ruta: &str,
    token: &str,
    secretos: &impl Secretos,
) -> R<()> {
    url(ruta)?;
    let ruta = ruta.trim();
    let token = token.trim();
    if token.len() > 2048 || (!token.is_empty() && !token.bytes().all(|b| (33..=126).contains(&b)))
    {
        return Err(error("Escribe un token válido sin espacios."));
    }
    let tx = c.transaction()?;
    let (a, version, anterior, ruta_anterior):(String,i64,Option<String>,Option<String>)=tx.query_row(
        "SELECT ambiente_actual,version,nubefact_token_ref,nubefact_ruta_api FROM configuracion WHERE id_config=1",[],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)))?;
    if matches!(modo,ModoToken::Administracion(v) if v!=version) {
        return Err(error("La configuración cambió. Recarga Facturación."));
    }
    let nueva = (!token.is_empty()).then(|| format!("nubefact-{}", uuid::Uuid::new_v4()));
    if nueva.is_none()
        && (matches!(modo, ModoToken::Recuperacion)
            || !anterior.as_deref().is_some_and(|r| secretos.existe(&a, r)))
    {
        return Err(error("Ingresa el token para completar la configuración."));
    }
    if let Some(r) = &nueva {
        if let Err(e) = secretos.guardar(&a, r, token) {
            secretos.borrar(&a, r);
            return Err(e);
        }
    }
    let resultado = (|| {
        tx.execute("UPDATE configuracion SET nubefact_ruta_api=?1,nubefact_token_ref=?2,revision_token_pendiente=CASE WHEN ?3 THEN 0 ELSE revision_token_pendiente END,version=version+1,fecha_actualizacion=datetime('now','-5 hours') WHERE id_config=1",
            params![ruta,nueva.as_ref().or(anterior.as_ref()),matches!(modo,ModoToken::Recuperacion)])?;
        let antes = json!({"ambiente":a,"ruta_configurada":ruta_anterior.is_some(),"token_registrado":anterior.is_some()});
        let despues = json!({"ambiente":a,"ruta_configurada":true,"ruta_modificada":ruta_anterior.as_deref()!=Some(ruta),"token_reemplazado":nueva.is_some()});
        if matches!(modo, ModoToken::Recuperacion) {
            tx.execute("INSERT INTO auditoria(id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,antes_json,despues_json,resultado)
                SELECT id_usuario,'USUARIO',username,rol,'BACKUP','TOKEN_RECUPERACION','configuracion',?2,?3,'OK' FROM usuarios WHERE id_usuario=?1",
                params![actor,antes.to_string(),despues.to_string()])?;
        } else {
            auditar(
                &tx,
                actor,
                "CONFIGURAR_NUBEFACT",
                ("configuracion", "1"),
                &antes,
                &despues,
                None,
            )?;
        }
        tx.commit()?;
        Ok(())
    })();
    if resultado.is_err() {
        if let Some(r) = &nueva {
            secretos.borrar(&a, r);
        }
    } else if nueva.is_some() {
        if let Some(r) = &anterior {
            secretos.borrar(&a, r);
        }
    }
    resultado
}

const CAMPOS:&str="s.id_serie,s.tipo_comprobante,s.serie,s.activa,s.requiere_conciliacion,s.correlativo_actual,s.mayor_previo,s.ultimo_real_comprobado,coalesce((SELECT max(d.correlativo) FROM comprobantes d WHERE d.id_serie=s.id_serie),0),s.referencia_verificacion";
#[derive(Serialize)]
struct Serie {
    id: i64,
    tipo: String,
    codigo: String,
    activa: bool,
    pendiente: bool,
    local: i64,
    conservado: i64,
    real: i64,
    ultimo_documento: i64,
    referencia: Option<String>,
}
impl Serie {
    fn maximo(&self) -> i64 {
        self.local
            .max(self.conservado)
            .max(self.real)
            .max(self.ultimo_documento)
    }
    fn vista(&self) -> Value {
        json!({"id":self.id,"tipo":self.tipo,"codigo":self.codigo,"activa":self.activa,"pendiente":self.pendiente,
        "ultimo":self.maximo(),"siguiente":siguiente(self.maximo()),"referencia":self.referencia})
    }
}
fn siguiente(maximo: i64) -> Option<i64> {
    (maximo < LIMITE).then_some(maximo + 1)
}
fn fila(r: &Row<'_>) -> rusqlite::Result<Serie> {
    Ok(Serie {
        id: r.get(0)?,
        tipo: r.get(1)?,
        codigo: r.get(2)?,
        activa: r.get(3)?,
        pendiente: r.get(4)?,
        local: r.get(5)?,
        conservado: r.get(6)?,
        real: r.get(7)?,
        ultimo_documento: r.get(8)?,
        referencia: r.get(9)?,
    })
}
fn serie(c: &Connection, id: i64) -> R<Serie> {
    c.query_row(&format!("SELECT {CAMPOS} FROM series_comprobante s WHERE s.id_serie=?1 AND s.ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)"),[id],fila).optional()?.ok_or_else(||error("La serie no existe en este ambiente."))
}
pub fn series(c: &Connection) -> R<Vec<Value>> {
    let mut st=c.prepare(&format!("SELECT {CAMPOS} FROM series_comprobante s WHERE s.ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1) ORDER BY s.tipo_comprobante,s.serie"))?;
    let filas = st
        .query_map([], fila)?
        .map(|s| s.map(|s| s.vista()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(filas)
}
pub fn series_recuperacion(c: &Connection) -> R<Vec<Value>> {
    let mut st = c.prepare(&format!("SELECT {CAMPOS} FROM series_comprobante s WHERE s.activa=1 AND s.ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1) ORDER BY s.tipo_comprobante,s.serie"))?;
    let filas = st
        .query_map([], fila)?
        .map(|s| {
            s.map(|s| json!({
        "id": s.id, "tipo": s.tipo, "serie": s.codigo, "local": s.local.max(s.ultimo_documento),
        "previo": s.conservado, "remoto": s.real, "pendiente": if s.pendiente {1} else {0},
        "siguiente": siguiente(s.maximo())
    }))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(filas)
}
pub fn validar_serie(tipo: &str, codigo: &str) -> R<String> {
    let prefijo = match tipo {
        "BOLETA" | "NC_BOLETA" => b'B',
        "FACTURA" | "NC_FACTURA" => b'F',
        _ => return Err(error("Selecciona un tipo de comprobante válido.")),
    };
    let codigo = codigo.trim().to_ascii_uppercase();
    if codigo.len() != 4
        || !codigo.bytes().all(|b| b.is_ascii_alphanumeric())
        || codigo.as_bytes()[0] != prefijo
    {
        return Err(error(format!(
            "El código debe tener cuatro caracteres alfanuméricos y empezar con {}.",
            char::from(prefijo)
        )));
    }
    Ok(codigo)
}
fn numero(n: i64) -> R<()> {
    if !(0..=LIMITE).contains(&n) {
        return Err(error("El último número debe estar entre 0 y 99 999 999."));
    }
    Ok(())
}
pub fn registrar(
    c: &mut Connection,
    actor: i64,
    tipo: &str,
    codigo: &str,
    real: i64,
    previo: i64,
    referencia: Option<&str>,
) -> R<Value> {
    let codigo = validar_serie(tipo, codigo)?;
    numero(real)?;
    numero(previo)?;
    if previo > 0 && real == 0 {
        return Err(error("Backup conserva un máximo para esta serie. Indica el último número usado y su referencia."));
    }
    let referencia = if real > 0 {
        Some(longitud(referencia.unwrap_or(""), 5, 120, "Referencia")?)
    } else {
        None
    };
    let tx = c.transaction()?;
    let a = ambiente(&tx)?;
    if tx.query_row("SELECT EXISTS(SELECT 1 FROM series_comprobante WHERE ambiente=?1 AND tipo_comprobante=?2 AND serie=?3)",params![a,tipo,codigo],|r|r.get::<_,bool>(0))? {return Err(error("Ya existe esa serie para el tipo y ambiente."));}
    let maximo = real.max(previo);
    tx.execute("INSERT INTO series_comprobante(ambiente,tipo_comprobante,serie,correlativo_actual,mayor_previo,ultimo_real_comprobado,referencia_verificacion,fecha_verificacion) VALUES(?1,?2,?3,?4,?5,?6,?7,CASE WHEN ?7 IS NOT NULL THEN datetime('now','-5 hours') END)",
        params![a,tipo,codigo,maximo,previo,real,referencia])?;
    let id = tx.last_insert_rowid();
    let vista = serie(&tx, id)?.vista();
    auditar(
        &tx,
        actor,
        "REGISTRAR_SERIE",
        ("series_comprobante", &id.to_string()),
        &Value::Null,
        &vista,
        referencia.as_deref(),
    )?;
    tx.commit()?;
    Ok(vista)
}

pub fn estado_serie(
    c: &mut Connection,
    actor: i64,
    id: i64,
    activar: bool,
    activa_esperada: Option<i64>,
) -> R<Value> {
    let tx = c.transaction()?;
    let s = serie(&tx, id)?;
    if s.activa == activar {
        return Ok(s.vista());
    }
    if s.pendiente || (activar && siguiente(s.maximo()).is_none()) {
        return Err(error("La serie está pendiente de conciliación o agotada."));
    }
    let a = ambiente(&tx)?;
    let otra:Option<i64>=tx.query_row("SELECT id_serie FROM series_comprobante WHERE ambiente=?1 AND tipo_comprobante=?2 AND activa=1",params![a,s.tipo],|r|r.get(0)).optional()?;
    if activar && otra != activa_esperada {
        return Err(error(
            "Cambió la serie activa. Recarga y confirma el reemplazo.",
        ));
    }
    let previa = if activar {
        otra.map(|id| serie(&tx, id).map(|s| s.vista()))
            .transpose()?
    } else {
        None
    };
    if activar {
        tx.execute("UPDATE series_comprobante SET activa=0 WHERE ambiente=?1 AND tipo_comprobante=?2 AND activa=1",params![a,s.tipo])?;
    }
    tx.execute(
        "UPDATE series_comprobante SET activa=?2 WHERE id_serie=?1",
        params![id, activar],
    )?;
    let vista = serie(&tx, id)?.vista();
    auditar(
        &tx,
        actor,
        if activar {
            "ACTIVAR_SERIE"
        } else {
            "DESACTIVAR_SERIE"
        },
        ("series_comprobante", &id.to_string()),
        &json!({"serie":s.vista(),"serie_reemplazada":previa}),
        &vista,
        None,
    )?;
    tx.commit()?;
    Ok(vista)
}

pub fn ajustar(
    c: &Connection,
    actor: i64,
    id: i64,
    real: i64,
    referencia: &str,
    esperado: Option<i64>,
    restauracion: Option<i64>,
) -> R<Value> {
    numero(real)?;
    let referencia = longitud(referencia, 5, 120, "Referencia")?;
    let s = serie(c, id)?;
    let actual = s.maximo();
    if let Some(esperado) = esperado {
        if actual != esperado {
            return Err(error("La numeración cambió. Recarga la serie."));
        }
        if s.pendiente || real <= actual {
            return Err(error(
                "Solo puedes aumentar el piso de una serie conciliada.",
            ));
        }
    } else if restauracion.is_none() || !s.activa || !s.pendiente {
        return Err(error(
            "No hay una conciliación de recuperación pendiente para esta serie.",
        ));
    }
    let maximo = actual.max(real);
    c.execute("UPDATE series_comprobante SET correlativo_actual=?2,ultimo_real_comprobado=max(ultimo_real_comprobado,?3),referencia_verificacion=?4,fecha_verificacion=datetime('now','-5 hours'),requiere_conciliacion=0 WHERE id_serie=?1",
        params![id,maximo,real,referencia])?;
    c.execute("INSERT INTO conciliaciones_series(id_operacion,id_restauracion,id_serie,mayor_local,mayor_previo,mayor_declarado_remoto,correlativo_conservado,evidencia,id_usuario)
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",params![uuid::Uuid::new_v4().to_string(),restauracion,id,s.local.max(s.ultimo_documento),s.conservado.max(s.real),real,maximo,json!({"referencia":referencia}).to_string(),actor])?;
    let vista = serie(c, id)?.vista();
    if restauracion.is_none() {
        auditar(
            c,
            actor,
            "AUMENTAR_PISO_SERIE",
            ("series_comprobante", &id.to_string()),
            &s.vista(),
            &vista,
            Some(&referencia),
        )?;
    }
    Ok(vista)
}

pub fn reservar(c: &Connection, tipo: &str) -> R<(i64, String, i64)> {
    let id=c.query_row("SELECT id_serie FROM series_comprobante WHERE ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1) AND tipo_comprobante=?1 AND activa=1 AND requiere_conciliacion=0",[tipo],|r|r.get(0)).optional()?.ok_or_else(||error("Configura una serie activa y conciliada para este tipo de comprobante."))?;
    let s = serie(c, id)?;
    let n = siguiente(s.maximo()).ok_or_else(|| {
        error("La serie agotó su numeración. Activa otra serie en Administración.")
    })?;
    c.execute(
        "UPDATE series_comprobante SET correlativo_actual=?2 WHERE id_serie=?1",
        params![id, n],
    )?;
    Ok((id, s.codigo, n))
}

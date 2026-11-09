use super::{auditar, error, longitud, R};
use crate::services::{
    auth_service::AuthService, inventario_service::idempotencia::IdempotenciaService as Idem,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct Usuario {
    id: i64,
    login: String,
    nombre: String,
    rol: String,
    activo: bool,
    bloqueado: bool,
    debe_cambiar_clave: bool,
    version: i64,
}
const CAMPOS: &str = "id_usuario,username,nombre_completo,rol,activo,coalesce(bloqueado_hasta>datetime('now','-5 hours'),0),debe_cambiar_clave,version";
fn fila(r: &Row<'_>) -> rusqlite::Result<Usuario> {
    Ok(Usuario {
        id: r.get(0)?,
        login: r.get(1)?,
        nombre: r.get(2)?,
        rol: r.get(3)?,
        activo: r.get(4)?,
        bloqueado: r.get(5)?,
        debe_cambiar_clave: r.get(6)?,
        version: r.get(7)?,
    })
}
fn leer(c: &Connection, id: i64) -> R<Usuario> {
    c.query_row(
        &format!("SELECT {CAMPOS} FROM usuarios WHERE id_usuario=?1"),
        [id],
        fila,
    )
    .optional()?
    .ok_or_else(|| error("La cuenta no existe."))
}
fn validar_rol(rol: &str) -> R<()> {
    if !matches!(rol, "ADMINISTRADOR" | "CAJERO" | "MECANICO") {
        return Err(error("Selecciona un rol válido."));
    }
    Ok(())
}
fn pendientes(c: &Connection, id: i64) -> R<Value> {
    let caja: Option<String> = c
        .query_row(
            "SELECT codigo_caja FROM caja_chica WHERE id_usuario=?1 AND estado='ABIERTA'",
            [id],
            |r| r.get(0),
        )
        .optional()?;
    let mut st=c.prepare("SELECT codigo_ot FROM ordenes_trabajo WHERE id_mecanico=?1 AND estado IN ('EN_ESPERA','EN_PROCESO') ORDER BY id_ot")?;
    let ots = st
        .query_map([id], |r| r.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({"caja":caja,"ordenes":ots}))
}
pub fn listar(
    c: &Connection,
    actor: i64,
    busqueda: &str,
    rol: &str,
    estado: &str,
    pagina: i64,
) -> R<Value> {
    if !rol.is_empty() {
        validar_rol(rol)?;
    }
    let activo = match estado {
        "" => None,
        "ACTIVO" => Some(true),
        "INACTIVO" => Some(false),
        _ => return Err(error("Filtro de estado inválido.")),
    };
    let b = busqueda.trim();
    if b.chars().count() > 100 {
        return Err(error("La búsqueda admite hasta 100 caracteres."));
    }
    let f="FROM usuarios WHERE (instr(lower(username),lower(?1))>0 OR instr(lower(nombre_completo),lower(?1))>0 OR ?1='') AND (?2='' OR rol=?2) AND (?3 IS NULL OR activo=?3)";
    let total: i64 = c.query_row(
        &format!("SELECT count(*) {f}"),
        params![b, rol, activo],
        |r| r.get(0),
    )?;
    let paginas = ((total + 19) / 20).max(1);
    let pagina = pagina.clamp(1, paginas);
    let mut st = c.prepare(&format!(
        "SELECT {CAMPOS} {f} ORDER BY username,id_usuario LIMIT 20 OFFSET ?4"
    ))?;
    let usuarios = st
        .query_map(params![b, rol, activo, (pagina - 1) * 20], fila)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({"usuarios":usuarios,"pagina":pagina,"paginas":paginas,"total":total,"actor":actor}))
}

pub fn detalle(c: &Connection, id: i64) -> R<Value> {
    Ok(json!({"usuario":leer(c,id)?,"pendientes":pendientes(c,id)?}))
}
fn temporal() -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
fn op_valida(op: &str) -> R<()> {
    if uuid::Uuid::parse_str(op).is_err() {
        return Err(error("Identificador de operación inválido."));
    }
    Ok(())
}

pub fn crear(
    c: &mut Connection,
    actor: i64,
    op: &str,
    login: &str,
    nombre: &str,
    rol: &str,
) -> R<Value> {
    op_valida(op)?;
    let login = AuthService::normalizar_username(login)?;
    let nombre = AuthService::normalizar_nombre_completo(nombre)?;
    validar_rol(rol)?;
    let tx = c.transaction()?;
    let datos = json!({"login":login,"nombre":nombre,"rol":rol});
    if let Some(previo) = Idem::evaluar_y_registrar(&tx, Some(op), "ADMIN_CREAR", &datos, actor)? {
        return Ok(previo);
    }
    if tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM usuarios WHERE username=?1)",
        [&login],
        |r| r.get::<_, bool>(0),
    )? {
        return Err(error("Ya existe una cuenta con ese login."));
    }
    let clave = zeroize::Zeroizing::new(temporal());
    let hash = AuthService::hash_password(&clave)?;
    tx.execute("INSERT INTO usuarios(username,nombre_completo,rol,password_hash,debe_cambiar_clave) VALUES(?1,?2,?3,?4,1)",params![login,nombre,rol,hash])?;
    let id = tx.last_insert_rowid();
    let registro =
        json!({"id":id,"login":login,"rol":rol,"nombre":nombre,"debe_cambiar_clave":true});
    auditar(
        &tx,
        actor,
        "CREAR_CUENTA",
        ("usuarios", &id.to_string()),
        &Value::Null,
        &registro,
        None,
    )?;
    let resultado = json!({"id":id,"login":login,"repetida":true});
    Idem::finalizar_solicitud(&tx, Some(op), "ADMIN_CREAR", &datos, actor, &resultado)?;
    tx.commit()?;
    Ok(json!({"id":id,"login":login,"temporal":clave.as_str(),"repetida":false}))
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edicion {
    pub id: i64,
    pub version: i64,
    pub nombre: String,
    pub rol: String,
    pub activo: bool,
    pub motivo: Option<String>,
}

pub fn editar(c: &mut Connection, actor: i64, e: Edicion) -> R<Value> {
    let nombre = AuthService::normalizar_nombre_completo(&e.nombre)?;
    validar_rol(&e.rol)?;
    if e.id == actor {
        return Err(error(
            "Tu cuenta es de solo lectura. Usa Mi perfil para cambiar tu clave.",
        ));
    }
    let tx = c.transaction()?;
    let antes = leer(&tx, e.id)?;
    if antes.version != e.version {
        return Err(error("La cuenta cambió. Vuelve a abrir la edición."));
    }
    let cambio = antes.rol != e.rol || antes.activo != e.activo;
    let motivo = if cambio {
        Some(longitud(
            e.motivo.as_deref().unwrap_or(""),
            5,
            250,
            "Motivo",
        )?)
    } else {
        None
    };
    if !antes.activo && e.activo && antes.rol != e.rol {
        return Err(error("Reactiva primero la cuenta con su rol actual."));
    }
    if antes.activo
        && antes.rol == "ADMINISTRADOR"
        && (!e.activo || e.rol != "ADMINISTRADOR")
        && tx.query_row(
            "SELECT count(*) FROM usuarios WHERE rol='ADMINISTRADOR' AND activo=1",
            [],
            |r| r.get::<_, i64>(0),
        )? <= 1
    {
        return Err(error("Debe permanecer al menos un administrador activo."));
    }
    if antes.rol != e.rol || (antes.activo && !e.activo) {
        let p = pendientes(&tx, e.id)?;
        let mut refs = Vec::new();
        if let Some(caja) = p["caja"].as_str() {
            refs.push(format!("Caja {caja}"));
        }
        if let Some(ots) = p["ordenes"].as_array() {
            refs.extend(
                ots.iter()
                    .filter_map(Value::as_str)
                    .map(|s| format!("OT {s}")),
            );
        }
        if !refs.is_empty() {
            return Err(error(format!("Resuelve antes: {}.", refs.join(", "))));
        }
    }
    let reactivar = !antes.activo && e.activo;
    tx.execute("UPDATE usuarios SET nombre_completo=?2,rol=?3,activo=?4,version=version+1,intentos_fallidos=CASE WHEN ?5 THEN 0 ELSE intentos_fallidos END,bloqueado_hasta=CASE WHEN ?5 THEN NULL ELSE bloqueado_hasta END WHERE id_usuario=?1",params![e.id,nombre,e.rol,e.activo,reactivar])?;
    let despues = json!(leer(&tx, e.id)?);
    auditar(
        &tx,
        actor,
        "EDITAR_CUENTA",
        ("usuarios", &e.id.to_string()),
        &json!(antes),
        &despues,
        motivo.as_deref(),
    )?;
    tx.commit()?;
    Ok(despues)
}

pub fn desbloquear(c: &mut Connection, actor: i64, id: i64, version: i64) -> R<Value> {
    if id == actor {
        return Err(error(
            "No puedes desbloquear tu propia cuenta desde Administración.",
        ));
    }
    let tx = c.transaction()?;
    let antes = leer(&tx, id)?;
    if antes.version != version || !antes.activo || !antes.bloqueado {
        return Err(error(
            "La cuenta ya no está activa y bloqueada en la versión seleccionada.",
        ));
    }
    tx.execute("UPDATE usuarios SET intentos_fallidos=0,bloqueado_hasta=NULL,version=version+1 WHERE id_usuario=?1",[id])?;
    let despues = json!(leer(&tx, id)?);
    auditar(
        &tx,
        actor,
        "DESBLOQUEAR_CUENTA",
        ("usuarios", &id.to_string()),
        &json!(antes),
        &despues,
        None,
    )?;
    tx.commit()?;
    Ok(despues)
}

pub fn restablecer(c: &mut Connection, actor: i64, id: i64, version: i64, op: &str) -> R<Value> {
    op_valida(op)?;
    if id == actor {
        return Err(error("Usa Mi perfil para cambiar tu contraseña."));
    }
    let tx = c.transaction()?;
    let datos = json!({"id":id,"version":version});
    if let Some(previo) =
        Idem::evaluar_y_registrar(&tx, Some(op), "ADMIN_RESTABLECER", &datos, actor)?
    {
        return Ok(previo);
    }
    let antes = leer(&tx, id)?;
    if antes.version != version || !antes.activo {
        return Err(error("La cuenta cambió o está inactiva."));
    }
    let clave = zeroize::Zeroizing::new(temporal());
    let hash = AuthService::hash_password(&clave)?;
    tx.execute("UPDATE usuarios SET password_hash=?2,debe_cambiar_clave=1,intentos_fallidos=0,bloqueado_hasta=NULL,version=version+1 WHERE id_usuario=?1",params![id,hash])?;
    let despues = leer(&tx, id)?;
    let login = despues.login.clone();
    auditar(
        &tx,
        actor,
        "RESTABLECER_CLAVE",
        ("usuarios", &id.to_string()),
        &json!(antes),
        &json!(despues),
        None,
    )?;
    Idem::finalizar_solicitud(
        &tx,
        Some(op),
        "ADMIN_RESTABLECER",
        &datos,
        actor,
        &json!({"id":id,"login":login,"repetida":true}),
    )?;
    tx.commit()?;
    Ok(json!({"id":id,"login":login,"temporal":clave.as_str(),"repetida":false}))
}

pub mod ambiente;
pub mod facturacion;
pub mod usuarios;

use crate::{errors::AppError, services::session_manager::SessionState};
use rusqlite::{params, Connection};
use serde_json::{json, Value};

pub type R<T> = Result<T, AppError>;
pub fn error(mensaje: impl Into<String>) -> AppError {
    AppError::Validation(mensaje.into())
}

pub fn autorizar(c: &Connection, sesion: &mut SessionState) -> R<i64> {
    let id = sesion.validar_sesion_operativa(c)?;
    if sesion.rol.as_deref() != Some("ADMINISTRADOR") {
        return Err(AppError::PermisoDenegado);
    }
    if c.query_row(
        "SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1",
        [],
        |r| r.get::<_, bool>(0),
    )? {
        return Err(error("Completa primero la recuperación en Backup."));
    }
    Ok(id)
}

pub fn ambiente(c: &Connection) -> R<String> {
    Ok(c.query_row(
        "SELECT ambiente_actual FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?)
}

pub fn longitud(valor: &str, min: usize, max: usize, campo: &str) -> R<String> {
    let valor = valor.trim();
    if !(min..=max).contains(&valor.chars().count()) || valor.chars().any(char::is_control) {
        return Err(error(format!(
            "{campo}: escribe entre {min} y {max} caracteres válidos."
        )));
    }
    Ok(valor.to_owned())
}

pub fn auditar(
    c: &Connection,
    actor: i64,
    accion: &str,
    entidad: (&str, &str),
    antes: &Value,
    despues: &Value,
    motivo: Option<&str>,
) -> R<()> {
    let n = c.execute("INSERT INTO auditoria(id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,id_entidad,antes_json,despues_json,motivo,resultado)
        SELECT id_usuario,'USUARIO',username,rol,'ADMIN',?2,?3,?4,?5,?6,?7,'OK' FROM usuarios WHERE id_usuario=?1",
        params![actor,accion,entidad.0,entidad.1,antes.to_string(),despues.to_string(),motivo])?;
    if n != 1 {
        return Err(AppError::PermisoDenegado);
    }
    Ok(())
}

fn ruc_fijo(c: &Connection) -> R<bool> {
    Ok(c.query_row(
        "SELECT EXISTS(SELECT 1 FROM comprobantes WHERE ambiente='PRODUCCION')",
        [],
        |r| r.get(0),
    )?)
}

pub fn empresa(c: &Connection) -> R<Value> {
    let mut v: Value = serde_json::from_str(&c.query_row(
        "SELECT json_object('ruc',ruc,'razon_social',razon_social,'nombre_comercial',nombre_comercial,'direccion',direccion,'ubigeo',ubigeo,'telefono',telefono,'correo',correo,'version',version) FROM configuracion WHERE id_config=1",
        [], |r| r.get::<_, String>(0))?).map_err(|_| error("No se pudo leer la empresa."))?;
    v["ruc_fijo"] = json!(ruc_fijo(c)?);
    Ok(v)
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Empresa {
    pub ruc: String,
    pub razon_social: String,
    pub nombre_comercial: String,
    pub direccion: String,
    pub ubigeo: String,
    pub telefono: String,
    pub correo: String,
    pub version: i64,
    pub motivo: Option<String>,
}

pub fn guardar_empresa(c: &mut Connection, actor: i64, e: Empresa) -> R<Value> {
    let ruc = e.ruc.trim();
    let digitos = ruc.as_bytes();
    if digitos.len() != 11
        || !digitos.iter().all(u8::is_ascii_digit)
        || (11
            - digitos
                .iter()
                .zip([5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2])
                .map(|(n, p)| u32::from(*n - b'0') * p)
                .sum::<u32>()
                % 11)
            % 10
            != u32::from(digitos[10] - b'0')
    {
        return Err(error(
            "El RUC debe tener 11 dígitos y un dígito de control válido.",
        ));
    }
    let razon = longitud(&e.razon_social, 3, 150, "Razón social")?;
    let comercial = longitud(&e.nombre_comercial, 3, 100, "Nombre comercial")?;
    let direccion = longitud(&e.direccion, 3, 200, "Dirección")?;
    let ubigeo = e.ubigeo.trim();
    if ubigeo.len() != 6 || !ubigeo.bytes().all(|b| b.is_ascii_digit()) {
        return Err(error("Ubigeo: escribe seis dígitos del distrito."));
    }
    // SISCONCODE (INEI), versión 2026. Solo códigos de nivel distrital.
    if !include_str!("ubigeos_inei_2026.txt")
        .split_ascii_whitespace()
        .any(|codigo| codigo == ubigeo)
    {
        return Err(error(
            "Ubigeo: el código no corresponde a un distrito del Perú.",
        ));
    }
    let tel = e.telefono.trim();
    let tel = tel.strip_prefix("+51").unwrap_or(tel);
    let tel: String = tel.chars().filter(|c| !matches!(c, ' ' | '-')).collect();
    if !tel.is_empty() && (!matches!(tel.len(), 7 | 9) || !tel.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err(error(
            "El teléfono debe tener 7 o 9 dígitos; puedes anteponer +51.",
        ));
    }
    let correo = e.correo.trim();
    if !correo.is_empty()
        && (correo.chars().count() > 150
            || correo.chars().any(|c| c.is_whitespace() || c.is_control())
            || !correo.split_once('@').is_some_and(|(a, b)| {
                !a.is_empty()
                    && !b.contains('@')
                    && b.split('.').count() > 1
                    && b.split('.').all(|s| !s.is_empty())
            }))
    {
        return Err(error("Escribe un correo válido de hasta 150 caracteres."));
    }
    let tx = c.transaction()?;
    let antes = empresa(&tx)?;
    if antes["version"].as_i64() != Some(e.version) {
        return Err(error("La empresa cambió. Recarga la ficha."));
    }
    let motivo = if antes["ruc"].as_str().is_some_and(|v| v != ruc) {
        if antes["ruc_fijo"] == true {
            return Err(error("El RUC ya se usó en un comprobante de Producción."));
        }
        Some(longitud(
            e.motivo.as_deref().unwrap_or(""),
            5,
            250,
            "Motivo del cambio de RUC",
        )?)
    } else {
        None
    };
    tx.execute("UPDATE configuracion SET ruc=?1,razon_social=?2,nombre_comercial=?3,direccion=?4,ubigeo=?5,telefono=?6,correo=?7,version=version+1,fecha_actualizacion=datetime('now','-5 hours') WHERE id_config=1",
        params![ruc,razon,comercial,direccion,ubigeo,(!tel.is_empty()).then_some(tel),(!correo.is_empty()).then_some(correo)])?;
    let despues = empresa(&tx)?;
    auditar(
        &tx,
        actor,
        "EDITAR_EMPRESA",
        ("configuracion", "1"),
        &antes,
        &despues,
        motivo.as_deref(),
    )?;
    tx.commit()?;
    Ok(despues)
}

pub fn taller(c: &Connection) -> R<Value> {
    let mut st = c.prepare("SELECT z.id_zanja,z.activa,o.codigo_ot FROM zanjas z LEFT JOIN ordenes_trabajo o ON o.id_zanja_ejecucion=z.id_zanja AND o.estado='EN_PROCESO' ORDER BY z.id_zanja")?;
    let zanjas = st.query_map([], |r| Ok(json!({"id":r.get::<_,i64>(0)?,"activa":r.get::<_,bool>(1)?,"ot":r.get::<_,Option<String>>(2)?})))?.collect::<Result<Vec<_>,_>>()?;
    Ok(json!({"cantidad":zanjas.iter().filter(|z|z["activa"]==true).count(),"zanjas":zanjas}))
}

pub fn guardar_taller(c: &mut Connection, actor: i64, cantidad: i64, anterior: i64) -> R<Value> {
    if !(1..=6).contains(&cantidad) {
        return Err(error("Indica entre 1 y 6 zanjas."));
    }
    let tx = c.transaction()?;
    let antes = taller(&tx)?;
    if antes["cantidad"].as_i64() != Some(anterior) {
        return Err(error("Las zanjas cambiaron. Recarga la pestaña."));
    }
    if let Some(z) = antes["zanjas"].as_array().and_then(|v| {
        v.iter()
            .find(|z| z["id"].as_i64().unwrap_or(0) > cantidad && z["ot"].is_string())
    }) {
        return Err(error(format!(
            "La zanja {} está ocupada por {}. Resuélvela en Taller.",
            z["id"],
            z["ot"].as_str().unwrap_or("")
        )));
    }
    tx.execute(
        "UPDATE zanjas SET activa=(id_zanja<=?1) WHERE activa<>(id_zanja<=?1)",
        [cantidad],
    )?;
    let despues = taller(&tx)?;
    auditar(
        &tx,
        actor,
        "AJUSTAR_ZANJAS",
        ("zanjas", "capacidad"),
        &antes,
        &despues,
        None,
    )?;
    tx.commit()?;
    Ok(despues)
}

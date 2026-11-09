use super::{auditar, error, R};
use crate::{
    db,
    services::backup_service::{ruta_activa, BackupState},
};
use rusqlite::{Connection, OptionalExtension};
use serde_json::json;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const ARCHIVO: &str = "ambiente-activo.txt";
pub fn directorio(raiz: &Path, ambiente: &str) -> R<PathBuf> {
    match ambiente {
        "PRODUCCION" => Ok(raiz.to_path_buf()),
        "PRUEBA" => Ok(raiz.join("entornos").join("prueba")),
        _ => Err(error("Selecciona Prueba o Producción.")),
    }
}
pub fn preferido(raiz: &Path) -> R<String> {
    let a = match fs::read_to_string(raiz.join(ARCHIVO)) {
        Ok(v) => v.trim().to_owned(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::env::var("LUBRICENTRO_AMBIENTE").unwrap_or_else(|_| "PRODUCCION".into())
        }
        Err(_) => return Err(error("No se pudo leer el ambiente seleccionado.")),
    };
    directorio(raiz, &a)?;
    Ok(a)
}
pub fn abrir(raiz: &Path, ambiente: &str, backup: &BackupState) -> R<Connection> {
    let ruta = ruta_activa(raiz)?;
    let nueva = ruta == raiz.join("lubricentro_secure.db") && !ruta.exists();
    let conn = db::abrir_db_ambiente(&ruta, nueva, ambiente)?;
    if super::ambiente(&conn)? != ambiente {
        return Err(error("La base no corresponde al ambiente seleccionado."));
    }
    if ruta != raiz.join("lubricentro_secure.db") {
        backup.validar_generacion_activa(&conn, &ruta)?;
    }
    backup.registrar_instalacion()?;
    Ok(conn)
}

pub fn preparar_cambio(c: &mut Connection, actor: i64, raiz: &Path, destino: &str) -> R<()> {
    let origen = super::ambiente(c)?;
    if destino == origen {
        return Err(error("Este ambiente ya está activo."));
    }
    let carpeta = directorio(raiz, destino)?;
    let caja: Option<String> = c
        .query_row(
            "SELECT codigo_caja FROM caja_chica WHERE estado='ABIERTA'",
            [],
            |r| r.get(0),
        )
        .optional()?;
    if let Some(caja) = caja {
        return Err(error(format!(
            "Cierra el turno {caja} antes de cambiar de ambiente."
        )));
    }
    let b = BackupState::new(&carpeta)?;
    let conn = abrir(&carpeta, destino, &b).map_err(|_| {
        error("No se pudo abrir el ambiente destino. El ambiente actual sigue activo.")
    })?;
    let integridad: String = conn
        .query_row("PRAGMA quick_check(1)", [], |r| r.get(0))
        .map_err(|_| error("No se pudo comprobar el ambiente destino. El actual sigue activo."))?;
    if integridad != "ok" {
        return Err(error(
            "El ambiente destino no pasó la comprobación. El actual sigue activo.",
        ));
    }
    drop(conn);

    let mut temp = tempfile::NamedTempFile::new_in(raiz)
        .map_err(|_| error("No se pudo preparar la selección del ambiente."))?;
    writeln!(temp, "{destino}")
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|_| error("No se pudo preparar la selección del ambiente."))?;
    let tx = c.transaction()?;
    auditar(
        &tx,
        actor,
        "SOLICITAR_CAMBIO_AMBIENTE",
        ("configuracion", "1"),
        &json!({"ambiente":origen}),
        &json!({"destino":destino}),
        None,
    )?;
    tx.commit()?;
    temp.persist(raiz.join(ARCHIVO))
        .map_err(|_| error("No se pudo guardar el ambiente destino. El actual sigue activo."))?;
    Ok(())
}

use crate::{
    errors::AppError,
    services::{
        backup_service::{vacia, Actor, BackupState, R},
        session_manager::SessionState,
    },
};
use rusqlite::{Connection, OptionalExtension};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{AppHandle, Manager, State};
use zeroize::Zeroizing;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolicitudBackup {
    accion: String,
    inicial: Option<bool>,
    tipo: Option<String>,
    pagina: Option<i64>,
    id_backup: Option<i64>,
    id_restauracion: Option<i64>,
    ruta: Option<String>,
    frase: Option<String>,
    seleccion: Option<String>,
    id_serie: Option<i64>,
    ultimo_real: Option<i64>,
    referencia: Option<String>,
    confirmado: Option<bool>,
}
fn invalid(m: &str) -> AppError {
    AppError::Validation(m.into())
}
fn actor(c: &Connection, s: &mut SessionState, inicial: bool) -> R<Actor> {
    if inicial {
        if s.usuario_id.is_some() || !vacia(c)? {
            return Err(AppError::PermisoDenegado);
        }
        return Ok(Actor {
            id: None,
            nombre: "Restauración inicial".into(),
        });
    }
    let uid = s.validar_sesion_operativa(c)?;
    if s.rol.as_deref() != Some("ADMINISTRADOR") {
        return Err(AppError::PermisoDenegado);
    }
    let username = c.query_row(
        "SELECT username FROM usuarios WHERE id_usuario=?1",
        [uid],
        |r| r.get(0),
    )?;
    Ok(Actor {
        id: Some(uid),
        nombre: username,
    })
}

#[tauri::command]
pub fn backup_contexto_cmd(
    db_state: State<'_, Mutex<Connection>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> R<Value> {
    let mut s = session_state.lock().map_err(|_| AppError::DbLock)?;
    let c = db_state.lock().map_err(|_| AppError::DbLock)?;
    let ambiente: String = c.query_row(
        "SELECT ambiente_actual FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?;
    let generacion: Option<String> = c
        .query_row(
            "SELECT evento_externo_id FROM restauraciones ORDER BY id_restauracion DESC LIMIT 1",
            [],
            |r| r.get(0),
        )
        .optional()?;
    if s.usuario_id.is_none() {
        return Ok(json!({"inicial":vacia(&c)?,"ambiente":ambiente,"generacion":generacion}));
    }
    s.validar_sesion_operativa(&c)?;
    if s.rol.as_deref() != Some("ADMINISTRADOR") {
        return Ok(json!({"ambiente":ambiente,"generacion":generacion}));
    }
    let pendiente: bool = c.query_row(
        "SELECT recuperacion_pendiente FROM configuracion WHERE id_config=1",
        [],
        |r| r.get(0),
    )?;
    Ok(
        json!({"pendiente":pendiente,"aviso":BackupState::aviso(&c)?,"ambiente":ambiente,"generacion":generacion}),
    )
}
#[tauri::command]
pub async fn backup_elegir_cmd(app: AppHandle, carpeta: bool, inicial: bool) -> R<Option<String>> {
    if carpeta && inicial {
        return Err(AppError::PermisoDenegado);
    }
    {
        let ss = app.state::<Mutex<SessionState>>();
        let db = app.try_state::<Mutex<Connection>>().ok_or_else(|| {
            invalid("La base no está disponible durante la recuperación de arranque.")
        })?;
        let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
        let c = db.lock().map_err(|_| AppError::DbLock)?;
        actor(&c, &mut s, inicial)?;
    }
    tauri::async_runtime::spawn_blocking(move || {
        crate::services::backup_service::elegir_ruta(carpeta)
    })
    .await
    .map_err(|_| invalid("No se pudo abrir el selector."))?
}

#[tauri::command]
pub async fn backup_operacion_cmd(app: AppHandle, req: SolicitudBackup) -> R<Value> {
    let inicial = req.inicial == Some(true);
    if inicial && !["comprobar", "restaurar", "cancelar"].contains(&req.accion.as_str()) {
        return Err(AppError::PermisoDenegado);
    }
    let snapshot_sesion = {
        let ss = app.state::<Mutex<SessionState>>();
        let db = app.try_state::<Mutex<Connection>>().ok_or_else(|| {
            invalid("La base no está disponible durante la recuperación de arranque.")
        })?;
        let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
        let c = db.lock().map_err(|_| AppError::DbLock)?;
        actor(&c, &mut s, inicial)?;
        s.session_id.clone()
    };
    // El envío remoto dura más que una transacción SQLite. Esperar también su respuesta
    // evita escribirla sobre otra generación o sobre un comprobante con ID reutilizado.
    let _envios = if req.accion == "restaurar" {
        Some(
            app.state::<BackupState>()
                .envios
                .clone()
                .write_owned()
                .await,
        )
    } else {
        None
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _envios = _envios;
        let db = app.try_state::<Mutex<Connection>>().ok_or_else(|| {
            invalid("La base no está disponible durante la recuperación de arranque.")
        })?;
        let ss = app.state::<Mutex<SessionState>>();
        let b = app.state::<BackupState>();
        // Consultar el historial no modifica respaldos ni necesita el bloqueo de escritura.
        let _trabajo = if matches!(req.accion.as_str(), "estado" | "detalle_restauracion") {
            None
        } else {
            Some(b.ocupacion()?)
        };
        if req.accion == "restaurar" {
            if let Some(t) = _trabajo.as_ref() {
                t.proteger_cierre();
            }
        }
        let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
        let mut c = db.lock().map_err(|_| AppError::DbLock)?;
        let a = actor(&c, &mut s, inicial)?;
        if s.session_id != snapshot_sesion {
            return Err(AppError::NoAutenticado);
        }
        let frase = req.frase.map(Zeroizing::new);
        let secreto = || {
            frase
                .as_ref()
                .map(|x| x.as_str())
                .ok_or_else(|| invalid("Ingrese la frase o credencial requerida."))
        };
        let seleccion = || {
            req.seleccion
                .as_deref()
                .ok_or_else(|| invalid("Seleccione una copia comprobada."))
        };
        let confirmacion = || {
            if req.confirmado == Some(true) {
                Ok(())
            } else {
                Err(invalid("Confirme la operación antes de continuar."))
            }
        };
        match req.accion.as_str() {
            "estado" => b.estado(&c, req.tipo.as_deref(), req.pagina.unwrap_or(1)),
            "detalle_restauracion" => {
                if inicial { return Err(AppError::PermisoDenegado); }
                let id = req.id_restauracion.ok_or_else(|| invalid("Indica la restauración."))?;
                Ok(c.query_row(
                    "SELECT nombre_copia_snapshot,actor_snapshot,fecha_inicio,fecha_fin,estado FROM restauraciones
                     WHERE id_restauracion=?1 AND ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)",
                    [id], |r| Ok(serde_json::json!({
                        "nombre":r.get::<_,String>(0)?, "actor":r.get::<_,String>(1)?,
                        "inicio":r.get::<_,String>(2)?, "fin":r.get::<_,Option<String>>(3)?, "estado":r.get::<_,String>(4)?,
                    }))
                )?)
            }
            "configurar" => b.configurar(&c, &a, secreto()?),
            "generar" => {
                drop(c);
                drop(s);
                b.generar(&db, &a, "MANUAL")
            }
            "comprobar" => {
                let sesion = s.session_id.clone();
                drop(c);
                drop(s);
                b.comprobar(
                    &db,
                    &a,
                    sesion,
                    req.id_backup,
                    req.ruta.map(PathBuf::from),
                    frase.as_ref().map(|x| x.as_str()),
                )
            }
            "revalidar" => b.revalidar(&c, &a, s.session_id.as_deref(), seleccion()?),
            "exportar" => b.exportar(
                &c,
                &a,
                s.session_id.as_deref(),
                seleccion()?,
                Path::new(
                    req.ruta
                        .as_deref()
                        .ok_or_else(|| invalid("Seleccione una carpeta."))?,
                ),
            ),
            "eliminar_previa" => {
                confirmacion()?;
                b.eliminar_previa(
                    &c,
                    &a,
                    req.id_backup
                        .ok_or_else(|| invalid("Seleccione una copia previa."))?,
                )
            }
            "restaurar" => {
                confirmacion()?;
                let data = b
                    .restaurar(&mut c, &a, s.session_id.as_deref(), seleccion()?)
                    .map_err(|e| invalid(&format!("{e} La base activa no se cambió.")))?;
                *s = SessionState::new();
                if let Err(e) = app.state::<crate::services::reportes_service::EstadoReportes>().limpiar() {
                    tracing::warn!("No se pudo liberar el corte de Reportes tras restaurar: {e}");
                }
                Ok(data)
            }
            "token" => b.guardar_token(&mut c, &a, secreto()?, req.ruta.as_deref()),
            "conciliar" => b.conciliar(
                &mut c,
                &a,
                req.id_serie
                    .ok_or_else(|| invalid("Seleccione una serie."))?,
                req.ultimo_real
                    .ok_or_else(|| invalid("Ingrese el último número real."))?,
                req.referencia.as_deref().unwrap_or(""),
            ),
            "finalizar" => b.finalizar(&mut c, &a),
            "cancelar" => {
                b.limpiar_seleccion()?;
                Ok(json!({"cancelada":true}))
            }
            _ => Err(invalid("Operación de Backup inválida.")),
        }
    })
    .await
    .map_err(|_| invalid("La tarea de Backup se interrumpió."))?
}

pub struct EstadoArranque {
    pub ambiente: String,
    pub bloqueo: Option<String>,
    intento: Mutex<Option<std::time::Instant>>,
}

impl EstadoArranque {
    pub fn new(ambiente: String, bloqueo: Option<String>) -> Self {
        Self {
            ambiente,
            bloqueo,
            intento: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn backup_arranque_cmd(app: AppHandle) -> Value {
    let estado = app.state::<EstadoArranque>();
    let bloqueado = app.try_state::<Mutex<Connection>>().is_none();
    let previa = bloqueado
        && app
            .state::<BackupState>()
            .previa_disponible(&estado.ambiente);
    json!({"bloqueado":bloqueado,"motivo":estado.bloqueo,"ambiente":estado.ambiente,"previa":previa})
}

#[tauri::command]
pub fn backup_progreso_cmd(backup: State<'_, BackupState>) -> Value {
    json!({"fase":backup.fase_actual(),"no_cerrar":backup.cierre_bloqueado()})
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolicitudArranque {
    usuario: String,
    clave: String,
    frase: String,
    confirmado: bool,
}

#[tauri::command]
pub async fn backup_recuperar_arranque_cmd(app: AppHandle, req: SolicitudArranque) -> R<()> {
    tauri::async_runtime::spawn_blocking(move || {
        let b = app.state::<BackupState>();
        let trabajo = b.ocupacion()?;
        let estado = app.state::<EstadoArranque>();
        if !req.confirmado
            || estado.bloqueo.is_none()
            || app.try_state::<Mutex<Connection>>().is_some()
        {
            return Err(AppError::PermisoDenegado);
        }
        if req.usuario.len() > 120 || req.clave.len() > 512 || req.frase.len() > 512 {
            return Err(invalid("Credenciales de recuperación inválidas."));
        }
        {
            let mut intento = estado.intento.lock().map_err(|_| AppError::DbLock)?;
            if intento.is_some_and(|i| i.elapsed().as_secs() < 3) {
                return Err(invalid("Espere unos segundos antes de reintentar."));
            }
            *intento = Some(std::time::Instant::now());
        }
        trabajo.proteger_cierre();
        let clave = Zeroizing::new(req.clave);
        let frase = Zeroizing::new(req.frase);
        let conn = b.recuperar_previa(&estado.ambiente, &frase, &req.usuario, &clave)?;
        app.manage(Mutex::new(conn));
        Ok(())
    })
    .await
    .map_err(|_| invalid("La recuperación de arranque se interrumpió."))?
}

use crate::{
    errors::AppError,
    services::{
        admin_service::{self as adm, ambiente, facturacion as fact, usuarios, R},
        backup_service::BackupState,
        session_manager::SessionState,
    },
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use zeroize::Zeroizing;

#[derive(Deserialize)]
#[serde(tag = "accion", rename_all = "snake_case", deny_unknown_fields)]
pub enum SolicitudAdmin {
    Usuarios {
        busqueda: String,
        rol: String,
        estado: String,
        pagina: i64,
    },
    DetalleUsuario {
        id: i64,
    },
    CrearUsuario {
        operacion: String,
        login: String,
        nombre: String,
        rol: String,
    },
    EditarUsuario {
        datos: usuarios::Edicion,
    },
    Desbloquear {
        id: i64,
        version: i64,
    },
    Restablecer {
        id: i64,
        version: i64,
        operacion: String,
    },
    Empresa,
    GuardarEmpresa {
        datos: adm::Empresa,
    },
    Facturacion,
    GuardarToken {
        ruta: String,
        token: String,
        version: i64,
    },
    RegistrarSerie {
        tipo: String,
        codigo: String,
        ultimo: i64,
        referencia: Option<String>,
    },
    EstadoSerie {
        id: i64,
        activa: bool,
        activa_esperada: Option<i64>,
    },
    AumentarSerie {
        id: i64,
        ultimo: i64,
        referencia: String,
        maximo_esperado: i64,
    },
    Taller,
    GuardarTaller {
        cantidad: i64,
        anterior: i64,
    },
}

#[tauri::command]
pub async fn admin_operacion_cmd(app: AppHandle, req: SolicitudAdmin) -> R<Value> {
    // Vincula la petición al inicio de sesión que la emitió antes de pasar a un hilo bloqueante.
    let inicio = {
        let ss = app.state::<Mutex<SessionState>>();
        let db = app
            .try_state::<Mutex<Connection>>()
            .ok_or(AppError::NoAutenticado)?;
        let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
        let c = db.lock().map_err(|_| AppError::DbLock)?;
        adm::autorizar(&c, &mut s)?;
        s.session_id.clone().ok_or(AppError::NoAutenticado)?
    };
    let bloqueo_envios = if matches!(&req, SolicitudAdmin::GuardarToken { .. }) {
        let envios = app.state::<BackupState>().envios.clone();
        Some(envios.write_owned().await)
    } else {
        None
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _envios = bloqueo_envios;
        let ss = app.state::<Mutex<SessionState>>();
        let db = app
            .try_state::<Mutex<Connection>>()
            .ok_or(AppError::NoAutenticado)?;
        let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
        let mut c = db.lock().map_err(|_| AppError::DbLock)?;
        if s.session_id.as_deref() != Some(inicio.as_str()) {
            return Err(AppError::NoAutenticado);
        }
        let actor = adm::autorizar(&c, &mut s)?;
        match req {
            SolicitudAdmin::Usuarios {
                busqueda,
                rol,
                estado,
                pagina,
            } => usuarios::listar(&c, actor, &busqueda, &rol, &estado, pagina),
            SolicitudAdmin::DetalleUsuario { id } => usuarios::detalle(&c, id),
            SolicitudAdmin::CrearUsuario {
                operacion,
                login,
                nombre,
                rol,
            } => usuarios::crear(&mut c, actor, &operacion, &login, &nombre, &rol),
            SolicitudAdmin::EditarUsuario { datos } => {
                let id = datos.id;
                let res = usuarios::editar(&mut c, actor, datos)?;
                s.invalidar_sesion_usuario(id);
                Ok(res)
            }
            SolicitudAdmin::Desbloquear { id, version } => {
                usuarios::desbloquear(&mut c, actor, id, version)
            }
            SolicitudAdmin::Restablecer {
                id,
                version,
                operacion,
            } => {
                let res = usuarios::restablecer(&mut c, actor, id, version, &operacion)?;
                s.invalidar_sesion_usuario(id);
                Ok(res)
            }
            SolicitudAdmin::Empresa => adm::empresa(&c),
            SolicitudAdmin::GuardarEmpresa { datos } => adm::guardar_empresa(&mut c, actor, datos),
            SolicitudAdmin::Facturacion => fact::configuracion(&c, &fact::AlmacenLocal),
            SolicitudAdmin::GuardarToken {
                ruta,
                token,
                version,
            } => {
                let token = Zeroizing::new(token);
                fact::guardar_token(
                    &mut c,
                    actor,
                    fact::ModoToken::Administracion(version),
                    &ruta,
                    &token,
                    &fact::AlmacenLocal,
                )?;
                Ok(json!({"guardado":true}))
            }
            SolicitudAdmin::RegistrarSerie {
                tipo,
                codigo,
                ultimo,
                referencia,
            } => {
                let codigo = fact::validar_serie(&tipo, &codigo)?;
                let previo = app
                    .state::<BackupState>()
                    .maximo_conservado(&c, &tipo, &codigo)?;
                fact::registrar(
                    &mut c,
                    actor,
                    &tipo,
                    &codigo,
                    ultimo,
                    previo,
                    referencia.as_deref(),
                )
            }
            SolicitudAdmin::EstadoSerie {
                id,
                activa,
                activa_esperada,
            } => fact::estado_serie(&mut c, actor, id, activa, activa_esperada),
            SolicitudAdmin::AumentarSerie {
                id,
                ultimo,
                referencia,
                maximo_esperado,
            } => {
                let tx = c.transaction()?;
                let res = fact::ajustar(
                    &tx,
                    actor,
                    id,
                    ultimo,
                    &referencia,
                    Some(maximo_esperado),
                    None,
                )?;
                tx.commit()?;
                Ok(res)
            }
            SolicitudAdmin::Taller => adm::taller(&c),
            SolicitudAdmin::GuardarTaller { cantidad, anterior } => {
                adm::guardar_taller(&mut c, actor, cantidad, anterior)
            }
        }
    })
    .await
    .map_err(|_| adm::error("Se interrumpió la operación de Administración."))?
}

#[tauri::command]
pub fn admin_cambiar_ambiente_cmd(
    app: AppHandle,
    backup: State<'_, BackupState>,
    db: State<'_, Mutex<Connection>>,
    sesiones: State<'_, Mutex<SessionState>>,
    destino: String,
    confirmado: bool,
) -> R<()> {
    if !confirmado {
        return Err(adm::error("Confirma el ambiente de destino."));
    }
    let _envios = backup
        .envios
        .try_write()
        .map_err(|_| adm::error("Espera a que termine el envío o la consulta a NubeFact."))?;
    let _tarea = backup.ocupacion()?;
    let mut s = sesiones
        .try_lock()
        .map_err(|_| adm::error("Hay una operación activa. Intenta de nuevo."))?;
    let mut c = db
        .try_lock()
        .map_err(|_| adm::error("Hay una operación activa. Intenta de nuevo."))?;
    let actor = adm::autorizar(&c, &mut s)?;
    let raiz = app
        .path()
        .app_data_dir()
        .map_err(|_| adm::error("No se pudo abrir la carpeta de datos."))?;
    ambiente::preparar_cambio(&mut c, actor, &raiz, &destino)?;
    *s = SessionState::new();
    // Mantiene las exclusiones hasta salir. El nuevo proceso abre solo el destino.
    app.restart()
}

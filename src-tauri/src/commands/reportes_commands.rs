use crate::{
    errors::AppError,
    services::{
        admin_service::{self as adm, R},
        backup_service::elegir_guardado,
        reportes_service::{
            self as rep,
            exportar::{self, Formato},
            EstadoReportes, Filtros, Reporte,
        },
        session_manager::SessionState,
    },
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{path::Path, sync::Mutex};
use tauri::{AppHandle, Manager};

#[derive(Deserialize)]
#[serde(tag = "accion", rename_all = "snake_case", deny_unknown_fields)]
pub enum SolicitudReportes {
    Consultar {
        reporte: Reporte,
        filtros: Filtros,
    },
    Pagina {
        id: String,
        pagina: usize,
    },
    Detalle {
        id: String,
        fila: usize,
    },
    Origen {
        id: String,
        fila: usize,
        detalle: Option<usize>,
        indice: usize,
    },
    Exportar {
        id: String,
        reporte: Reporte,
        filtros: Filtros,
        formato: Formato,
    },
    Descartar {
        id: String,
    },
}

fn con_sesion<T>(
    app: &AppHandle,
    esperada: Option<&str>,
    f: impl FnOnce(&mut Connection, &str, &str, i64) -> R<T>,
) -> R<T> {
    let ss = app.state::<Mutex<SessionState>>();
    let db = app
        .try_state::<Mutex<Connection>>()
        .ok_or(AppError::NoAutenticado)?;
    let mut s = ss.lock().map_err(|_| AppError::DbLock)?;
    if esperada.is_some_and(|e| s.session_id.as_deref() != Some(e)) {
        return Err(AppError::NoAutenticado);
    }
    let mut c = db.lock().map_err(|_| AppError::DbLock)?;
    let id = rep::autorizar(&c, &mut s, esperada.is_none())?;
    let sesion = s.session_id.as_deref().ok_or(AppError::NoAutenticado)?;
    let ambiente = adm::ambiente(&c)?;
    f(&mut c, sesion, &ambiente, id)
}

#[tauri::command]
pub async fn reportes_operacion_cmd(app: AppHandle, req: SolicitudReportes) -> R<Value> {
    let (inicio, consulta) = con_sesion(&app, None, |_, s, _, _| {
        let consulta = matches!(&req, SolicitudReportes::Consultar { .. })
            .then(|| app.state::<EstadoReportes>().iniciar_consulta());
        Ok((s.to_owned(), consulta))
    })?;

    tauri::async_runtime::spawn_blocking(move || {
        let estado = app.state::<EstadoReportes>();
        if let SolicitudReportes::Exportar {
            id,
            reporte,
            filtros,
            formato,
        } = req
        {
            return exportar_corte(&app, &inicio, &estado, &id, reporte, &filtros, formato);
        }
        con_sesion(&app, Some(&inicio), |db, sesion, ambiente, _| match req {
            SolicitudReportes::Consultar { reporte, filtros } => {
                let tx = db.transaction()?;
                let c = rep::consultar(&tx, reporte, filtros, ambiente, rep::ahora_lima())
                    .inspect_err(|e| {
                        if let AppError::Database(d) = e {
                            tracing::warn!(error = %d, "Falló una consulta de reportes");
                        }
                    })?;
                tx.commit()?;
                let numero = consulta.ok_or_else(|| rep::error("Consulta no iniciada."))?;
                let corte = estado.publicar(sesion, c, numero)?;
                Ok(json!(corte.pagina(1)?))
            }
            SolicitudReportes::Pagina { id, pagina } => {
                let corte = estado.obtener(sesion, ambiente, &id)?;
                Ok(json!(corte.pagina(pagina)?))
            }
            SolicitudReportes::Detalle { id, fila } => {
                let corte = estado.obtener(sesion, ambiente, &id)?;
                Ok(json!(corte.detalle(fila)?))
            }
            SolicitudReportes::Origen {
                id,
                fila,
                detalle,
                indice,
            } => {
                let corte = estado.obtener(sesion, ambiente, &id)?;
                let mut origen = corte.origen(fila, detalle, indice)?.clone();
                origen.vigente(db, ambiente)?;
                if origen.destino == rep::Destino::Inventario {
                    origen.producto = Some(db.query_row(
                        "SELECT codigo,nombre,unidad_medida FROM productos WHERE id_producto=?1",
                        [origen.id],
                        |r| {
                            Ok(rep::ProductoOrigen {
                                id_producto: origen.id,
                                codigo: r.get(0)?,
                                nombre: r.get(1)?,
                                unidad_medida: r.get(2)?,
                            })
                        },
                    )?);
                }
                Ok(json!(origen))
            }
            SolicitudReportes::Descartar { id } => {
                estado.descartar(sesion, ambiente, &id)?;
                Ok(Value::Null)
            }
            SolicitudReportes::Exportar { .. } => unreachable!(),
        })
    })
    .await
    .map_err(|_| rep::error("No se pudo completar la tarea de Reportes."))?
}

fn exportar_corte(
    app: &AppHandle,
    inicio: &str,
    estado: &EstadoReportes,
    id: &str,
    reporte: Reporte,
    filtros: &Filtros,
    formato: Formato,
) -> R<Value> {
    let corte = con_sesion(app, Some(inicio), |_, sesion, ambiente, _| {
        let corte = estado.obtener(sesion, ambiente, id)?;
        if corte.reporte != reporte || &corte.filtros != filtros {
            return Err(rep::error(
                "Pulsa Consultar para aplicar los filtros antes de exportar.",
            ));
        }
        Ok(corte)
    })?;
    // El selector y la generación del archivo no mantienen bloqueos de sesión ni SQLite.
    #[cfg(windows)]
    let ventana = app
        .get_webview_window("main")
        .and_then(|w| w.hwnd().ok())
        .map_or(0, |h| h.0 as isize);
    #[cfg(not(windows))]
    let ventana = 0;
    let destino = match elegir_guardado(&formato.nombre(&corte), formato.extension(), ventana) {
        Ok(Some(ruta)) => ruta,
        Ok(None) => return Ok(json!({"guardado": false})),
        Err(e) => {
            con_sesion(app, Some(inicio), |db, sesion, ambiente, actor| {
                estado.obtener(sesion, ambiente, id)?;
                rep::registrar_exportacion(db, actor, &corte, formato.extension(), false)
            })?;
            return Err(e);
        }
    };
    let destino = Path::new(&destino);
    let preparado = exportar::preparar(&corte, formato, destino);
    con_sesion(app, Some(inicio), |db, sesion, ambiente, actor| {
        estado.obtener(sesion, ambiente, id)?;
        let resultado = preparado.and_then(|f| exportar::publicar(f, destino));
        let auditado =
            rep::registrar_exportacion(db, actor, &corte, formato.extension(), resultado.is_ok());
        resultado?;
        auditado.map_err(|_| {
            rep::error(
                "Se guardó el archivo, pero no se pudo registrar su exportación en auditoría.",
            )
        })?;
        Ok(json!({"guardado": true}))
    })
}

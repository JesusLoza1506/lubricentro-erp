use crate::dto::dashboard_dto::{
    ActividadTurno, DashboardDto, DatosDashboard, EstadosTrabajo, PendientesDashboard,
    PuntoDashboard, TurnoDashboard,
};
use crate::errors::AppError;
use crate::services::{reportes_service::ahora_lima, session_manager::SessionState};
use chrono::{Days, NaiveDateTime, Timelike};
use rusqlite::{params, Connection, OptionalExtension, Params};
use std::collections::BTreeMap;

type R<T> = Result<T, AppError>;

pub struct DashboardService;

impl DashboardService {
    pub fn obtener_para_sesion(
        conn: &Connection,
        sesion: &mut SessionState,
        session_id: &str,
    ) -> R<DashboardDto> {
        if sesion.session_id.as_deref() != Some(session_id) {
            return Err(AppError::NoAutenticado);
        }
        let usuario = sesion.validar_sesion_operativa(conn)?;
        Self::obtener_dashboard(conn, usuario)
    }

    pub fn obtener_dashboard(conn: &Connection, usuario: i64) -> R<DashboardDto> {
        Self::obtener_en(conn, usuario, ahora_lima())
    }

    fn obtener_en(conn: &Connection, usuario: i64, ahora: NaiveDateTime) -> R<DashboardDto> {
        let tx = conn.unchecked_transaction()?;
        let (ambiente, recuperacion): (String, bool) = tx.query_row(
            "SELECT ambiente_actual, recuperacion_pendiente FROM configuracion WHERE id_config=1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        if recuperacion {
            return Err(AppError::AccessDenied(
                "Completa primero la recuperación en Backup.".into(),
            ));
        }
        let rol: String = tx
            .query_row(
                "SELECT rol FROM usuarios WHERE id_usuario=?1 AND activo=1",
                [usuario],
                |r| r.get(0),
            )
            .optional()?
            .ok_or(AppError::NoAutenticado)?;
        let fecha = ahora.date();
        let corte = ahora.format("%Y-%m-%d %H:%M:%S").to_string();
        let dias: Vec<String> = (0..7)
            .rev()
            .map(|n| {
                fecha
                    .checked_sub_days(Days::new(n))
                    .map(|d| d.to_string())
                    .ok_or_else(|| AppError::Validation("Fecha de Inicio fuera de rango.".into()))
            })
            .collect::<R<_>>()?;
        let desde = format!("{} 00:00:00", dias[0]);
        let hoy = format!("{fecha} 00:00:00");
        let datos = match rol.as_str() {
            "ADMINISTRADOR" => {
                administrador(&tx, &ambiente, &dias, &desde, &hoy, &corte, ahora.hour())?
            }
            "MECANICO" => mecanico(&tx, usuario, &dias, &desde, &corte)?,
            "CAJERO" => cajero(&tx, usuario, &ambiente, &corte)?,
            _ => return Err(AppError::PermisoDenegado),
        };
        tx.commit()?;
        Ok(DashboardDto {
            ambiente,
            fecha: fecha.to_string(),
            actualizado_en: corte,
            datos,
        })
    }
}

fn serie(conn: &Connection, sql: &str, args: impl Params) -> R<BTreeMap<String, i64>> {
    let mut s = conn.prepare(sql)?;
    let filas = s.query_map(args, |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(filas.collect::<Result<_, _>>()?)
}

fn completar(
    periodos: impl IntoIterator<Item = String>,
    mut datos: BTreeMap<String, i64>,
) -> Vec<PuntoDashboard> {
    periodos
        .into_iter()
        .map(|periodo| PuntoDashboard {
            valor: datos.remove(&periodo).unwrap_or(0),
            periodo,
        })
        .collect()
}

fn mediana(conn: &Connection, sql: &str, args: impl Params) -> R<Option<i64>> {
    let mut s = conn.prepare(sql)?;
    let filas = s.query_map(args, |r| r.get::<_, i64>(0))?;
    let mut segundos = filas.collect::<Result<Vec<_>, _>>()?;
    segundos.sort_unstable();
    let mitad = segundos.len() / 2;
    Ok(if segundos.is_empty() {
        None
    } else if segundos.len() % 2 == 0 {
        let (a, b) = (segundos[mitad - 1], segundos[mitad]);
        Some(a + (b - a) / 2)
    } else {
        Some(segundos[mitad])
    })
}

fn administrador(
    conn: &Connection,
    ambiente: &str,
    dias: &[String],
    desde: &str,
    hoy: &str,
    corte: &str,
    hora: u32,
) -> R<DatosDashboard> {
    // Misma vista económica que R01, sin usar comprobantes o filas de pago como ventas.
    let ventas = serie(
        conn,
        "SELECT substr(fecha_operacion,1,10) AS dia, sum(total_cent)
         FROM v_operaciones_comerciales WHERE ambiente=?1 AND fecha_operacion BETWEEN ?2 AND ?3
         GROUP BY dia",
        params![ambiente, desde, corte],
    )?;
    let terminadas = serie(
        conn,
        "SELECT substr(fecha_fin,1,13) AS hora, count(*)
         FROM ordenes_trabajo WHERE estado='FINALIZADO' AND fecha_fin BETWEEN ?1 AND ?2
         GROUP BY hora",
        params![hoy, corte],
    )?;
    let trabajos_terminados_hoy = terminadas.values().sum();
    let mediana_cobro_seg = mediana(
        conn,
        "SELECT unixepoch(v.fecha_venta)-unixepoch(o.fecha_fin)
         FROM ventas v JOIN ordenes_trabajo o ON o.id_ot=v.id_ot
         WHERE v.estado_registro='CONFIRMADA' AND v.ambiente=?1 AND o.estado='FINALIZADO'
         AND v.fecha_venta BETWEEN ?2 AND ?3 AND unixepoch(v.fecha_venta)>=unixepoch(o.fecha_fin)",
        params![ambiente, desde, corte],
    )?;
    Ok(DatosDashboard::Administrador {
        ventas_netas_hoy_cent: ventas.get(&hoy[..10]).copied().unwrap_or(0),
        trabajos_terminados_hoy,
        mediana_cobro_seg,
        ventas_por_dia: completar(dias.iter().cloned(), ventas),
        trabajos_por_hora: completar(
            (0..=hora).map(|h| format!("{} {h:02}", &hoy[..10])),
            terminadas,
        ),
    })
}

fn mecanico(
    conn: &Connection,
    usuario: i64,
    dias: &[String],
    desde: &str,
    corte: &str,
) -> R<DatosDashboard> {
    let (en_espera, en_proceso): (i64, i64) = conn.query_row(
        "SELECT count(CASE WHEN estado='EN_ESPERA' THEN 1 END),
                count(CASE WHEN estado='EN_PROCESO' THEN 1 END)
         FROM ordenes_trabajo WHERE id_mecanico=?1 AND fecha_ingreso<=?2
         AND estado IN ('EN_ESPERA','EN_PROCESO')",
        params![usuario, corte],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    // Taller conserva el responsable de la finalización en id_mecanico_terminal.
    let trabajos = serie(
        conn,
        "SELECT substr(fecha_fin,1,10) AS dia, count(*) FROM ordenes_trabajo
         WHERE estado='FINALIZADO' AND coalesce(id_mecanico_terminal,id_mecanico)=?1
         AND fecha_fin BETWEEN ?2 AND ?3 GROUP BY dia",
        params![usuario, desde, corte],
    )?;
    let trabajos_terminados_hoy = trabajos.get(&corte[..10]).copied().unwrap_or(0);
    let mediana_trabajo_seg = mediana(
        conn,
        "SELECT unixepoch(fecha_fin)-unixepoch(fecha_inicio) FROM ordenes_trabajo
         WHERE estado='FINALIZADO' AND coalesce(id_mecanico_terminal,id_mecanico)=?1
         AND fecha_fin BETWEEN ?2 AND ?3 AND unixepoch(fecha_fin)>=unixepoch(fecha_inicio)",
        params![usuario, desde, corte],
    )?;
    Ok(DatosDashboard::Mecanico {
        trabajos_terminados_hoy,
        trabajos_abiertos: en_espera + en_proceso,
        mediana_trabajo_seg,
        por_estado: EstadosTrabajo {
            en_espera,
            en_proceso,
            finalizadas_hoy: trabajos_terminados_hoy,
        },
        trabajos_por_dia: completar(dias.iter().cloned(), trabajos),
    })
}

fn cajero(conn: &Connection, usuario: i64, ambiente: &str, corte: &str) -> R<DatosDashboard> {
    let caja: Option<(i64, i64, String)> = conn
        .query_row(
            "SELECT id_caja,id_usuario,fecha_apertura FROM caja_chica
             WHERE estado='ABIERTA' AND ambiente=?1 AND fecha_apertura<=?2",
            params![ambiente, corte],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    let (turno, actividad_turno) = match caja {
        Some((id, titular, apertura)) if titular == usuario => {
            // Una venta puede tener varios medios, pero sigue siendo un solo cobro.
            let cobros = serie(
                conn,
                "SELECT substr(fecha,1,13) AS hora, count(*) FROM (
                   SELECT min(m.fecha_operacion) AS fecha
                   FROM movimientos_caja m JOIN ventas v ON v.id_venta=m.id_venta
                   WHERE m.id_caja=?1 AND m.id_usuario=?2 AND m.tipo='COBRO'
                   AND v.id_caja=?1 AND v.id_emisor=?2 AND v.estado_registro='CONFIRMADA'
                   AND v.ambiente=?3 AND m.fecha_operacion BETWEEN ?4 AND ?5
                   GROUP BY m.id_venta
                 ) GROUP BY hora",
                params![id, usuario, ambiente, apertura, corte],
            )?;
            let actividad = ActividadTurno {
                cobros: cobros.values().sum(),
                cobros_por_hora: cobros
                    .into_iter()
                    .map(|(periodo, valor)| PuntoDashboard { periodo, valor })
                    .collect(),
            };
            (TurnoDashboard::Propio { apertura }, Some(actividad))
        }
        Some(_) => (TurnoDashboard::Ajeno, None),
        None => (TurnoDashboard::SinCaja, None),
    };
    // La elegibilidad coincide con el selector de OT finalizadas del POS.
    let pendientes = conn.query_row(
        "WITH cola AS (
            SELECT unixepoch(?1)-unixepoch(o.fecha_fin) AS espera
            FROM ordenes_trabajo o WHERE o.estado='FINALIZADO'
            AND (o.fecha_fin IS NULL OR o.fecha_fin<=?1)
            AND NOT EXISTS(SELECT 1 FROM ventas v WHERE v.id_ot=o.id_ot)
         )
         SELECT count(*),max(espera),count(CASE WHEN espera>=0 AND espera<1800 THEN 1 END),
                count(CASE WHEN espera BETWEEN 1800 AND 3600 THEN 1 END),
                count(CASE WHEN espera>3600 THEN 1 END) FROM cola",
        [corte],
        |r| {
            Ok(PendientesDashboard {
                total: r.get(0)?,
                mayor_espera_seg: r.get(1)?,
                menos_30: r.get(2)?,
                entre_30_y_60: r.get(3)?,
                mas_60: r.get(4)?,
            })
        },
    )?;
    Ok(DatosDashboard::Cajero {
        turno,
        actividad_turno,
        pendientes,
    })
}

#[cfg(test)]
mod tests;

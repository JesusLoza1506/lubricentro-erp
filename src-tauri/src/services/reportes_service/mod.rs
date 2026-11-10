mod auditoria;
pub use auditoria::registrar as registrar_exportacion;
mod comercial;
pub mod exportar;
mod operativos;

use crate::errors::AppError;
use chrono::{Datelike, Days, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use rusqlite::{Connection, Row, ToSql};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

type R<T> = Result<T, AppError>;
pub const POR_PAGINA: usize = 20;
pub fn error(msg: impl Into<String>) -> AppError {
    AppError::Validation(msg.into())
}

pub fn autorizar(
    db: &Connection,
    sesion: &mut super::session_manager::SessionState,
    auditar_denegacion: bool,
) -> R<i64> {
    let resultado = super::admin_service::autorizar(db, sesion);
    if auditar_denegacion && matches!(resultado, Err(AppError::PermisoDenegado)) {
        if let Some(id) = sesion.usuario_id {
            let _ = db.execute(
                "INSERT INTO auditoria(id_usuario,actor,username_snapshot,rol_snapshot,modulo,accion,entidad,resultado)
                 SELECT id_usuario,'USUARIO',username,rol,'REPORTES','ACCESO_DENEGADO','reportes','DENEGADO' FROM usuarios WHERE id_usuario=?1",
                [id],
            );
        }
    }
    resultado
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Reporte {
    R01,
    R02,
    R03,
    R04,
    R05,
    R06,
    R07,
    R08,
}
impl Reporte {
    pub fn nombre(self) -> &'static str {
        match self {
            Self::R01 => "Ventas por periodo",
            Self::R02 => "Cobros y devoluciones",
            Self::R03 => "Productos y margen",
            Self::R04 => "Stock actual y crítico",
            Self::R05 => "Cierres de caja",
            Self::R06 => "Trabajo del taller",
            Self::R07 => "Comprobantes",
            Self::R08 => "Auditoría",
        }
    }
    pub fn codigo(self) -> &'static str {
        match self {
            Self::R01 => "R01",
            Self::R02 => "R02",
            Self::R03 => "R03",
            Self::R04 => "R04",
            Self::R05 => "R05",
            Self::R06 => "R06",
            Self::R07 => "R07",
            Self::R08 => "R08",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Periodo {
    Hoy,
    Ayer,
    EstaSemana,
    #[default]
    EsteMes,
    MesAnterior,
    Personalizado,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Filtros {
    pub periodo: Periodo,
    pub desde: Option<String>,
    pub hasta: Option<String>,
    pub medio: Option<String>,
    pub tipo: Option<String>,
    pub ver_todos: bool,
    pub buscar: String,
    pub categoria: Option<String>,
    pub estado: Option<String>,
    pub solo_criticos: bool,
    pub usuario: Option<i64>,
    pub con_diferencia: bool,
    pub actividad: Option<String>,
    pub actor: Option<String>,
    pub modulo: Option<String>,
}

pub fn ahora_lima() -> NaiveDateTime {
    Utc::now()
        .with_timezone(&FixedOffset::west_opt(5 * 3600).expect("UTC-05"))
        .naive_local()
}

fn rango(f: &Filtros, hoy: NaiveDate) -> R<(NaiveDate, NaiveDate)> {
    let mes = hoy.with_day(1).ok_or_else(|| error("Fecha inválida."))?;
    let dia_anterior = |d: NaiveDate| d.pred_opt().ok_or_else(|| error("Fecha inválida."));
    let leer = |s: &Option<String>| {
        s.as_deref()
            .filter(|d| d.len() == 10)
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .ok_or_else(|| error("Indica fechas válidas Desde y Hasta."))
    };
    let (desde, hasta) = match f.periodo {
        Periodo::Hoy => (hoy, hoy),
        Periodo::Ayer => {
            let ayer = dia_anterior(hoy)?;
            (ayer, ayer)
        }
        Periodo::EstaSemana => (
            hoy.checked_sub_days(Days::new(hoy.weekday().num_days_from_monday().into()))
                .ok_or_else(|| error("Fecha inválida."))?,
            hoy,
        ),
        Periodo::EsteMes => (mes, hoy),
        Periodo::MesAnterior => {
            let fin = dia_anterior(mes)?;
            (
                fin.with_day(1).ok_or_else(|| error("Fecha inválida."))?,
                fin,
            )
        }
        Periodo::Personalizado => (leer(&f.desde)?, leer(&f.hasta)?),
    };
    if desde > hasta || hasta > hoy || (hasta - desde).num_days() >= 366 {
        return Err(error(
            "El rango debe estar ordenado, no incluir fechas futuras y tener hasta 366 días.",
        ));
    }
    Ok((desde, hasta))
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "tipo", content = "valor", rename_all = "snake_case")]
pub enum Celda {
    Texto(String),
    Numero(String),
    NoAplica,
}
impl Celda {
    pub fn texto(&self) -> &str {
        match self {
            Self::Texto(s) | Self::Numero(s) => s,
            Self::NoAplica => "No aplica",
        }
    }
}
fn t(s: impl Into<String>) -> Celda {
    Celda::Texto(s.into())
}
fn n(v: impl ToString) -> Celda {
    Celda::Numero(v.to_string())
}
fn decimal(v: i128, d: u32) -> Celda {
    let u = v.unsigned_abs();
    let f = 10_u128.pow(d);
    n(format!(
        "{}{}.{:0width$}",
        if v < 0 { "-" } else { "" },
        u / f,
        u % f,
        width = d as usize
    ))
}
fn soles(v: impl Into<i128>) -> Celda {
    decimal(v.into(), 2)
}
fn valor(v: i128) -> Celda {
    let redondeado = (v.unsigned_abs() + 5_000) / 10_000;
    soles(if v < 0 {
        -(redondeado as i128)
    } else {
        redondeado as i128
    })
}
fn dato(s: Option<String>) -> Celda {
    t(s.filter(|v| !v.is_empty())
        .unwrap_or_else(|| "No registrado".into()))
}
fn txt(r: &Row<'_>, i: usize) -> R<Celda> {
    Ok(dato(r.get(i)?))
}
fn moneda(r: &Row<'_>, i: usize) -> R<Celda> {
    Ok(r.get::<_, Option<i64>>(i)?
        .map_or_else(|| t("No registrado"), soles))
}
fn consulta<T>(
    db: &Connection,
    sql: &str,
    args: &[&dyn ToSql],
    mut f: impl FnMut(&Row<'_>) -> R<T>,
) -> R<Vec<T>> {
    let mut stmt = db.prepare(sql)?;
    let mut filas = stmt.query(args)?;
    let mut resultado = Vec::new();
    while let Some(r) = filas.next()? {
        resultado.push(f(r)?);
    }
    Ok(resultado)
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Destino {
    Pos,
    Caja,
    Inventario,
    Taller,
    Administracion,
    Backup,
}
#[derive(Clone, Debug, Serialize)]
pub struct Origen {
    pub destino: Destino,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producto: Option<ProductoOrigen>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ProductoOrigen {
    pub id_producto: i64,
    pub codigo: String,
    pub nombre: String,
    pub unidad_medida: String,
}
impl Origen {
    fn nuevo(destino: Destino, id: i64) -> Self {
        Self {
            destino,
            id,
            producto: None,
        }
    }
    pub fn vigente(&self, db: &Connection, ambiente: &str) -> R<()> {
        let sql = match self.destino {
            Destino::Pos => "SELECT EXISTS(SELECT 1 FROM ventas WHERE id_venta=?1 AND ambiente=?2 AND estado_registro='CONFIRMADA')",
            Destino::Caja => "SELECT EXISTS(SELECT 1 FROM caja_chica WHERE id_caja=?1 AND ambiente=?2)",
            Destino::Inventario => "SELECT EXISTS(SELECT 1 FROM productos WHERE id_producto=?1 AND ?2 IS NOT NULL)",
            Destino::Taller => "SELECT EXISTS(SELECT 1 FROM ordenes_trabajo WHERE id_ot=?1 AND ?2 IS NOT NULL)",
            Destino::Administracion => "SELECT EXISTS(SELECT 1 FROM usuarios WHERE id_usuario=?1 AND ?2 IS NOT NULL)",
            Destino::Backup => "SELECT EXISTS(SELECT 1 FROM restauraciones WHERE id_restauracion=?1 AND ambiente=?2)",
        };
        if !db.query_row(sql, (&self.id, ambiente), |r| r.get::<_, bool>(0))? {
            return Err(error("El registro de origen ya no está disponible."));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Fila {
    pub celdas: Vec<Celda>,
    pub origenes: Vec<Origen>,
}
impl Fila {
    fn nueva(celdas: Vec<Celda>, origenes: impl IntoIterator<Item = Origen>) -> Self {
        Self {
            celdas,
            origenes: origenes.into_iter().collect(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Default)]
pub struct Detalle {
    pub columnas: Vec<&'static str>,
    pub filas: Vec<Fila>,
}
impl Detalle {
    fn pares(pares: Vec<(&str, Celda)>) -> Self {
        Self {
            columnas: vec!["Campo", "Valor"],
            filas: pares
                .into_iter()
                .map(|(k, v)| Fila::nueva(vec![t(k), v], []))
                .collect(),
        }
    }
}

#[derive(Debug, Default)]
struct Datos {
    columnas: Vec<&'static str>,
    filas: Vec<Fila>,
    detalles: Vec<Detalle>,
    totales: Vec<(&'static str, Celda)>,
    disponibles: Option<usize>,
}
impl Datos {
    fn nuevo(columnas: Vec<&'static str>) -> Self {
        Self {
            columnas,
            ..Default::default()
        }
    }
    fn agregar(&mut self, fila: Fila, detalle: Detalle) {
        self.filas.push(fila);
        self.detalles.push(detalle);
    }
}

#[derive(Debug, Serialize)]
pub struct UsuarioFiltro {
    pub id: i64,
    pub nombre: String,
    pub rol: String,
    pub responsable_ot: bool,
}
#[derive(Debug)]
pub struct Corte {
    pub id: String,
    pub reporte: Reporte,
    pub ambiente: String,
    pub hora: String,
    pub criterio: &'static str,
    pub filtros: Filtros,
    pub filtros_texto: String,
    pub rango: Option<(String, String)>,
    pub usuarios: Vec<UsuarioFiltro>,
    datos: Datos,
}
#[derive(Serialize)]
pub struct Pagina<'a> {
    id: &'a str,
    reporte: Reporte,
    ambiente: &'a str,
    hora: &'a str,
    criterio: &'a str,
    filtros: &'a Filtros,
    filtros_texto: &'a str,
    rango: &'a Option<(String, String)>,
    usuarios: &'a [UsuarioFiltro],
    columnas: &'a [&'static str],
    filas: &'a [Fila],
    pagina: usize,
    total_filas: usize,
    disponibles: Option<usize>,
    totales: &'a [(&'static str, Celda)],
}
impl Corte {
    pub fn pagina(&self, pagina: usize) -> R<Pagina<'_>> {
        let total = self.datos.filas.len();
        let paginas = total.div_ceil(POR_PAGINA).max(1);
        if pagina == 0 || pagina > paginas {
            return Err(error("Página de reporte no válida."));
        }
        let inicio = (pagina - 1) * POR_PAGINA;
        Ok(Pagina {
            id: &self.id,
            reporte: self.reporte,
            ambiente: &self.ambiente,
            hora: &self.hora,
            criterio: self.criterio,
            filtros: &self.filtros,
            filtros_texto: &self.filtros_texto,
            rango: &self.rango,
            usuarios: &self.usuarios,
            columnas: &self.datos.columnas,
            filas: &self.datos.filas[inicio..(inicio + POR_PAGINA).min(total)],
            pagina,
            total_filas: total,
            disponibles: self.datos.disponibles,
            totales: &self.datos.totales,
        })
    }
    pub fn detalle(&self, fila: usize) -> R<&Detalle> {
        self.datos
            .detalles
            .get(fila)
            .ok_or_else(|| error("Fila de reporte no válida."))
    }
    pub fn origen(&self, fila: usize, detalle: Option<usize>, indice: usize) -> R<&Origen> {
        let f = match detalle {
            Some(d) => self.detalle(fila)?.filas.get(d),
            None => self.datos.filas.get(fila),
        };
        f.and_then(|f| f.origenes.get(indice))
            .ok_or_else(|| error("Origen de reporte no válido."))
    }
}
#[derive(Default)]
pub struct EstadoReportes {
    corte: Mutex<Option<(String, Arc<Corte>)>>,
    consulta: AtomicU64,
}
impl EstadoReportes {
    pub fn iniciar_consulta(&self) -> u64 {
        self.consulta.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
    }
    pub fn publicar(&self, sesion: &str, corte: Corte, solicitud: u64) -> R<Arc<Corte>> {
        let mut estado = self.corte.lock().map_err(|_| AppError::DbLock)?;
        if self.consulta.load(Ordering::SeqCst) != solicitud {
            return Err(error(
                "La consulta fue sustituida por una petición más reciente.",
            ));
        }
        let c = Arc::new(corte);
        *estado = Some((sesion.into(), Arc::clone(&c)));
        Ok(c)
    }
    pub fn obtener(&self, sesion: &str, ambiente: &str, id: &str) -> R<Arc<Corte>> {
        let mut estado = self.corte.lock().map_err(|_| AppError::DbLock)?;
        if estado
            .as_ref()
            .is_some_and(|(s, c)| s != sesion || c.ambiente != ambiente)
        {
            *estado = None;
        }
        estado
            .as_ref()
            .filter(|(_, c)| c.id == id)
            .map(|(_, c)| Arc::clone(c))
            .ok_or_else(|| error("El resultado ya no está vigente. Vuelve a consultar."))
    }
    pub fn descartar(&self, sesion: &str, ambiente: &str, id: &str) -> R<()> {
        let mut estado = self.corte.lock().map_err(|_| AppError::DbLock)?;
        if estado
            .as_ref()
            .is_some_and(|(s, c)| s == sesion && c.ambiente == ambiente && c.id == id)
        {
            *estado = None;
        }
        Ok(())
    }
    pub fn limpiar(&self) -> R<()> {
        *self.corte.lock().map_err(|_| AppError::DbLock)? = None;
        Ok(())
    }
}

fn catalogo(valor: Option<&str>, opciones: &[&str]) -> R<()> {
    if valor.is_some_and(|v| !opciones.contains(&v)) {
        Err(error("El filtro no pertenece al catálogo del reporte."))
    } else {
        Ok(())
    }
}

pub fn consultar(
    db: &Connection,
    reporte: Reporte,
    f: Filtros,
    ambiente: &str,
    ahora: NaiveDateTime,
) -> R<Corte> {
    use Reporte::*;
    if !matches!(ambiente, "PRUEBA" | "PRODUCCION") {
        return Err(error("Ambiente inválido."));
    }
    if f.buscar.chars().count() > 100 || f.buscar.chars().any(char::is_control) {
        return Err(error(
            "La búsqueda debe tener hasta 100 caracteres sin controles.",
        ));
    }
    let usuarios = if matches!(reporte, R05 | R06 | R08) {
        let consulta_usuarios = if reporte == R06 {
            "WITH responsables(id) AS (
                SELECT id_mecanico FROM ordenes_trabajo WHERE id_mecanico IS NOT NULL
                UNION SELECT id_mecanico_terminal FROM ordenes_trabajo WHERE id_mecanico_terminal IS NOT NULL
             ) SELECT id_usuario,nombre_completo,rol,(rol='MECANICO' OR id_usuario IN (SELECT id FROM responsables))
               FROM usuarios ORDER BY nombre_completo,id_usuario"
        } else {
            "SELECT id_usuario,nombre_completo,rol,0 FROM usuarios ORDER BY nombre_completo,id_usuario"
        };
        consulta(db, consulta_usuarios, &[], |r| {
            Ok(UsuarioFiltro {
                id: r.get(0)?,
                nombre: r.get(1)?,
                rol: r.get(2)?,
                responsable_ot: r.get(3)?,
            })
        })?
    } else {
        Vec::new()
    };
    let mut etiquetas = Vec::new();
    let mut aplicar = Filtros {
        periodo: f.periodo,
        desde: f.desde.clone(),
        hasta: f.hasta.clone(),
        ..Default::default()
    };
    let (desde, hasta, fechas) = if reporte == R04 {
        (String::new(), String::new(), None)
    } else {
        let (a, b) = rango(&aplicar, ahora.date())?;
        let siguiente = b.succ_opt().ok_or_else(|| error("Fecha inválida."))?;
        (
            format!("{a} 00:00:00"),
            format!("{siguiente} 00:00:00"),
            Some((a.to_string(), b.to_string())),
        )
    };
    let corte = ahora.format("%Y-%m-%d %H:%M:%S").to_string();
    let (criterio, datos) = match reporte {
        R01 => (
            "Fecha económica de venta o devolución",
            comercial::ventas(db, ambiente, &desde, &hasta)?,
        ),
        R02 => {
            catalogo(f.medio.as_deref(), &comercial::MEDIOS)?;
            aplicar.medio = f.medio;
            etiquetas.push(format!(
                "Medio: {}",
                aplicar.medio.as_deref().map_or("Todos", comercial::medio)
            ));
            (
                "Fecha del cobro o reintegro",
                comercial::cobros(db, ambiente, &desde, &hasta, aplicar.medio.as_deref())?,
            )
        }
        R03 => {
            catalogo(f.tipo.as_deref(), &["PRODUCTOS", "SERVICIOS"])?;
            let servicio = f.tipo.as_deref() == Some("SERVICIOS");
            aplicar.tipo = f.tipo;
            aplicar.ver_todos = f.ver_todos;
            etiquetas.push(format!(
                "{} · {}",
                if servicio { "Servicios" } else { "Productos" },
                if f.ver_todos { "Ver todos" } else { "Top 10" }
            ));
            (
                "Fecha económica de venta o devolución",
                comercial::margen(db, ambiente, &desde, &hasta, servicio, f.ver_todos)?,
            )
        }
        R04 => {
            catalogo(
                f.categoria.as_deref(),
                &["ACEITE", "FILTRO", "ADITIVO", "GRANEL", "OTRO"],
            )?;
            catalogo(f.estado.as_deref(), &["ACTIVOS", "INACTIVOS", "AMBOS"])?;
            aplicar.buscar = f.buscar;
            aplicar.estado = f.estado;
            aplicar.categoria = f.categoria;
            aplicar.solo_criticos = f.solo_criticos;
            if !aplicar.buscar.trim().is_empty() {
                etiquetas.push(format!("Búsqueda: {}", aplicar.buscar.trim()));
            }
            etiquetas.push(format!(
                "Categoría: {}",
                aplicar.categoria.as_deref().unwrap_or("Todas")
            ));
            etiquetas.push(
                match aplicar.estado.as_deref() {
                    Some("ACTIVOS") => "Activos",
                    Some("INACTIVOS") => "Inactivos con stock",
                    _ => "Activos e inactivos con stock",
                }
                .into(),
            );
            if f.solo_criticos {
                etiquetas.push("Solo críticos".into());
            }
            (
                "Stock al momento de la consulta",
                operativos::stock(db, &aplicar)?,
            )
        }
        R05 | R06 => {
            let filtro = f
                .usuario
                .map(|id| {
                    usuarios
                        .iter()
                        .find(|u| u.id == id)
                        .ok_or_else(|| error("El usuario seleccionado no existe."))
                })
                .transpose()?;
            if reporte == R06 && filtro.is_some_and(|u| !u.responsable_ot) {
                return Err(error("El responsable debe ser mecánico."));
            }
            etiquetas.push(format!(
                "{}: {}",
                if reporte == R05 {
                    "Titular"
                } else {
                    "Responsable"
                },
                filtro.map_or("Todos", |u| u.nombre.as_str())
            ));
            aplicar.usuario = f.usuario;
            if reporte == R05 {
                aplicar.con_diferencia = f.con_diferencia;
                if f.con_diferencia {
                    etiquetas.push("Con diferencia de efectivo".into());
                }
                (
                    "Fecha de cierre",
                    operativos::cierres(db, ambiente, &desde, &hasta, &aplicar)?,
                )
            } else {
                catalogo(f.actividad.as_deref(), &["FINALIZADAS", "RECIBIDAS"])?;
                let recibidas = f.actividad.as_deref() == Some("RECIBIDAS");
                aplicar.actividad = f.actividad;
                etiquetas.push(
                    if recibidas {
                        "Recibidas"
                    } else {
                        "Finalizadas"
                    }
                    .into(),
                );
                (
                    if recibidas {
                        "Fecha de ingreso"
                    } else {
                        "Fecha de finalización"
                    },
                    operativos::taller(db, &desde, &hasta, &aplicar, recibidas)?,
                )
            }
        }
        R07 => {
            catalogo(
                f.estado.as_deref(),
                &["PENDIENTE", "ENVIANDO", "ERROR", "ACEPTADO", "RECHAZADO"],
            )?;
            aplicar.buscar = f.buscar;
            aplicar.estado = f.estado;
            etiquetas.push(format!(
                "Estado: {}",
                aplicar.estado.as_deref().unwrap_or("Todos")
            ));
            if !aplicar.buscar.trim().is_empty() {
                etiquetas.push(format!("Serie/número: {}", aplicar.buscar.trim()));
            }
            (
                "Fecha de emisión",
                operativos::documentos(db, ambiente, &desde, &hasta, &aplicar)?,
            )
        }
        R08 => {
            catalogo(f.modulo.as_deref(), &auditoria::MODULOS)?;
            aplicar.actor = f.actor;
            aplicar.modulo = f.modulo;
            let actor = aplicar
                .actor
                .as_deref()
                .map(|a| {
                    if a == "SISTEMA" || a == "NO_IDENTIFICADO" {
                        return Ok(a);
                    }
                    usuarios
                        .iter()
                        .find(|u| u.id.to_string() == a)
                        .map(|u| u.nombre.as_str())
                        .ok_or_else(|| error("El actor seleccionado no existe."))
                })
                .transpose()?;
            etiquetas.push(format!("Actor: {}", actor.unwrap_or("Todos")));
            etiquetas.push(format!(
                "Módulo: {}",
                aplicar.modulo.as_deref().unwrap_or("Todos")
            ));
            (
                "Fecha del evento",
                auditoria::eventos(db, ambiente, &desde, &hasta, &aplicar)?,
            )
        }
    };
    Ok(Corte {
        id: uuid::Uuid::new_v4().to_string(),
        reporte,
        ambiente: ambiente.into(),
        hora: corte,
        criterio,
        filtros: aplicar,
        filtros_texto: if etiquetas.is_empty() {
            "Sin filtros adicionales".into()
        } else {
            etiquetas.join(" · ")
        },
        rango: fechas,
        usuarios,
        datos,
    })
}

#[cfg(test)]
mod tests;

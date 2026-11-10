use super::*;
use rust_xlsxwriter::{Format as Estilo, Workbook, Worksheet};
use std::{fs::File, io::Write, path::Path};
use tempfile::NamedTempFile;

mod pdf;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Formato {
    Pdf,
    Xlsx,
}
impl Formato {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Xlsx => "xlsx",
        }
    }
    pub fn nombre(self, corte: &Corte) -> String {
        format!(
            "{}_{}_{}.{}",
            corte.reporte.codigo(),
            corte.ambiente,
            corte.hora.replace([' ', ':'], "-"),
            self.extension()
        )
    }
}

fn metadatos(c: &Corte) -> Vec<String> {
    let periodo = c.rango.as_ref().map_or_else(
        || "Corte actual".into(),
        |(a, b)| format!("Periodo: {a} a {b}"),
    );
    let mut meta = vec![
        format!("{} · {}", c.reporte.codigo(), c.reporte.nombre()),
        format!("Ambiente: {}  |  Corte: {} (Lima)", c.ambiente, c.hora),
        format!("{periodo}  |  {}", c.criterio),
        format!("Filtros: {}", c.filtros_texto),
        format!(
            "Filas: {}{}",
            c.datos.filas.len(),
            c.datos
                .disponibles
                .filter(|x| *x > c.datos.filas.len())
                .map_or_else(String::new, |x| format!(" de {x}"))
        ),
    ];
    if c.reporte == Reporte::R03 {
        meta.push("El margen de productos no es la utilidad total del negocio.".into());
    }
    meta
}

pub fn preparar(c: &Corte, formato: Formato, destino: &Path) -> R<NamedTempFile> {
    if !destino
        .extension()
        .and_then(|x| x.to_str())
        .is_some_and(|x| x.eq_ignore_ascii_case(formato.extension()))
    {
        return Err(error(format!(
            "El archivo debe tener extensión .{}.",
            formato.extension()
        )));
    }
    let carpeta = destino
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| error("Selecciona una carpeta de destino."))?;
    let mut temporal = NamedTempFile::new_in(carpeta)
        .map_err(|_| error("No se pudo crear el archivo temporal en el destino."))?;
    match formato {
        Formato::Pdf => pdf::guardar(c, temporal.as_file_mut())?,
        Formato::Xlsx => excel(c, temporal.as_file_mut())?,
    }
    temporal
        .as_file()
        .sync_all()
        .map_err(|_| error("No se pudo terminar de escribir el reporte."))?;
    Ok(temporal)
}

pub fn publicar(temporal: NamedTempFile, destino: &Path) -> R<()> {
    temporal.persist(destino).map_err(|_| {
        error("No se pudo guardar el reporte. Comprueba el acceso y si el archivo está abierto.")
    })?;
    Ok(())
}

fn excel(c: &Corte, archivo: &mut File) -> R<()> {
    fn celda(
        w: &mut Worksheet,
        r: u32,
        col: u16,
        v: &Celda,
        decimales: &[Estilo],
    ) -> Result<(), rust_xlsxwriter::XlsxError> {
        if let Celda::Numero(s) = v {
            let precision = s.split_once('.').map_or(0, |(_, d)| d.len());
            let cifras = s.trim_start_matches('-').replace('.', "");
            if cifras.trim_start_matches('0').len() <= 15 {
                if let (Ok(x), Some(f)) = (s.parse::<f64>(), decimales.get(precision)) {
                    w.write_number_with_format(r, col, x, f)?;
                    return Ok(());
                }
            }
        }
        w.write_string(r, col, v.texto())?;
        Ok(())
    }

    fn escribir(c: &Corte, archivo: &mut File) -> Result<(), rust_xlsxwriter::XlsxError> {
        let mut book = Workbook::new();
        let titulo = Estilo::new()
            .set_bold()
            .set_font_size(14)
            .set_font_color("#183D68");
        let cab = Estilo::new()
            .set_bold()
            .set_background_color("#193756")
            .set_font_color("#FFFFFF");
        let decimales = (0..=6)
            .map(|i| {
                Estilo::new().set_num_format(if i == 0 {
                    "0".into()
                } else {
                    format!("0.{}", "0".repeat(i))
                })
            })
            .collect::<Vec<_>>();
        let w = book.add_worksheet();
        w.set_name("Resultado")?;
        let mut r = 0;
        for m in metadatos(c) {
            if r == 0 {
                w.write_string_with_format(r, 0, &m, &titulo)?;
            } else {
                w.write_string(r, 0, &m)?;
            }
            r += 1;
        }
        for (k, v) in &c.datos.totales {
            w.write_string(r, 0, *k)?;
            celda(w, r, 1, v, &decimales)?;
            r += 1;
        }
        r += 1;
        let cabecera = r;
        for (i, k) in c.datos.columnas.iter().enumerate() {
            w.set_column_width(i as u16, (k.chars().count() + 6).clamp(17, 32) as f64)?;
            w.write_string_with_format(r, i as u16, *k, &cab)?;
        }
        w.set_freeze_panes(cabecera + 1, 0)?;
        for f in &c.datos.filas {
            r += 1;
            for (i, v) in f.celdas.iter().enumerate() {
                celda(w, r, i as u16, v, &decimales)?;
            }
        }
        if c.datos.filas.is_empty() {
            w.write_string(r + 1, 0, "Sin resultados")?;
        }
        book.save_to_writer(archivo)
    }
    escribir(c, archivo).map_err(|_| error("No se pudo generar el archivo Excel."))
}

use super::*;
use krilla::{
    color::rgb,
    geom::{PathBuilder, Rect},
    page::PageSettings,
    paint::Fill,
    surface::Surface,
    Document,
};

use std::{collections::HashMap, rc::Rc};

mod texto;
use texto::{Linea, Tipografia};

const ANCHO: f32 = 841.89;
const ALTO: f32 = 595.28;
const MARGEN: f32 = 30.0;
const LETRA: f32 = 8.5;
const LINEA: f32 = 11.0;
const CACHE_COLUMNA: usize = 256;
type Lineas = Rc<[Linea]>;

fn color(surface: &mut Surface<'_>, r: u8, g: u8, b: u8) {
    surface.set_fill(Some(Fill {
        paint: rgb::Color::new(r, g, b).into(),
        ..Default::default()
    }));
}

fn rect(surface: &mut Surface<'_>, y: f32, h: f32) {
    let mut path = PathBuilder::new();
    path.push_rect(Rect::from_xywh(MARGEN, y, ANCHO - 2.0 * MARGEN, h).expect("rectángulo válido"));
    surface.draw_path(&path.finish().expect("rectángulo válido"));
}
fn pesos(c: &Corte) -> Vec<f32> {
    c.datos
        .columnas
        .iter()
        .map(|k| match *k {
            "Producto" | "Ítem" | "Cliente" | "Receptor" | "Motivo" | "Referencia"
            | "Responsable" | "Actor" | "Titular" => 1.60,
            "Fecha y hora" | "Apertura" | "Cierre" | "Emisión" | "Finalización" | "Ingreso"
            | "Acción" | "Serie / número" => 1.35,
            "Stock" | "Unidad" | "Crítico" | "Turno" | "OT" => 0.74,
            _ => 1.0,
        })
        .collect()
}
struct Tabla {
    tipo: Tipografia,
    anchos: Vec<f32>,
    cache: Vec<HashMap<String, Lineas>>,
    cabecera: Vec<Vec<Linea>>,
    alto_cabecera: usize,
    metadatos: Vec<(f32, f32, Linea)>,
    y: f32,
}

impl Tabla {
    fn nueva(c: &Corte) -> R<Self> {
        let tipo = Tipografia::nueva()?;
        let pesos = pesos(c);
        let total: f32 = pesos.iter().sum();
        let anchos: Vec<_> = pesos
            .into_iter()
            .map(|v| v * (ANCHO - 2.0 * MARGEN) / total)
            .collect();
        let cabecera = c
            .datos
            .columnas
            .iter()
            .zip(&anchos)
            .map(|(s, w)| tipo.envolver(s, *w - 8.0, LETRA, true))
            .collect::<R<Vec<_>>>()?;
        let alto_cabecera = cabecera.iter().map(Vec::len).max().unwrap_or(1);
        let mut metadatos_pdf = Vec::new();
        let mut datos = metadatos(c);
        if !c.datos.totales.is_empty() {
            datos.push(
                c.datos
                    .totales
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", v.texto()))
                    .collect::<Vec<_>>()
                    .join("   |   "),
            );
        }
        let total_en_negrita = if c.datos.totales.is_empty() {
            usize::MAX
        } else {
            datos.len() - 1
        };
        for (i, m) in datos.iter().enumerate() {
            let (letra, salto) = if i == 0 { (16.0, 20.0) } else { (9.0, 12.0) };
            for linea in tipo.envolver(
                m,
                ANCHO - 2.0 * MARGEN,
                letra,
                i == 0 || i == total_en_negrita,
            )? {
                metadatos_pdf.push((letra, salto, linea));
            }
        }
        let cache = (0..anchos.len()).map(|_| HashMap::new()).collect();
        Ok(Self {
            tipo,
            anchos,
            cache,
            cabecera,
            alto_cabecera,
            metadatos: metadatos_pdf,
            y: 0.0,
        })
    }

    fn iniciar(&mut self, surface: &mut Surface<'_>) -> R<()> {
        self.y = MARGEN;
        color(surface, 28, 43, 61);
        for (letra, salto, linea) in &self.metadatos {
            self.tipo.texto(surface, MARGEN, self.y, *letra, linea)?;
            self.y += salto;
        }
        self.y += 5.0;
        color(surface, 227, 235, 245);
        rect(
            surface,
            self.y - LETRA - 1.0,
            self.alto_cabecera as f32 * LINEA + 6.0,
        );
        let cabecera = std::mem::take(&mut self.cabecera);
        self.tramo(surface, &cabecera, 0, self.alto_cabecera)?;
        self.cabecera = cabecera;
        if self.espacio() == 0 {
            return Err(error(
                "El encabezado es demasiado largo para una página del PDF.",
            ));
        }
        Ok(())
    }

    fn espacio(&self) -> usize {
        ((ALTO - MARGEN - 6.0 - self.y) / LINEA).floor().max(0.0) as usize
    }

    fn tramo(
        &mut self,
        surface: &mut Surface<'_>,
        celdas: &[impl AsRef<[Linea]>],
        desde: usize,
        lineas: usize,
    ) -> R<()> {
        color(surface, 28, 43, 61);
        let mut x = MARGEN;
        for (w, c) in self.anchos.iter().zip(celdas) {
            for (i, s) in c.as_ref().iter().skip(desde).take(lineas).enumerate() {
                self.tipo
                    .texto(surface, x + 3.0, self.y + i as f32 * LINEA, LETRA, s)?;
            }
            x += w;
        }
        self.y += lineas as f32 * LINEA + 6.0;
        color(surface, 214, 222, 230);
        rect(surface, self.y - 6.0, 0.35);
        Ok(())
    }

    fn celda(&mut self, columna: usize, texto: &str) -> R<Lineas> {
        if let Some(v) = self.cache[columna].get(texto) {
            return Ok(Rc::clone(v));
        }
        let v: Lineas = self
            .tipo
            .envolver(texto, self.anchos[columna] - 8.0, LETRA, false)?
            .into();
        if self.cache[columna].len() < CACHE_COLUMNA {
            self.cache[columna].insert(texto.to_owned(), Rc::clone(&v));
        }
        Ok(v)
    }

    fn pie(&mut self, surface: &mut Surface<'_>, pagina: usize) -> R<()> {
        color(surface, 28, 43, 61);
        let s = format!("Lubricentro Gloria · Reporte interno | Página {pagina}");
        for linea in self.tipo.envolver(&s, ANCHO - 2.0 * MARGEN, 8.0, false)? {
            self.tipo.texto(surface, MARGEN, ALTO - 19.0, 8.0, &linea)?;
        }
        Ok(())
    }
}

pub(super) fn guardar(c: &Corte, archivo: &mut File) -> R<()> {
    let mut tabla = Tabla::nueva(c)?;
    let mut documento = Document::new();
    let ajustes = || PageSettings::from_wh(ANCHO, ALTO).expect("tamaño válido");
    let mut filas = c.datos.filas.iter();
    let mut pendiente: Option<(Vec<Lineas>, usize, usize)> = None;
    let mut pagina_n = 1;
    loop {
        let mut pagina = documento.start_page_with(ajustes());
        let mut surface = pagina.surface();
        tabla.iniciar(&mut surface)?;
        let mut libre = true;
        let completo = loop {
            if pendiente.is_none() {
                let Some(fila) = filas.next() else { break true };
                let celdas = fila
                    .celdas
                    .iter()
                    .enumerate()
                    .map(|(i, s)| tabla.celda(i, s.texto()))
                    .collect::<R<Vec<_>>>()?;
                let lineas = celdas.iter().map(|x| x.len()).max().unwrap_or(1);
                pendiente = Some((celdas, 0, lineas));
            }
            let (celdas, desde, total) = pendiente.as_mut().expect("fila disponible");
            let espacio = tabla.espacio();
            if espacio == 0 || (!libre && *desde == 0 && *total > espacio) {
                break false;
            }
            let n = espacio.min(*total - *desde);
            tabla.tramo(&mut surface, celdas, *desde, n)?;
            libre = false;
            *desde += n;
            if *desde == *total {
                pendiente = None;
            }
        };
        if c.datos.filas.is_empty() {
            for linea in tabla
                .tipo
                .envolver("Sin resultados", ANCHO - 2.0 * MARGEN, 10.0, false)?
            {
                tabla
                    .tipo
                    .texto(&mut surface, MARGEN, tabla.y + 5.0, 10.0, &linea)?;
            }
        }
        tabla.pie(&mut surface, pagina_n)?;
        surface.finish();
        pagina.finish();
        if completo {
            break;
        }
        pagina_n += 1;
    }
    let bytes = documento
        .finish()
        .map_err(|_| error("No se pudo generar el archivo PDF."))?;
    archivo
        .write_all(&bytes)
        .map_err(|_| error("No se pudo escribir el archivo PDF."))
}

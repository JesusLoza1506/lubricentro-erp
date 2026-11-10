use super::*;
use krilla::{
    geom::Point,
    text::{Font, GlyphId, KrillaGlyph},
};
use rustybuzz::{Direction, Face, UnicodeBuffer};
use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation;

macro_rules! fuente {
    ($n:literal) => {
        include_bytes!(concat!("../../../../../resources/reportes-fonts/", $n))
    };
}

const FUENTES: &[&[u8]] = &[
    fuente!("NotoSans-Regular.ttf"),
    fuente!("NotoSans-Bold.ttf"),
    fuente!("NotoSansArabic-Regular.ttf"),
    fuente!("NotoSansHebrew-Regular.ttf"),
    fuente!("NotoSansDevanagari-Regular.ttf"),
    fuente!("NotoSansThai-Regular.ttf"),
    fuente!("NotoSansSymbols2-Regular.ttf"),
    fuente!("NotoSansCJKsc-Regular.otf"),
    fuente!("NotoMusic-Regular.ttf"),
    fuente!("NotoSansMath-Regular.ttf"),
    fuente!("NotoEmoji-Variable.ttf"),
];
const EMOJI: usize = FUENTES.len() - 1;

pub struct Tipografia {
    caras: Vec<Face<'static>>,
    fuentes: Vec<Option<Font>>,
}
pub struct Linea {
    texto: String,
    partes: Vec<Parte>,
    ancho: f32,
}
struct Parte {
    fuente: usize,
    rango: std::ops::Range<usize>,
    glifos: Vec<KrillaGlyph>,
    avance: f32,
}

fn invisible(c: char) -> bool {
    matches!(c, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2060}' | '\u{2066}'..='\u{2069}'
        | '\u{fe00}'..='\u{fe0f}' | '\u{e0100}'..='\u{e01ef}' | '\u{e0020}'..='\u{e007f}')
}
impl Tipografia {
    pub fn nueva() -> R<Self> {
        let caras = FUENTES
            .iter()
            .map(|f| Face::from_slice(f, 0))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| error("No se pudieron cargar las fuentes del PDF."))?;
        Ok(Self {
            caras,
            fuentes: vec![None; FUENTES.len()],
        })
    }

    fn fuente(&self, grupo: &str, negrita: bool) -> R<usize> {
        let principal = usize::from(negrita);
        let soporta = |i: usize| {
            grupo
                .chars()
                .all(|c| invisible(c) || self.caras[i].glyph_index(c).is_some())
        };
        // Pide la variante de emoji si el texto la indica explícitamente.
        if grupo.contains('\u{fe0f}') && soporta(EMOJI) {
            return Ok(EMOJI);
        }
        std::iter::once(principal)
            .chain(2..FUENTES.len())
            .find(|&i| soporta(i))
            .ok_or_else(|| error("El PDF contiene caracteres sin una fuente disponible. Puedes conservar el texto completo exportando a Excel."))
    }

    fn formar(&self, texto: &str, letra: f32, negrita: bool) -> R<Linea> {
        let mut partes = Vec::new();
        let mut ancho = 0.0;
        let bidi = BidiInfo::new(texto, None);
        for parrafo in &bidi.paragraphs {
            let (niveles, rangos) = bidi.visual_runs(parrafo, parrafo.range.clone());
            for rango in rangos {
                let derecha = niveles[rango.start].is_rtl();
                let mut grupos: Vec<(usize, std::ops::Range<usize>)> = Vec::new();
                for (i, s) in texto[rango.clone()].grapheme_indices(true) {
                    let a = rango.start + i;
                    let b = a + s.len();
                    let fuente = if s.chars().all(invisible) {
                        grupos.last().map_or(usize::from(negrita), |g| g.0)
                    } else {
                        self.fuente(s, negrita)?
                    };
                    if let Some((_, tramo)) = grupos.last_mut().filter(|g| g.0 == fuente) {
                        tramo.end = b;
                    } else {
                        grupos.push((fuente, a..b));
                    }
                }
                if derecha {
                    grupos.reverse();
                }
                for (fuente, rango) in grupos {
                    let face = &self.caras[fuente];
                    let unidades = face.units_per_em() as f32;
                    let s = &texto[rango.clone()];
                    let mut buffer = UnicodeBuffer::new();
                    buffer.push_str(s);
                    buffer.set_direction(if derecha {
                        Direction::RightToLeft
                    } else {
                        Direction::LeftToRight
                    });
                    buffer.guess_segment_properties();
                    let forma = rustybuzz::shape(face, &[], buffer);
                    let mut limites: Vec<_> = forma
                        .glyph_infos()
                        .iter()
                        .map(|g| g.cluster as usize)
                        .collect();
                    limites.sort_unstable();
                    limites.dedup();
                    let mut avance = 0.0;
                    let mut glifos = Vec::with_capacity(forma.len());
                    for (g, p) in forma.glyph_infos().iter().zip(forma.glyph_positions()) {
                        let ini = g.cluster as usize;
                        let fin = limites
                            .get(limites.partition_point(|&x| x <= ini))
                            .copied()
                            .unwrap_or(s.len());
                        if g.glyph_id == 0 && !s[ini..fin].chars().all(invisible) {
                            return Err(error("El PDF contiene una combinación de caracteres que no se pudo representar. Puedes exportar a Excel."));
                        }
                        avance += p.x_advance as f32 / unidades;
                        glifos.push(KrillaGlyph::new(
                            GlyphId::new(g.glyph_id),
                            p.x_advance as f32 / unidades,
                            p.x_offset as f32 / unidades,
                            p.y_offset as f32 / unidades,
                            p.y_advance as f32 / unidades,
                            ini..fin,
                            None,
                        ));
                    }
                    ancho += avance * letra;
                    partes.push(Parte {
                        fuente,
                        rango,
                        glifos,
                        avance,
                    });
                }
            }
        }
        Ok(Linea {
            texto: texto.to_owned(),
            partes,
            ancho,
        })
    }

    pub fn envolver(&self, s: &str, limite: f32, letra: f32, negrita: bool) -> R<Vec<Linea>> {
        let mut salida = Vec::new();
        for parrafo in s.split(['\n', '\r']) {
            let p = parrafo.replace('\t', "    ");
            let linea = self.formar(&p, letra, negrita)?;
            if linea.ancho <= limite {
                salida.push(linea);
                continue;
            }
            let mut resto = p.as_str();
            while !resto.is_empty() {
                // Busca el último límite de palabra que quepa. Si una palabra sola
                // no cabe, corta únicamente entre grafemas.
                let mut elegida = None;
                for (i, g) in resto.split_word_bound_indices() {
                    let n = i + g.len();
                    let candidata = self.formar(&resto[..n], letra, negrita)?;
                    if candidata.ancho > limite {
                        break;
                    }
                    elegida = Some((n, candidata));
                }
                if elegida.is_none() {
                    for (i, g) in resto.grapheme_indices(true) {
                        let n = i + g.len();
                        let candidata = self.formar(&resto[..n], letra, negrita)?;
                        if candidata.ancho > limite && elegida.is_some() {
                            break;
                        }
                        elegida = Some((n, candidata));
                    }
                }
                let (n, linea) = elegida.expect("un texto no vacío tiene al menos un grafema");
                salida.push(linea);
                resto = &resto[n..];
            }
        }
        Ok(salida)
    }

    pub fn texto(
        &mut self,
        surface: &mut Surface<'_>,
        mut x: f32,
        y: f32,
        letra: f32,
        linea: &Linea,
    ) -> R<()> {
        for p in &linea.partes {
            let fuente = if let Some(f) = &self.fuentes[p.fuente] {
                f.clone()
            } else {
                let f = Font::new(
                    (std::sync::Arc::new(FUENTES[p.fuente])
                        as std::sync::Arc<dyn AsRef<[u8]> + Send + Sync>)
                        .into(),
                    0,
                )
                .ok_or_else(|| error("No se pudo incorporar una fuente al PDF."))?;
                self.fuentes[p.fuente] = Some(f.clone());
                f
            };
            surface.draw_glyphs(
                Point::from_xy(x, y),
                &p.glifos,
                fuente,
                &linea.texto[p.rango.clone()],
                letra,
                false,
            );
            x += p.avance * letra;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_ajuste_no_separa_el_acento_de_su_letra() {
        let tipografia = Tipografia::nueva().unwrap();
        let s = "e\u{301}e\u{301}e\u{301}";
        let lineas = tipografia.envolver(s, 6.0, 8.5, false).unwrap();
        assert_eq!(lineas.len(), 3);
        assert!(lineas.iter().all(|l| l.texto == "e\u{301}"));
    }
}

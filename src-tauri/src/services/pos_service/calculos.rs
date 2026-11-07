use super::{error, json_error};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineaCalculada {
    pub id_producto: Option<i64>,
    pub id_servicio: Option<i64>,
    pub id_consumo_ot: Option<i64>,
    pub id_servicio_ot: Option<i64>,
    pub codigo: String,
    pub nombre: String,
    pub unidad: String,
    pub cantidad: i64,
    pub precio_cent: i64,
    pub bruto_cent: i64,
    pub descuento_cent: i64,
    pub base_cent: i64,
    pub igv_cent: i64,
    pub total_cent: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costo_unitario_micro: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costo_total_micro: Option<i64>,
}
pub fn redondear(n: i128, d: i128) -> i64 {
    ((n + d / 2) / d) as i64
}
pub fn calcular(lineas: &mut [LineaCalculada], bp: i64) -> Result<serde_json::Value, AppError> {
    if !(0..=9999).contains(&bp) || lineas.is_empty() {
        return Err(error("Revisa las líneas o el porcentaje de descuento."));
    }
    let mut bruto = 0i64;
    for l in lineas.iter_mut() {
        if l.cantidad <= 0
            || l.cantidad > 9_999_999_999
            || l.precio_cent <= 0
            || l.precio_cent > 99_999_999
            || (l.unidad == "UND" && l.cantidad > 9_999_999)
        {
            return Err(error("Cantidad o precio fuera de los límites de POS."));
        }
        l.bruto_cent = redondear(
            l.cantidad as i128 * l.precio_cent as i128,
            if l.unidad == "ML" { 1000 } else { 1 },
        );
        if l.bruto_cent <= 0 || l.bruto_cent > 9_999_999_999 {
            return Err(error("El importe de una línea está fuera de los límites."));
        }
        bruto = bruto
            .checked_add(l.bruto_cent)
            .ok_or_else(|| error("Importe fuera de rango."))?;
    }
    if bruto > 9_999_999_999 {
        return Err(error("El total supera S/ 99 999 999,99."));
    }
    let descuento = redondear(bruto as i128 * bp as i128, 10000);
    let mut asignado = 0;
    let mut restos = Vec::new();
    for (i, l) in lineas.iter_mut().enumerate() {
        let n = descuento as i128 * l.bruto_cent as i128;
        l.descuento_cent = (n / bruto as i128) as i64;
        asignado += l.descuento_cent;
        restos.push((i, n % bruto as i128));
    }
    restos.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (i, _) in restos.into_iter().take((descuento - asignado) as usize) {
        lineas[i].descuento_cent += 1;
    }
    let mut base = 0;
    let mut igv = 0;
    for l in lineas.iter_mut() {
        l.total_cent = l.bruto_cent - l.descuento_cent;
        if l.total_cent <= 0 {
            return Err(error("El descuento deja una línea sin importe. Redúcelo."));
        }
        l.base_cent = redondear(l.total_cent as i128 * 10000, 11800);
        l.igv_cent = l.total_cent - l.base_cent;
        base += l.base_cent;
        igv += l.igv_cent;
    }
    Ok(
        serde_json::json!({"bruto_cent":bruto,"descuento_cent":descuento,"base_cent":base,"igv_cent":igv,"total_cent":bruto-descuento}),
    )
}
pub fn publicas(lineas: &[LineaCalculada]) -> Result<serde_json::Value, AppError> {
    let mut v = serde_json::to_value(lineas).map_err(json_error)?;
    for l in v.as_array_mut().into_iter().flatten() {
        if let Some(o) = l.as_object_mut() {
            o.remove("costo_unitario_micro");
            o.remove("costo_total_micro");
        }
    }
    Ok(v)
}

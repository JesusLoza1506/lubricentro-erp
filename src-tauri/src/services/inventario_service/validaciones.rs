use crate::errors::AppError;

/// Precio capturado por UND o litro: de S/ 0.01 a S/ 999999.99, sin recortar decimales.
pub(super) fn costo_capturado(costo: i64) -> Result<i64, AppError> {
    if !(10_000..=999_999_990_000).contains(&costo) || costo % 10_000 != 0 {
        return Err(AppError::Validation(
            "El costo debe ser de S/ 0.01 a S/ 999999.99 y tener como máximo dos decimales.".into(),
        ));
    }
    Ok(costo)
}

pub(super) fn cantidad_base(cantidad: i64, unidad: &str) -> Result<i64, AppError> {
    let maximo = if unidad == "ML" {
        9_999_999_999
    } else {
        9_999_999
    };
    if !(0..=maximo).contains(&cantidad) {
        return Err(AppError::Validation(
            "La cantidad está fuera del rango permitido.".into(),
        ));
    }
    Ok(cantidad)
}

pub(super) fn cantidad_comercial(cantidad: f64, unidad: &str) -> Result<i64, AppError> {
    let base = cantidad * if unidad == "ML" { 1000.0 } else { 1.0 };
    if !base.is_finite() || base < 0.0 || (base - base.round()).abs() > 0.000001 {
        return Err(AppError::Validation(
            "Use cantidades enteras en UND o un máximo de 3 decimales en litros.".into(),
        ));
    }
    if base
        > if unidad == "ML" {
            9_999_999_999.0
        } else {
            9_999_999.0
        }
    {
        return Err(AppError::Validation(
            "La cantidad está fuera del rango permitido.".into(),
        ));
    }
    cantidad_base(base.round() as i64, unidad)
}

/// Mismo techo persistente del esquema; usa i128 para validar antes de convertir.
pub(super) fn valor_stock(valor: i128) -> Result<i64, AppError> {
    if !(0..=9_000_000_000_000_000).contains(&valor) {
        return Err(AppError::Validation(
            "El valor total de inventario supera el límite permitido.".into(),
        ));
    }
    Ok(valor as i64)
}

pub(super) fn aporte_micro(
    cantidad: i64,
    costo_comercial: i64,
    unidad: &str,
) -> Result<i64, AppError> {
    let bruto = cantidad as i128 * costo_comercial as i128;
    valor_stock(if unidad == "ML" {
        (bruto + 500) / 1000
    } else {
        bruto
    })
}

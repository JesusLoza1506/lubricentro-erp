pub fn calcular_montos(monto_total: f64) -> (f64, f64, f64) {
    let subtotal = monto_total / 1.18;
    let igv = monto_total - subtotal;
    // Redondeo contable a 2 decimales
    ((subtotal * 100.0).round() / 100.0, (igv * 100.0).round() / 100.0, monto_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculo_igv_precision() {
        // Caso de prueba: precisión numérica
        let (subtotal, igv, total) = calcular_montos(100.0);
        assert_eq!(subtotal, 84.75);
        assert_eq!(igv, 15.25);
        assert_eq!(total, 100.0);
    }
}
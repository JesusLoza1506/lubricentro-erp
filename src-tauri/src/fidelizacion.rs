pub enum TipoAceite {
    Mineral,
    Sintetico,
}

pub fn calcular_proximo_mantenimiento(kilometraje_actual: u32, tipo: TipoAceite) -> u32 {
    match tipo {
        TipoAceite::Mineral => kilometraje_actual + 5_000,
        TipoAceite::Sintetico => kilometraje_actual + 10_000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proyeccion_kilometraje() {
        // Caso de prueba: Tabla de decisión[cite: 9]
        assert_eq!(calcular_proximo_mantenimiento(50_000, TipoAceite::Mineral), 55_000);
        assert_eq!(calcular_proximo_mantenimiento(50_000, TipoAceite::Sintetico), 60_000);
    }
}
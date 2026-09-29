#[derive(Debug, PartialEq)]
pub enum EstadoCaja {
    Abierta,
    Cerrada,
}

pub struct CajaChica {
    pub monto_apertura: f64,
    pub estado: EstadoCaja,
}

impl CajaChica {
    pub fn abrir(monto: f64) -> Result<Self, String> {
        if monto < 0.0 {
            return Err("El monto de apertura no puede ser negativo".to_string());
        }
        Ok(Self {
            monto_apertura: monto,
            estado: EstadoCaja::Abierta,
        })
    }

    pub fn cerrar(&mut self) -> Result<(), String> {
        if self.estado == EstadoCaja::Cerrada {
            return Err("La caja ya está cerrada".to_string());
        }
        self.estado = EstadoCaja::Cerrada;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apertura_y_cierre_caja() {
        // Valores límite y transición de estados
        assert!(
            CajaChica::abrir(-10.0).is_err(),
            "Debe rechazar montos negativos"
        );

        let mut caja = CajaChica::abrir(150.50).unwrap();
        assert_eq!(caja.estado, EstadoCaja::Abierta);

        caja.cerrar().unwrap();
        assert_eq!(caja.estado, EstadoCaja::Cerrada);

        let error = caja.cerrar().unwrap_err();
        assert_eq!(
            error, "La caja ya está cerrada",
            "No se puede cerrar dos veces"
        );
    }
}

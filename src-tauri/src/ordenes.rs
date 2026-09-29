#[derive(Debug, PartialEq, Clone)]
pub enum EstadoOT {
    EnEspera,
    EnProceso,
    Finalizado,
    Cancelado,
}

pub struct OrdenTrabajo {
    pub id: u32,
    pub estado: EstadoOT,
}

impl OrdenTrabajo {
    pub fn nueva(id: u32) -> Self {
        Self { id, estado: EstadoOT::EnEspera }
    }

    pub fn avanzar_a_proceso(&mut self) -> Result<(), String> {
        if self.estado != EstadoOT::EnEspera {
            return Err("Solo se puede pasar a EN_PROCESO desde EN_ESPERA".to_string());
        }
        self.estado = EstadoOT::EnProceso;
        Ok(())
    }

    pub fn finalizar(&mut self) -> Result<(), String> {
        if self.estado != EstadoOT::EnProceso {
            return Err("Solo se puede FINALIZAR una orden EN_PROCESO".to_string());
        }
        self.estado = EstadoOT::Finalizado;
        Ok(())
    }

    pub fn cancelar(&mut self) -> Result<(), String> {
        if self.estado == EstadoOT::Finalizado {
            return Err("No se puede CANCELAR una orden ya FINALIZADA".to_string());
        }
        self.estado = EstadoOT::Cancelado;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transiciones_estado_ot() {
        let mut ot = OrdenTrabajo::nueva(1);
        ot.avanzar_a_proceso().unwrap();
        ot.finalizar().unwrap();

        // Validar que se rechace un salto inválido (FINALIZADO a EN_PROCESO)
        assert!(ot.avanzar_a_proceso().is_err(), "Debe rechazar pasar a EN_PROCESO desde FINALIZADO");

        // Validar que se rechace finalizar una orden recién creada (EN_ESPERA)
        let mut ot2 = OrdenTrabajo::nueva(2);
        assert!(ot2.finalizar().is_err(), "Debe rechazar finalizar desde EN_ESPERA");

        // Validar que se rechace cancelar una orden ya finalizada
        let mut ot3 = OrdenTrabajo::nueva(3);
        ot3.avanzar_a_proceso().unwrap();
        ot3.finalizar().unwrap();
        assert!(ot3.cancelar().is_err(), "Debe rechazar cancelar desde FINALIZADO");
    }
}
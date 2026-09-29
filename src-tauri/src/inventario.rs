pub struct ProductoGranel {
    pub id: u32,
    pub stock_actual: f64,
    pub stock_minimo: f64,
}

impl ProductoGranel {
    pub fn nuevo(id: u32, stock_inicial: f64, stock_minimo: f64) -> Self {
        Self { id, stock_actual: stock_inicial, stock_minimo }
    }

    pub fn despachar(&mut self, cantidad: f64) -> Result<(), String> {
        if cantidad <= 0.0 {
            return Err("La cantidad debe ser mayor a cero".to_string());
        }
        if self.stock_actual < cantidad {
            return Err("Stock insuficiente".to_string());
        }
        self.stock_actual -= cantidad;
        Ok(())
    }

    pub fn en_alerta_critica(&self) -> bool {
        self.stock_actual <= self.stock_minimo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_despacho_y_limites_inventario() {
        let mut producto = ProductoGranel::nuevo(1, 10.5, 2.0); // 10.5 litros
        
        // Llegar al stock mínimo
        producto.despachar(8.5).unwrap();
        assert_eq!(producto.stock_actual, 2.0);
        assert!(producto.en_alerta_critica(), "Debe alertar al alcanzar el stock mínimo");

        // Intentar despachar 0.01 litros más de lo disponible (pedimos 2.01)
        assert!(producto.despachar(2.01).is_err(), "Debe fallar al pedir más del stock disponible");
        
        // Despachar exactamente el restante
        producto.despachar(2.0).unwrap();
        assert_eq!(producto.stock_actual, 0.0);

        // Validar ruta de error: cantidad menor o igual a cero
        assert!(producto.despachar(0.0).is_err(), "Debe rechazar despachos de 0 o negativos");
    }
}
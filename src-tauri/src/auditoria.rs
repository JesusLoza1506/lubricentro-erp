pub struct RegistroAuditoria {
    pub id_usuario: u32,
    pub tabla_afectada: String,
    pub accion: String,
}

pub fn registrar_accion(id_usuario: u32, tabla: &str, accion: &str) -> RegistroAuditoria {
    RegistroAuditoria {
        id_usuario,
        tabla_afectada: tabla.to_string(),
        accion: accion.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion_registro_auditoria() {
        // Cobertura de ruta básica
        let registro = registrar_accion(5, "comprobantes", "INSERT");
        assert_eq!(registro.id_usuario, 5);
        assert_eq!(registro.tabla_afectada, "comprobantes");
        assert_eq!(registro.accion, "INSERT");
    }
}

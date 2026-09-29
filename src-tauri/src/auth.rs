use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

/// Genera un hash seguro a partir de una contraseña en texto plano
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verifica si la contraseña en texto plano coincide con el hash almacenado
pub fn verify_password(password: &str, hashed: &str) -> Result<bool, BcryptError> {
    verify(password, hashed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_y_verificar_password() {
        // Caso de prueba: Partición de equivalencia (credenciales válidas vs inválidas)
        let password = "mi_clave_secreta_123";

        // Generamos el hash
        let hashed = hash_password(password).expect("Fallo al hashear la contraseña");

        // Verificamos que la contraseña correcta pase
        assert!(
            verify_password(password, &hashed).unwrap(),
            "La contraseña correcta debería ser aceptada"
        );

        // Verificamos que una contraseña incorrecta sea rechazada
        assert!(
            !verify_password("clave_equivocada", &hashed).unwrap(),
            "Una contraseña incorrecta debería ser rechazada"
        );
    }
}

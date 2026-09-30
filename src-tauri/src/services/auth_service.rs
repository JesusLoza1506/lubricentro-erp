use crate::dto::usuario_dto::UsuarioSesionDTO;
use crate::entities::usuario::UsuarioEntity;

pub struct AuthService;

impl AuthService {
    /// Transforma una Entity de usuario en un DTO de sesión seguro para el frontend.
    /// Excluye explícitamente el hash de la contraseña.
    pub fn mapear_a_dto(usuario: UsuarioEntity) -> UsuarioSesionDTO {
        UsuarioSesionDTO {
            id_usuario: usuario.id_usuario,
            nombre_completo: usuario.nombre_completo,
            usuario: usuario.usuario,
            rol: usuario.rol,
            activo: usuario.activo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuga_campos_sensibles_usuario() {
        // 1. Instanciamos una Entity que sí contiene el hash sensible
        let usuario_entity = UsuarioEntity {
            id_usuario: 1,
            nombre_completo: "Jesús Loza".to_string(),
            usuario: "jloza".to_string(),
            password_hash: "$2b$12$e8yC3i2m1qS0vW...hash_muy_secreto".to_string(),
            rol: "ADMINISTRADOR".to_string(),
            activo: true,
        };

        // 2. Lo mapeamos a su DTO de salida
        let dto = AuthService::mapear_a_dto(usuario_entity);

        // 3. Serializamos el DTO a JSON (tal como viajaría hacia React)
        let json_salida = serde_json::to_string(&dto).expect("Fallo al serializar DTO");

        // 4. VERIFICACIÓN DE SEGURIDAD: Confirmamos por código que el hash NO existe en la cadena
        assert!(!json_salida.contains("password_hash"));
        assert!(!json_salida.contains("e8yC3i2m1qS0vW"));
        assert_eq!(dto.usuario, "jloza");
    }
}

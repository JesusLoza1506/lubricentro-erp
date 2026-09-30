use crate::dto::usuario_dto::UsuarioSesionDTO;
use crate::entities::usuario::UsuarioEntity;
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::Connection;

pub struct AuthService;

impl AuthService {
    /// Autentica a un usuario contra la base de datos de SQLite utilizando bcrypt::verify.
    pub fn autenticar_usuario(
        conn: &Connection,
        usuario_input: &str,
        password_input: &str,
    ) -> Result<UsuarioSesionDTO, String> {
        println!(
            "🔐 [AuthService] Intentando autenticar al usuario: '{}'",
            usuario_input
        );

        if password_input.trim().is_empty() {
            println!("❌ [AuthService] La contraseña ingresada está vacía.");
            return Err("La contraseña no puede estar vacía.".to_string());
        }

        let mut stmt = match conn.prepare("SELECT id_usuario, nombre_completo, usuario, password_hash, rol, activo FROM usuarios WHERE usuario = ?1 AND activo = 1") {
            Ok(s) => s,
            Err(e) => {
                println!("❌ [AuthService] Error preparando la consulta SQL: {}", e);
                return Err(format!("Error en consulta SQL: {}", e));
            }
        };

        let usuario_entity = match stmt.query_row([usuario_input], |row| {
            Ok(UsuarioEntity {
                id_usuario: row.get(0)?,
                nombre_completo: row.get(1)?,
                usuario: row.get(2)?,
                password_hash: row.get(3)?,
                rol: row.get(4)?,
                activo: row.get(5)?,
            })
        }) {
            Ok(ent) => {
                println!(
                    "✅ [AuthService] Usuario encontrado en la BD: ID {}, Usuario: '{}'",
                    ent.id_usuario, ent.usuario
                );
                ent
            }
            Err(e) => {
                println!("❌ [AuthService] No se encontró el usuario activo '{}' en la BD o error de mapeo. Detalle: {}", usuario_input, e);
                return Err("Usuario o contraseña incorrectos.".to_string());
            }
        };

        // Validación real y segura con Bcrypt
        println!("🔍 [AuthService] Verificando hash bcrypt para el usuario...");
        let es_valido = verify(password_input, &usuario_entity.password_hash).unwrap_or(false);
        if !es_valido {
            println!(
                "❌ [AuthService] La contraseña ingresada NO coincide con el hash almacenado."
            );
            return Err("Usuario o contraseña incorrectos.".to_string());
        }

        println!(
            "🎉 [AuthService] ¡Autenticación exitosa! Contraseña correcta para: {}",
            usuario_entity.usuario
        );
        Ok(Self::mapear_a_dto(usuario_entity))
    }

    /// Genera el hash seguro (Bcrypt) de la contraseña y registra el usuario en SQLite.
    pub fn crear_usuario(
        conn: &Connection,
        nombre_completo: &str,
        usuario: &str,
        password_plano: &str,
        rol: &str,
    ) -> Result<(), String> {
        let hash_pwd =
            hash(password_plano, DEFAULT_COST).map_err(|_| "Error al encriptar la contraseña")?;

        conn.execute(
            "INSERT INTO usuarios (nombre_completo, usuario, password_hash, rol, activo) VALUES (?1, ?2, ?3, ?4, 1)",
            [nombre_completo, usuario, &hash_pwd, rol],
        ).map_err(|e| format!("Error al crear usuario en la BD: {}", e))?;

        Ok(())
    }

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
        let usuario_entity = UsuarioEntity {
            id_usuario: 1,
            nombre_completo: "Jesús Loza".to_string(),
            usuario: "jloza".to_string(),
            password_hash: "$2b$12$e8yC3i2m1qS0vW...hash_muy_secreto".to_string(),
            rol: "ADMINISTRADOR".to_string(),
            activo: true,
        };

        let dto = AuthService::mapear_a_dto(usuario_entity);
        let json_salida = serde_json::to_string(&dto).expect("Fallo al serializar DTO");

        assert!(!json_salida.contains("password_hash"));
        assert!(!json_salida.contains("e8yC3i2m1qS0vW"));
        assert_eq!(dto.usuario, "jloza");
    }
}

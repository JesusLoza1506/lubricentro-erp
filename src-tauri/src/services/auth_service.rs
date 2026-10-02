use crate::dto::permiso_dto::PermisoDto;
use crate::dto::usuario_dto::{CrearUsuarioRequestDto, LoginRequestDto, LoginResponseDto};
use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::Connection;

pub struct AuthService;

impl AuthService {
    /// Autentica a un usuario contra la base de datos V3 y carga su matriz de permisos.
    pub fn login(conn: &Connection, req: LoginRequestDto) -> Result<LoginResponseDto, AppError> {
        if req.password.trim().is_empty() {
            return Err(AppError::Validation(
                "La contraseña no puede estar vacía.".to_string(),
            ));
        }

        let mut stmt = conn.prepare(
            "SELECT id_usuario, nombre_completo, username, password_hash, rol, activo 
             FROM usuarios 
             WHERE username = ?1 AND activo = 1",
        )?;

        let mut rows = stmt.query([&req.username])?;

        if let Some(row) = rows.next()? {
            let id_usuario: i64 = row.get(0)?;
            let nombre_completo: String = row.get(1)?;
            let username: String = row.get(2)?;
            let password_hash: String = row.get(3)?;
            let rol: String = row.get(4)?;
            let activo: bool = row.get::<_, i32>(5)? == 1;

            // Validación de la contraseña con bcrypt
            let es_valido = verify(&req.password, &password_hash)
                .map_err(|_| AppError::Auth("Error al verificar la contraseña.".to_string()))?;

            if !es_valido {
                return Err(AppError::Auth(
                    "Usuario o contraseña incorrectos.".to_string(),
                ));
            }

            // Carga de la matriz de permisos correspondiente al rol
            let permisos = Self::obtener_permisos_rol(conn, &rol)?;

            Ok(LoginResponseDto {
                id_usuario,
                nombre_completo,
                username,
                rol,
                activo,
                permisos,
            })
        } else {
            Err(AppError::Auth(
                "Usuario o contraseña incorrectos.".to_string(),
            ))
        }
    }

    /// Obtiene los datos del usuario en sesión activa mediante su ID.
    pub fn obtener_usuario_sesion(
        conn: &Connection,
        id_usuario: i64,
    ) -> Result<LoginResponseDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_usuario, nombre_completo, username, rol, activo 
             FROM usuarios 
             WHERE id_usuario = ?1 AND activo = 1",
        )?;

        let mut rows = stmt.query([id_usuario])?;

        if let Some(row) = rows.next()? {
            let id_usuario: i64 = row.get(0)?;
            let nombre_completo: String = row.get(1)?;
            let username: String = row.get(2)?;
            let rol: String = row.get(3)?;
            let activo: bool = row.get::<_, i32>(4)? == 1;

            let permisos = Self::obtener_permisos_rol(conn, &rol)?;

            Ok(LoginResponseDto {
                id_usuario,
                nombre_completo,
                username,
                rol,
                activo,
                permisos,
            })
        } else {
            Err(AppError::NotFound(
                "Usuario no encontrado o inactivo.".to_string(),
            ))
        }
    }

    /// Genera el hash seguro (Bcrypt) y crea un nuevo usuario en la BD.
    pub fn crear_usuario(conn: &Connection, req: CrearUsuarioRequestDto) -> Result<i64, AppError> {
        if !["ADMINISTRADOR", "MECANICO", "CAJERO"].contains(&req.rol.as_str()) {
            return Err(AppError::Validation("Rol no válido.".to_string()));
        }

        let hash_pwd = hash(&req.password, DEFAULT_COST)
            .map_err(|_| AppError::Validation("Error al encriptar la contraseña.".to_string()))?;

        conn.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo) 
             VALUES (?1, ?2, ?3, ?4, 1)",
            [&req.nombre_completo, &req.username, &hash_pwd, &req.rol],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Consulta auxiliar para obtener los permisos de la tabla `permisos_rol`.
    fn obtener_permisos_rol(conn: &Connection, rol: &str) -> Result<Vec<PermisoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT modulo, puede_ver, puede_crear, puede_editar, puede_eliminar 
             FROM permisos_rol 
             WHERE rol = ?1",
        )?;

        let permisos_iter = stmt.query_map([rol], |row| {
            Ok(PermisoDto {
                modulo: row.get(0)?,
                puede_ver: row.get::<_, i32>(1)? == 1,
                puede_crear: row.get::<_, i32>(2)? == 1,
                puede_editar: row.get::<_, i32>(3)? == 1,
                puede_eliminar: row.get::<_, i32>(4)? == 1,
            })
        })?;

        let mut permisos = Vec::new();
        for p in permisos_iter {
            permisos.push(p?);
        }

        Ok(permisos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_login_exitoso_y_permisos_por_los_tres_roles() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        let pass_hash = hash("secret123", DEFAULT_COST).unwrap();

        // 1. Probar usuario Administrador
        conn.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo) 
             VALUES ('Admin Test', 'admin_test', ?1, 'ADMINISTRADOR', 1)",
            [&pass_hash],
        )
        .unwrap();

        let res_admin = AuthService::login(
            &conn,
            LoginRequestDto {
                username: "admin_test".to_string(),
                password: "secret123".to_string(),
            },
        )
        .unwrap();

        assert_eq!(res_admin.username, "admin_test");
        assert_eq!(res_admin.rol, "ADMINISTRADOR");
        assert!(res_admin.activo);
        assert!(!res_admin.permisos.is_empty());

        // 2. Probar usuario Mecánico
        conn.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo) 
             VALUES ('Mecanico Test', 'mecanico_test', ?1, 'MECANICO', 1)",
            [&pass_hash],
        )
        .unwrap();

        let res_mecanico = AuthService::login(
            &conn,
            LoginRequestDto {
                username: "mecanico_test".to_string(),
                password: "secret123".to_string(),
            },
        )
        .unwrap();

        assert_eq!(res_mecanico.username, "mecanico_test");
        assert_eq!(res_mecanico.rol, "MECANICO");
        assert!(res_mecanico.activo);
        assert!(!res_mecanico.permisos.is_empty());

        // 3. Probar usuario Cajero
        conn.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo) 
             VALUES ('Cajero Test', 'cajero_test', ?1, 'CAJERO', 1)",
            [&pass_hash],
        )
        .unwrap();

        let res_cajero = AuthService::login(
            &conn,
            LoginRequestDto {
                username: "cajero_test".to_string(),
                password: "secret123".to_string(),
            },
        )
        .unwrap();

        assert_eq!(res_cajero.username, "cajero_test");
        assert_eq!(res_cajero.rol, "CAJERO");
        assert!(res_cajero.activo);
        assert!(!res_cajero.permisos.is_empty());
    }
}

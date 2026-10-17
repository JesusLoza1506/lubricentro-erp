use crate::dto::permiso_dto::PermisoDto;
use crate::dto::usuario_dto::{
    AutorizarDescuentoRequestDto, CambiarClaveObligatoriaRequestDto,
    CambiarClaveVoluntariaRequestDto, InstalacionInicialRequestDto, InstalacionInicialResponseDto,
    LoginRequestDto, LoginResponseDto, RecuperarAccesoAdminRequestDto, SesionResumenDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::session_manager::SessionState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;
use rusqlite::{params, Connection};

pub struct AuthService;

const DUMMY_ARGON2_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| {
                AppError::Validation("Error al procesar el hash de la contraseña.".to_string())
            })
    }

    pub fn verify_password(password: &str, hash_str: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(hash_str) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }

    fn generar_codigo_recuperacion() -> String {
        const CHARS: &[u8] = b"23456789ABCDEFGHJKLMNPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();
        let raw: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect();

        format!(
            "{}-{}-{}-{}-{}-{}-{}-{}",
            &raw[0..4],
            &raw[4..8],
            &raw[8..12],
            &raw[12..16],
            &raw[16..20],
            &raw[20..24],
            &raw[24..28],
            &raw[28..32]
        )
    }

    pub fn normalizar_codigo(codigo: &str) -> String {
        codigo.replace(['-', ' '], "").trim().to_uppercase()
    }

    pub fn validar_politica_password(password: &str, username: &str) -> Result<(), AppError> {
        let len = password.chars().count();
        if !(7..=128).contains(&len) {
            return Err(AppError::Validation(
                "La contraseña debe tener entre 7 y 128 caracteres.".to_string(),
            ));
        }

        let tiene_letra = password.chars().any(|c| c.is_alphabetic());
        let tiene_numero = password.chars().any(|c| c.is_numeric());

        if !tiene_letra || !tiene_numero {
            return Err(AppError::Validation(
                "La contraseña debe incluir al menos una letra y un número.".to_string(),
            ));
        }

        if !username.is_empty() && password.to_lowercase() == username.to_lowercase() {
            return Err(AppError::Validation(
                "La contraseña no puede ser igual al nombre de usuario.".to_string(),
            ));
        }

        Ok(())
    }

    pub fn normalizar_username(username: &str) -> Result<String, AppError> {
        let clean = username.trim().to_lowercase();
        if clean.len() < 3 || clean.len() > 30 {
            return Err(AppError::Validation(
                "El nombre de usuario debe tener entre 3 y 30 caracteres.".to_string(),
            ));
        }
        if clean.contains(' ') {
            return Err(AppError::Validation(
                "El nombre de usuario no puede contener espacios.".to_string(),
            ));
        }
        Ok(clean)
    }

    pub fn normalizar_nombre_completo(nombre: &str) -> Result<String, AppError> {
        let clean = nombre.trim();
        if clean.len() < 3 || clean.len() > 100 {
            return Err(AppError::Validation(
                "El nombre completo debe tener entre 3 y 100 caracteres.".to_string(),
            ));
        }
        Ok(clean.to_string())
    }

    pub fn instalacion_inicial(
        conn: &mut Connection,
        req: InstalacionInicialRequestDto,
    ) -> Result<InstalacionInicialResponseDto, AppError> {
        let clean_nombre = Self::normalizar_nombre_completo(&req.nombre_completo)?;
        let clean_username = Self::normalizar_username(&req.username)?;
        Self::validar_politica_password(&req.password, &clean_username)?;

        let hash_pwd = Self::hash_password(&req.password)?;
        let codigo_raw = Self::generar_codigo_recuperacion();
        let codigo_norm = Self::normalizar_codigo(&codigo_raw);
        let codigo_hash = Self::hash_password(&codigo_norm)?;

        let tx = conn.transaction()?;

        let count_tx: i64 = tx.query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))?;
        if count_tx > 0 {
            return Err(AppError::Validation(
                "La instalación inicial ya fue realizada.".to_string(),
            ));
        }

        tx.execute(
            "INSERT INTO usuarios (nombre_completo, username, password_hash, rol, activo, debe_cambiar_clave)
             VALUES (?1, ?2, ?3, 'ADMINISTRADOR', 1, 0)",
            params![clean_nombre, clean_username, hash_pwd],
        )?;

        let id_usuario = tx.last_insert_rowid();

        tx.execute(
            "UPDATE configuracion SET recuperacion_hash = ?1, fecha_codigo_recuperacion = datetime('now', '-5 hours') WHERE id_config = 1",
            params![codigo_hash],
        )?;

        AuditoriaService::registrar_evento_login(
            &tx,
            Some(id_usuario),
            Some(&clean_username),
            Some("ADMINISTRADOR"),
            "INSTALACION_INICIAL",
            "OK",
            Some("Primer arranque del sistema"),
        )?;

        tx.commit()?;

        Ok(InstalacionInicialResponseDto {
            id_usuario,
            username: clean_username,
            codigo_recuperacion: codigo_raw,
        })
    }

    pub fn login(
        conn: &Connection,
        req: LoginRequestDto,
    ) -> Result<(LoginResponseDto, i64), AppError> {
        let clean_username = req.username.trim().to_lowercase();

        let mut stmt = conn.prepare(
            "SELECT id_usuario, nombre_completo, username, password_hash, rol, activo,
                    debe_cambiar_clave, intentos_fallidos, bloqueado_hasta, version
             FROM usuarios
             WHERE username = ?1",
        )?;

        let mut rows = stmt.query([&clean_username])?;

        if let Some(row) = rows.next()? {
            let id_usuario: i64 = row.get(0)?;
            let nombre_completo: String = row.get(1)?;
            let username: String = row.get(2)?;
            let password_hash: String = row.get(3)?;
            let rol: String = row.get(4)?;
            let activo: bool = row.get::<_, i32>(5)? == 1;
            let debe_cambiar_clave: bool = row.get::<_, i32>(6)? == 1;
            let mut intentos_fallidos: i32 = row.get(7)?;
            let bloqueado_hasta: Option<String> = row.get(8)?;
            let usuario_version: i64 = row.get(9)?;

            if !activo {
                AuditoriaService::registrar_evento_login(
                    conn,
                    Some(id_usuario),
                    Some(&username),
                    Some(&rol),
                    "LOGIN_DENEGADO",
                    "DENEGADO",
                    Some("Cuenta inactiva"),
                )?;
                return Err(AppError::Auth("Credenciales inválidas.".to_string()));
            }

            if let Some(ref hasta) = bloqueado_hasta {
                let esta_bloqueado: bool = conn
                    .query_row("SELECT datetime('now', '-5 hours') < ?1", [hasta], |r| {
                        r.get(0)
                    })
                    .unwrap_or(false);

                if esta_bloqueado {
                    AuditoriaService::registrar_evento_login(
                        conn,
                        Some(id_usuario),
                        Some(&username),
                        Some(&rol),
                        "LOGIN_DENEGADO",
                        "DENEGADO",
                        Some("Cuenta bloqueada temporalmente"),
                    )?;
                    return Err(AppError::Auth("Credenciales inválidas.".to_string()));
                } else {
                    conn.execute(
                        "UPDATE usuarios SET intentos_fallidos = 0, bloqueado_hasta = NULL WHERE id_usuario = ?1",
                        [id_usuario],
                    )?;
                    intentos_fallidos = 0;
                }
            }

            let es_valido = Self::verify_password(&req.password, &password_hash);

            if !es_valido {
                intentos_fallidos += 1;
                if intentos_fallidos >= 5 {
                    conn.execute(
                        "UPDATE usuarios SET intentos_fallidos = 5, 
                                    bloqueado_hasta = datetime('now', '-5 hours', '+15 minutes') 
                             WHERE id_usuario = ?1",
                        [id_usuario],
                    )?;
                    AuditoriaService::registrar_evento_login(
                        conn,
                        Some(id_usuario),
                        Some(&username),
                        Some(&rol),
                        "BLOQUEO_CUENTA",
                        "OK",
                        Some("Bloqueo de 15 min por 5 intentos fallidos consecutivos"),
                    )?;
                } else {
                    conn.execute(
                        "UPDATE usuarios SET intentos_fallidos = ?1 WHERE id_usuario = ?2",
                        params![intentos_fallidos, id_usuario],
                    )?;
                }

                AuditoriaService::registrar_evento_login(
                    conn,
                    Some(id_usuario),
                    Some(&username),
                    Some(&rol),
                    "LOGIN_DENEGADO",
                    "DENEGADO",
                    Some("Contraseña incorrecta"),
                )?;
                return Err(AppError::Auth("Credenciales inválidas.".to_string()));
            }

            conn.execute(
                "UPDATE usuarios SET intentos_fallidos = 0, bloqueado_hasta = NULL WHERE id_usuario = ?1",
                [id_usuario],
            )?;

            let permisos = Self::obtener_permisos_estaticos(&rol);
            let estado_sesion = if debe_cambiar_clave {
                "CAMBIO_OBLIGATORIO".to_string()
            } else {
                "NORMAL".to_string()
            };

            AuditoriaService::registrar_evento_login(
                conn,
                Some(id_usuario),
                Some(&username),
                Some(&rol),
                "LOGIN_OK",
                "OK",
                Some("Inicio de sesión exitoso"),
            )?;

            Ok((
                LoginResponseDto {
                    id_usuario,
                    nombre_completo,
                    username,
                    rol,
                    activo,
                    debe_cambiar_clave,
                    estado_sesion,
                    permisos,
                    ambiente_actual: "PRODUCCION".to_string(),
                },
                usuario_version,
            ))
        } else {
            let _ = Self::verify_password(&req.password, DUMMY_ARGON2_HASH);
            AuditoriaService::registrar_evento_login(
                conn,
                None,
                Some(&clean_username),
                None,
                "LOGIN_DENEGADO",
                "DENEGADO",
                Some("Usuario inexistente"),
            )?;
            Err(AppError::Auth("Credenciales inválidas.".to_string()))
        }
    }

    pub fn obtener_usuario_sesion(
        conn: &Connection,
        session: &mut SessionState,
        id_usuario: i64,
        estado_memoria: &str,
    ) -> Result<SesionResumenDto, AppError> {
        let (username, nombre_completo, rol, activo, debe_cambiar_clave, version_bd): (
            String,
            String,
            String,
            i32,
            i32,
            i64,
        ) = match conn.query_row(
            "SELECT username, nombre_completo, rol, activo, debe_cambiar_clave, version FROM usuarios WHERE id_usuario = ?1",
            [id_usuario],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
        ) {
            Ok(res) => res,
            Err(_) => {
                session.invalidar_sesion_usuario(id_usuario);
                return Err(AppError::NoAutenticado);
            }
        };

        let rol_memoria = session.rol.as_deref().unwrap_or("");
        let version_memoria = session.usuario_version.unwrap_or(-1);

        if activo == 0
            || (!rol_memoria.is_empty() && rol_memoria != rol)
            || version_bd != version_memoria
        {
            session.invalidar_sesion_usuario(id_usuario);
            return Err(AppError::NoAutenticado);
        }

        let permisos = Self::obtener_permisos_estaticos(&rol);
        let estado_sesion = if debe_cambiar_clave == 1 {
            "CAMBIO_OBLIGATORIO".to_string()
        } else {
            estado_memoria.to_string()
        };

        Ok(SesionResumenDto {
            id_usuario,
            username,
            nombre_completo,
            rol,
            estado_sesion,
            permisos,
            ambiente_actual: "PRODUCCION".to_string(),
        })
    }

    pub fn recuperar_acceso_admin(
        conn: &mut Connection,
        req: RecuperarAccesoAdminRequestDto,
    ) -> Result<InstalacionInicialResponseDto, AppError> {
        let (mut fallos_rec, bloq_rec): (i32, Option<String>) = conn.query_row(
            "SELECT recuperacion_fallos, recuperacion_bloqueada_hasta FROM configuracion WHERE id_config = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;

        if let Some(ref hasta) = bloq_rec {
            let esta_bloqueado: bool = conn
                .query_row("SELECT datetime('now', '-5 hours') < ?1", [hasta], |r| {
                    r.get(0)
                })
                .unwrap_or(false);

            if esta_bloqueado {
                return Err(AppError::Auth(
                    "No se pudo recuperar el acceso con los datos ingresados.".to_string(),
                ));
            } else {
                conn.execute(
                    "UPDATE configuracion SET recuperacion_fallos = 0, recuperacion_bloqueada_hasta = NULL WHERE id_config = 1",
                    [],
                )?;
                fallos_rec = 0;
            }
        }

        let clean_username = req.username.trim().to_lowercase();
        let codigo_norm = Self::normalizar_codigo(&req.codigo_recuperacion);

        let (rec_hash_opt,): (Option<String>,) = conn.query_row(
            "SELECT recuperacion_hash FROM configuracion WHERE id_config = 1",
            [],
            |r| Ok((r.get(0)?,)),
        )?;

        let hash_vigente = match rec_hash_opt {
            Some(h) => h,
            None => {
                return Err(AppError::Auth(
                    "No se pudo recuperar el acceso con los datos ingresados.".to_string(),
                ))
            }
        };

        let codigo_valido = Self::verify_password(&codigo_norm, &hash_vigente);

        if !codigo_valido {
            let nuevos_fallos = fallos_rec + 1;
            if nuevos_fallos >= 5 {
                conn.execute(
                    "UPDATE configuracion SET recuperacion_fallos = 5, 
                                    recuperacion_bloqueada_hasta = datetime('now', '-5 hours', '+15 minutes') 
                     WHERE id_config = 1",
                    [],
                )?;
            } else {
                conn.execute(
                    "UPDATE configuracion SET recuperacion_fallos = ?1 WHERE id_config = 1",
                    [nuevos_fallos],
                )?;
            }
            AuditoriaService::registrar_evento_login(
                conn,
                None,
                Some(&clean_username),
                None,
                "RECUPERACION_ADMIN_FALLO",
                "DENEGADO",
                Some("Código de recuperación inválido"),
            )?;
            return Err(AppError::Auth(
                "No se pudo recuperar el acceso con los datos ingresados.".to_string(),
            ));
        }

        let (id_usuario, current_password_hash): (i64, String) = {
            let mut stmt = conn.prepare(
                "SELECT id_usuario, password_hash FROM usuarios WHERE username = ?1 AND rol = 'ADMINISTRADOR' AND activo = 1",
            )?;
            match stmt.query_row([&clean_username], |r| Ok((r.get(0)?, r.get(1)?))) {
                Ok(data) => data,
                Err(_) => {
                    return Err(AppError::Auth(
                        "No se pudo recuperar el acceso con los datos ingresados.".to_string(),
                    ))
                }
            }
        };

        Self::validar_politica_password(&req.nueva_password, &clean_username)?;

        if Self::verify_password(&req.nueva_password, &current_password_hash) {
            return Err(AppError::Validation(
                "La nueva contraseña debe ser diferente a la contraseña actual.".to_string(),
            ));
        }

        let nueva_pwd_hash = Self::hash_password(&req.nueva_password)?;
        let nuevo_codigo_raw = Self::generar_codigo_recuperacion();
        let nuevo_codigo_norm = Self::normalizar_codigo(&nuevo_codigo_raw);
        let nuevo_codigo_hash = Self::hash_password(&nuevo_codigo_norm)?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE usuarios 
             SET password_hash = ?1, debe_cambiar_clave = 0, intentos_fallidos = 0, bloqueado_hasta = NULL,
                 fecha_cambio_clave = datetime('now', '-5 hours'), version = version + 1
             WHERE id_usuario = ?2",
            params![nueva_pwd_hash, id_usuario],
        )?;

        tx.execute(
            "UPDATE configuracion 
             SET recuperacion_hash = ?1, fecha_codigo_recuperacion = datetime('now', '-5 hours'),
                 recuperacion_fallos = 0, recuperacion_bloqueada_hasta = NULL, version = version + 1
             WHERE id_config = 1",
            params![nuevo_codigo_hash],
        )?;

        AuditoriaService::registrar_evento_login(
            &tx,
            Some(id_usuario),
            Some(&clean_username),
            Some("ADMINISTRADOR"),
            "RECUPERACION_ADMIN",
            "OK",
            Some("Acceso restablecido exitosamente mediante código de recuperación"),
        )?;

        tx.commit()?;

        Ok(InstalacionInicialResponseDto {
            id_usuario,
            username: clean_username,
            codigo_recuperacion: nuevo_codigo_raw,
        })
    }

    pub fn cambiar_clave_obligatoria(
        conn: &mut Connection,
        session: &mut SessionState,
        id_usuario: i64,
        req: CambiarClaveObligatoriaRequestDto,
    ) -> Result<(), AppError> {
        let usr_id_valido = session.validar_sesion_vigente(conn, true)?;
        if usr_id_valido != id_usuario {
            return Err(AppError::NoAutenticado);
        }

        let (username, password_hash, rol, debe_cambiar_clave): (String, String, String, i32) = conn
            .query_row(
                "SELECT username, password_hash, rol, debe_cambiar_clave FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [id_usuario],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )?;

        if debe_cambiar_clave != 1 {
            return Err(AppError::Validation(
                "Operación no permitida: El usuario no requiere cambio obligatorio de clave."
                    .to_string(),
            ));
        }

        if Self::verify_password(&req.nueva_password, &password_hash) {
            return Err(AppError::Validation(
                "La nueva contraseña debe ser diferente de la contraseña actual.".to_string(),
            ));
        }

        Self::validar_politica_password(&req.nueva_password, &username)?;

        let nueva_hash = Self::hash_password(&req.nueva_password)?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE usuarios 
             SET password_hash = ?1, debe_cambiar_clave = 0, intentos_fallidos = 0, bloqueado_hasta = NULL,
                 fecha_cambio_clave = datetime('now', '-5 hours'), version = version + 1
             WHERE id_usuario = ?2",
            params![nueva_hash, id_usuario],
        )?;

        let nueva_version: i64 = tx.query_row(
            "SELECT version FROM usuarios WHERE id_usuario = ?1",
            [id_usuario],
            |r| r.get(0),
        )?;

        AuditoriaService::registrar_evento_login(
            &tx,
            Some(id_usuario),
            Some(&username),
            Some(&rol),
            "CAMBIO_CLAVE_OBLIGATORIO",
            "OK",
            Some("Cambio obligatorio de contraseña completado exitosamente"),
        )?;

        tx.commit()?;

        session.confirmaciones_pendientes.clear();
        session.actualizar_version_sesion(nueva_version);
        session.habilitar_sesion_normal();

        Ok(())
    }

    pub fn cambiar_clave_voluntaria(
        conn: &mut Connection,
        session: &mut SessionState,
        id_usuario: i64,
        req: CambiarClaveVoluntariaRequestDto,
    ) -> Result<(), AppError> {
        let usr_id_valido = session.validar_sesion_operativa(conn)?;
        if usr_id_valido != id_usuario {
            return Err(AppError::NoAutenticado);
        }

        let (username, password_hash, rol): (String, String, String) = conn.query_row(
            "SELECT username, password_hash, rol FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
            [id_usuario],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )?;

        if !Self::verify_password(&req.password_actual, &password_hash) {
            return Err(AppError::Auth(
                "La contraseña actual es incorrecta.".to_string(),
            ));
        }

        if req.password_actual == req.nueva_password {
            return Err(AppError::Validation(
                "La nueva contraseña debe ser diferente de la contraseña actual.".to_string(),
            ));
        }

        Self::validar_politica_password(&req.nueva_password, &username)?;

        let nueva_hash = Self::hash_password(&req.nueva_password)?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE usuarios 
             SET password_hash = ?1, debe_cambiar_clave = 0,
                 fecha_cambio_clave = datetime('now', '-5 hours'), version = version + 1
             WHERE id_usuario = ?2",
            params![nueva_hash, id_usuario],
        )?;

        let nueva_version: i64 = tx.query_row(
            "SELECT version FROM usuarios WHERE id_usuario = ?1",
            [id_usuario],
            |r| r.get(0),
        )?;

        AuditoriaService::registrar_evento_login(
            &tx,
            Some(id_usuario),
            Some(&username),
            Some(&rol),
            "CAMBIO_CLAVE_VOLUNTARIO",
            "OK",
            Some("Actualización voluntaria de contraseña completada con éxito"),
        )?;

        tx.commit()?;

        session.actualizar_version_sesion(nueva_version);

        Ok(())
    }

    pub fn autorizar_descuento(
        conn: &Connection,
        session: &mut SessionState,
        req: AutorizarDescuentoRequestDto,
    ) -> Result<i64, AppError> {
        let clean_admin = req.username_admin.trim().to_lowercase();

        let (id_admin, password_hash, rol, activo, debe_cambiar_clave, bloqueado_hasta): (
            i64,
            String,
            String,
            i32,
            i32,
            Option<String>,
        ) = match conn.query_row(
            "SELECT id_usuario, password_hash, rol, activo, debe_cambiar_clave, bloqueado_hasta
             FROM usuarios WHERE username = ?1",
            [&clean_admin],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        ) {
            Ok(u) => u,
            Err(_) => {
                let _ = Self::verify_password(&req.password_admin, DUMMY_ARGON2_HASH);
                return Err(AppError::Auth(
                    "Credenciales de autorizante inválidas.".to_string(),
                ));
            }
        };

        if session.esta_verificacion_admin_pausada(id_admin) {
            return Err(AppError::Auth(
                "Demasiados intentos de verificación administrativa. Intente nuevamente en 15 minutos."
                    .to_string(),
            ));
        }

        if activo == 0 || rol != "ADMINISTRADOR" {
            return Err(AppError::Auth(
                "El usuario autorizante no está facultado.".to_string(),
            ));
        }

        if debe_cambiar_clave == 1 {
            return Err(AppError::Validation(
                "El autorizante debe cambiar su contraseña antes de otorgar autorizaciones."
                    .to_string(),
            ));
        }

        if let Some(ref hasta) = bloqueado_hasta {
            let esta_bloqueado: bool = conn
                .query_row("SELECT datetime('now', '-5 hours') < ?1", [hasta], |r| {
                    r.get(0)
                })
                .unwrap_or(false);

            if esta_bloqueado {
                return Err(AppError::Auth(
                    "Cuenta de autorizante bloqueada temporalmente.".to_string(),
                ));
            }
        }

        if !Self::verify_password(&req.password_admin, &password_hash) {
            let se_pauso = session.registrar_fallo_verificacion_admin(id_admin);
            if se_pauso {
                return Err(AppError::Auth(
                    "Demasiados intentos de verificación administrativa. Intente nuevamente en 15 minutos."
                        .to_string(),
                ));
            }
            return Err(AppError::Auth(
                "Credenciales de autorizante inválidas.".to_string(),
            ));
        }

        session.limpiar_fallos_verificacion_admin(id_admin);

        Ok(id_admin)
    }

    fn obtener_permisos_estaticos(rol: &str) -> Vec<PermisoDto> {
        let modulos = vec![
            "LOGIN",
            "DASHBOARD",
            "FICHA_VEHICULAR",
            "OT",
            "INVENTARIO",
            "POS",
            "CAJA",
            "BACKUP",
            "ADMIN",
            "REPORTES",
        ];

        modulos
            .into_iter()
            .map(|mod_name| match rol {
                "ADMINISTRADOR" => PermisoDto {
                    modulo: mod_name.to_string(),
                    puede_ver: true,
                    puede_crear: true,
                    puede_editar: true,
                    puede_eliminar: true,
                },
                "MECANICO" => PermisoDto {
                    modulo: mod_name.to_string(),
                    puede_ver: matches!(
                        mod_name,
                        "DASHBOARD" | "FICHA_VEHICULAR" | "OT" | "INVENTARIO"
                    ),
                    puede_crear: matches!(mod_name, "OT"),
                    puede_editar: matches!(mod_name, "OT"),
                    puede_eliminar: false,
                },
                "CAJERO" => PermisoDto {
                    modulo: mod_name.to_string(),
                    puede_ver: matches!(
                        mod_name,
                        "DASHBOARD" | "FICHA_VEHICULAR" | "OT" | "INVENTARIO" | "POS" | "CAJA"
                    ),
                    puede_crear: matches!(mod_name, "OT" | "POS" | "CAJA"),
                    puede_editar: matches!(mod_name, "POS" | "CAJA"),
                    puede_eliminar: false,
                },
                _ => PermisoDto {
                    modulo: mod_name.to_string(),
                    puede_ver: false,
                    puede_crear: false,
                    puede_editar: false,
                    puede_eliminar: false,
                },
            })
            .collect()
    }
}

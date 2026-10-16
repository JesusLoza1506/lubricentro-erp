use crate::errors::AppError;
use rusqlite::Connection;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstadoSesion {
    SinSesion,
    CambioObligatorio,
    Normal,
}

impl EstadoSesion {
    pub fn as_str(&self) -> &'static str {
        match self {
            EstadoSesion::SinSesion => "SIN_SESION",
            EstadoSesion::CambioObligatorio => "CAMBIO_OBLIGATORIO",
            EstadoSesion::Normal => "NORMAL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfirmacionData {
    pub autorizante_id: i64,
    pub accion: String,
    pub referencia: String,
    pub expiracion: Instant,
}

#[derive(Debug, Clone)]
pub struct IntentoVerificacionAdmin {
    pub fallos: i32,
    pub pausado_hasta: Option<Instant>,
}

pub struct SessionState {
    pub session_id: Option<String>,
    pub usuario_id: Option<i64>,
    pub username: Option<String>,
    pub rol: Option<String>,
    pub estado_sesion: EstadoSesion,
    pub ultima_actividad: Instant,
    pub confirmaciones_pendientes: HashMap<String, ConfirmacionData>,
    /// RB03: Límite independiente en memoria para verificaciones administrativas (5 fallos = 15 min pausa)
    pub verificaciones_admin_fallos: HashMap<i64, IntentoVerificacionAdmin>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            session_id: None,
            usuario_id: None,
            username: None,
            rol: None,
            estado_sesion: EstadoSesion::SinSesion,
            ultima_actividad: Instant::now(),
            confirmaciones_pendientes: HashMap::new(),
            verificaciones_admin_fallos: HashMap::new(),
        }
    }
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abrir una nueva sesión e iniciar el timer de inactividad
    pub fn abrir_sesion(
        &mut self,
        usuario_id: i64,
        username: String,
        rol: String,
        es_cambio_obligatorio: bool,
    ) -> String {
        let session_id = format!("sess_{}", uuid::Uuid::new_v4().simple());
        self.session_id = Some(session_id.clone());
        self.usuario_id = Some(usuario_id);
        self.username = Some(username);
        self.rol = Some(rol);
        self.estado_sesion = if es_cambio_obligatorio {
            EstadoSesion::CambioObligatorio
        } else {
            EstadoSesion::Normal
        };
        self.ultima_actividad = Instant::now();
        self.confirmaciones_pendientes.clear();

        session_id
    }

    /// Cerrar la sesión actual y limpiar la memoria de interfaz
    pub fn cerrar_sesion(&mut self) {
        self.session_id = None;
        self.usuario_id = None;
        self.username = None;
        self.rol = None;
        self.estado_sesion = EstadoSesion::SinSesion;
        self.confirmaciones_pendientes.clear();
    }

    /// Destruye inmediatamente la sesión activa en memoria si corresponde al id_usuario modificado/desactivado
    pub fn invalidar_sesion_usuario(&mut self, id_usuario_modificado: i64) {
        if self.usuario_id == Some(id_usuario_modificado) {
            self.cerrar_sesion();
        }
    }

    /// Transicionar de CAMBIO_OBLIGATORIO a NORMAL tras cambiar la clave
    pub fn habilitar_sesion_normal(&mut self) {
        if self.estado_sesion == EstadoSesion::CambioObligatorio {
            self.estado_sesion = EstadoSesion::Normal;
            self.ultima_actividad = Instant::now();
        }
    }

    /// Renovar la última actividad del usuario (exclusivo para registrar_actividad)
    pub fn renovar_actividad(&mut self) {
        if self.estado_sesion != EstadoSesion::SinSesion {
            self.ultima_actividad = Instant::now();
        }
    }

    /// Verificar si han transcurrido 15 minutos sin actividad humana (RB04)
    pub fn verificar_inactividad(&mut self) -> bool {
        if self.estado_sesion == EstadoSesion::SinSesion {
            return true;
        }

        if self.ultima_actividad.elapsed() >= Duration::from_secs(900) {
            self.cerrar_sesion();
            true
        } else {
            false
        }
    }

    /// Valida centralizadamente que exista una sesión activa en estado NORMAL
    /// y verifica contra la BD que el usuario siga activo y conserve el rol original (H04).
    pub fn validar_sesion_operativa(&mut self, conn: &Connection) -> Result<i64, AppError> {
        if self.verificar_inactividad() {
            return Err(AppError::NoAutenticado);
        }

        if self.estado_sesion == EstadoSesion::CambioObligatorio {
            return Err(AppError::Validation(
                "Debe cambiar su contraseña antes de realizar cualquier operación en el sistema."
                    .to_string(),
            ));
        }

        if self.estado_sesion != EstadoSesion::Normal {
            return Err(AppError::NoAutenticado);
        }

        let usr_id = self.usuario_id.ok_or(AppError::NoAutenticado)?;

        // Validación en BD y Revocación Dinámica por Desactivación o Cambio de Rol (H04 / TC-LOG-10_v7)
        let (activo, rol_bd): (i32, String) = match conn.query_row(
            "SELECT activo, rol FROM usuarios WHERE id_usuario = ?1",
            [usr_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        ) {
            Ok(data) => data,
            Err(_) => {
                self.invalidar_sesion_usuario(usr_id);
                return Err(AppError::NoAutenticado);
            }
        };

        let rol_memoria = self.rol.as_deref().unwrap_or("");

        if activo == 0 || rol_bd != rol_memoria {
            self.invalidar_sesion_usuario(usr_id);
            return Err(AppError::NoAutenticado);
        }

        Ok(usr_id)
    }

    /// RB03: Comprobar si un administrador está pausado para autorizaciones en memoria
    pub fn esta_verificacion_admin_pausada(&self, admin_id: i64) -> bool {
        if let Some(intento) = self.verificaciones_admin_fallos.get(&admin_id) {
            if let Some(pausa) = intento.pausado_hasta {
                return Instant::now() < pausa;
            }
        }
        false
    }

    /// RB03: Registrar un fallo de verificación administrativa sin afectar la BD
    pub fn registrar_fallo_verificacion_admin(&mut self, admin_id: i64) -> bool {
        let entry =
            self.verificaciones_admin_fallos
                .entry(admin_id)
                .or_insert(IntentoVerificacionAdmin {
                    fallos: 0,
                    pausado_hasta: None,
                });

        entry.fallos += 1;
        if entry.fallos >= 5 {
            entry.pausado_hasta = Some(Instant::now() + Duration::from_secs(900));
            true // Se pausó por 15 min
        } else {
            false
        }
    }

    /// RB03: Limpiar fallos de verificación administrativa tras un intento exitoso
    pub fn limpiar_fallos_verificacion_admin(&mut self, admin_id: i64) {
        self.verificaciones_admin_fallos.remove(&admin_id);
    }

    /// Guardar una autorización temporal de 60 segundos
    pub fn guardar_autorizacion(
        &mut self,
        clave_token: String,
        autorizante_id: i64,
        accion: String,
        referencia: String,
    ) {
        let expiracion = Instant::now() + Duration::from_secs(60);
        self.confirmaciones_pendientes.insert(
            clave_token,
            ConfirmacionData {
                autorizante_id,
                accion,
                referencia,
                expiracion,
            },
        );
    }

    /// Consumir una autorización previa para uso único
    pub fn consumir_autorizacion(
        &mut self,
        clave_token: &str,
        accion_esperada: &str,
        ref_esperada: &str,
    ) -> bool {
        if let Some(conf) = self.confirmaciones_pendientes.remove(clave_token) {
            if conf.expiracion >= Instant::now()
                && conf.accion == accion_esperada
                && conf.referencia == ref_esperada
            {
                return true;
            }
        }
        false
    }
}

use crate::errors::AppError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditoriaDto {
    pub id_auditoria: i64,
    pub id_operacion: Option<String>,
    pub id_usuario: Option<i64>,
    pub actor: String,
    pub username_snapshot: Option<String>,
    pub rol_snapshot: Option<String>,
    pub id_autorizante: Option<i64>,
    pub modulo: String,
    pub accion: String,
    pub entidad: Option<String>,
    pub id_entidad: Option<String>,
    pub antes_json: Option<String>,
    pub despues_json: Option<String>,
    pub motivo: Option<String>,
    pub resultado: String,
    pub fecha_evento: String,
}

pub struct AuditoriaService;

impl AuditoriaService {
    #[allow(clippy::too_many_arguments)]
    pub fn registrar_v5(
        conn: &Connection,
        id_usuario: Option<i64>,
        actor: &str,
        username_snapshot: Option<&str>,
        rol_snapshot: Option<&str>,
        id_autorizante: Option<i64>,
        modulo: &str,
        accion: &str,
        entidad: Option<&str>,
        id_entidad: Option<&str>,
        antes_json: Option<&str>,
        despues_json: Option<&str>,
        motivo: Option<&str>,
        resultado: &str,
    ) -> Result<(), AppError> {
        let id_op = format!("aud_{}", uuid::Uuid::new_v4().simple());

        let es_actor_usuario = actor == "USUARIO"
            && id_usuario.is_some()
            && username_snapshot.is_some()
            && rol_snapshot.is_some();

        let actor_final = if es_actor_usuario {
            "USUARIO"
        } else if actor == "SISTEMA" {
            "SISTEMA"
        } else {
            "NO_IDENTIFICADO"
        };

        let id_usr_final = if actor_final == "USUARIO" {
            id_usuario
        } else {
            None
        };
        let user_snap_final = if actor_final == "USUARIO" {
            username_snapshot
        } else {
            None
        };
        let rol_snap_final = if actor_final == "USUARIO" {
            rol_snapshot
        } else {
            None
        };

        conn.execute(
            "INSERT INTO auditoria (
                id_operacion, id_usuario, actor, username_snapshot, rol_snapshot,
                id_autorizante, modulo, accion, entidad, id_entidad,
                antes_json, despues_json, motivo, resultado
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14
            )",
            params![
                id_op,
                id_usr_final,
                actor_final,
                user_snap_final,
                rol_snap_final,
                id_autorizante,
                modulo,
                accion,
                entidad,
                id_entidad,
                antes_json,
                despues_json,
                motivo,
                resultado
            ],
        )?;

        Ok(())
    }

    pub fn registrar_evento_login(
        conn: &Connection,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
        accion: &str,
        resultado: &str,
        motivo: Option<&str>,
    ) -> Result<(), AppError> {
        let actor = if id_usuario.is_some() && username.is_some() && rol.is_some() {
            "USUARIO"
        } else {
            "NO_IDENTIFICADO"
        };

        Self::registrar_v5(
            conn,
            if actor == "USUARIO" { id_usuario } else { None },
            actor,
            if actor == "USUARIO" { username } else { None },
            if actor == "USUARIO" { rol } else { None },
            None,
            "LOGIN",
            accion,
            Some("usuarios"),
            id_usuario.map(|id| id.to_string()).as_deref(),
            None,
            None,
            motivo,
            resultado,
        )
    }

    pub fn registrar_cierre_sesion(
        conn: &Connection,
        id_usuario: i64,
        username: &str,
        rol: &str,
    ) -> Result<(), AppError> {
        Self::registrar_v5(
            conn,
            Some(id_usuario),
            "USUARIO",
            Some(username),
            Some(rol),
            None,
            "LOGIN",
            "CERRAR_SESION",
            Some("usuarios"),
            Some(&id_usuario.to_string()),
            None,
            None,
            Some("Cierre voluntario de sesión"),
            "OK",
        )
    }

    pub fn listar_auditorias(
        conn: &Connection,
        limite: i64,
    ) -> Result<Vec<AuditoriaDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_auditoria, id_operacion, id_usuario, actor, username_snapshot,
                    rol_snapshot, id_autorizante, modulo, accion, entidad,
                    id_entidad, antes_json, despues_json, motivo, resultado, fecha_evento
             FROM auditoria 
             ORDER BY fecha_evento DESC 
             LIMIT ?1",
        )?;

        let registros = stmt.query_map([limite], |row| {
            Ok(AuditoriaDto {
                id_auditoria: row.get(0)?,
                id_operacion: row.get(1)?,
                id_usuario: row.get(2)?,
                actor: row.get(3)?,
                username_snapshot: row.get(4)?,
                rol_snapshot: row.get(5)?,
                id_autorizante: row.get(6)?,
                modulo: row.get(7)?,
                accion: row.get(8)?,
                entidad: row.get(9)?,
                id_entidad: row.get(10)?,
                antes_json: row.get(11)?,
                despues_json: row.get(12)?,
                motivo: row.get(13)?,
                resultado: row.get(14)?,
                fecha_evento: row.get(15)?,
            })
        })?;

        let mut resultado = Vec::new();
        for r in registros {
            resultado.push(r?);
        }

        Ok(resultado)
    }
}

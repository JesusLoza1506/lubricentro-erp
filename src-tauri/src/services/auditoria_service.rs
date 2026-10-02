use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditoriaDto {
    pub id_auditoria: i32,
    pub id_usuario: Option<i32>,
    pub tabla_afectada: String,
    pub accion: String,
    pub detalle: Option<String>,
    pub fecha: String,
}

pub struct AuditoriaService;

impl AuditoriaService {
    pub fn registrar(
        conn: &Connection,
        id_usuario: Option<i32>,
        tabla_afectada: &str,
        accion: &str,
        detalle: &str,
    ) -> Result<(), String> {
        conn.execute(
            "INSERT INTO auditoria (id_usuario, tabla_afectada, accion, detalle) 
             VALUES (?1, ?2, ?3, ?4)",
            params![id_usuario, tabla_afectada, accion, detalle],
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn listar_auditorias(conn: &Connection, limite: i32) -> Result<Vec<AuditoriaDto>, String> {
        let mut stmt = conn
            .prepare(
                "SELECT id_auditoria, id_usuario, tabla_afectada, accion, detalle, fecha 
             FROM auditoria 
             ORDER BY fecha DESC 
             LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;

        let registros = stmt
            .query_map([limite], |row| {
                Ok(AuditoriaDto {
                    id_auditoria: row.get(0)?,
                    id_usuario: row.get(1)?,
                    tabla_afectada: row.get(2)?,
                    accion: row.get(3)?,
                    detalle: row.get(4)?,
                    fecha: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut resultado = Vec::new();
        for r in registros {
            resultado.push(r.map_err(|e| e.to_string())?);
        }

        Ok(resultado)
    }
}

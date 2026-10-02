use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidatoFidelizacionDto {
    pub id_seguimiento: i32,
    pub placa: String,
    pub fecha_programada: String,
    pub contactado: bool,
    pub observacion: Option<String>,
    pub id_usuario: Option<i32>,
}

pub struct FidelizacionService;

impl FidelizacionService {
    pub fn listar_candidatos(conn: &Connection) -> Result<Vec<CandidatoFidelizacionDto>, String> {
        let mut stmt = conn.prepare(
            "SELECT id_seguimiento, placa, fecha_programada, contactado, observacion, id_usuario 
             FROM seguimientos_fidelizacion 
             WHERE contactado = 0 
             ORDER BY fecha_programada ASC"
        ).map_err(|e| e.to_string())?;

        let candidatos = stmt
            .query_map([], |row| {
                Ok(CandidatoFidelizacionDto {
                    id_seguimiento: row.get(0)?,
                    placa: row.get(1)?,
                    fecha_programada: row.get(2)?,
                    contactado: row.get::<_, i32>(3)? == 1,
                    observacion: row.get(4)?,
                    id_usuario: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut resultado = Vec::new();
        for c in candidatos {
            resultado.push(c.map_err(|e| e.to_string())?);
        }

        Ok(resultado)
    }

    pub fn marcar_contactado(
        conn: &Connection,
        id_seguimiento: i32,
        id_usuario: i32,
        observacion: &str,
    ) -> Result<(), String> {
        conn.execute(
            "UPDATE seguimientos_fidelizacion 
             SET contactado = 1, fecha_contacto = CURRENT_TIMESTAMP, id_usuario = ?1, observacion = ?2 
             WHERE id_seguimiento = ?3",
            params![id_usuario, observacion, id_seguimiento],
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}

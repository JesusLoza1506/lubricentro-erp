use crate::dto::orden_trabajo_dto::{CambiarEstadoOtDto, CrearOrdenTrabajoDto, OrdenTrabajoDto};
use crate::entities::orden_trabajo::OrdenTrabajoEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct OrdenesService;

impl OrdenesService {
    /// Genera un código correlativo único para la OT (ej: OT-000001)
    fn generar_codigo_ot(conn: &Connection) -> Result<String, AppError> {
        let max_id: i64 = conn.query_row(
            "SELECT COALESCE(MAX(id_ot), 0) FROM ordenes_trabajo",
            [],
            |row| row.get(0),
        )?;
        Ok(format!("OT-{:06}", max_id + 1))
    }

    /// Registra una nueva Orden de Trabajo
    pub fn crear_orden_trabajo(
        conn: &Connection,
        req: CrearOrdenTrabajoDto,
    ) -> Result<OrdenTrabajoDto, AppError> {
        let placa = req.placa.trim().to_uppercase();

        if placa.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo es obligatoria.".to_string(),
            ));
        }

        if req.proximo_kilometraje < req.kilometraje_ingreso {
            return Err(AppError::Validation(
                "El próximo kilometraje debe ser mayor o igual al kilometraje de ingreso."
                    .to_string(),
            ));
        }

        // Verificar existencia del vehículo
        let existe_vehiculo: i32 = conn.query_row(
            "SELECT COUNT(*) FROM vehiculos WHERE placa = ?1 AND activo = 1",
            [&placa],
            |row| row.get(0),
        )?;

        if existe_vehiculo == 0 {
            return Err(AppError::NotFound(format!(
                "El vehículo con placa {} no se encuentra registrado.",
                placa
            )));
        }

        // Si se asigna zanja, validar que esté dentro de los valores permitidos (1 o 2) y no esté ocupada
        if let Some(z) = req.zanja {
            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado = 'EN_PROCESO'",
                [z],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya se encuentra ocupada por otra orden en proceso.",
                    z
                )));
            }
        }

        let codigo_ot = Self::generar_codigo_ot(conn)?;

        // Inserción en la base de datos (se activará el trigger trg_valida_rol_mecanico_insert si el id_mecanico no es MECANICO)
        conn.execute(
            "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso)
             VALUES (?1, ?2, ?3, ?4, 'EN_ESPERA', ?5, ?6, ?7, CURRENT_TIMESTAMP)",
            params![
                codigo_ot,
                placa,
                req.id_mecanico,
                req.zanja,
                req.kilometraje_ingreso,
                req.proximo_kilometraje,
                req.observaciones,
            ],
        )?;

        let id_ot = conn.last_insert_rowid();

        // Actualizar el kilometraje actual en la ficha vehicular
        conn.execute(
            "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
            params![req.kilometraje_ingreso, placa],
        )?;

        Self::obtener_ot_por_id(conn, id_ot)
    }

    /// Obtiene una OT por su ID
    pub fn obtener_ot_por_id(conn: &Connection, id_ot: i64) -> Result<OrdenTrabajoDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso
             FROM ordenes_trabajo WHERE id_ot = ?1",
        )?;

        let entity = stmt
            .query_row([id_ot], |row| {
                Ok(OrdenTrabajoEntity {
                    id_ot: row.get(0)?,
                    codigo_ot: row.get(1)?,
                    placa: row.get(2)?,
                    id_mecanico: row.get(3)?,
                    zanja: row.get(4)?,
                    estado: row.get(5)?,
                    kilometraje_ingreso: row.get(6)?,
                    proximo_kilometraje: row.get(7)?,
                    observaciones: row.get(8)?,
                    fecha_ingreso: row.get(9)?,
                })
            })
            .map_err(|_| AppError::NotFound(format!("No se encontró la OT con ID {}", id_ot)))?;

        Ok(OrdenTrabajoDto::from(entity))
    }

    /// Obtiene una OT por su código único (ej: OT-000001)
    pub fn obtener_ot_por_codigo(
        conn: &Connection,
        codigo: &str,
    ) -> Result<OrdenTrabajoDto, AppError> {
        let codigo_clean = codigo.trim().to_uppercase();
        let mut stmt = conn.prepare(
            "SELECT id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso
             FROM ordenes_trabajo WHERE codigo_ot = ?1",
        )?;

        let entity = stmt
            .query_row([&codigo_clean], |row| {
                Ok(OrdenTrabajoEntity {
                    id_ot: row.get(0)?,
                    codigo_ot: row.get(1)?,
                    placa: row.get(2)?,
                    id_mecanico: row.get(3)?,
                    zanja: row.get(4)?,
                    estado: row.get(5)?,
                    kilometraje_ingreso: row.get(6)?,
                    proximo_kilometraje: row.get(7)?,
                    observaciones: row.get(8)?,
                    fecha_ingreso: row.get(9)?,
                })
            })
            .map_err(|_| {
                AppError::NotFound(format!("No se encontró la OT con código {}", codigo_clean))
            })?;

        Ok(OrdenTrabajoDto::from(entity))
    }

    /// Máquina de estados para actualización de OT
    pub fn cambiar_estado_ot(
        conn: &Connection,
        req: CambiarEstadoOtDto,
    ) -> Result<OrdenTrabajoDto, AppError> {
        let ot_actual = Self::obtener_ot_por_id(conn, req.id_ot)?;
        let nuevo_estado = req.nuevo_estado.trim().to_uppercase();

        let estados_validos = ["EN_ESPERA", "EN_PROCESO", "FINALIZADO", "CANCELADO"];
        if !estados_validos.contains(&nuevo_estado.as_str()) {
            return Err(AppError::Validation(format!(
                "Estado inválido: {}. Los estados permitidos son EN_ESPERA, EN_PROCESO, FINALIZADO, CANCELADO.",
                nuevo_estado
            )));
        }

        // Si cambia a EN_PROCESO, requerir zanja y validar disponibilidad
        if nuevo_estado == "EN_PROCESO" {
            let zanja_asignada = req.zanja.or(ot_actual.zanja);
            if zanja_asignada.is_none() {
                return Err(AppError::Validation(
                    "Para pasar a estado EN_PROCESO se debe asignar una zanja (1 o 2).".to_string(),
                ));
            }

            let z = zanja_asignada.unwrap();
            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado = 'EN_PROCESO' AND id_ot != ?2",
                params![z, req.id_ot],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya está ocupada por otra orden en proceso.",
                    z
                )));
            }

            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1, zanja = ?2 WHERE id_ot = ?3",
                params![nuevo_estado, z, req.id_ot],
            )?;
        } else if nuevo_estado == "FINALIZADO" || nuevo_estado == "CANCELADO" {
            // Al finalizar o cancelar, se libera la zanja
            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1, zanja = NULL WHERE id_ot = ?2",
                params![nuevo_estado, req.id_ot],
            )?;
        } else {
            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1 WHERE id_ot = ?2",
                params![nuevo_estado, req.id_ot],
            )?;
        }

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    /// Lista todas las órdenes de trabajo activas o filtradas por estado
    pub fn listar_ordenes_trabajo(
        conn: &Connection,
        filtro_estado: Option<String>,
    ) -> Result<Vec<OrdenTrabajoDto>, AppError> {
        let query = if let Some(ref estado) = filtro_estado {
            format!(
                "SELECT id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso
                 FROM ordenes_trabajo WHERE estado = '{}' ORDER BY id_ot DESC",
                estado.trim().to_uppercase()
            )
        } else {
            "SELECT id_ot, codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso
             FROM ordenes_trabajo ORDER BY id_ot DESC".to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let iter = stmt.query_map([], |row| {
            let entity = OrdenTrabajoEntity {
                id_ot: row.get(0)?,
                codigo_ot: row.get(1)?,
                placa: row.get(2)?,
                id_mecanico: row.get(3)?,
                zanja: row.get(4)?,
                estado: row.get(5)?,
                kilometraje_ingreso: row.get(6)?,
                proximo_kilometraje: row.get(7)?,
                observaciones: row.get(8)?,
                fecha_ingreso: row.get(9)?,
            };
            Ok(OrdenTrabajoDto::from(entity))
        })?;

        let mut ordenes = Vec::new();
        for item in iter {
            ordenes.push(item?);
        }

        Ok(ordenes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    fn setup_mecanico_y_vehiculo(conn: &Connection) -> (i64, String) {
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (1, 'Mecanico Juan', 'mecanico1', 'hash', 'MECANICO', 1)",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, activo)
             VALUES ('DNI', '12345678', 'Cliente Test', 1)",
            [],
        )
        .unwrap();

        let id_cliente = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, kilometraje_actual, activo)
             VALUES ('ABC-123', ?1, 'Toyota', 'Yaris', 45000, 1)",
            [id_cliente],
        )
        .unwrap();

        (1, "ABC-123".to_string())
    }

    #[test]
    fn test_flujo_completo_orden_trabajo_y_zanja() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();
        let (id_mecanico, placa) = setup_mecanico_y_vehiculo(&conn);

        let req = CrearOrdenTrabajoDto {
            placa,
            id_mecanico,
            zanja: Some(1),
            kilometraje_ingreso: 45000,
            proximo_kilometraje: 50000,
            observaciones: Some("Cambio de aceite".to_string()),
        };

        let ot = OrdenesService::crear_orden_trabajo(&conn, req).unwrap();
        assert_eq!(ot.codigo_ot, "OT-000001");
        assert_eq!(ot.estado, "EN_ESPERA");

        // Transición a EN_PROCESO en Zanja 1
        let ot_en_proceso = OrdenesService::cambiar_estado_ot(
            &conn,
            CambiarEstadoOtDto {
                id_ot: ot.id_ot,
                nuevo_estado: "EN_PROCESO".to_string(),
                zanja: Some(1),
            },
        )
        .unwrap();
        assert_eq!(ot_en_proceso.estado, "EN_PROCESO");
        assert_eq!(ot_en_proceso.zanja, Some(1));

        // Transición a FINALIZADO libera la zanja
        let ot_finalizada = OrdenesService::cambiar_estado_ot(
            &conn,
            CambiarEstadoOtDto {
                id_ot: ot.id_ot,
                nuevo_estado: "FINALIZADO".to_string(),
                zanja: None,
            },
        )
        .unwrap();
        assert_eq!(ot_finalizada.estado, "FINALIZADO");
        assert_eq!(ot_finalizada.zanja, None);
    }
}

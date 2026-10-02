use crate::dto::caja_dto::{AbrirCajaDto, CajaChicaDto, CierreCajaDto};
use crate::entities::caja_chica::CajaChicaEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct CajaService;

impl CajaService {
    /// Abre una nueva caja chica para un usuario (requiere que no tenga otra caja abierta)
    pub fn abrir_caja(conn: &Connection, req: AbrirCajaDto) -> Result<i64, AppError> {
        if req.monto_apertura < 0 {
            return Err(AppError::Validation(
                "El monto de apertura no puede ser negativo.".to_string(),
            ));
        }

        // Verificar que el usuario tenga rol CAJERO (según los triggers y esquema)
        let rol: String = conn
            .query_row(
                "SELECT rol FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [req.id_usuario],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Usuario no encontrado o inactivo.".to_string()))?;

        if rol != "CAJERO" && rol != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "El usuario no tiene permisos de cajero para abrir caja.".to_string(),
            ));
        }

        // Validar si ya tiene una caja abierta (el índice único uq_caja_abierta_usuario también lo protege)
        let caja_abierta: i32 = conn.query_row(
            "SELECT COUNT(*) FROM caja_chica WHERE id_usuario = ?1 AND estado = 'ABIERTA'",
            [req.id_usuario],
            |row| row.get(0),
        )?;

        if caja_abierta > 0 {
            return Err(AppError::Validation(
                "El usuario ya cuenta con una caja chica abierta actualmente.".to_string(),
            ));
        }

        conn.execute(
            "INSERT INTO caja_chica (id_usuario, monto_apertura, estado, fecha_apertura)
             VALUES (?1, ?2, 'ABIERTA', CURRENT_TIMESTAMP)",
            params![req.id_usuario, req.monto_apertura],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Realiza el cierre de caja calculando diferencias en céntimos
    pub fn cerrar_caja(conn: &Connection, req: CierreCajaDto) -> Result<CajaChicaDto, AppError> {
        let caja = Self::obtener_caja_por_id(conn, req.id_caja)?;

        if caja.estado != "ABIERTA" {
            return Err(AppError::Validation(
                "La caja especificada ya se encuentra cerrada.".to_string(),
            ));
        }

        // Calcular ventas en efectivo registradas en movimientos_caja para esta caja
        let total_ventas_efectivo: i64 = conn.query_row(
            "SELECT COALESCE(SUM(monto), 0) FROM movimientos_caja WHERE id_caja = ?1 AND tipo_movimiento = 'INGRESO_VENTA'",
            [req.id_caja],
            |row| row.get(0),
        )?;

        let saldo_teorico = caja.monto_apertura + total_ventas_efectivo;
        let diferencia = req.monto_cierre_efectivo - saldo_teorico;

        conn.execute(
            "UPDATE caja_chica 
             SET monto_cierre_efectivo = ?1, monto_cierre_digital = ?2, monto_diferencia = ?3, estado = 'CERRADA', fecha_cierre = CURRENT_TIMESTAMP
             WHERE id_caja = ?4",
            params![
                req.monto_cierre_efectivo,
                req.monto_cierre_digital,
                diferencia,
                req.id_caja
            ],
        )?;

        Self::obtener_caja_por_id(conn, req.id_caja)
    }

    pub fn obtener_caja_por_id(conn: &Connection, id_caja: i64) -> Result<CajaChicaDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_caja, id_usuario, monto_apertura, monto_cierre_efectivo, monto_cierre_digital, monto_diferencia, estado, fecha_apertura, fecha_cierre
             FROM caja_chica WHERE id_caja = ?1",
        )?;

        let entity = stmt
            .query_row([id_caja], |row| {
                Ok(CajaChicaEntity {
                    id_caja: row.get(0)?,
                    id_usuario: row.get(1)?,
                    monto_apertura: row.get(2)?,
                    monto_cierre_efectivo: row.get(3)?,
                    monto_cierre_digital: row.get(4)?,
                    monto_diferencia: row.get(5)?,
                    estado: row.get(6)?,
                    fecha_apertura: row.get(7)?,
                    fecha_cierre: row.get(8)?,
                })
            })
            .map_err(|_| {
                AppError::NotFound(format!("Caja chica con ID {} no encontrada.", id_caja))
            })?;

        Ok(CajaChicaDto {
            id_caja: entity.id_caja,
            id_usuario: entity.id_usuario,
            monto_apertura: entity.monto_apertura,
            monto_cierre_efectivo: entity.monto_cierre_efectivo,
            monto_cierre_digital: entity.monto_cierre_digital,
            monto_diferencia: entity.monto_diferencia,
            estado: entity.estado,
            fecha_apertura: entity.fecha_apertura,
            fecha_cierre: entity.fecha_cierre,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_apertura_y_cierre_caja_centimos() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (1, 'Cajero Test', 'cajero1', 'hash', 'CAJERO', 1)",
            [],
        )
        .unwrap();

        let id_caja = CajaService::abrir_caja(
            &conn,
            AbrirCajaDto {
                id_usuario: 1,
                monto_apertura: 10000, // S/ 100.00 en céntimos
            },
        )
        .unwrap();

        assert!(id_caja > 0);

        let cierre = CajaService::cerrar_caja(
            &conn,
            CierreCajaDto {
                id_caja,
                monto_cierre_efectivo: 10000,
                monto_cierre_digital: 5000,
            },
        )
        .unwrap();

        assert_eq!(cierre.estado, "CERRADA");
        assert_eq!(cierre.monto_diferencia, Some(0));
    }
}

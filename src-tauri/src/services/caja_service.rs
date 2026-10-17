use crate::dto::caja_dto::{AbrirCajaDto, CajaChicaDto, CierreCajaDto};
use crate::entities::caja_chica::CajaChicaEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};
use uuid::Uuid;

pub struct CajaService;

impl CajaService {
    /// Abre una nueva caja chica para un usuario (requiere rol CAJERO o ADMINISTRADOR)
    pub fn abrir_caja(conn: &Connection, req: AbrirCajaDto) -> Result<i64, AppError> {
        if req.monto_apertura < 0 {
            return Err(AppError::Validation(
                "El monto de apertura no puede ser negativo.".to_string(),
            ));
        }

        let rol: String = conn
            .query_row(
                "SELECT rol FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [req.id_usuario],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Usuario no encontrado o inactivo.".to_string()))?;

        if rol != "CAJERO" && rol != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "El usuario no tiene permisos para abrir caja.".to_string(),
            ));
        }

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

        let id_operacion = Uuid::new_v4().to_string();
        let codigo_caja = format!("CAJA-{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        conn.execute(
            "INSERT INTO caja_chica (
                id_operacion, codigo_caja, id_usuario, ambiente, monto_apertura_cent, 
                estado, fecha_apertura
            ) VALUES (?1, ?2, ?3, 'PRODUCCION', ?4, 'ABIERTA', datetime('now', '-5 hours'))",
            params![
                id_operacion,
                codigo_caja,
                req.id_usuario,
                req.monto_apertura,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Realiza el cierre de caja registrando el ID real del usuario ejecutor de la sesión activa
    pub fn cerrar_caja(
        conn: &Connection,
        req: CierreCajaDto,
        id_usuario_cierre: i64,
    ) -> Result<CajaChicaDto, AppError> {
        let caja = Self::obtener_caja_por_id(conn, req.id_caja)?;

        if caja.estado != "ABIERTA" {
            return Err(AppError::Validation(
                "La caja especificada ya se encuentra cerrada.".to_string(),
            ));
        }

        let efectivo_movs: i64 = conn.query_row(
            "SELECT COALESCE(SUM(signo * monto_cent), 0) FROM movimientos_caja WHERE id_caja = ?1 AND medio_pago = 'EFECTIVO'",
            params![req.id_caja],
            |row| row.get(0),
        )?;
        let efectivo_esperado = caja.monto_apertura + efectivo_movs;

        let tarjeta_neto: i64 = conn.query_row(
            "SELECT COALESCE(SUM(signo * monto_cent), 0) FROM movimientos_caja WHERE id_caja = ?1 AND medio_pago = 'TARJETA'",
            params![req.id_caja],
            |row| row.get(0),
        )?;

        let yape_neto: i64 = conn.query_row(
            "SELECT COALESCE(SUM(signo * monto_cent), 0) FROM movimientos_caja WHERE id_caja = ?1 AND medio_pago = 'YAPE'",
            params![req.id_caja],
            |row| row.get(0),
        )?;

        let plin_neto: i64 = conn.query_row(
            "SELECT COALESCE(SUM(signo * monto_cent), 0) FROM movimientos_caja WHERE id_caja = ?1 AND medio_pago = 'PLIN'",
            params![req.id_caja],
            |row| row.get(0),
        )?;

        let transferencia_neto: i64 = conn.query_row(
            "SELECT COALESCE(SUM(signo * monto_cent), 0) FROM movimientos_caja WHERE id_caja = ?1 AND medio_pago = 'TRANSFERENCIA'",
            params![req.id_caja],
            |row| row.get(0),
        )?;

        let id_operacion_cierre = Uuid::new_v4().to_string();
        let retiro_cierre = req.monto_cierre_efectivo;
        let fondo_siguiente = 0;

        conn.execute(
            "UPDATE caja_chica 
             SET estado = 'CERRADA',
                 fecha_cierre = datetime('now', '-5 hours'),
                 id_operacion_cierre = ?1,
                 id_usuario_cierre = ?2,
                 tipo_cierre = 'NORMAL',
                 efectivo_esperado_cent = ?3,
                 efectivo_contado_cent = ?4,
                 tarjeta_neto_cent = ?5,
                 yape_neto_cent = ?6,
                 plin_neto_cent = ?7,
                 transferencia_neto_cent = ?8,
                 fondo_siguiente_cent = ?9,
                 retiro_cierre_cent = ?10
             WHERE id_caja = ?11",
            params![
                id_operacion_cierre,
                id_usuario_cierre, // Identidad real validada
                efectivo_esperado,
                req.monto_cierre_efectivo,
                tarjeta_neto,
                yape_neto,
                plin_neto,
                transferencia_neto,
                fondo_siguiente,
                retiro_cierre,
                req.id_caja
            ],
        )?;

        Self::obtener_caja_por_id(conn, req.id_caja)
    }

    pub fn obtener_caja_por_id(conn: &Connection, id_caja: i64) -> Result<CajaChicaDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_caja, id_usuario, monto_apertura_cent, efectivo_contado_cent, tarjeta_neto_cent, yape_neto_cent, diferencia_efectivo_cent, estado, fecha_apertura, fecha_cierre
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
                    monto_diferencia: row.get(6)?,
                    estado: row.get(7)?,
                    fecha_apertura: row.get(8)?,
                    fecha_cierre: row.get(9)?,
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

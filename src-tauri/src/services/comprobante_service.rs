use crate::dto::comprobante_dto::{ComprobanteDto, EmitirComprobanteDto};
use crate::entities::comprobante::ComprobanteEntity;
use crate::errors::AppError;
use rusqlite::Connection;

pub struct ComprobanteService;

impl ComprobanteService {
    /// Emite un comprobante, actualiza la serie/correlativo y registra el pago
    /// Exige estrictamente que la caja abierta pertenezca al usuario ejecutor (incluso si es Administrador)
    pub fn emitir_comprobante(
        conn: &Connection,
        req: EmitirComprobanteDto,
    ) -> Result<ComprobanteDto, AppError> {
        let propio: bool = conn.query_row("SELECT EXISTS(SELECT 1 FROM caja_chica WHERE id_caja=?1 AND id_usuario=?2 AND estado='ABIERTA')", rusqlite::params![req.id_caja,req.id_cajero], |r| r.get(0))?;
        if !propio {
            return Err(AppError::AccessDenied("No tiene un turno de caja propio abierto. Debe abrir su propia caja para registrar cobros.".into()));
        }
        Err(AppError::Validation(
            "La emisión aislada anterior fue sustituida por la confirmación atómica del POS."
                .into(),
        ))
    }

    pub fn obtener_comprobante_por_id(
        conn: &Connection,
        id_comprobante: i64,
    ) -> Result<ComprobanteDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT c.id_comprobante, c.tipo_comprobante, s.serie, c.correlativo, c.base_cent, c.igv_cent, c.total_cent, c.fecha_emision
             FROM comprobantes c
             LEFT JOIN series_comprobante s ON c.id_serie = s.id_serie
             WHERE c.id_comprobante = ?1",
        )?;

        let entity = stmt
            .query_row([id_comprobante], |row| {
                Ok((
                    ComprobanteEntity {
                        id_comprobante: row.get(0)?,
                        tipo_comprobante: row.get(1)?,
                        id_serie: None,
                        correlativo: row.get(3)?,
                        monto_subtotal: row.get(4)?,
                        monto_igv: row.get(5)?,
                        monto_total: row.get(6)?,
                        estado_sunat: "EMITIDO".to_string(),
                        fecha_emision: row.get(7)?,
                        id_ot: None,
                        id_cliente: 0,
                        id_cajero: 0,
                        id_caja: 0,
                    },
                    row.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(|_| {
                AppError::NotFound(format!("Comprobante ID {} no encontrado.", id_comprobante))
            })?;

        Ok(ComprobanteDto {
            id_comprobante: entity.0.id_comprobante,
            tipo_comprobante: entity.0.tipo_comprobante,
            serie: entity.1,
            correlativo: entity.0.correlativo,
            monto_subtotal: entity.0.monto_subtotal,
            monto_igv: entity.0.monto_igv,
            monto_total: entity.0.monto_total,
            estado_sunat: entity.0.estado_sunat,
            fecha_emision: entity.0.fecha_emision,
        })
    }
}

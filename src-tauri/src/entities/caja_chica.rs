use rusqlite::{Result, Row};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CajaChicaEntity {
    pub id_caja: i64,
    pub codigo_caja: String,
    pub id_usuario: i64,
    pub titular: String,
    pub ambiente: String,
    pub monto_apertura_cent: i64,
    pub estado: String,
    pub por_conciliar: bool,
    pub fecha_apertura: String,
    pub fecha_cierre: Option<String>,
    pub id_usuario_cierre: Option<i64>,
    pub quien_cerro: Option<String>,
    pub tipo_cierre: Option<String>,
    pub motivo_cierre: Option<String>,
    pub efectivo_esperado_cent: Option<i64>,
    pub efectivo_contado_cent: Option<i64>,
    pub diferencia_efectivo_cent: Option<i64>,
    pub tarjeta_neto_cent: Option<i64>,
    pub yape_neto_cent: Option<i64>,
    pub plin_neto_cent: Option<i64>,
    pub transferencia_neto_cent: Option<i64>,
    pub aclaracion: Option<String>,
    pub id_usuario_aclaracion: Option<i64>,
    pub autor_aclaracion: Option<String>,
    pub fecha_aclaracion: Option<String>,
    pub fondo_siguiente_cent: Option<i64>,
    pub retiro_cierre_cent: Option<i64>,
    pub version: i64,
}

impl CajaChicaEntity {
    pub(crate) fn desde_fila(r: &Row<'_>) -> Result<Self> {
        Ok(Self {
            id_caja: r.get("id_caja")?,
            codigo_caja: r.get("codigo_caja")?,
            id_usuario: r.get("id_usuario")?,
            titular: r.get("titular")?,
            ambiente: r.get("ambiente")?,
            monto_apertura_cent: r.get("monto_apertura_cent")?,
            estado: r.get("estado")?,
            por_conciliar: r.get("por_conciliar")?,
            fecha_apertura: r.get("fecha_apertura")?,
            fecha_cierre: r.get("fecha_cierre")?,
            id_usuario_cierre: r.get("id_usuario_cierre")?,
            quien_cerro: r.get("quien_cerro")?,
            tipo_cierre: r.get("tipo_cierre")?,
            motivo_cierre: r.get("motivo_cierre")?,
            efectivo_esperado_cent: r.get("efectivo_esperado_cent")?,
            efectivo_contado_cent: r.get("efectivo_contado_cent")?,
            diferencia_efectivo_cent: r.get("diferencia_efectivo_cent")?,
            tarjeta_neto_cent: r.get("tarjeta_neto_cent")?,
            yape_neto_cent: r.get("yape_neto_cent")?,
            plin_neto_cent: r.get("plin_neto_cent")?,
            transferencia_neto_cent: r.get("transferencia_neto_cent")?,
            aclaracion: r.get("aclaracion")?,
            id_usuario_aclaracion: r.get("id_usuario_aclaracion")?,
            autor_aclaracion: r.get("autor_aclaracion")?,
            fecha_aclaracion: r.get("fecha_aclaracion")?,
            fondo_siguiente_cent: r.get("fondo_siguiente_cent")?,
            retiro_cierre_cent: r.get("retiro_cierre_cent")?,
            version: r.get("version")?,
        })
    }
}

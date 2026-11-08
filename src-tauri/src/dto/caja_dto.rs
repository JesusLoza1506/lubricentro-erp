use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbrirCajaDto {
    pub id_operacion: String,
    pub monto_apertura_cent: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CierreCajaDto {
    pub id_operacion: String,
    pub id_caja: i64,
    pub version_esperada: i64,
    pub efectivo_contado_cent: i64,
    pub fondo_siguiente_cent: i64,
    pub motivo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManualCajaDto {
    pub id_operacion: String,
    pub id_caja: i64,
    pub tipo: String,
    pub categoria: String,
    pub concepto: String,
    pub monto_cent: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevertirCajaDto {
    pub id_operacion: String,
    pub id_movimiento: i64,
    pub motivo: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AclararCajaDto {
    pub id_operacion: String,
    pub id_caja: i64,
    pub aclaracion: String,
}

#[derive(Debug, Deserialize)]
pub struct FiltroMovimientosCaja {
    pub id_caja: i64,
    pub pagina: Option<i64>,
    pub origen: Option<String>,
    pub direccion: Option<String>,
    pub buscar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FiltroHistorialCaja {
    pub pagina: Option<i64>,
    pub desde: Option<String>,
    pub hasta: Option<String>,
    pub codigo: Option<String>,
    pub con_diferencia: Option<bool>,
    pub id_usuario: Option<i64>,
    pub estado: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct ResumenMedioCaja {
    pub cobros_cent: i64,
    pub reintegros_cent: i64,
    pub neto_cent: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct ResumenCaja {
    pub efectivo_esperado_cent: i64,
    pub efectivo: ResumenMedioCaja,
    pub tarjeta: ResumenMedioCaja,
    pub yape: ResumenMedioCaja,
    pub plin: ResumenMedioCaja,
    pub transferencia: ResumenMedioCaja,
    pub ingresos_manuales_cent: i64,
    pub egresos_manuales_cent: i64,
}

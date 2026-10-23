use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdenTrabajoEntity {
    pub id_ot: i64,
    pub id_operacion: String,
    pub codigo_ot: String,
    pub id_vehiculo: i64,
    pub id_cliente_atendido: i64,
    pub placa_snapshot: String,
    pub cliente_snapshot: String,
    pub id_creador: i64,
    pub id_mecanico: Option<i64>,
    pub id_mecanico_terminal: Option<i64>,
    pub id_zanja_ejecucion: Option<i32>,
    pub estado: String, // "EN_ESPERA" | "EN_PROCESO" | "FINALIZADO" | "CANCELADO"
    pub kilometraje_ingreso: i64,
    pub kilometraje_salida: Option<i64>,
    pub pedido_cliente: Option<String>,
    pub resumen_publico: Option<String>,
    pub nota_interna: Option<String>,
    pub intervalo_aceite_km: Option<i32>,
    pub km_proximo: Option<i64>,
    pub corregido: i32,
    pub tipo_aceite: Option<String>, // "MINERAL" | "SINTETICO"
    pub fecha_ingreso: String,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub fecha_cancelacion: Option<String>,
    pub motivo_cancelacion: Option<String>,
    pub id_usuario_cancelacion: Option<i64>,
    pub version: i64,
}

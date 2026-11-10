use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DashboardDto {
    pub ambiente: String,
    pub fecha: String,
    pub actualizado_en: String,
    #[serde(flatten)]
    pub datos: DatosDashboard,
}

#[derive(Debug, Serialize)]
#[serde(tag = "rol")]
pub enum DatosDashboard {
    #[serde(rename = "ADMINISTRADOR")]
    Administrador {
        ventas_netas_hoy_cent: i64,
        trabajos_terminados_hoy: i64,
        mediana_cobro_seg: Option<i64>,
        ventas_por_dia: Vec<PuntoDashboard>,
        trabajos_por_hora: Vec<PuntoDashboard>,
    },
    #[serde(rename = "MECANICO")]
    Mecanico {
        trabajos_terminados_hoy: i64,
        trabajos_abiertos: i64,
        mediana_trabajo_seg: Option<i64>,
        por_estado: EstadosTrabajo,
        trabajos_por_dia: Vec<PuntoDashboard>,
    },
    #[serde(rename = "CAJERO")]
    Cajero {
        turno: TurnoDashboard,
        actividad_turno: Option<ActividadTurno>,
        pendientes: PendientesDashboard,
    },
}

#[derive(Debug, Serialize)]
pub struct PuntoDashboard {
    pub periodo: String,
    pub valor: i64,
}

#[derive(Debug, Serialize)]
pub struct EstadosTrabajo {
    pub en_espera: i64,
    pub en_proceso: i64,
    pub finalizadas_hoy: i64,
}

#[derive(Debug, Serialize)]
#[serde(tag = "estado", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TurnoDashboard {
    Propio { apertura: String },
    Ajeno,
    SinCaja,
}

#[derive(Debug, Serialize)]
pub struct ActividadTurno {
    pub cobros: i64,
    pub cobros_por_hora: Vec<PuntoDashboard>,
}

#[derive(Debug, Serialize)]
pub struct PendientesDashboard {
    pub total: i64,
    pub mayor_espera_seg: Option<i64>,
    pub menos_30: i64,
    pub entre_30_y_60: i64,
    pub mas_60: i64,
}

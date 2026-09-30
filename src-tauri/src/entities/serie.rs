#[derive(Debug, Clone, PartialEq)]
pub struct SerieEntity {
    pub id_serie: i32,
    pub tipo_comprobante: String,
    pub serie: String,
    pub correlativo_actual: i32,
    pub activa: bool,
}

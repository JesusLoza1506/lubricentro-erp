#[derive(Debug, Clone, PartialEq)]
pub struct OrdenTrabajoEntity {
    pub id_ot: i32,
    pub codigo_ot: String,
    pub placa: String,
    pub id_mecanico: i32,
    pub zanja: Option<i32>,
    pub estado: String,
    pub kilometraje_ingreso: i32,
    pub proximo_kilometraje: i32,
    pub fecha_ingreso: Option<String>,
}

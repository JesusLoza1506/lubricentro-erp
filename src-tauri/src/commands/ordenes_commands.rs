use crate::dto::orden_trabajo_dto::OrdenTrabajoDTO;

#[tauri::command]
pub async fn obtener_ot_por_codigo_cmd(codigo: String) -> Result<OrdenTrabajoDTO, String> {
    Ok(OrdenTrabajoDTO {
        id_ot: 1,
        codigo_ot: codigo,
        placa: "ABC-123".to_string(),
        id_mecanico: 2,
        zanja: Some(1),
        estado: "EN_PROCESO".to_string(),
        kilometraje_ingreso: 45000,
        proximo_kilometraje: 50000,
        fecha_ingreso: Some("2026-09-29 10:00:00".to_string()),
    })
}

use crate::dto::cliente_dto::ClienteDTO;

#[tauri::command]
pub async fn buscar_cliente_por_doc_cmd(doc: String) -> Result<Option<ClienteDTO>, String> {
    Ok(Some(ClienteDTO {
        id_cliente: 1,
        tipo_documento: "RUC".to_string(),
        numero_documento: doc,
        nombre_razon_social: "NUBEFACT SA".to_string(),
        telefono: Some("987654321".to_string()),
        direccion: Some("AV. LIBERTAD 123".to_string()),
    }))
}

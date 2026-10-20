use crate::errors::AppError;

pub fn sanitizar_placa(placa: &str) -> String {
    placa.replace(['-', ' '], "").trim().to_uppercase()
}

pub fn sanitizar_telefono(telefono: Option<&String>) -> Option<String> {
    telefono.and_then(|t| {
        let clean = t.trim().replace([' ', '-'], "");
        let sin_prefijo = clean.strip_prefix("+51").unwrap_or(&clean);
        if sin_prefijo.is_empty() {
            None
        } else {
            Some(sin_prefijo.to_string())
        }
    })
}

pub fn validar_placa(placa_clean: &str) -> Result<(), AppError> {
    if placa_clean.len() != 6 || !placa_clean.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AppError::Validation(
            "La placa debe contener exactamente 6 caracteres alfanuméricos (ej: ABC123)."
                .to_string(),
        ));
    }
    Ok(())
}

pub fn validar_documento(tipo: &str, doc: &str) -> Result<(), AppError> {
    let t = tipo.trim().to_uppercase();
    let d = doc.trim();

    match t.as_str() {
        "DNI" => {
            if d.len() != 8 || !d.chars().all(|c| c.is_ascii_digit()) {
                return Err(AppError::Validation(
                    "DNI inválido: debe tener exactamente 8 dígitos numéricos.".to_string(),
                ));
            }
        }
        "RUC" => {
            if d.len() != 11 || !d.chars().all(|c| c.is_ascii_digit()) {
                return Err(AppError::Validation(
                    "RUC inválido: debe tener exactamente 11 dígitos numéricos.".to_string(),
                ));
            }
        }
        "CE" => {
            if d.len() < 6 || d.len() > 12 || !d.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(AppError::Validation(
                    "Carné de Extranjería (CE) inválido: debe tener entre 6 y 12 caracteres alfanuméricos.".to_string(),
                ));
            }
        }
        _ => {
            return Err(AppError::Validation(
                "Tipo de documento no soportado. Elija DNI, RUC o CE.".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validar_telefono(telefono: Option<&String>) -> Result<(), AppError> {
    if let Some(t) = telefono {
        let clean = t.trim().replace([' ', '-'], "");
        if !clean.is_empty() {
            let sin_prefijo = clean.strip_prefix("+51").unwrap_or(&clean);
            if (sin_prefijo.len() != 7 && sin_prefijo.len() != 9)
                || !sin_prefijo.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::Validation(
                    "El teléfono debe contener 7 (fijo) o 9 (móvil) dígitos numéricos.".to_string(),
                ));
            }
        }
    }
    Ok(())
}

pub fn validar_motivo(motivo: &str) -> Result<String, AppError> {
    let m = motivo.trim();
    if m.len() < 5 || m.len() > 250 {
        return Err(AppError::Validation(
            "El motivo de la acción es obligatorio y debe contener entre 5 y 250 caracteres."
                .to_string(),
        ));
    }
    Ok(m.to_string())
}

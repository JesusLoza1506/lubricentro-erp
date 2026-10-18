use lubricentro_erp_lib::{errors::AppError, services::session_manager::SessionState};
use std::time::{Duration, Instant};
#[test] fn serialized_contract_has_single_prefix() {
    assert_eq!(serde_json::to_value(AppError::NoAutenticado).unwrap().as_str().unwrap(), "NO_AUTENTICADO: Debes iniciar sesión para realizar esta acción.");
}
#[test] fn fresh_interaction_after_14m30s_resets_inactivity() {
    let mut session = SessionState::new();
    session.abrir_sesion(1, 1, "admin".into(), "ADMINISTRADOR".into(), false);
    session.ultima_actividad = Instant::now() - Duration::from_secs(870);
    // AuthContext sends this exact delta for an immediate event at 14m30s.
    session.renovar_actividad_con_delta(870000);
    assert!(session.ultima_actividad.elapsed() < Duration::from_secs(1), "A new immediate interaction remains dated 870 seconds in the past");
}

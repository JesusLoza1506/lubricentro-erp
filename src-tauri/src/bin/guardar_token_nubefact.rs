//! Utilidad local: recibe el token únicamente por stdin y lo guarda en el almacén del SO.
//! No consulta NubeFact, no abre SQLite y no imprime el token.
use std::io::{self, Read};
fn main() {
    if let Err(e) = guardar() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn guardar() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2
        || !["PRUEBA", "PRODUCCION"].contains(&args[0].as_str())
        || args[1].is_empty()
        || args[1].len() > 128
    {
        return Err(
            "Uso: guardar_token_nubefact PRUEBA|PRODUCCION referencia. Envía el token por stdin."
                .into(),
        );
    }
    let mut token = String::new();
    io::stdin()
        .read_to_string(&mut token)
        .map_err(|_| "No se pudo leer el token por stdin.")?;
    let token = token.trim();
    if token.is_empty() || token.contains(['\n', '\r']) {
        return Err("Token vacío o inválido.".into());
    }
    keyring::Entry::new(&format!("lubricentro_erp_nubefact_{}", args[0]), &args[1])
        .map_err(|_| "No se pudo abrir el almacén de credenciales.")?
        .set_password(token)
        .map_err(|_| "No se pudo guardar el token protegido.")?;
    println!("Token guardado en el almacén de credenciales del ambiente indicado. No se realizó ningún envío.");
    Ok(())
}

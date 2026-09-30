fn main() {
    // Cambia "123456" por la contraseña que quieras encriptar
    let password = "123456";

    match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(hash) => {
            println!("\n==================================================");
            println!(" CONTRASEÑA ENCRIPTADA EXITOSAMENTE:");
            println!("--------------------------------------------------");
            println!("{}", hash);
            println!("==================================================\n");
        }
        Err(e) => eprintln!("Error al generar el hash: {}", e),
    }
}

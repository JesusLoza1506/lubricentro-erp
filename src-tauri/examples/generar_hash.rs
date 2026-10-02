
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "123456";
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => {
            println!("\n====================================================");
            println!("   HASH GENERADO EXITOSAMENTE PARA: '{}'", password);
            println!("====================================================");
            println!("{}\n", hashed);
        }
        Err(e) => println!("Error al generar hash: {}", e),
    }
}
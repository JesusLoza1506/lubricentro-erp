use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    let password = b"q123456";

    // Generar un Salt seguro
    let salt = SaltString::generate(&mut OsRng);

    // Instanciar Argon2id
    let argon2 = Argon2::default();

    // Generar el hash
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error al generar el hash")
        .to_string();

    println!("\n=== HASH GENERADO ===");
    println!("{}", password_hash);
    println!("=====================\n");
}

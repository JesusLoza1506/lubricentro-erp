pub mod auditoria;
pub mod auth;
pub mod caja;
pub mod comprobantes;
pub mod db;
pub mod fidelizacion;
pub mod inventario;
pub mod nubefact;
pub mod ordenes;

// --- Nuevas Capas (Fase 3.5) ---
pub mod commands;
pub mod dto;
pub mod entities;
pub mod services;

use commands::{
    auth_commands::*, caja_commands::*, cliente_commands::*, comprobante_commands::*,
    inventario_commands::*, ordenes_commands::*,
};
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Carga las variables de entorno desde el archivo .env si está presente
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Fallo al obtener el directorio de la aplicación");

            tracing::info!("📂 Directorio de Datos de la App (Ruta SQLite): {:?}", app_data_dir);

            let mut conn = crate::db::init_db(app_data_dir.clone()).expect("Error al inicializar la base de datos");

            {
                let tx = conn.transaction().expect("Error al iniciar transacción");

                let count: i32 = tx
                    .query_row("SELECT COUNT(*) FROM comprobantes", [], |row| row.get(0))
                    .unwrap_or(0);

                if count == 0 {
                    // Hash Bcrypt real correspondiente a la contraseña 'admin123'
                    let hash_admin = "$2b$12$e8M3yWJ.E/bZ9mO1W0G5e.XGvU12z71F3N5I8lM0N1P2Q3R4S5T6U";

                    tx.execute(
                        "INSERT OR IGNORE INTO usuarios (id_usuario, nombre_completo, usuario, password_hash, rol) 
                         VALUES (1, 'Administrador General', 'admin', ?1, 'ADMINISTRADOR')",
                        [hash_admin],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT OR IGNORE INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, direccion) 
                         VALUES (1, 'RUC', '20600695771', 'NUBEFACT SA', 'CALLE LIBERTAD 116 MIRAFLORES - LIMA')",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT OR IGNORE INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) 
                         VALUES (1, '01', 'FFF1', 68, 1)",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT OR IGNORE INTO comprobantes (id_comprobante, id_cliente, id_cajero, tipo_comprobante, id_serie, correlativo, monto_subtotal, monto_igv, monto_total, medio_pago, estado_sunat) 
                         VALUES (1, 1, 1, '01', 1, 68, 600.0, 108.0, 708.0, 'Efectivo', 'PENDIENTE')",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT OR IGNORE INTO cola_envio_sunat (id_cola, id_comprobante, estado_envio, intentos) 
                         VALUES (1, 1, 'PENDIENTE', 0)",
                        [],
                    )
                    .unwrap();

                    tx.commit().expect("Error al hacer commit de los datos iniciales");
                    tracing::info!("🌱 [SEEDING] Datos iniciales reales inyectados correctamente en la BD.");
                }
            }

            // Administramos la conexión a la base de datos globalmente para los comandos de Tauri
            app.manage(Mutex::new(conn));

            tauri::async_runtime::spawn(async move {
                crate::nubefact::iniciar_worker_nubefact(app_data_dir).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            obtener_usuario_sesion_cmd,
            crear_usuario_cmd,
            buscar_cliente_por_doc_cmd,
            listar_productos_publicos_cmd,
            obtener_ot_por_codigo_cmd,
            emitir_comprobante_cmd,
            cerrar_caja_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

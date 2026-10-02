pub mod commands;
pub mod db;
pub mod dto;
pub mod entities;
pub mod errors;
pub mod services;

use commands::{
    auth_commands::*, caja_commands::*, cliente_commands::*, comprobante_commands::*,
    dashboard_commands::*, inventario_commands::*, ordenes_commands::*,
};
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Fallo al obtener el directorio de la aplicación");

            tracing::info!("📂 Directorio de Datos de la App (SQLite V3): {:?}", app_data_dir);

            let conn = crate::db::init_db(app_data_dir.clone())
                .expect("Error al inicializar la base de datos");

            // --- Seed de Usuario Administrador Inicial ---
            {
                let count: i32 = conn
                    .query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))
                    .unwrap_or(0);

                if count == 0 {
                    let hash_admin = bcrypt::hash("admin123", 4).expect("Error al generar hash");

                    conn.execute(
                        "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo) 
                         VALUES (1, 'Administrador General', 'admin', ?1, 'ADMINISTRADOR', 1)",
                        [&hash_admin],
                    )
                    .expect("Error al insertar usuario admin inicial");

                    tracing::info!("🌱 [SEEDING] Usuario 'admin' creado exitosamente.");
                }
            }

            app.manage(Mutex::new(conn));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Auth & Usuario
            login_cmd,
            obtener_usuario_sesion_cmd,
            crear_usuario_cmd,
            // Clientes & Vehículos (Fase 3)
            registrar_cliente_cmd,
            buscar_cliente_por_doc_cmd,
            registrar_vehiculo_cmd,
            listar_vehiculos_por_cliente_cmd,
            // Inventario (Fase 2)
            listar_productos_cmd,
            listar_productos_publicos_cmd,
            registrar_producto_cmd,
            actualizar_producto_cmd,
            eliminar_producto_cmd,
            listar_categorias_cmd,
            // Órdenes de Trabajo (Fase 3)
            crear_orden_trabajo_cmd,
            obtener_ot_por_codigo_cmd,
            cambiar_estado_ot_cmd,
            listar_ordenes_trabajo_cmd,
            // Caja Chica & POS (Fase 4)
            abrir_caja_cmd,
            cerrar_caja_cmd,
            emitir_comprobante_cmd,
            // Dashboard, Fidelización & Backups (Fase 5)
            obtener_dashboard_cmd,
            listar_alertas_fidelizacion_cmd,
            generar_backup_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}

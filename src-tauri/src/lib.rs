pub mod auditoria;
pub mod auth;
pub mod caja;
pub mod comprobantes;
pub mod db;
pub mod fidelizacion;
pub mod inventario;
pub mod nubefact;
pub mod ordenes;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Fallo al obtener el directorio de la aplicación");

            tracing::info!("📂 Directorio de Datos de la App (Ruta SQLite): {:?}", app_data_dir);

            if let Ok(mut conn) = crate::db::init_db(app_data_dir.clone()) {
                let tx = conn.transaction().expect("Error al iniciar transacción");

                let count: i32 = tx
                    .query_row("SELECT COUNT(*) FROM comprobantes", [], |row| row.get(0))
                    .unwrap_or(0);

                if count == 0 {
                    tx.execute(
                        "INSERT INTO usuarios (id_usuario, nombre_completo, usuario, password_hash, rol) VALUES (1, 'Administrador General', 'admin', 'hash_seguro', 'ADMINISTRADOR')",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT INTO clientes (id_cliente, tipo_documento, numero_documento, nombre_razon_social, direccion) VALUES (1, 'RUC', '20600695771', 'NUBEFACT SA', 'CALLE LIBERTAD 116 MIRAFLORES - LIMA')",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT INTO series_comprobante (id_serie, tipo_comprobante, serie, correlativo_actual, activa) VALUES (1, '01', 'FFF1', 68, 1)",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT INTO comprobantes (id_comprobante, id_cliente, id_cajero, tipo_comprobante, id_serie, correlativo, monto_subtotal, monto_igv, monto_total, medio_pago, estado_sunat) VALUES (1, 1, 1, '01', 1, 68, 600.0, 108.0, 708.0, 'Efectivo', 'PENDIENTE')",
                        [],
                    )
                    .unwrap();

                    tx.execute(
                        "INSERT INTO cola_envio_sunat (id_cola, id_comprobante, estado_envio, intentos) VALUES (1, 1, 'PENDIENTE', 0)",
                        [],
                    )
                    .unwrap();

                    tx.commit().expect("Error al hacer commit de los datos iniciales");
                    tracing::info!("🌱 [SEEDING] Datos iniciales reales inyectados correctamente en la BD.");
                }
            }

            tauri::async_runtime::spawn(async move {
                crate::nubefact::iniciar_worker_nubefact(app_data_dir).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

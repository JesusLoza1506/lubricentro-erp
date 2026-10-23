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
use services::session_manager::SessionState;
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

            tracing::info!(
                "📂 Directorio de Datos de la App (SQLite V7.0): {:?}",
                app_data_dir
            );

            let conn = crate::db::init_db(app_data_dir.clone())
                .expect("Error al inicializar la base de datos");

            let _ = conn.execute_batch(
                "
                PRAGMA default_cache_size = 10000;
                ",
            );

            app.manage(Mutex::new(conn));
            app.manage(Mutex::new(SessionState::new()));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Auth & Sesión V7.0
            obtener_ambiente_inicial,
            requiere_instalacion,
            login_usuario,
            instalacion_inicial,
            recuperar_acceso_admin,
            obtener_sesion,
            cambiar_clave_obligatoria,
            cambiar_clave_voluntaria,
            registrar_actividad,
            cerrar_sesion,
            reautenticar,
            autorizar_descuento,
            // Clientes & Vehículos V7.0 (Módulo 03)
            registrar_cliente_cmd,
            actualizar_cliente_cmd,
            buscar_cliente_por_doc_cmd,
            buscar_clientes_por_termino_cmd,
            registrar_vehiculo_cmd,
            actualizar_vehiculo_cmd,
            cambiar_estado_vehiculo_cmd,
            corregir_historial_ot_cmd,
            obtener_vehiculo_por_placa_cmd,
            obtener_ficha_vehicular_completa_cmd,
            obtener_proximos_cambios_aceite_cmd,
            // Inventario
            listar_productos_cmd,
            listar_productos_publicos_cmd,
            listar_productos_criticos_cmd,
            registrar_producto_cmd,
            actualizar_producto_cmd,
            eliminar_producto_cmd,
            despachar_producto_granel_cmd,
            ajustar_stock_cmd,
            listar_categorias_cmd,
            // Órdenes de Trabajo V7.0
            obtener_tablero_zanjas_cmd,
            listar_ordenes_trabajo_cmd,
            crear_orden_trabajo_cmd,
            iniciar_trabajo_ot_cmd,
            reasignar_mecanico_ot_cmd,
            finalizar_ot_cmd,
            cancelar_ot_cmd,
            registrar_insumo_ot_cmd,
            devolver_insumo_ot_cmd,
            agregar_servicio_ot_cmd,
            modificar_servicio_ot_cmd,
            eliminar_servicio_ot_cmd,
            obtener_historial_por_placa_cmd,
            listar_mecanicos_cmd,
            listar_servicios_cmd,
            // Caja Chica & POS
            abrir_caja_cmd,
            cerrar_caja_cmd,
            emitir_comprobante_cmd,
            // Dashboard, Fidelización & Backups
            obtener_dashboard_cmd,
            listar_alertas_fidelizacion_cmd,
            generar_backup_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}

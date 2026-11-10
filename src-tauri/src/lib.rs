pub mod commands;
pub mod db;
pub mod dto;
pub mod entities;
pub mod errors;
pub mod nubefact;
pub mod services;

use commands::{
    admin_commands::*, auth_commands::*, backup_commands::*, caja_commands::*, cliente_commands::*,
    comprobante_commands::*, dashboard_commands::*, inventario_commands::*, ordenes_commands::*,
    pos_commands::*, reportes_commands::*,
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

            use services::admin_service::ambiente;
            let seleccionado = ambiente::preferido(&app_data_dir)?;
            let app_data_dir = ambiente::directorio(&app_data_dir, &seleccionado)?;
            let backup = services::backup_service::BackupState::new(&app_data_dir)?;
            let arranque = ambiente::abrir(&app_data_dir, &seleccionado, &backup).inspect(|conn| {
                let _ = conn.execute_batch("PRAGMA default_cache_size = 10000;");
            });
            let bloqueo = match arranque {
                Ok(conn) => {
                    app.manage(Mutex::new(conn));
                    None
                }
                Err(e) => {
                    tracing::error!("Arranque de la base bloqueado: {e}");
                    Some(e.to_string())
                }
            };
            app.manage(EstadoArranque::new(seleccionado, bloqueo));
            app.manage(backup);
            app.manage(services::reportes_service::EstadoReportes::default());
            app.manage(Mutex::new(SessionState::new()));

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    let app = handle.clone();
                    let _ = tauri::async_runtime::spawn_blocking(move || {
                        let b = app.state::<services::backup_service::BackupState>();
                        if let Some(db) = app.try_state::<Mutex<rusqlite::Connection>>() {
                            let _ = b.automatico(&db);
                        }
                    })
                    .await;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window
                    .try_state::<services::backup_service::BackupState>()
                    .is_some_and(|b| b.cierre_bloqueado())
                {
                    api.prevent_close();
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            admin_operacion_cmd,
            admin_cambiar_ambiente_cmd,
            pos_operacion_cmd,
            pos_archivo_local_cmd,
            crate::nubefact::pos_documento_cmd,
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
            buscar_vehiculos_candidatos_cmd,
            // Inventario, Proveedores, Ingresos & Servicios V7.0 (Módulo 05 - FASE 4)
            listar_productos_cmd,
            proponer_codigo_inventario_cmd,
            listar_productos_admin_paginado_cmd,
            listar_productos_publicos_cmd,
            listar_productos_publicos_paginado_cmd,
            listar_productos_criticos_cmd,
            registrar_producto_cmd,
            actualizar_producto_cmd,
            eliminar_producto_cmd,
            reactivar_producto_cmd,
            despachar_producto_granel_cmd,
            ajustar_stock_cmd,
            procesar_conteo_fisico_cmd,
            listar_kardex_producto_cmd,
            obtener_origen_kardex_cmd,
            obtener_resumen_saldos_kardex_cmd,
            listar_categorias_cmd,
            registrar_ingreso_compra_cmd,
            listar_ingresos_stock_cmd,
            anular_ingreso_compra_cmd,
            listar_proveedores_cmd,
            crear_proveedor_cmd,
            actualizar_proveedor_cmd,
            listar_servicios_admin_cmd,
            listar_servicios_publicos_cmd,
            crear_servicio_cmd,
            actualizar_servicio_cmd,
            cambiar_estado_servicio_cmd,
            // Órdenes de Trabajo V7.0
            obtener_ot_por_id_cmd,
            obtener_tablero_zanjas_cmd,
            listar_ordenes_trabajo_cmd,
            listar_ordenes_trabajo_paginado_cmd,
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
            modificar_nota_interna_ot_cmd,
            obtener_historial_por_placa_cmd,
            listar_mecanicos_cmd,
            listar_servicios_cmd,
            // Caja Chica & POS
            caja_operacion_cmd,
            emitir_comprobante_cmd,
            // Dashboard, Fidelización & Backups
            obtener_dashboard_cmd,
            listar_alertas_fidelizacion_cmd,
            backup_arranque_cmd,
            backup_recuperar_arranque_cmd,
            backup_progreso_cmd,
            backup_contexto_cmd,
            backup_elegir_cmd,
            backup_operacion_cmd,
            reportes_operacion_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}

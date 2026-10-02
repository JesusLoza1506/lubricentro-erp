use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
use crate::errors::AppError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub struct DashboardService;

impl DashboardService {
    /// Obtiene las métricas del dashboard filtrando información sensible según el rol del usuario
    pub fn obtener_dashboard(conn: &Connection, id_usuario: i64) -> Result<DashboardDto, AppError> {
        let rol: String = conn
            .query_row(
                "SELECT rol FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [id_usuario],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Usuario no encontrado o inactivo.".to_string()))?;

        // Órdenes en proceso (visible para todos)
        let ot_en_proceso: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ordenes_trabajo WHERE estado = 'EN_PROCESO'",
            [],
            |row| row.get(0),
        )?;

        // Productos con stock bajo o igual al mínimo
        let productos_stock_bajo: i64 = conn.query_row(
            "SELECT COUNT(*) FROM productos WHERE stock_actual <= stock_minimo AND activo = 1",
            [],
            |row| row.get(0),
        )?;

        // Alertas de fidelización (vehículos cuyo kilometraje actual >= próximo kilometraje de su última OT)
        let alertas_fidelizacion: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT v.placa) 
             FROM vehiculos v
             JOIN ordenes_trabajo ot ON v.placa = ot.placa
             WHERE v.kilometraje_actual >= ot.proximo_kilometraje AND ot.estado = 'FINALIZADO'",
            [],
            |row| row.get(0),
        )?;

        // Ventas de hoy en céntimos (Solo ADMINISTRADOR y CAJERO)
        let total_ventas_hoy = if rol == "ADMINISTRADOR" || rol == "CAJERO" {
            let ventas: i64 = conn.query_row(
                "SELECT COALESCE(SUM(monto_total), 0) FROM comprobantes WHERE date(fecha_emision) = date('now') AND tipo_comprobante != 'PROFORMA'",
                [],
                |row| row.get(0),
            ).unwrap_or(0);
            Some(ventas)
        } else {
            None
        };

        Ok(DashboardDto {
            rol,
            total_ventas_hoy,
            ot_en_proceso,
            productos_stock_bajo,
            alertas_fidelizacion,
        })
    }

    /// Lista los vehículos pendientes de mantenimiento por kilometraje
    pub fn listar_alertas_fidelizacion(
        conn: &Connection,
    ) -> Result<Vec<AlertaFidelizacionDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT v.placa, v.marca, v.modelo, v.kilomet_actual, ot.proximo_kilometraje, c.nombre_razon_social, c.telefono
             FROM vehiculos v
             JOIN clientes c ON v.id_cliente = c.id_cliente
             JOIN ordenes_trabajo ot ON v.placa = ot.placa
             WHERE v.kilometraje_actual >= ot.proximo_kilometraje AND ot.estado = 'FINALIZADO'
             GROUP BY v.placa"
        ).or_else(|_| {
            // Fallback de consulta si cambia el nombre de columna en algún entorno
            conn.prepare(
                "SELECT v.placa, v.marca, v.modelo, v.kilometraje_actual, ot.proximo_kilometraje, c.nombre_razon_social, c.telefono
                 FROM vehiculos v
                 JOIN clientes c ON v.id_cliente = c.id_cliente
                 JOIN ordenes_trabajo ot ON v.placa = ot.placa
                 WHERE v.kilometraje_actual >= ot.proximo_kilometraje
                 GROUP BY v.placa"
            )
        })?;

        let iter = stmt.query_map([], |row| {
            Ok(AlertaFidelizacionDto {
                placa: row.get(0)?,
                marca: row.get(1)?,
                modelo: row.get(2)?,
                kilometraje_actual: row.get(3)?,
                proximo_kilometraje: row.get(4)?,
                nombre_cliente: row.get(5)?,
                telefono: row.get(6)?,
            })
        })?;

        let mut alertas = Vec::new();
        for item in iter {
            alertas.push(item?);
        }

        Ok(alertas)
    }

    /// Genera un respaldo físico (copia de seguridad) de la base de datos SQLite
    pub fn generar_backup_db(db_path: &PathBuf, destino_dir: &PathBuf) -> Result<String, AppError> {
        if !db_path.exists() {
            return Err(AppError::NotFound(
                "La base de datos de origen no existe.".to_string(),
            ));
        }

        if !destino_dir.exists() {
            fs::create_dir_all(destino_dir).map_err(|e| {
                AppError::Validation(format!("No se pudo crear el directorio de destino: {}", e))
            })?;
        }

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let nombre_backup = format!("backup_lubricentro_{}.db", timestamp);
        let destino_path = destino_dir.join(nombre_backup);

        fs::copy(db_path, &destino_path).map_err(|e| {
            AppError::Validation(format!(
                "Error al copiar el archivo de base de datos: {}",
                e
            ))
        })?;

        Ok(destino_path.to_string_lossy().into_owned())
    }
}

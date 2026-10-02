use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
use crate::errors::AppError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub struct DashboardService;

impl DashboardService {
    /// Obtiene las métricas del dashboard aplicando filtros de seguridad por rol directamente en BD
    pub fn obtener_dashboard(conn: &Connection, id_usuario: i64) -> Result<DashboardDto, AppError> {
        let rol: String = conn
            .query_row(
                "SELECT rol FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [id_usuario],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Usuario no encontrado o inactivo.".to_string()))?;

        // 1. Órdenes activas (filtradas por mecánico si aplica)
        let ots_activas: i64 = if rol == "MECANICO" {
            conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE estado IN ('EN_ESPERA', 'EN_PROCESO') AND id_mecanico = ?1",
                [id_usuario],
                |row| row.get(0),
            )
            .unwrap_or(0)
        } else {
            conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE estado IN ('EN_ESPERA', 'EN_PROCESO')",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0)
        };

        // 2. Productos con stock crítico
        let productos_stock_critico: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM productos WHERE stock_actual <= stock_minimo AND activo = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // 3. Ventas y Comprobantes (Omitidos para MECANICO por seguridad)
        let (ventas_del_dia, comprobantes_emitidos) = if rol == "MECANICO" {
            (0.0, 0)
        } else {
            let (ventas_centimos, count): (i64, i64) = conn
                .query_row(
                    "SELECT 
                    COALESCE(SUM(monto_total), 0), 
                    COUNT(*) 
                 FROM comprobantes 
                 WHERE date(fecha_emision) = date('now') AND tipo_comprobante != 'PROFORMA'",
                    [],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap_or((0, 0));

            (ventas_centimos as f64 / 100.0, count)
        };

        // 4. Estado de caja (Filtrado por usuario para CAJERO, global para ADMIN, N/A para MECANICO)
        let estado_caja = if rol == "MECANICO" {
            "N/A".to_string()
        } else if rol == "CAJERO" {
            let caja_propia: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM caja_chica WHERE estado = 'ABIERTA' AND id_usuario = ?1",
                    [id_usuario],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            if caja_propia > 0 {
                "Abierta".to_string()
            } else {
                "Cerrada".to_string()
            }
        } else {
            let caja_global: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM caja_chica WHERE estado = 'ABIERTA'",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            if caja_global > 0 {
                "Abierta".to_string()
            } else {
                "Cerrada".to_string()
            }
        };

        Ok(DashboardDto {
            rol,
            ventas_del_dia,
            comprobantes_emitidos,
            ots_activas,
            productos_stock_critico,
            estado_caja,
        })
    }

    /// Lista los vehículos pendientes de mantenimiento por kilometraje
    pub fn listar_alertas_fidelizacion(
        conn: &Connection,
    ) -> Result<Vec<AlertaFidelizacionDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT v.placa, v.marca, v.modelo, v.kilometraje_actual, ot.proximo_kilometraje, c.nombre_razon_social, c.telefono
             FROM vehiculos v
             JOIN clientes c ON v.id_cliente = c.id_cliente
             JOIN ordenes_trabajo ot ON v.placa = ot.placa
             WHERE v.kilometraje_actual >= ot.proximo_kilometraje AND ot.estado = 'FINALIZADO'
             GROUP BY v.placa"
        ).or_else(|_| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_dashboard_filtrado_por_rol() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        // 1. Insertar usuario Mecánico
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (10, 'Mecanico Uno', 'meca1', 'hash', 'MECANICO', 1)",
            [],
        )
        .unwrap();

        // 2. Insertar usuario Cajero
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (20, 'Cajero Uno', 'caja1', 'hash', 'CAJERO', 1)",
            [],
        )
        .unwrap();

        // 3. Probar Dashboard de Mecánico (Ventas en 0.0 y Estado de caja en N/A)
        let dash_meca = DashboardService::obtener_dashboard(&conn, 10).unwrap();
        assert_eq!(dash_meca.rol, "MECANICO");
        assert_eq!(dash_meca.ventas_del_dia, 0.0);
        assert_eq!(dash_meca.estado_caja, "N/A");

        // 4. Probar Dashboard de Cajero (Caja cerrada por defecto)
        let dash_caja = DashboardService::obtener_dashboard(&conn, 20).unwrap();
        assert_eq!(dash_caja.rol, "CAJERO");
        assert_eq!(dash_caja.estado_caja, "Cerrada");
    }
}

use crate::dto::dashboard_dto::{AlertaFidelizacionDto, DashboardDto};
use crate::errors::AppError;
use rusqlite::Connection;

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

        // La misma fuente económica de R01: ventas confirmadas y devoluciones por fecha propia.
        let (ventas_del_dia, ventas_confirmadas, comprobantes_emitidos) = if rol == "MECANICO" {
            (0.0, 0, 0)
        } else {
            let hoy = crate::services::reportes_service::ahora_lima().date();
            let desde = format!("{hoy} 00:00:00");
            let hasta = format!("{} 00:00:00", hoy.succ_opt().expect("fecha de hoy válida"));
            let alcance = (rol == "CAJERO").then_some(id_usuario);
            let (neto, cantidad): (i64, i64) = conn.query_row(
                "SELECT coalesce(sum(total_cent),0),coalesce(sum(tipo='VENTA'),0)
                 FROM v_operaciones_comerciales
                 WHERE ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)
                 AND fecha_operacion>=?1 AND fecha_operacion<?2 AND (?3 IS NULL OR id_usuario=?3)",
                (&desde, &hasta, alcance),
                |r| Ok((r.get(0)?, r.get(1)?)),
            )?;
            let comprobantes: i64 = conn.query_row(
                "SELECT count(*) FROM comprobantes c JOIN ventas v ON v.id_venta=c.id_venta
                 WHERE c.ambiente=(SELECT ambiente_actual FROM configuracion WHERE id_config=1)
                 AND fecha_emision>=?1 AND fecha_emision<?2 AND (?3 IS NULL OR v.id_emisor=?3)",
                (&desde, &hasta, alcance),
                |r| r.get(0),
            )?;
            (neto as f64 / 100.0, cantidad, comprobantes)
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
            ventas_confirmadas,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    const DUMMY_ARGON2_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$Z3VpZGVsaW5lc3NhbHQ$5Vw5O7hK5W9u8P0s5vX+A1B2C3D4E5F6G7H8I9J0K1L";

    #[test]
    fn test_dashboard_filtrado_por_rol() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        // 1. Insertar usuario Mecánico
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (10, 'Mecanico Uno', 'meca1', ?1, 'MECANICO', 1)",
            [DUMMY_ARGON2_HASH],
        )
        .unwrap();

        // 2. Insertar usuario Cajero
        conn.execute(
            "INSERT INTO usuarios (id_usuario, nombre_completo, username, password_hash, rol, activo)
             VALUES (20, 'Cajero Uno', 'caja1', ?1, 'CAJERO', 1)",
            [DUMMY_ARGON2_HASH],
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

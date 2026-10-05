use crate::dto::orden_trabajo_dto::{
    CambiarEstadoOtDto, CrearOrdenTrabajoDto, DetalleItemOtDto, OrdenTrabajoHistorialDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};

pub struct OrdenesService;

impl OrdenesService {
    /// Genera un código correlativo único para la OT (ej: OT-000001)
    fn generar_codigo_ot(conn: &Connection) -> Result<String, AppError> {
        let max_id: i64 = conn.query_row(
            "SELECT COALESCE(MAX(id_ot), 0) FROM ordenes_trabajo",
            [],
            |row| row.get(0),
        )?;
        Ok(format!("OT-{:06}", max_id + 1))
    }

    /// Busca en los productos de la OT si existe un insumo de tipo Aceite/Lubricante
    fn obtener_tipo_aceite_ot(conn: &Connection, id_ot: i64) -> Result<Option<String>, AppError> {
        let sql = "SELECT p.descripcion
                   FROM detalle_ot_productos dp
                   JOIN productos p ON dp.id_producto = p.id_producto
                   LEFT JOIN categorias c ON p.id_categoria = c.id_categoria
                   WHERE dp.id_ot = ?1
                     AND (UPPER(c.nombre_categoria) LIKE '%ACEITE%'
                       OR UPPER(c.nombre_categoria) LIKE '%LUBRICANTE%'
                       OR UPPER(p.descripcion) LIKE '%ACEITE%'
                       OR UPPER(p.descripcion) LIKE '%20W%'
                       OR UPPER(p.descripcion) LIKE '%10W%'
                       OR UPPER(p.descripcion) LIKE '%5W%')
                   LIMIT 1";

        let resultado = conn.query_row(sql, [id_ot], |row| row.get(0)).optional()?;
        Ok(resultado)
    }

    /// Registra una nueva Orden de Trabajo
    pub fn crear_orden_trabajo(
        conn: &Connection,
        req: CrearOrdenTrabajoDto,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let placa = req.placa.trim().to_uppercase();

        if placa.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo es obligatoria.".to_string(),
            ));
        }

        if req.proximo_kilometraje < req.kilometraje_ingreso {
            return Err(AppError::Validation(
                "El próximo kilometraje debe ser mayor o igual al kilometraje de ingreso."
                    .to_string(),
            ));
        }

        // Verificar existencia del vehículo
        let existe_vehiculo: i32 = conn.query_row(
            "SELECT COUNT(*) FROM vehiculos WHERE placa = ?1 AND activo = 1",
            [&placa],
            |row| row.get(0),
        )?;

        if existe_vehiculo == 0 {
            return Err(AppError::NotFound(format!(
                "El vehículo con placa {} no se encuentra registrado.",
                placa
            )));
        }

        // Si se asigna zanja, validar que esté dentro de los valores permitidos (1 o 2) y no esté ocupada
        if let Some(z) = req.zanja {
            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado = 'EN_PROCESO'",
                [z],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya se encuentra ocupada por otra orden en proceso.",
                    z
                )));
            }
        }

        let codigo_ot = Self::generar_codigo_ot(conn)?;

        conn.execute(
            "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso)
             VALUES (?1, ?2, ?3, ?4, 'EN_ESPERA', ?5, ?6, ?7, CURRENT_TIMESTAMP)",
            params![
                codigo_ot,
                placa,
                req.id_mecanico,
                req.zanja,
                req.kilometraje_ingreso,
                req.proximo_kilometraje,
                req.observaciones,
            ],
        )?;

        let id_ot = conn.last_insert_rowid();

        // Actualizar el kilometraje actual en la ficha vehicular
        conn.execute(
            "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
            params![req.kilometraje_ingreso, placa],
        )?;

        Self::obtener_ot_por_id(conn, id_ot)
    }

    /// Obtiene los detalles de insumos y servicios de una OT específica
    pub fn obtener_detalles_ot(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<Vec<DetalleItemOtDto>, AppError> {
        let mut detalles = Vec::new();

        // 1. Obtener Productos aplicados en la OT
        let mut stmt_prod = conn.prepare(
            "SELECT
                dp.id_detalle,
                p.descripcion,
                dp.cantidad,
                (dp.precio_aplicado / 100.0) AS precio_unitario,
                ((dp.cantidad * dp.precio_aplicado - dp.descuento) / 100.0) AS subtotal
             FROM detalle_ot_productos dp
             JOIN productos p ON dp.id_producto = p.id_producto
             WHERE dp.id_ot = ?1",
        )?;

        let iter_prod = stmt_prod.query_map([id_ot], |row| {
            Ok(DetalleItemOtDto {
                id_detalle: row.get(0)?,
                descripcion: row.get(1)?,
                cantidad: row.get(2)?,
                precio_unitario: row.get(3)?,
                subtotal: row.get(4)?,
                tipo: "PRODUCTO".to_string(),
            })
        })?;

        for prod in iter_prod {
            detalles.push(prod?);
        }

        // 2. Obtener Servicios aplicados en la OT
        let mut stmt_serv = conn.prepare(
            "SELECT
                ds.id_detalle,
                s.descripcion,
                1.0 AS cantidad,
                (ds.precio_aplicado / 100.0) AS precio_unitario,
                ((ds.precio_aplicado - ds.descuento) / 100.0) AS subtotal
             FROM detalle_ot_servicios ds
             JOIN servicios s ON ds.id_servicio = s.id_servicio
             WHERE ds.id_ot = ?1",
        )?;

        let iter_serv = stmt_serv.query_map([id_ot], |row| {
            Ok(DetalleItemOtDto {
                id_detalle: row.get(0)?,
                descripcion: row.get(1)?,
                cantidad: row.get(2)?,
                precio_unitario: row.get(3)?,
                subtotal: row.get(4)?,
                tipo: "SERVICIO".to_string(),
            })
        })?;

        for serv in iter_serv {
            detalles.push(serv?);
        }

        Ok(detalles)
    }

    /// Obtiene una OT por su ID con sus detalles e información del mecánico
    pub fn obtener_ot_por_id(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT
                ot.id_ot, ot.codigo_ot, ot.placa, ot.id_mecanico, ot.zanja, ot.estado,
                ot.kilometraje_ingreso, ot.proximo_kilometraje, ot.observaciones, ot.fecha_ingreso,
                u.nombre_completo
             FROM ordenes_trabajo ot
             LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
             WHERE ot.id_ot = ?1",
        )?;

        let raw_data = stmt
            .query_row([id_ot], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<i32>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, String>(9)?,
                    row.get::<_, Option<String>>(10)?,
                ))
            })
            .map_err(|_| AppError::NotFound(format!("No se encontró la OT con ID {}", id_ot)))?;

        let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
        let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;

        Ok(OrdenTrabajoHistorialDto {
            id_ot: raw_data.0,
            codigo_ot: raw_data.1,
            placa: raw_data.2,
            id_mecanico: raw_data.3,
            zanja: raw_data.4,
            estado: raw_data.5,
            kilometraje_ingreso: raw_data.6,
            proximo_kilometraje: raw_data.7,
            observaciones: raw_data.8,
            fecha_ingreso: Some(raw_data.9),
            nombre_mecanico: raw_data.10,
            tipo_aceite,
            detalles,
        })
    }

    /// Máquina de estados para actualización de OT
    pub fn cambiar_estado_ot(
        conn: &Connection,
        req: CambiarEstadoOtDto,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let ot_actual = Self::obtener_ot_por_id(conn, req.id_ot)?;
        let nuevo_estado = req.nuevo_estado.trim().to_uppercase();

        let estados_validos = ["EN_ESPERA", "EN_PROCESO", "FINALIZADO", "CANCELADO"];
        if !estados_validos.contains(&nuevo_estado.as_str()) {
            return Err(AppError::Validation(format!(
                "Estado inválido: {}. Los estados permitidos son EN_ESPERA, EN_PROCESO, FINALIZADO, CANCELADO.",
                nuevo_estado
            )));
        }

        if nuevo_estado == "EN_PROCESO" {
            let zanja_asignada = req.zanja.or(ot_actual.zanja);
            if zanja_asignada.is_none() {
                return Err(AppError::Validation(
                    "Para pasar a estado EN_PROCESO se debe asignar una zanja (1 o 2).".to_string(),
                ));
            }

            let z = zanja_asignada.unwrap();
            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado = 'EN_PROCESO' AND id_ot != ?2",
                params![z, req.id_ot],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya está ocupada por otra orden en proceso.",
                    z
                )));
            }

            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1, zanja = ?2 WHERE id_ot = ?3",
                params![nuevo_estado, z, req.id_ot],
            )?;
        } else if nuevo_estado == "FINALIZADO" || nuevo_estado == "CANCELADO" {
            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1, zanja = NULL WHERE id_ot = ?2",
                params![nuevo_estado, req.id_ot],
            )?;
        } else {
            conn.execute(
                "UPDATE ordenes_trabajo SET estado = ?1 WHERE id_ot = ?2",
                params![nuevo_estado, req.id_ot],
            )?;
        }

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    /// Obtiene el historial completo de órdenes de trabajo asociadas a una placa
    pub fn obtener_historial_por_placa(
        conn: &Connection,
        placa: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        let placa_clean = placa.trim().to_uppercase();
        let mut stmt = conn.prepare(
            "SELECT
                ot.id_ot, ot.codigo_ot, ot.placa, ot.id_mecanico, ot.zanja, ot.estado,
                ot.kilometraje_ingreso, ot.proximo_kilometraje, ot.observaciones, ot.fecha_ingreso,
                u.nombre_completo
             FROM ordenes_trabajo ot
             LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
             WHERE ot.placa = ?1
             ORDER BY ot.fecha_ingreso DESC, ot.id_ot DESC",
        )?;

        let iter = stmt.query_map([&placa_clean], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Option<i32>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, i64>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, Option<String>>(10)?,
            ))
        })?;

        let mut historial = Vec::new();
        for item in iter {
            let raw_data = item?;

            let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
            let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;

            historial.push(OrdenTrabajoHistorialDto {
                id_ot: raw_data.0,
                codigo_ot: raw_data.1,
                placa: raw_data.2,
                id_mecanico: raw_data.3,
                zanja: raw_data.4,
                estado: raw_data.5,
                kilometraje_ingreso: raw_data.6,
                proximo_kilometraje: raw_data.7,
                observaciones: raw_data.8,
                fecha_ingreso: Some(raw_data.9),
                nombre_mecanico: raw_data.10,
                tipo_aceite,
                detalles,
            });
        }

        Ok(historial)
    }
}

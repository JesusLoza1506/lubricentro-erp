use crate::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, AgregarServicioOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto,
    DetalleItemOtDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto, ServicioResumenDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

pub struct OrdenesService;

impl OrdenesService {
    fn generar_codigo_ot(conn: &Connection) -> Result<String, AppError> {
        let max_id: i64 = conn.query_row(
            "SELECT COALESCE(MAX(id_ot), 0) FROM ordenes_trabajo",
            [],
            |row| row.get(0),
        )?;

        let anio_actual = chrono::Local::now().format("%Y").to_string();
        Ok(format!("OT-{}-{:04}", anio_actual, max_id + 1))
    }

    fn obtener_tipo_aceite_ot(conn: &Connection, id_ot: i64) -> Result<Option<String>, AppError> {
        let sql = "SELECT tipo_aceite FROM ordenes_trabajo WHERE id_ot = ?1";
        if let Ok(Some(Some(t))) = conn
            .query_row(sql, [id_ot], |row| row.get::<_, Option<String>>(0))
            .optional()
        {
            if !t.trim().is_empty() {
                return Ok(Some(t));
            }
        }

        let sql_prod = "SELECT p.nombre
                        FROM detalle_ot_productos dp
                        JOIN productos p ON dp.id_producto = p.id_producto
                        WHERE dp.id_ot = ?1
                          AND (UPPER(p.categoria) = 'ACEITE' OR UPPER(p.categoria) = 'GRANEL')
                        LIMIT 1";

        let resultado = conn
            .query_row(sql_prod, [id_ot], |row| row.get(0))
            .optional()?;
        Ok(resultado)
    }

    fn tiene_cambio_aceite(conn: &Connection, id_ot: i64) -> bool {
        let serv_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM detalle_ot_servicios WHERE id_ot = ?1 AND es_cambio_aceite = 1",
                [id_ot],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if serv_count > 0 {
            return true;
        }

        let prod_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) 
                 FROM detalle_ot_productos dp
                 JOIN productos p ON dp.id_producto = p.id_producto
                 WHERE dp.id_ot = ?1 AND (UPPER(p.categoria) = 'ACEITE' OR UPPER(p.categoria) = 'GRANEL')",
                [id_ot],
                |row| row.get(0),
            )
            .unwrap_or(0);

        prod_count > 0
    }

    fn calcular_proximo_kilometraje(
        conn: &Connection,
        id_ot: i64,
        km_base: i64,
        tipo_aceite: Option<&str>,
    ) -> Option<i64> {
        if !Self::tiene_cambio_aceite(conn, id_ot) {
            return None;
        }

        let intervalo = match tipo_aceite {
            Some(t) if t.trim().eq_ignore_ascii_case("SINTETICO") => 10000,
            _ => 5000,
        };

        Some(km_base + intervalo)
    }

    fn recalcular_kilometraje_vehiculo(conn: &Connection, placa: &str) -> Result<(), AppError> {
        let max_km: Option<i64> = conn
            .query_row(
                "SELECT MAX(COALESCE(kilometraje_salida, kilometraje_ingreso)) FROM ordenes_trabajo WHERE placa_snapshot = ?1 AND estado != 'CANCELADO'",
                [placa],
                |row| row.get(0),
            )
            .optional()?
            .flatten();

        if let Some(km) = max_km {
            conn.execute(
                "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
                params![km, placa],
            )?;
        }
        Ok(())
    }

    fn validar_mecanico_activo(conn: &Connection, id_mecanico: i64) -> Result<(), AppError> {
        let es_mecanico_valido: Option<bool> = conn
            .query_row(
                "SELECT (UPPER(rol) = 'MECANICO' AND activo = 1) FROM usuarios WHERE id_usuario = ?1",
                [id_mecanico],
                |row| row.get(0),
            )
            .optional()?;

        match es_mecanico_valido {
            Some(true) => Ok(()),
            Some(false) => Err(AppError::Validation(format!(
                "El usuario asignado con ID {} no es un mecánico activo.",
                id_mecanico
            ))),
            None => Err(AppError::NotFound(format!(
                "No se encontró el mecánico con ID {}.",
                id_mecanico
            ))),
        }
    }

    fn validar_transicion_estado(estado_actual: &str, nuevo_estado: &str) -> Result<(), AppError> {
        let actual = estado_actual.trim().to_uppercase();
        let nuevo = nuevo_estado.trim().to_uppercase();

        if actual == nuevo {
            return Ok(());
        }

        let es_valida = matches!(
            (actual.as_str(), nuevo.as_str()),
            ("EN_ESPERA", "EN_PROCESO")
                | ("EN_ESPERA", "CANCELADO")
                | ("EN_PROCESO", "FINALIZADO")
                | ("EN_PROCESO", "CANCELADO")
        );

        if !es_valida {
            return Err(AppError::Validation(format!(
                "Transición de estado no permitida: No se puede cambiar de '{}' a '{}'.",
                actual, nuevo
            )));
        }

        Ok(())
    }

    fn validar_permiso_operacion_ot(
        conn: &Connection,
        id_ot: i64,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let (rol_usuario, activo): (String, i32) = conn
            .query_row(
                "SELECT UPPER(rol), activo FROM usuarios WHERE id_usuario = ?1",
                [id_usuario],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|_| AppError::Validation("Usuario ejecutor no encontrado.".to_string()))?;

        if activo != 1 {
            return Err(AppError::Validation("Usuario inactivo.".to_string()));
        }

        if rol_usuario == "ADMINISTRADOR" {
            return Ok(());
        }

        if rol_usuario == "MECANICO" {
            let id_mecanico_ot: Option<i64> = conn
                .query_row(
                    "SELECT id_mecanico FROM ordenes_trabajo WHERE id_ot = ?1",
                    [id_ot],
                    |row| row.get(0),
                )
                .map_err(|_| {
                    AppError::NotFound(format!("No se encontró la OT con ID {}.", id_ot))
                })?;

            if id_mecanico_ot != Some(id_usuario) {
                return Err(AppError::Validation(
                    "Acceso denegado: No tienes permiso para modificar la OT de otro mecánico."
                        .to_string(),
                ));
            }
            return Ok(());
        }

        Err(AppError::Validation(
            "Acceso denegado: Tu rol no permite modificar detalles de Órdenes de Trabajo."
                .to_string(),
        ))
    }

    pub fn crear_orden_trabajo(
        conn: &mut Connection,
        req: CrearOrdenTrabajoDto,
        id_creador: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let placa = req.placa.trim().to_uppercase();

        if placa.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo es obligatoria.".to_string(),
            ));
        }

        let tipo_aceite_upper = req.tipo_aceite.trim().to_uppercase();
        if tipo_aceite_upper != "MINERAL" && tipo_aceite_upper != "SINTETICO" {
            return Err(AppError::Validation(format!(
                "Tipo de aceite '{}' no válido. Solo se permite 'MINERAL' o 'SINTETICO'.",
                req.tipo_aceite
            )));
        }

        Self::validar_mecanico_activo(conn, req.id_mecanico)?;

        let (id_vehiculo, id_cliente_atendido, km_actual_ficha, nombre_cliente): (i64, i64, i64, String) = conn
            .query_row(
                "SELECT v.id_vehiculo, v.id_cliente, COALESCE(v.kilometraje_actual, 0), c.nombre_razon_social
                 FROM vehiculos v
                 JOIN clientes c ON v.id_cliente = c.id_cliente
                 WHERE v.placa = ?1 AND v.activo = 1",
                [&placa],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .map_err(|_| AppError::NotFound(format!("El vehículo con placa {} no se encuentra registrado.", placa)))?;

        if req.kilometraje_ingreso < km_actual_ficha {
            return Err(AppError::Validation(format!(
                "El kilometraje de ingreso ({} km) no puede ser inferior al kilometraje actual registrado en la Ficha Vehicular ({} km).",
                req.kilometraje_ingreso, km_actual_ficha
            )));
        }

        if let Some(z) = req.zanja {
            if !(1..=6).contains(&z) {
                return Err(AppError::Validation(
                    "La zanja asignada debe estar entre 1 y 6.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE id_zanja_ejecucion = ?1 AND estado IN ('EN_ESPERA', 'EN_PROCESO')",
                [z],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya se encuentra ocupada por otra orden activa.",
                    z
                )));
            }
        }

        let codigo_ot = Self::generar_codigo_ot(conn)?;
        let id_operacion = Uuid::new_v4().to_string();
        let cliente_snapshot = format!(r#"{{"nombre":"{}"}}"#, nombre_cliente);

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        tx.execute(
            "INSERT INTO ordenes_trabajo (
                id_operacion, codigo_ot, id_vehiculo, id_cliente_atendido, placa_snapshot, cliente_snapshot,
                id_creador, id_mecanico, id_zanja_ejecucion, estado, kilometraje_ingreso, pedido_cliente, tipo_aceite, fecha_ingreso
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'EN_ESPERA', ?10, ?11, ?12, datetime('now', '-5 hours'))",
            params![
                id_operacion,
                codigo_ot,
                id_vehiculo,
                id_cliente_atendido,
                placa,
                cliente_snapshot,
                id_creador,
                req.id_mecanico,
                req.zanja,
                req.kilometraje_ingreso,
                req.observaciones,
                tipo_aceite_upper
            ],
        )?;

        let id_ot = tx.last_insert_rowid();

        tx.execute(
            "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE id_vehiculo = ?2",
            params![req.kilometraje_ingreso, id_vehiculo],
        )?;

        tx.commit()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::obtener_ot_por_id(conn, id_ot)
    }

    pub fn cambiar_estado_ot(
        conn: &mut Connection,
        req: CambiarEstadoOtDto,
        id_usuario: Option<i64>,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let ot_actual = Self::obtener_ot_por_id(conn, req.id_ot)?;
        let nuevo_estado = req.nuevo_estado.trim().to_uppercase();

        if ot_actual.estado == nuevo_estado {
            return Self::obtener_ot_por_id(conn, req.id_ot);
        }

        Self::validar_transicion_estado(&ot_actual.estado, &nuevo_estado)?;

        let id_u = id_usuario.ok_or_else(|| {
            AppError::Validation(
                "Se requiere un usuario válido para cambiar el estado.".to_string(),
            )
        })?;

        let rol_usuario: String = conn
            .query_row(
                "SELECT UPPER(rol) FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                [id_u],
                |row| row.get(0),
            )
            .map_err(|_| AppError::Validation("Actor inactivo o rol no autorizado".to_string()))?;

        if nuevo_estado == "CANCELADO" && rol_usuario != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "Acceso denegado: Solo un usuario ADMINISTRADOR puede cancelar una Orden de Trabajo."
                    .to_string(),
            ));
        }

        if rol_usuario == "MECANICO" && ot_actual.id_mecanico != id_u {
            return Err(AppError::Validation(
                "Acceso denegado: Solo puedes modificar las órdenes de trabajo asignadas a ti."
                    .to_string(),
            ));
        }

        if nuevo_estado == "EN_PROCESO" {
            let zanja_asignada = req.zanja.or(ot_actual.zanja);
            let z = zanja_asignada.ok_or_else(|| {
                AppError::Validation(
                    "Para pasar a estado EN_PROCESO debe especificar la zanja.".to_string(),
                )
            })?;

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE id_zanja_ejecucion = ?1 AND estado = 'EN_PROCESO' AND id_ot != ?2",
                params![z, req.id_ot],
                |row| row.get(0),
            )?;

            if ocupada > 0 {
                return Err(AppError::Validation(format!(
                    "La zanja {} ya está ocupada por otra orden activa.",
                    z
                )));
            }

            conn.execute(
                "UPDATE ordenes_trabajo 
                 SET estado = ?1, id_zanja_ejecucion = ?2, id_mecanico = ?3, fecha_inicio = datetime('now', '-5 hours') 
                 WHERE id_ot = ?4",
                params![nuevo_estado, z, id_u, req.id_ot],
            )?;
        } else if nuevo_estado == "CANCELADO" {
            let tx = conn
                .transaction()
                .map_err(|e| AppError::Validation(e.to_string()))?;

            tx.execute(
                "UPDATE ordenes_trabajo 
                 SET estado = 'CANCELADO', id_zanja_ejecucion = NULL, fecha_cancelacion = datetime('now', '-5 hours'), id_usuario_cancelacion = ?1, motivo_cancelacion = 'Cancelación por usuario administrador' 
                 WHERE id_ot = ?2",
                params![id_u, req.id_ot],
            )?;

            tx.commit()
                .map_err(|e| AppError::Validation(e.to_string()))?;

            Self::recalcular_kilometraje_vehiculo(conn, &ot_actual.placa)?;
        } else if nuevo_estado == "FINALIZADO" {
            conn.execute(
                "UPDATE ordenes_trabajo 
                 SET estado = 'FINALIZADO', id_zanja_ejecucion = NULL, fecha_fin = datetime('now', '-5 hours'), kilometraje_salida = kilometraje_ingreso, id_mecanico_terminal = id_mecanico, resumen_publico = COALESCE(pedido_cliente, 'Mantenimiento finalizado correctamente') 
                 WHERE id_ot = ?1",
                params![req.id_ot],
            )?;
        }

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    pub fn reasignar_mecanico_ot(
        conn: &Connection,
        req: ReasignarMecanicoOtDto,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let ot = Self::obtener_ot_por_id(conn, req.id_ot)?;

        if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
            return Err(AppError::Validation(
                "No se puede reasignar mecánico en una orden de trabajo finalizada o cancelada."
                    .to_string(),
            ));
        }

        Self::validar_mecanico_activo(conn, req.nuevo_id_mecanico)?;

        conn.execute(
            "UPDATE ordenes_trabajo SET id_mecanico = ?1 WHERE id_ot = ?2",
            params![req.nuevo_id_mecanico, req.id_ot],
        )?;

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    pub fn listar_ordenes_trabajo(
        conn: &Connection,
        id_usuario: Option<i64>,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        let mut rol_usuario = String::new();
        let mut id_u_valido = 0;

        if let Some(id_u) = id_usuario {
            if id_u > 0 {
                id_u_valido = id_u;
                rol_usuario = conn
                    .query_row(
                        "SELECT UPPER(rol) FROM usuarios WHERE id_usuario = ?1 AND activo = 1",
                        [id_u],
                        |row| row.get(0),
                    )
                    .unwrap_or_default();
            }
        }

        let mut sql = String::from(
            "SELECT
                ot.id_ot, ot.codigo_ot, ot.placa_snapshot, ot.id_mecanico, ot.id_zanja_ejecucion, ot.estado,
                COALESCE(ot.kilometraje_salida, ot.kilometraje_ingreso), COALESCE(ot.resumen_publico, ot.pedido_cliente), ot.fecha_ingreso,
                u.nombre_completo, ot.corregido
             FROM ordenes_trabajo ot
             LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario",
        );

        if rol_usuario == "MECANICO" {
            sql.push_str(&format!(" WHERE ot.id_mecanico = {}", id_u_valido));
        }

        sql.push_str(" ORDER BY ot.id_ot DESC");

        let mut stmt = conn.prepare(&sql)?;
        let iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<i64>>(3)?,
                row.get::<_, Option<i32>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<i32>>(10)?.unwrap_or(0),
            ))
        })?;

        let mut lista = Vec::new();
        for item in iter {
            let raw_data = item?;
            let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
            let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;
            let proximo_km = Self::calcular_proximo_kilometraje(
                conn,
                raw_data.0,
                raw_data.6,
                tipo_aceite.as_deref(),
            );

            lista.push(OrdenTrabajoHistorialDto {
                id_ot: raw_data.0,
                codigo_ot: raw_data.1,
                placa: raw_data.2,
                id_mecanico: raw_data.3.unwrap_or(0),
                zanja: raw_data.4,
                estado: raw_data.5,
                kilometraje_ingreso: raw_data.6,
                proximo_kilometraje: proximo_km,
                observaciones: raw_data.7,
                fecha_ingreso: Some(raw_data.8),
                nombre_mecanico: raw_data.9,
                tipo_aceite,
                detalles,
                corregido: Some(raw_data.10),
            });
        }

        Ok(lista)
    }

    pub fn obtener_detalles_ot(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<Vec<DetalleItemOtDto>, AppError> {
        let mut detalles = Vec::new();

        let mut stmt_prod = conn.prepare(
            "SELECT
                dp.id_detalle,
                dp.nombre_snapshot,
                dp.cantidad,
                (dp.precio_unitario_cent / 100.0) AS precio_unitario,
                ((dp.cantidad * dp.precio_unitario_cent) / 100.0) AS subtotal
             FROM detalle_ot_productos dp
             WHERE dp.id_ot = ?1",
        )?;

        let iter_prod = stmt_prod.query_map([id_ot], |row| {
            Ok(DetalleItemOtDto {
                id_detalle: row.get(0)?,
                descripcion: row.get(1)?,
                cantidad: row.get::<_, i64>(2)? as f64,
                precio_unitario: row.get(3)?,
                subtotal: row.get(4)?,
                tipo: "PRODUCTO".to_string(),
            })
        })?;

        for prod in iter_prod {
            detalles.push(prod?);
        }

        let mut stmt_serv = conn.prepare(
            "SELECT
                ds.id_detalle,
                ds.nombre_snapshot,
                ds.cantidad,
                (ds.precio_unitario_cent / 100.0) AS precio_unitario,
                ((ds.cantidad * ds.precio_unitario_cent) / 100.0) AS subtotal
             FROM detalle_ot_servicios ds
             WHERE ds.id_ot = ?1",
        )?;

        let iter_serv = stmt_serv.query_map([id_ot], |row| {
            Ok(DetalleItemOtDto {
                id_detalle: row.get(0)?,
                descripcion: row.get(1)?,
                cantidad: row.get::<_, i64>(2)? as f64,
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

    pub fn obtener_ot_por_id(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let mut stmt = conn.prepare(
            "SELECT
                ot.id_ot, ot.codigo_ot, ot.placa_snapshot, ot.id_mecanico, ot.id_zanja_ejecucion, ot.estado,
                COALESCE(ot.kilometraje_salida, ot.kilometraje_ingreso), COALESCE(ot.resumen_publico, ot.pedido_cliente), ot.fecha_ingreso,
                u.nombre_completo, ot.corregido
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
                    row.get::<_, Option<i64>>(3)?,
                    row.get::<_, Option<i32>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, Option<String>>(7)?,
                    row.get::<_, String>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<i32>>(10)?.unwrap_or(0),
                ))
            })
            .map_err(|_| AppError::NotFound(format!("No se encontró la OT con ID {}", id_ot)))?;

        let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
        let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;
        let proximo_km = Self::calcular_proximo_kilometraje(
            conn,
            raw_data.0,
            raw_data.6,
            tipo_aceite.as_deref(),
        );

        Ok(OrdenTrabajoHistorialDto {
            id_ot: raw_data.0,
            codigo_ot: raw_data.1,
            placa: raw_data.2,
            id_mecanico: raw_data.3.unwrap_or(0),
            zanja: raw_data.4,
            estado: raw_data.5,
            kilometraje_ingreso: raw_data.6,
            proximo_kilometraje: proximo_km,
            observaciones: raw_data.7,
            fecha_ingreso: Some(raw_data.8),
            nombre_mecanico: raw_data.9,
            tipo_aceite,
            detalles,
            corregido: Some(raw_data.10),
        })
    }

    pub fn obtener_historial_por_id_vehiculo(
        conn: &Connection,
        id_vehiculo: i64,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT
                ot.id_ot, ot.codigo_ot, ot.placa_snapshot, ot.id_mecanico, ot.id_zanja_ejecucion, ot.estado,
                COALESCE(ot.kilometraje_salida, ot.kilometraje_ingreso), COALESCE(ot.resumen_publico, ot.pedido_cliente), COALESCE(ot.fecha_fin, ot.fecha_ingreso),
                u.nombre_completo, ot.corregido
             FROM ordenes_trabajo ot
             LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
             WHERE ot.id_vehiculo = ?1
               AND ot.estado = 'FINALIZADO'
             ORDER BY COALESCE(ot.fecha_fin, ot.fecha_ingreso) DESC, ot.id_ot DESC",
        )?;

        let iter = stmt.query_map([id_vehiculo], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<i64>>(3)?,
                row.get::<_, Option<i32>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<i32>>(10)?.unwrap_or(0),
            ))
        })?;

        let mut historial = Vec::new();
        for item in iter {
            let raw_data = item?;
            let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
            let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;
            let proximo_km = Self::calcular_proximo_kilometraje(
                conn,
                raw_data.0,
                raw_data.6,
                tipo_aceite.as_deref(),
            );

            historial.push(OrdenTrabajoHistorialDto {
                id_ot: raw_data.0,
                codigo_ot: raw_data.1,
                placa: raw_data.2,
                id_mecanico: raw_data.3.unwrap_or(0),
                zanja: raw_data.4,
                estado: raw_data.5,
                kilometraje_ingreso: raw_data.6,
                proximo_kilometraje: proximo_km,
                observaciones: raw_data.7,
                fecha_ingreso: Some(raw_data.8),
                nombre_mecanico: raw_data.9,
                tipo_aceite,
                detalles,
                corregido: Some(raw_data.10),
            });
        }

        Ok(historial)
    }

    pub fn obtener_historial_por_placa(
        conn: &Connection,
        placa: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        let placa_clean = placa.trim().to_uppercase();
        let id_vehiculo: Option<i64> = conn
            .query_row(
                "SELECT id_vehiculo FROM vehiculos WHERE placa = ?1",
                [&placa_clean],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(id_v) = id_vehiculo {
            Self::obtener_historial_por_id_vehiculo(conn, id_v)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn agregar_producto_ot(
        conn: &mut Connection,
        req: AgregarProductoOtDto,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        Self::validar_permiso_operacion_ot(conn, req.id_ot, id_usuario_ejecutor)?;

        let ot = Self::obtener_ot_por_id(conn, req.id_ot)?;
        if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
            return Err(AppError::Validation(
                "No se pueden agregar productos a una orden finalizada o cancelada.".to_string(),
            ));
        }

        if req.cantidad <= 0.0 {
            return Err(AppError::Validation(
                "La cantidad debe ser mayor a 0.".to_string(),
            ));
        }

        let (precio_venta_cent, stock_actual, nombre_prod, codigo_prod, unidad_medida): (i64, i64, String, String, String) = conn
            .query_row(
                "SELECT precio_venta_cent, stock_actual, nombre, codigo, unidad_medida FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        let cant_entera = req.cantidad as i64;
        if stock_actual < cant_entera {
            return Err(AppError::Validation(format!(
                "Stock insuficiente para '{}'. Disponible: {}, solicitado: {}",
                nombre_prod, stock_actual, cant_entera
            )));
        }

        let id_op_det = Uuid::new_v4().to_string();

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        tx.execute(
            "INSERT INTO detalle_ot_productos (
                id_operacion, id_ot, id_producto, codigo_snapshot, nombre_snapshot, unidad_medida,
                cantidad, precio_unitario_cent, costo_unitario_micro, valor_consumido_micro, id_usuario
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, 1, ?9)",
            params![
                id_op_det,
                req.id_ot,
                req.id_producto,
                codigo_prod,
                nombre_prod,
                unidad_medida,
                cant_entera,
                precio_venta_cent,
                id_usuario_ejecutor
            ],
        )?;

        let id_detalle_ot = tx.last_insert_rowid();
        let id_op_kardex = Uuid::new_v4().to_string();

        tx.execute(
            "INSERT INTO movimientos_inventario (
                id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
                stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
                valor_resultante_micro, costo_unitario_micro
            ) VALUES (?1, ?2, ?3, 'CONSUMO_OT', ?4, ?5, ?6, ?7, ?8, 1, -1, 0, 1)",
            params![
                id_op_kardex,
                req.id_producto,
                id_usuario_ejecutor,
                id_detalle_ot,
                unidad_medida,
                stock_actual,
                -cant_entera,
                stock_actual - cant_entera
            ],
        )?;

        tx.commit()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    pub fn eliminar_producto_ot(
        conn: &mut Connection,
        id_detalle: i64,
        id_ot: i64,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        Self::validar_permiso_operacion_ot(conn, id_ot, id_usuario_ejecutor)?;

        let ot = Self::obtener_ot_por_id(conn, id_ot)?;
        if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
            return Err(AppError::Validation(
                "No se pueden eliminar productos de una orden finalizada o cancelada.".to_string(),
            ));
        }

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let detalle_opt: Option<(i64, i64, String)> = tx
            .query_row(
                "SELECT id_producto, cantidad, unidad_medida FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
                params![id_detalle, id_ot],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;

        if let Some((id_prod, cant, unidad_medida)) = detalle_opt {
            let stock_actual: i64 = tx.query_row(
                "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                [id_prod],
                |row| row.get(0),
            )?;

            let stock_resultante = stock_actual + cant;
            let id_op_kardex = Uuid::new_v4().to_string();

            tx.execute(
                "INSERT INTO movimientos_inventario (
                    id_operacion, id_producto, id_usuario, tipo_movimiento, id_consumo_ot, unidad_medida,
                    stock_anterior, diferencia, stock_resultante, valor_anterior_micro, valor_delta_micro,
                    valor_resultante_micro, costo_unitario_micro, motivo
                ) VALUES (?1, ?2, ?3, 'REPOSICION_OT', ?4, ?5, ?6, ?7, ?8, 0, 1, 1, 1, 'Devolución de producto por eliminación de detalle en OT')",
                params![
                    id_op_kardex,
                    id_prod,
                    id_usuario_ejecutor,
                    id_detalle,
                    unidad_medida,
                    stock_actual,
                    cant,
                    stock_resultante
                ],
            )?;

            tx.execute(
                "DELETE FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
                params![id_detalle, id_ot],
            )?;
        }

        tx.commit()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::obtener_ot_por_id(conn, id_ot)
    }

    pub fn agregar_servicio_ot(
        conn: &Connection,
        req: AgregarServicioOtDto,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        Self::validar_permiso_operacion_ot(conn, req.id_ot, id_usuario_ejecutor)?;

        let ot = Self::obtener_ot_por_id(conn, req.id_ot)?;
        if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
            return Err(AppError::Validation(
                "No se pueden agregar servicios a una orden finalizada o cancelada.".to_string(),
            ));
        }

        let (precio_venta_cent, nombre_serv, codigo_serv, es_cambio): (i64, String, String, i32) = conn
            .query_row(
                "SELECT precio_venta_cent, nombre, codigo, es_cambio_aceite FROM servicios WHERE id_servicio = ?1 AND activo = 1",
                [req.id_servicio],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .map_err(|_| AppError::NotFound("Servicio no encontrado o inactivo.".to_string()))?;

        let id_op_serv = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO detalle_ot_servicios (
                id_operacion, id_ot, id_servicio, codigo_snapshot, nombre_snapshot, cantidad, precio_unitario_cent, es_cambio_aceite, id_usuario
            ) VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7, ?8)",
            params![
                id_op_serv,
                req.id_ot,
                req.id_servicio,
                codigo_serv,
                nombre_serv,
                precio_venta_cent,
                es_cambio,
                id_usuario_ejecutor
            ],
        )?;

        Self::obtener_ot_por_id(conn, req.id_ot)
    }

    pub fn eliminar_servicio_ot(
        conn: &Connection,
        id_detalle: i64,
        id_ot: i64,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        Self::validar_permiso_operacion_ot(conn, id_ot, id_usuario_ejecutor)?;

        let ot = Self::obtener_ot_por_id(conn, id_ot)?;
        if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
            return Err(AppError::Validation(
                "No se pueden eliminar servicios de una orden finalizada o cancelada.".to_string(),
            ));
        }

        conn.execute(
            "DELETE FROM detalle_ot_servicios WHERE id_detalle = ?1 AND id_ot = ?2",
            params![id_detalle, id_ot],
        )?;

        Self::obtener_ot_por_id(conn, id_ot)
    }

    pub fn listar_servicios(conn: &Connection) -> Result<Vec<ServicioResumenDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_servicio, nombre, (precio_venta_cent / 100.0) AS precio_base
             FROM servicios
             WHERE activo = 1
             ORDER BY nombre ASC",
        )?;

        let iter = stmt.query_map([], |row| {
            Ok(ServicioResumenDto {
                id_servicio: row.get(0)?,
                descripcion: row.get(1)?,
                precio_base: row.get(2)?,
            })
        })?;

        let mut lista = Vec::new();
        for item in iter {
            lista.push(item?);
        }

        Ok(lista)
    }
}

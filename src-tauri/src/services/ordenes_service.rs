use crate::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, AgregarServicioOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto,
    DetalleItemOtDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto, ServicioResumenDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection, OptionalExtension};

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

    /// Recalcula el kilometraje actual del vehículo ignorando las órdenes canceladas
    fn recalcular_kilometraje_vehiculo(conn: &Connection, placa: &str) -> Result<(), AppError> {
        let max_km: Option<i64> = conn
            .query_row(
                "SELECT MAX(kilometraje_ingreso) FROM ordenes_trabajo WHERE placa = ?1 AND estado != 'CANCELADO'",
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

    /// Valida que el ID de mecánico exista, esté activo y tenga rol 'MECANICO'
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

    /// Valida las transiciones legales de la máquina de estados
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

    /// Valida si el usuario ejecutor tiene permiso para modificar los detalles de la OT
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
            let id_mecanico_ot: i64 = conn
                .query_row(
                    "SELECT id_mecanico FROM ordenes_trabajo WHERE id_ot = ?1",
                    [id_ot],
                    |row| row.get(0),
                )
                .map_err(|_| {
                    AppError::NotFound(format!("No se encontró la OT con ID {}.", id_ot))
                })?;

            if id_mecanico_ot != id_usuario {
                return Err(AppError::Validation(
                    "Acceso denegado: No tienes permiso para modificar los productos o servicios de la OT de otro mecánico."
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
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        let placa = req.placa.trim().to_uppercase();

        if placa.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo es obligatoria.".to_string(),
            ));
        }

        // 1. Validar estrictamente el tipo de aceite (Solamente MINERAL o SINTETICO)
        let tipo_aceite_upper = req.tipo_aceite.trim().to_uppercase();
        if tipo_aceite_upper != "MINERAL" && tipo_aceite_upper != "SINTETICO" {
            return Err(AppError::Validation(format!(
                "Tipo de aceite '{}' no válido. Solo se permite 'MINERAL' o 'SINTETICO'.",
                req.tipo_aceite
            )));
        }

        // 2. Validar que el mecánico sea válido y activo
        Self::validar_mecanico_activo(conn, req.id_mecanico)?;

        // 3. Verificar existencia del vehículo y comparar kilometraje actual de la Ficha
        let (existe_vehiculo, km_actual_ficha): (i32, i64) = conn
            .query_row(
                "SELECT COUNT(*), COALESCE(kilometraje_actual, 0) FROM vehiculos WHERE placa = ?1 AND activo = 1",
                [&placa],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )?;

        if existe_vehiculo == 0 {
            return Err(AppError::NotFound(format!(
                "El vehículo con placa {} no se encuentra registrado.",
                placa
            )));
        }

        if req.kilometraje_ingreso < km_actual_ficha {
            return Err(AppError::Validation(format!(
                "El kilometraje de ingreso ({} km) no puede ser inferior al kilometraje actual registrado en la Ficha Vehicular ({} km).",
                req.kilometraje_ingreso, km_actual_ficha
            )));
        }

        // 4. Verificar Zanja única para OTs activas
        if let Some(z) = req.zanja {
            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado IN ('EN_ESPERA', 'EN_PROCESO')",
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

        let incremento_km = if tipo_aceite_upper == "SINTETICO" {
            10000
        } else {
            5000
        };
        let proximo_kilometraje = req.kilometraje_ingreso + incremento_km;
        let codigo_ot = Self::generar_codigo_ot(conn)?;

        // 5. Transacción SQLite Atómica
        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        tx.execute(
            "INSERT INTO ordenes_trabajo (codigo_ot, placa, id_mecanico, zanja, estado, kilometraje_ingreso, proximo_kilometraje, observaciones, fecha_ingreso)
             VALUES (?1, ?2, ?3, ?4, 'EN_ESPERA', ?5, ?6, ?7, datetime('now', 'localtime'))",
            params![
                codigo_ot,
                placa,
                req.id_mecanico,
                req.zanja,
                req.kilometraje_ingreso,
                proximo_kilometraje,
                req.observaciones,
            ],
        )?;

        let id_ot = tx.last_insert_rowid();

        tx.execute(
            "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE placa = ?2",
            params![req.kilometraje_ingreso, placa],
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

        // 1. Validar máquina de estados
        Self::validar_transicion_estado(&ot_actual.estado, &nuevo_estado)?;

        // 2. Validar rol y permisos específicos
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
            .map_err(|_| AppError::Validation("Usuario no encontrado o inactivo.".to_string()))?;

        // Cancelación es exclusiva del ADMINISTRADOR
        if nuevo_estado == "CANCELADO" && rol_usuario != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "Acceso denegado: Solo un usuario ADMINISTRADOR puede cancelar una Orden de Trabajo."
                    .to_string(),
            ));
        }

        // Restricción de Mecánico: Solo opera sus propias OTs
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
                    "Para pasar a estado EN_PROCESO debe especificar la zanja (1 o 2).".to_string(),
                )
            })?;

            if z != 1 && z != 2 {
                return Err(AppError::Validation(
                    "La zanja asignada debe ser 1 o 2.".to_string(),
                ));
            }

            let ocupada: i32 = conn.query_row(
                "SELECT COUNT(*) FROM ordenes_trabajo WHERE zanja = ?1 AND estado IN ('EN_ESPERA', 'EN_PROCESO') AND id_ot != ?2",
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
                "UPDATE ordenes_trabajo SET estado = ?1, zanja = ?2 WHERE id_ot = ?3",
                params![nuevo_estado, z, req.id_ot],
            )?;
        } else if nuevo_estado == "CANCELADO" {
            let tx = conn
                .transaction()
                .map_err(|e| AppError::Validation(e.to_string()))?;

            {
                let mut stmt_prods = tx.prepare(
                    "SELECT id_detalle, id_producto, cantidad FROM detalle_ot_productos WHERE id_ot = ?1",
                )?;

                let prods_iter = stmt_prods.query_map([req.id_ot], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, f64>(2)?,
                    ))
                })?;

                for prod_res in prods_iter {
                    let (id_det, id_prod, cant) = prod_res?;

                    let mov_existente: Option<i64> = tx
                        .query_row(
                            "SELECT id_movimiento FROM movimientos_inventario WHERE id_detalle_ot = ?1",
                            [id_det],
                            |row| row.get(0),
                        )
                        .optional()?;

                    if mov_existente.is_some() {
                        let stock_actual: f64 = tx.query_row(
                            "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                            [id_prod],
                            |row| row.get(0),
                        )?;

                        let nuevo_stock = stock_actual + cant;

                        tx.execute(
                            "UPDATE productos SET stock_actual = ?1 WHERE id_producto = ?2",
                            params![nuevo_stock, id_prod],
                        )?;

                        tx.execute(
                            "INSERT INTO movimientos_inventario (id_producto, id_usuario, id_detalle_ot, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento)
                             VALUES (?1, ?2, ?3, 'AJUSTE', ?4, ?5, ?6, ?7, datetime('now', 'localtime'))",
                            params![
                                id_prod,
                                id_u,
                                id_det,
                                stock_actual,
                                cant,
                                nuevo_stock,
                                format!("Devolución por cancelación de OT {}", ot_actual.codigo_ot)
                            ],
                        )?;
                    }
                }
            }

            tx.execute(
                "UPDATE ordenes_trabajo SET estado = 'CANCELADO', zanja = NULL WHERE id_ot = ?1",
                params![req.id_ot],
            )?;

            tx.commit()
                .map_err(|e| AppError::Validation(e.to_string()))?;

            Self::recalcular_kilometraje_vehiculo(conn, &ot_actual.placa)?;
        } else if nuevo_estado == "FINALIZADO" {
            conn.execute(
                "UPDATE ordenes_trabajo SET estado = 'FINALIZADO', zanja = NULL WHERE id_ot = ?1",
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

        // Validar que el nuevo mecánico sea válido y activo
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
                ot.id_ot, ot.codigo_ot, ot.placa, ot.id_mecanico, ot.zanja, ot.estado,
                ot.kilometraje_ingreso, ot.proximo_kilometraje, ot.observaciones, ot.fecha_ingreso,
                u.nombre_completo
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

        let mut lista = Vec::new();
        for item in iter {
            let raw_data = item?;
            let detalles = Self::obtener_detalles_ot(conn, raw_data.0)?;
            let tipo_aceite = Self::obtener_tipo_aceite_ot(conn, raw_data.0)?;

            lista.push(OrdenTrabajoHistorialDto {
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

        let (precio_pen_cents, stock_actual, descripcion_prod): (i64, f64, String) = conn
            .query_row(
                "SELECT precio_venta, stock_actual, descripcion FROM productos WHERE id_producto = ?1 AND activo = 1",
                [req.id_producto],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .map_err(|_| AppError::NotFound("Producto no encontrado o inactivo.".to_string()))?;

        if stock_actual < req.cantidad {
            return Err(AppError::Validation(format!(
                "Stock insuficiente para '{}'. Disponible: {}, solicitado: {}",
                descripcion_prod, stock_actual, req.cantidad
            )));
        }

        // Transacción Atómica SQLite
        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        tx.execute(
            "INSERT INTO detalle_ot_productos (id_ot, id_producto, cantidad, precio_aplicado, descuento)
             VALUES (?1, ?2, ?3, ?4, 0)",
            params![req.id_ot, req.id_producto, req.cantidad, precio_pen_cents],
        )?;

        let id_detalle_ot = tx.last_insert_rowid();

        let stock_resultante = stock_actual - req.cantidad;
        tx.execute(
            "UPDATE productos SET stock_actual = ?1 WHERE id_producto = ?2",
            params![stock_resultante, req.id_producto],
        )?;

        tx.execute(
            "INSERT INTO movimientos_inventario (id_producto, id_usuario, id_detalle_ot, tipo_movimiento, stock_anterior, diferencia, stock_resultante, referencia, fecha_movimiento)
             VALUES (?1, ?2, ?3, 'EGRESO_VENTA', ?4, ?5, ?6, ?7, datetime('now', 'localtime'))",
            params![
                req.id_producto,
                id_usuario_ejecutor,
                id_detalle_ot,
                stock_actual,
                -req.cantidad,
                stock_resultante,
                format!("Asignado a OT {}", ot.codigo_ot)
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

        let detalle_opt: Option<(i64, f64)> = conn
            .query_row(
                "SELECT id_producto, cantidad FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
                params![id_detalle, id_ot],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        if let Some((id_prod, cant)) = detalle_opt {
            let mov_existente: Option<i64> = tx
                .query_row(
                    "SELECT id_movimiento FROM movimientos_inventario WHERE id_detalle_ot = ?1",
                    [id_detalle],
                    |row| row.get(0),
                )
                .optional()?;

            if mov_existente.is_some() {
                let stock_actual: f64 = tx.query_row(
                    "SELECT stock_actual FROM productos WHERE id_producto = ?1",
                    [id_prod],
                    |row| row.get(0),
                )?;

                let stock_resultante = stock_actual + cant;

                tx.execute(
                    "UPDATE productos SET stock_actual = ?1 WHERE id_producto = ?2",
                    params![stock_resultante, id_prod],
                )?;

                tx.execute(
                    "DELETE FROM movimientos_inventario WHERE id_detalle_ot = ?1",
                    [id_detalle],
                )?;
            }
        }

        tx.execute(
            "DELETE FROM detalle_ot_productos WHERE id_detalle = ?1 AND id_ot = ?2",
            params![id_detalle, id_ot],
        )?;

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

        let precio_referencial: i64 = conn
            .query_row(
                "SELECT precio_referencial FROM servicios WHERE id_servicio = ?1 AND activo = 1",
                [req.id_servicio],
                |row| row.get(0),
            )
            .map_err(|_| AppError::NotFound("Servicio no encontrado o inactivo.".to_string()))?;

        conn.execute(
            "INSERT INTO detalle_ot_servicios (id_ot, id_servicio, id_mecanico_ejecutor, precio_aplicado, descuento)
             VALUES (?1, ?2, ?3, ?4, 0)",
            params![req.id_ot, req.id_servicio, id_usuario_ejecutor, precio_referencial],
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
            "SELECT id_servicio, descripcion, (precio_referencial / 100.0) AS precio_base
             FROM servicios
             WHERE activo = 1
             ORDER BY descripcion ASC",
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

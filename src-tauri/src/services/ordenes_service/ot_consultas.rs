use super::helpers::{extraer_identidad_cliente_snapshot, sanitizar_ot_por_rol};
use crate::dto::orden_trabajo_dto::{
    InsumoOtDto, OrdenTrabajoHistorialDto, ServicioOtDto, TableroZanjasDto, ZanjaEstadoDto,
};
use crate::errors::AppError;
use rusqlite::Connection;

pub fn obtener_ot_por_id(
    conn: &Connection,
    id_ot: i64,
    _id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let mut stmt = conn.prepare(
        "SELECT ot.id_ot, ot.codigo_ot, ot.id_vehiculo, ot.placa_snapshot, ot.id_cliente_atendido,
                c.nombre_razon_social, c.numero_documento, ot.id_creador, u_crea.nombre_completo,
                ot.id_mecanico, u_mec.nombre_completo, ot.id_mecanico_terminal, ot.id_zanja_ejecucion,
                ot.estado, ot.kilometraje_ingreso, ot.kilometraje_salida, ot.pedido_cliente,
                ot.resumen_publico, ot.nota_interna, ot.tipo_aceite, ot.intervalo_aceite_km,
                ot.km_proximo, ot.corregido, ot.fecha_ingreso, ot.fecha_inicio, ot.fecha_fin,
                ot.fecha_cancelacion, ot.motivo_cancelacion, ot.cliente_snapshot
         FROM ordenes_trabajo ot
         LEFT JOIN clientes c ON ot.id_cliente_atendido = c.id_cliente
         LEFT JOIN usuarios u_crea ON ot.id_creador = u_crea.id_usuario
         LEFT JOIN usuarios u_mec ON ot.id_mecanico = u_mec.id_usuario
         WHERE ot.id_ot = ?1",
    )?;

    let raw = stmt
        .query_row([id_ot], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, i64>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<i64>>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, Option<i64>>(11)?,
                row.get::<_, Option<i32>>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, i64>(14)?,
                row.get::<_, Option<i64>>(15)?,
                row.get::<_, Option<String>>(16)?,
                row.get::<_, Option<String>>(17)?,
                row.get::<_, Option<String>>(18)?,
                row.get::<_, Option<String>>(19)?,
                row.get::<_, Option<i32>>(20)?,
                row.get::<_, Option<i64>>(21)?,
                row.get::<_, i32>(22)?,
                row.get::<_, String>(23)?,
                row.get::<_, Option<String>>(24)?,
                row.get::<_, Option<String>>(25)?,
                row.get::<_, Option<String>>(26)?,
                row.get::<_, Option<String>>(27)?,
                row.get::<_, Option<String>>(28)?,
            ))
        })
        .map_err(|_| {
            AppError::NotFound(format!(
                "No se encontró la Orden de Trabajo con ID {}.",
                id_ot
            ))
        })?;

    let (nombre_cli, doc_cli, _) = extraer_identidad_cliente_snapshot(raw.28, raw.5, raw.6, None);

    // Cargar Servicios
    let mut stmt_serv = conn.prepare(
        "SELECT id_detalle, id_servicio, codigo_snapshot, nombre_snapshot, cantidad,
                precio_unitario_cent, es_cambio_aceite
         FROM detalle_ot_servicios WHERE id_ot = ?1",
    )?;

    let servicios_iter = stmt_serv.query_map([id_ot], |row| {
        let cant: i32 = row.get(4)?;
        let pu_cent: i64 = row.get(5)?;
        let subtotal_cent = (cant as i64) * pu_cent;

        Ok(ServicioOtDto {
            id_detalle: row.get(0)?,
            id_servicio: row.get(1)?,
            codigo_snapshot: row.get(2)?,
            nombre_snapshot: row.get(3)?,
            cantidad: cant,
            precio_unitario_cent: pu_cent,
            precio_unitario_soles: (pu_cent as f64) / 100.0,
            subtotal_cent,
            subtotal_soles: (subtotal_cent as f64) / 100.0,
            es_cambio_aceite: row.get(6)?,
        })
    })?;

    let mut servicios = Vec::new();
    let mut total_servicios_cent: i64 = 0;
    for s in servicios_iter {
        let item = s?;
        total_servicios_cent += item.subtotal_cent;
        servicios.push(item);
    }

    // Cargar Insumos
    let mut stmt_ins = conn.prepare(
        "SELECT dp.id_detalle, dp.id_producto, dp.codigo_snapshot, dp.nombre_snapshot, dp.unidad_medida,
                dp.cantidad, dp.precio_unitario_cent, dp.costo_unitario_micro,
                COALESCE(SUM(mi.diferencia), 0) as devuelto
         FROM detalle_ot_productos dp
         LEFT JOIN movimientos_inventario mi ON mi.id_consumo_ot = dp.id_detalle AND mi.tipo_movimiento = 'REPOSICION_OT'
         WHERE dp.id_ot = ?1
         GROUP BY dp.id_detalle",
    )?;

    let insumos_iter = stmt_ins.query_map([id_ot], |row| {
        let um: String = row.get(4)?;
        let cant_raw: i64 = row.get(5)?;
        let devuelto_raw: i64 = row.get(8)?;
        let neto_raw = cant_raw - devuelto_raw;

        let pu_cent: i64 = row.get(6)?;

        let subtotal_cent = if um == "ML" {
            ((neto_raw as f64 / 1000.0) * (pu_cent as f64)).round() as i64
        } else {
            neto_raw * pu_cent
        };

        let cant_fmt = if um == "ML" {
            (cant_raw as f64) / 1000.0
        } else {
            cant_raw as f64
        };

        Ok(InsumoOtDto {
            id_detalle: row.get(0)?,
            id_producto: row.get(1)?,
            codigo_snapshot: row.get(2)?,
            nombre_snapshot: row.get(3)?,
            unidad_medida: um,
            cantidad: cant_raw,
            cantidad_formateada: cant_fmt,
            precio_unitario_cent: pu_cent,
            precio_unitario_soles: (pu_cent as f64) / 100.0,
            subtotal_cent,
            subtotal_soles: (subtotal_cent as f64) / 100.0,
            devuelto_ml_und: devuelto_raw,
            neto_ml_und: neto_raw,
            costo_unitario_micro: Some(row.get(7)?),
        })
    })?;

    let mut insumos = Vec::new();
    let mut total_insumos_cent: i64 = 0;
    for i in insumos_iter {
        let item = i?;
        total_insumos_cent += item.subtotal_cent;
        insumos.push(item);
    }

    let total_general_cent = total_servicios_cent + total_insumos_cent;

    let zanja_activa = if raw.13 == "EN_PROCESO" { raw.12 } else { None };

    let dto = OrdenTrabajoHistorialDto {
        id_ot: raw.0,
        codigo_ot: raw.1,
        id_vehiculo: raw.2,
        placa: raw.3,
        id_cliente_atendido: raw.4,
        cliente_nombre: nombre_cli.unwrap_or_else(|| "Cliente Varios".to_string()),
        cliente_documento: doc_cli,
        id_creador: raw.7,
        nombre_creador: raw.8.unwrap_or_default(),
        id_mecanico: raw.9,
        nombre_mecanico: raw.10,
        id_mecanico_terminal: raw.11,
        id_zanja_ejecucion: zanja_activa,
        estado: raw.13,
        kilometraje_ingreso: raw.14,
        kilometraje_salida: raw.15,
        pedido_cliente: raw.16,
        resumen_publico: raw.17,
        nota_interna: raw.18,
        tipo_aceite: raw.19,
        intervalo_aceite_km: raw.20,
        km_proximo: raw.21,
        corregido: raw.22,
        fecha_ingreso: raw.23,
        fecha_inicio: raw.24,
        fecha_fin: raw.25,
        fecha_cancelacion: raw.26,
        motivo_cancelacion: raw.27,
        servicios,
        insumos,
        total_servicios_cent,
        total_insumos_cent,
        total_general_cent,
        total_general_soles: (total_general_cent as f64) / 100.0,
    };

    Ok(sanitizar_ot_por_rol(dto, rol))
}

pub fn obtener_tablero_zanjas(
    conn: &Connection,
    id_usuario: i64,
    rol: &str,
) -> Result<TableroZanjasDto, AppError> {
    let mut zanjas = Vec::new();

    for z in 1..=6 {
        let ot_id: Option<i64> = conn
            .query_row(
                "SELECT id_ot FROM ordenes_trabajo WHERE id_zanja_ejecucion = ?1 AND estado = 'EN_PROCESO'",
                [z],
                |row| row.get(0),
            )
            .ok();

        let (ocupada, mut ot_actual) = if let Some(id) = ot_id {
            (true, Some(obtener_ot_por_id(conn, id, id_usuario, rol)?))
        } else {
            (false, None)
        };

        if rol == "MECANICO" {
            if let Some(ref mut ot) = ot_actual {
                if ot.id_mecanico != Some(id_usuario) {
                    ot.placa = "***".to_string();
                    ot.cliente_nombre = "Ocupada".to_string();
                    ot.cliente_documento = None;
                    ot.servicios.clear();
                    ot.insumos.clear();
                }
            }
        }

        zanjas.push(ZanjaEstadoDto {
            id_zanja: z,
            activa: 1,
            ocupada,
            ot_actual,
        });
    }

    let mut stmt_cola = conn.prepare(
        "SELECT id_ot FROM ordenes_trabajo WHERE estado = 'EN_ESPERA' ORDER BY id_ot ASC",
    )?;

    let cola_ids = stmt_cola.query_map([], |row| row.get::<_, i64>(0))?;
    let mut cola_espera = Vec::new();

    for id_res in cola_ids {
        cola_espera.push(obtener_ot_por_id(conn, id_res?, id_usuario, rol)?);
    }

    Ok(TableroZanjasDto {
        zanjas,
        cola_espera,
    })
}

pub fn listar_ordenes_trabajo(
    conn: &Connection,
    id_usuario: i64,
    rol: &str,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut sql = String::from("SELECT id_ot FROM ordenes_trabajo");

    if rol == "MECANICO" {
        sql.push_str(&format!(" WHERE id_mecanico = {}", id_usuario));
    }

    sql.push_str(" ORDER BY id_ot DESC");

    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map([], |row| row.get::<_, i64>(0))?;

    let mut lista = Vec::new();
    for id_res in iter {
        lista.push(obtener_ot_por_id(conn, id_res?, id_usuario, rol)?);
    }

    Ok(lista)
}

/// Punto 1 y 2 (RF-FVH-04 y RB05): Consulta el historial por `id_vehiculo`
/// filtrando estrictamente las OTs con estado 'FINALIZADO'.
pub fn obtener_historial_por_id_vehiculo(
    conn: &Connection,
    id_vehiculo: i64,
    id_usuario: i64,
    rol: &str,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id_ot 
         FROM ordenes_trabajo 
         WHERE id_vehiculo = ?1 AND estado = 'FINALIZADO' 
         ORDER BY fecha_fin DESC, id_ot DESC",
    )?;

    let iter = stmt.query_map([id_vehiculo], |row| row.get::<_, i64>(0))?;

    let mut lista = Vec::new();
    for id_res in iter {
        lista.push(obtener_ot_por_id(conn, id_res?, id_usuario, rol)?);
    }

    Ok(lista)
}

/// Punto 8: Consulta el historial paginado en BD por `id_vehiculo` con LIMIT y OFFSET.
/// Mantiene el desempate determinista por `id_ot DESC`.
pub fn obtener_historial_paginado_por_id_vehiculo(
    conn: &Connection,
    id_vehiculo: i64,
    id_usuario: i64,
    rol: &str,
    limite: u32,
    offset: u32,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id_ot 
         FROM ordenes_trabajo 
         WHERE id_vehiculo = ?1 AND estado = 'FINALIZADO' 
         ORDER BY fecha_fin DESC, id_ot DESC 
         LIMIT ?2 OFFSET ?3",
    )?;

    let iter = stmt.query_map(rusqlite::params![id_vehiculo, limite, offset], |row| {
        row.get::<_, i64>(0)
    })?;

    let mut lista = Vec::new();
    for id_res in iter {
        lista.push(obtener_ot_por_id(conn, id_res?, id_usuario, rol)?);
    }

    Ok(lista)
}

/// Mantiene compatibilidad buscando primero el ID del vehículo a partir de su placa.
pub fn obtener_historial_por_placa(
    conn: &Connection,
    placa: &str,
    id_usuario: i64,
    rol: &str,
) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
    let placa_clean = placa.trim().to_uppercase();

    let id_vehiculo_opt: Option<i64> = conn
        .query_row(
            "SELECT id_vehiculo FROM vehiculos WHERE placa = ?1",
            [&placa_clean],
            |row| row.get(0),
        )
        .ok();

    if let Some(id_vehiculo) = id_vehiculo_opt {
        obtener_historial_por_id_vehiculo(conn, id_vehiculo, id_usuario, rol)
    } else {
        let mut stmt = conn.prepare(
            "SELECT id_ot 
             FROM ordenes_trabajo 
             WHERE placa_snapshot = ?1 AND estado = 'FINALIZADO' 
             ORDER BY fecha_fin DESC, id_ot DESC",
        )?;

        let iter = stmt.query_map([&placa_clean], |row| row.get::<_, i64>(0))?;

        let mut lista = Vec::new();
        for id_res in iter {
            lista.push(obtener_ot_por_id(conn, id_res?, id_usuario, rol)?);
        }

        Ok(lista)
    }
}

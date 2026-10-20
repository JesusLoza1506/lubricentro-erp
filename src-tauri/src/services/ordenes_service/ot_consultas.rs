use super::helpers::{
    calcular_proximo_kilometraje, extraer_identidad_cliente_snapshot, obtener_tipo_aceite_ot,
};
use crate::dto::orden_trabajo_dto::{DetalleItemOtDto, OrdenTrabajoHistorialDto};
use crate::errors::AppError;
use rusqlite::{Connection, OptionalExtension};

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
            u.nombre_completo, ot.corregido, c.nombre_razon_social, c.numero_documento, c.tipo_documento,
            v.marca, v.modelo, ot.cliente_snapshot
         FROM ordenes_trabajo ot
         LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
         LEFT JOIN clientes c ON ot.id_cliente_atendido = c.id_cliente
         LEFT JOIN vehiculos v ON ot.id_vehiculo = v.id_vehiculo",
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
            row.get::<_, Option<String>>(11)?,
            row.get::<_, Option<String>>(12)?,
            row.get::<_, Option<String>>(13)?,
            row.get::<_, Option<String>>(14)?,
            row.get::<_, Option<String>>(15)?,
            row.get::<_, Option<String>>(16)?,
        ))
    })?;

    let mut lista = Vec::new();
    for item in iter {
        let raw_data = item?;
        let detalles = obtener_detalles_ot(conn, raw_data.0)?;
        let tipo_aceite = obtener_tipo_aceite_ot(conn, raw_data.0)?;
        let proximo_km =
            calcular_proximo_kilometraje(conn, raw_data.0, raw_data.6, tipo_aceite.as_deref());

        let (nombre_cli, doc_cli, tipo_doc_cli) =
            extraer_identidad_cliente_snapshot(raw_data.16, raw_data.11, raw_data.12, raw_data.13);

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
            nombre_cliente: nombre_cli,
            documento_cliente: doc_cli,
            tipo_documento_cliente: tipo_doc_cli,
            marca_vehiculo: raw_data.14,
            modelo_vehiculo: raw_data.15,
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
            u.nombre_completo, ot.corregido, c.nombre_razon_social, c.numero_documento, c.tipo_documento,
            v.marca, v.modelo, ot.cliente_snapshot
         FROM ordenes_trabajo ot
         LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
         LEFT JOIN clientes c ON ot.id_cliente_atendido = c.id_cliente
         LEFT JOIN vehiculos v ON ot.id_vehiculo = v.id_vehiculo
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
                row.get::<_, Option<String>>(11)?,
                row.get::<_, Option<String>>(12)?,
                row.get::<_, Option<String>>(13)?,
                row.get::<_, Option<String>>(14)?,
                row.get::<_, Option<String>>(15)?,
                row.get::<_, Option<String>>(16)?,
            ))
        })
        .map_err(|_| AppError::NotFound(format!("No se encontró la OT con ID {}", id_ot)))?;

    let detalles = obtener_detalles_ot(conn, raw_data.0)?;
    let tipo_aceite = obtener_tipo_aceite_ot(conn, raw_data.0)?;
    let proximo_km =
        calcular_proximo_kilometraje(conn, raw_data.0, raw_data.6, tipo_aceite.as_deref());

    let (nombre_cli, doc_cli, tipo_doc_cli) =
        extraer_identidad_cliente_snapshot(raw_data.16, raw_data.11, raw_data.12, raw_data.13);

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
        nombre_cliente: nombre_cli,
        documento_cliente: doc_cli,
        tipo_documento_cliente: tipo_doc_cli,
        marca_vehiculo: raw_data.14,
        modelo_vehiculo: raw_data.15,
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
            u.nombre_completo, ot.corregido, c.nombre_razon_social, c.numero_documento, c.tipo_documento,
            v.marca, v.modelo, ot.cliente_snapshot
         FROM ordenes_trabajo ot
         LEFT JOIN usuarios u ON ot.id_mecanico = u.id_usuario
         LEFT JOIN clientes c ON ot.id_cliente_atendido = c.id_cliente
         LEFT JOIN vehiculos v ON ot.id_vehiculo = v.id_vehiculo
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
            row.get::<_, Option<String>>(11)?,
            row.get::<_, Option<String>>(12)?,
            row.get::<_, Option<String>>(13)?,
            row.get::<_, Option<String>>(14)?,
            row.get::<_, Option<String>>(15)?,
            row.get::<_, Option<String>>(16)?,
        ))
    })?;

    let mut historial = Vec::new();
    for item in iter {
        let raw_data = item?;
        let detalles = obtener_detalles_ot(conn, raw_data.0)?;
        let tipo_aceite = obtener_tipo_aceite_ot(conn, raw_data.0)?;
        let proximo_km =
            calcular_proximo_kilometraje(conn, raw_data.0, raw_data.6, tipo_aceite.as_deref());

        let (nombre_cli, doc_cli, tipo_doc_cli) =
            extraer_identidad_cliente_snapshot(raw_data.16, raw_data.11, raw_data.12, raw_data.13);

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
            nombre_cliente: nombre_cli,
            documento_cliente: doc_cli,
            tipo_documento_cliente: tipo_doc_cli,
            marca_vehiculo: raw_data.14,
            modelo_vehiculo: raw_data.15,
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
        obtener_historial_por_id_vehiculo(conn, id_v)
    } else {
        Ok(Vec::new())
    }
}

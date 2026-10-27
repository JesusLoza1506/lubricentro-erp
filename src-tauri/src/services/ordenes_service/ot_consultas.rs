use super::helpers::{extraer_identidad_cliente_snapshot, sanitizar_ot_por_rol_usuario};
use crate::dto::orden_trabajo_dto::{
    InsumoOtDto, OrdenTrabajoHistorialDto, ServicioOtDto, TableroZanjasDto, ZanjaEstadoDto,
};
use crate::errors::AppError;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FiltroPaginacionOtDto {
    pub pagina: Option<u32>,
    pub limite: Option<u32>,
    pub placa: Option<String>,
    pub codigo_ot: Option<String>,
    pub estado: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub id_mecanico: Option<i64>,
    pub solo_propias: Option<bool>,
    pub solo_libres: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ResultadoPaginadoOtDto {
    pub items: Vec<OrdenTrabajoHistorialDto>,
    pub total_items: u32,
    pub pagina_actual: u32,
    pub total_paginas: u32,
}

pub fn obtener_ot_por_id(
    conn: &Connection,
    id_ot: i64,
    id_usuario: i64,
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

    let rol_upper = rol.to_uppercase();
    if rol_upper == "MECANICO" {
        let es_ot_propia = raw.9 == Some(id_usuario);
        let es_ot_libre_espera = raw.9.is_none() && raw.13 == "EN_ESPERA";

        if !es_ot_propia && !es_ot_libre_espera && raw.13 != "FINALIZADO" {
            return Err(AppError::Validation(
                "No tiene autorización para consultar esta Orden de Trabajo ajena en proceso."
                    .to_string(),
            ));
        }
    }

    let (nombre_cli, doc_cli, _) = extraer_identidad_cliente_snapshot(raw.28, raw.5, raw.6, None);

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

    let mut dto = OrdenTrabajoHistorialDto {
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

    if dto.estado == "EN_ESPERA" && rol_upper != "ADMINISTRADOR" {
        let es_propietario_o_creador =
            dto.id_mecanico == Some(id_usuario) || dto.id_creador == id_usuario;
        if !es_propietario_o_creador {
            dto.nota_interna = None;
        }
    }

    if rol_upper == "MECANICO" {
        let es_ot_propia = dto.id_mecanico == Some(id_usuario);
        let es_ot_libre = dto.id_mecanico.is_none() && dto.estado == "EN_ESPERA";

        if !es_ot_propia && !es_ot_libre {
            dto.nota_interna = None;
            dto.pedido_cliente = None;
            dto.cliente_documento = None;
            if dto.estado == "EN_PROCESO" {
                dto.codigo_ot = "***".to_string();
                dto.placa = "***".to_string();
                dto.cliente_nombre = "Ocupada".to_string();
                dto.nombre_mecanico = None;
                dto.servicios.clear();
                dto.insumos.clear();
                dto.total_servicios_cent = 0;
                dto.total_insumos_cent = 0;
                dto.total_general_cent = 0;
                dto.total_general_soles = 0.0;
            }
        }
    }

    Ok(sanitizar_ot_por_rol_usuario(dto, rol, id_usuario))
}

/// Consulta pública sanitizada por rol para OTs FINALIZADAS en Ficha Vehicular e Historial
pub fn obtener_ot_finalizada_publica(
    conn: &Connection,
    id_ot: i64,
    id_usuario: i64,
    rol: &str,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let dto = obtener_ot_por_id(conn, id_ot, id_usuario, rol)?;
    Ok(sanitizar_ot_por_rol_usuario(dto, rol, id_usuario))
}

/// Auxiliar seguro para construir DTOs del tablero de zanjas sin romper la ejecución si una OT es ajena
fn obtener_ot_para_tablero(
    conn: &Connection,
    id_ot: i64,
    id_usuario: i64,
    rol: &str,
) -> Option<OrdenTrabajoHistorialDto> {
    match obtener_ot_por_id(conn, id_ot, id_usuario, rol) {
        Ok(dto) => Some(dto),
        Err(AppError::Validation(_)) => {
            // Si fue rechazada por ser de otro mecánico en proceso, devolvemos un DTO sanitizado de "Ocupada"
            let (placa, cod): (String, String) = conn
                .query_row(
                    "SELECT placa_snapshot, codigo_ot FROM ordenes_trabajo WHERE id_ot = ?1",
                    [id_ot],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .unwrap_or_else(|_| ("***".to_string(), "***".to_string()));

            Some(OrdenTrabajoHistorialDto {
                id_ot,
                codigo_ot: if rol.eq_ignore_ascii_case("MECANICO") {
                    "***".to_string()
                } else {
                    cod
                },
                id_vehiculo: 0,
                placa: if rol.eq_ignore_ascii_case("MECANICO") {
                    "***".to_string()
                } else {
                    placa
                },
                id_cliente_atendido: 0,
                cliente_nombre: "Ocupada".to_string(),
                cliente_documento: None,
                id_creador: 0,
                nombre_creador: String::new(),
                id_mecanico: None,
                nombre_mecanico: None,
                id_mecanico_terminal: None,
                id_zanja_ejecucion: None,
                estado: "EN_PROCESO".to_string(),
                kilometraje_ingreso: 0,
                kilometraje_salida: None,
                pedido_cliente: None,
                resumen_publico: None,
                nota_interna: None,
                tipo_aceite: None,
                intervalo_aceite_km: None,
                km_proximo: None,
                corregido: 0,
                fecha_ingreso: String::new(),
                fecha_inicio: None,
                fecha_fin: None,
                fecha_cancelacion: None,
                motivo_cancelacion: None,
                servicios: vec![],
                insumos: vec![],
                total_servicios_cent: 0,
                total_insumos_cent: 0,
                total_general_cent: 0,
                total_general_soles: 0.0,
            })
        }
        Err(_) => None,
    }
}

pub fn obtener_tablero_zanjas(
    conn: &Connection,
    id_usuario: i64,
    rol: &str,
) -> Result<TableroZanjasDto, AppError> {
    let mut zanjas = Vec::new();

    let mut stmt_zanjas =
        conn.prepare("SELECT id_zanja, activa FROM zanjas WHERE activa = 1 ORDER BY id_zanja ASC")?;
    let zanjas_rows =
        stmt_zanjas.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)))?;

    for z_res in zanjas_rows {
        let (z_id, activa) = z_res?;

        let ot_id: Option<i64> = conn
            .query_row(
                "SELECT id_ot FROM ordenes_trabajo WHERE id_zanja_ejecucion = ?1 AND estado = 'EN_PROCESO'",
                [z_id],
                |row| row.get(0),
            )
            .ok();

        let (ocupada, ot_actual) = if let Some(id) = ot_id {
            let dto_opt = obtener_ot_para_tablero(conn, id, id_usuario, rol);
            (dto_opt.is_some(), dto_opt)
        } else {
            (false, None)
        };

        zanjas.push(ZanjaEstadoDto {
            id_zanja: z_id,
            activa,
            ocupada,
            ot_actual,
        });
    }

    let is_mecanico = rol.eq_ignore_ascii_case("MECANICO");
    let ids_cola: Vec<i64> = if is_mecanico {
        let mut stmt = conn.prepare(
            "SELECT id_ot FROM ordenes_trabajo WHERE estado = 'EN_ESPERA' AND (id_mecanico IS NULL OR id_mecanico = ?1) ORDER BY id_ot ASC"
        )?;
        let rows = stmt.query_map([id_usuario], |row| row.get::<_, i64>(0))?;
        rows.collect::<Result<Vec<_>, _>>()?
    } else {
        let mut stmt = conn.prepare(
            "SELECT id_ot FROM ordenes_trabajo WHERE estado = 'EN_ESPERA' ORDER BY id_ot ASC",
        )?;
        let rows = stmt.query_map([], |row| row.get::<_, i64>(0))?;
        rows.collect::<Result<Vec<_>, _>>()?
    };

    let mut cola_espera = Vec::new();
    for id_ot in ids_cola {
        if let Some(dto) = obtener_ot_para_tablero(conn, id_ot, id_usuario, rol) {
            cola_espera.push(dto);
        }
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

    if rol.to_uppercase() == "MECANICO" {
        sql.push_str(&format!(
            " WHERE (id_mecanico IS NULL OR id_mecanico = {})",
            id_usuario
        ));
    }

    sql.push_str(" ORDER BY id_ot DESC");

    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map([], |row| row.get::<_, i64>(0))?;

    let mut lista = Vec::new();
    for id_res in iter {
        if let Some(dto) = obtener_ot_para_tablero(conn, id_res?, id_usuario, rol) {
            lista.push(dto);
        }
    }

    Ok(lista)
}

pub fn listar_ordenes_trabajo_paginado(
    conn: &Connection,
    filtros: FiltroPaginacionOtDto,
    id_usuario: i64,
    rol: &str,
) -> Result<ResultadoPaginadoOtDto, AppError> {
    let pagina = filtros.pagina.unwrap_or(1).max(1);
    let limite = filtros.limite.unwrap_or(20).max(1);
    let offset = (pagina - 1) * limite;

    let mut condiciones = Vec::new();
    let mut params_where: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let (Some(ref fi), Some(ref ff)) = (&filtros.fecha_inicio, &filtros.fecha_fin) {
        if !fi.trim().is_empty() && !ff.trim().is_empty() {
            let diff_dias: Option<f64> = conn
                .query_row(
                    "SELECT JULIANDAY(?2) - JULIANDAY(?1)",
                    [fi.trim(), ff.trim()],
                    |row| row.get(0),
                )
                .ok();

            if let Some(diff) = diff_dias {
                if diff < 0.0 {
                    return Err(AppError::Validation(
                        "La fecha de fin no puede ser anterior a la fecha de inicio.".to_string(),
                    ));
                }
                if diff > 366.0 {
                    return Err(AppError::Validation(
                        "El rango de fechas no puede superar los 366 días.".to_string(),
                    ));
                }
            }
        }
    }

    if rol.to_uppercase() == "MECANICO" {
        condiciones.push("(id_mecanico IS NULL OR id_mecanico = ?)".to_string());
        params_where.push(Box::new(id_usuario));
    }

    if let Some(ref p) = filtros.placa {
        if !p.trim().is_empty() {
            condiciones.push("placa_snapshot LIKE ?".to_string());
            params_where.push(Box::new(format!("%{}%", p.trim().to_uppercase())));
        }
    }

    if let Some(ref c) = filtros.codigo_ot {
        if !c.trim().is_empty() {
            condiciones.push("codigo_ot LIKE ?".to_string());
            params_where.push(Box::new(format!("%{}%", c.trim().to_uppercase())));
        }
    }

    if let Some(ref est) = filtros.estado {
        if !est.trim().is_empty() {
            condiciones.push("estado = ?".to_string());
            params_where.push(Box::new(est.trim().to_uppercase()));
        }
    }

    if let Some(id_mec) = filtros.id_mecanico {
        if id_mec > 0 {
            condiciones.push("id_mecanico = ?".to_string());
            params_where.push(Box::new(id_mec));
        }
    }

    if filtros.solo_propias == Some(true) {
        condiciones.push("id_mecanico = ?".to_string());
        params_where.push(Box::new(id_usuario));
    }

    if filtros.solo_libres == Some(true) {
        condiciones.push("id_mecanico IS NULL".to_string());
    }

    if let Some(ref fi) = filtros.fecha_inicio {
        if !fi.trim().is_empty() {
            condiciones.push("DATE(fecha_ingreso) >= DATE(?)".to_string());
            params_where.push(Box::new(fi.trim().to_string()));
        }
    }

    if let Some(ref ff) = filtros.fecha_fin {
        if !ff.trim().is_empty() {
            condiciones.push("DATE(fecha_ingreso) <= DATE(?)".to_string());
            params_where.push(Box::new(ff.trim().to_string()));
        }
    }

    let where_clause = if condiciones.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", condiciones.join(" AND "))
    };

    let sql_count = format!("SELECT COUNT(*) FROM ordenes_trabajo{}", where_clause);
    let mut stmt_count = conn.prepare(&sql_count)?;
    let params_slice: Vec<&dyn rusqlite::ToSql> = params_where.iter().map(|p| p.as_ref()).collect();
    let total_items: u32 = stmt_count.query_row(params_slice.as_slice(), |row| row.get(0))?;

    let sql_select = format!(
        "SELECT id_ot FROM ordenes_trabajo{} ORDER BY id_ot DESC LIMIT ? OFFSET ?",
        where_clause
    );

    let mut stmt_select = conn.prepare(&sql_select)?;
    let mut params_final = params_where;
    params_final.push(Box::new(limite));
    params_final.push(Box::new(offset));

    let params_final_slice: Vec<&dyn rusqlite::ToSql> =
        params_final.iter().map(|p| p.as_ref()).collect();
    let iter = stmt_select.query_map(params_final_slice.as_slice(), |row| row.get::<_, i64>(0))?;

    let mut items = Vec::new();
    for id_res in iter {
        if let Some(dto) = obtener_ot_para_tablero(conn, id_res?, id_usuario, rol) {
            items.push(dto);
        }
    }

    let total_paginas = (total_items as f64 / limite as f64).ceil() as u32;

    Ok(ResultadoPaginadoOtDto {
        items,
        total_items,
        pagina_actual: pagina,
        total_paginas,
    })
}

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
        lista.push(obtener_ot_finalizada_publica(
            conn, id_res?, id_usuario, rol,
        )?);
    }

    Ok(lista)
}

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
        lista.push(obtener_ot_finalizada_publica(
            conn, id_res?, id_usuario, rol,
        )?);
    }

    Ok(lista)
}

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
            lista.push(obtener_ot_finalizada_publica(
                conn, id_res?, id_usuario, rol,
            )?);
        }

        Ok(lista)
    }
}

use super::validaciones::validar_motivo;
use super::vehiculo_crud::obtener_vehiculo_por_placa;
use crate::dto::vehiculo_dto::{
    CorregirHistorialOtPayloadDto, FichaVehicularCompletaDto, HistorialOtPublicoDto,
    InsumoOtPublicoDto, ProximoCambioAceiteDto, ResultadoPaginadoDto, ServicioOtPublicoDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::{params, Connection};

pub fn corregir_historial_ot(
    conn: &Connection,
    payload: CorregirHistorialOtPayloadDto,
    id_usuario: Option<i64>,
    username: Option<&str>,
    rol: Option<&str>,
) -> Result<(), AppError> {
    let motivo = validar_motivo(&payload.motivo)?;
    let resumen = payload.resumen_publico.trim();

    if resumen.is_empty() || resumen.len() > 1000 {
        return Err(AppError::Validation(
            "El resumen público debe tener entre 1 y 1000 caracteres.".to_string(),
        ));
    }

    let (resumen_old, km_old, tipo_aceite_old): (String, i64, Option<String>) = conn
        .query_row(
            "SELECT resumen_publico, kilometraje_salida, tipo_aceite FROM ordenes_trabajo WHERE id_ot = ?1 AND estado = 'FINALIZADO'",
            [payload.id_ot],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_| AppError::NotFound("OT finalizada no encontrada.".to_string()))?;

    let intervalo_km = match payload.tipo_aceite.as_deref() {
        Some("MINERAL") => Some(5000),
        Some("SINTETICO") => Some(10000),
        _ => None,
    };

    conn.execute(
        "UPDATE ordenes_trabajo
         SET resumen_publico = ?1,
             kilometraje_salida = ?2,
             tipo_aceite = ?3,
             intervalo_aceite_km = ?4,
             corregido = 1
         WHERE id_ot = ?5 AND estado = 'FINALIZADO'",
        params![
            resumen,
            payload.kilometraje_salida,
            payload.tipo_aceite,
            intervalo_km,
            payload.id_ot
        ],
    )?;

    let antes_json = serde_json::json!({
        "resumen_publico": resumen_old,
        "kilometraje_salida": km_old,
        "tipo_aceite": tipo_aceite_old,
    })
    .to_string();

    let despues_json = serde_json::json!({
        "resumen_publico": resumen,
        "kilometraje_salida": payload.kilometraje_salida,
        "tipo_aceite": payload.tipo_aceite,
    })
    .to_string();

    AuditoriaService::registrar_v5(
        conn,
        id_usuario,
        if id_usuario.is_some() {
            "USUARIO"
        } else {
            "NO_IDENTIFICADO"
        },
        username,
        rol,
        None,
        "FICHA_VEHICULAR",
        "CORREGIR_HISTORIAL_OT",
        Some("ordenes_trabajo"),
        Some(&payload.id_ot.to_string()),
        Some(&antes_json),
        Some(&despues_json),
        Some(&motivo),
        "OK",
    )?;

    Ok(())
}

/// Obtiene la ficha vehicular con historial paginado e indicadores estáticos independientes de la página.
pub fn obtener_ficha_vehicular_completa(
    conn: &Connection,
    placa: &str,
    rol_ejecutor: Option<&str>,
    pagina: u32,
    limite: u32,
) -> Result<FichaVehicularCompletaDto, AppError> {
    let mut vehiculo = obtener_vehiculo_por_placa(conn, placa, rol_ejecutor)?;
    let id_u = 0;
    let r = rol_ejecutor.unwrap_or("ADMINISTRADOR");
    let es_mecanico = r.to_uppercase() == "MECANICO";

    if es_mecanico {
        vehiculo.telefono_cliente = None;
        vehiculo.correo_cliente = None;
        vehiculo.direccion_cliente = None;
    }

    let limite_validado = if limite == 0 { 20 } else { limite };
    let pagina_validada = if pagina == 0 { 1 } else { pagina };
    let offset = (pagina_validada - 1) * limite_validado;

    // Conteo total de OTs strictly FINALIZADAS para este vehículo
    let total_registros: u64 = conn.query_row(
        "SELECT COUNT(*) FROM ordenes_trabajo WHERE id_vehiculo = ?1 AND estado = 'FINALIZADO'",
        [vehiculo.id_vehiculo],
        |row| row.get(0),
    )?;

    let total_paginas = ((total_registros as f64) / (limite_validado as f64)).ceil() as u32;
    let total_paginas_final = if total_paginas == 0 { 1 } else { total_paginas };

    // Obtención del historial con paginación real en BD
    let historial_raw = OrdenesService::obtener_historial_paginado_por_id_vehiculo(
        conn,
        vehiculo.id_vehiculo,
        id_u,
        r,
        limite_validado,
        offset,
    )?;

    let historial_publico: Vec<HistorialOtPublicoDto> = historial_raw
        .into_iter()
        .map(|ot| {
            let nombre_cliente = if ot.cliente_nombre.trim().is_empty() {
                None
            } else {
                Some(ot.cliente_nombre)
            };

            HistorialOtPublicoDto {
                id_ot: ot.id_ot,
                codigo_ot: ot.codigo_ot,
                placa_snapshot: ot.placa,
                fecha_fin: ot.fecha_fin,
                kilometraje_salida: ot.kilometraje_salida,
                resumen_publico: ot.resumen_publico,
                tipo_aceite: ot.tipo_aceite,
                km_proximo: ot.km_proximo,
                corregido: ot.corregido,
                nombre_cliente,
                documento_cliente: ot.cliente_documento,
                servicios: ot
                    .servicios
                    .into_iter()
                    .map(|s| ServicioOtPublicoDto {
                        id_detalle: s.id_detalle,
                        id_servicio: Some(s.id_servicio),
                        codigo_snapshot: s.codigo_snapshot,
                        nombre_snapshot: s.nombre_snapshot,
                        cantidad: s.cantidad,
                        es_cambio_aceite: s.es_cambio_aceite,
                    })
                    .collect(),
                insumos: ot
                    .insumos
                    .into_iter()
                    .map(|i| InsumoOtPublicoDto {
                        id_detalle: i.id_detalle,
                        id_producto: Some(i.id_producto),
                        codigo_snapshot: i.codigo_snapshot,
                        nombre_snapshot: i.nombre_snapshot,
                        unidad_medida: i.unidad_medida,
                        cantidad_formateada: i.cantidad_formateada,
                    })
                    .collect(),
            }
        })
        .collect();

    // Consulta global e independiente del ÚLTIMO SERVICIO (Página 1, offset 0, limit 1)
    let ultimo_servicio = if total_registros > 0 {
        let raw_servicio = OrdenesService::obtener_historial_paginado_por_id_vehiculo(
            conn,
            vehiculo.id_vehiculo,
            id_u,
            r,
            1,
            0,
        )?;
        raw_servicio
            .into_iter()
            .next()
            .map(|ot| HistorialOtPublicoDto {
                id_ot: ot.id_ot,
                codigo_ot: ot.codigo_ot,
                placa_snapshot: ot.placa,
                fecha_fin: ot.fecha_fin,
                kilometraje_salida: ot.kilometraje_salida,
                resumen_publico: ot.resumen_publico,
                tipo_aceite: ot.tipo_aceite.clone(),
                km_proximo: ot.km_proximo,
                corregido: ot.corregido,
                nombre_cliente: if ot.cliente_nombre.trim().is_empty() {
                    None
                } else {
                    Some(ot.cliente_nombre)
                },
                documento_cliente: ot.cliente_documento,
                servicios: vec![],
                insumos: vec![],
            })
    } else {
        None
    };

    // Consulta global e independiente del ÚLTIMO CAMBIO DE ACEITE por FECHA_FIN DESC, ID_OT DESC (Punto 3 de reevaluación)
    let ultimo_cambio_aceite = {
        let mut stmt = conn.prepare(
            "SELECT id_ot, codigo_ot, placa_snapshot FROM ordenes_trabajo 
             WHERE id_vehiculo = ?1 AND estado = 'FINALIZADO' AND tipo_aceite IS NOT NULL AND tipo_aceite != ''
             ORDER BY fecha_fin DESC, id_ot DESC LIMIT 1",
        )?;
        let row_opt = stmt
            .query_row([vehiculo.id_vehiculo], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .ok();

        if let Some((id_ot, codigo_ot, placa_snapshot)) = row_opt {
            conn.query_row(
                "SELECT fecha_fin, kilometraje_salida, resumen_publico, tipo_aceite, (kilometraje_salida + COALESCE(intervalo_aceite_km, 0)) 
                 FROM ordenes_trabajo WHERE id_ot = ?1",
                [id_ot],
                |r| {
                    Ok(HistorialOtPublicoDto {
                        id_ot,
                        codigo_ot,
                        placa_snapshot,
                        fecha_fin: r.get(0)?,
                        kilometraje_salida: r.get(1)?,
                        resumen_publico: r.get(2)?,
                        tipo_aceite: r.get(3)?,
                        km_proximo: r.get(4)?,
                        corregido: 0,
                        nombre_cliente: None,
                        documento_cliente: None,
                        servicios: vec![],
                        insumos: vec![],
                    })
                },
            )
            .ok()
        } else {
            None
        }
    };

    Ok(FichaVehicularCompletaDto {
        vehiculo,
        historial: ResultadoPaginadoDto {
            items: historial_publico,
            total_registros,
            pagina_actual: pagina_validada,
            total_paginas: total_paginas_final,
        },
        ultimo_servicio,
        ultimo_cambio_aceite,
    })
}

/// Obtiene los próximos cambios de aceite con paginación y filtros dinámicos en SQL (Puntos 1 y 2 de reevaluación)
pub fn obtener_proximos_cambios_aceite_paginado(
    conn: &Connection,
    rol_ejecutor: Option<&str>,
    pagina: u32,
    limite: u32,
    busqueda: Option<&str>,
    filtro_causa: Option<&str>,
) -> Result<ResultadoPaginadoDto<ProximoCambioAceiteDto>, AppError> {
    let limite_validado = if limite == 0 { 20 } else { limite };
    let pagina_validada = if pagina == 0 { 1 } else { pagina };
    let offset = (pagina_validada - 1) * limite_validado;

    let mut where_clause = String::from(" WHERE 1=1");
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(b) = busqueda {
        let b_clean = b.trim();
        if !b_clean.is_empty() {
            where_clause.push_str(" AND (placa LIKE ? OR nombre_razon_social LIKE ?)");
            let pattern = format!("%{}%", b_clean);
            params_vec.push(Box::new(pattern.clone()));
            params_vec.push(Box::new(pattern));
        }
    }

    if let Some(c) = filtro_causa {
        let c_clean = c.trim().to_uppercase();
        if !c_clean.is_empty() && c_clean != "TODOS" {
            where_clause.push_str(" AND causa = ?");
            params_vec.push(Box::new(c_clean));
        }
    }

    let sql_count = format!("SELECT COUNT(*) FROM v_proximos_cambios{}", where_clause);
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    let total_registros: u64 = conn.query_row(&sql_count, params_refs.as_slice(), |r| r.get(0))?;

    let total_paginas = ((total_registros as f64) / (limite_validado as f64)).ceil() as u32;
    let total_paginas_final = if total_paginas == 0 { 1 } else { total_paginas };

    let sql_select = format!(
        "SELECT id_vehiculo, placa, kilometraje_actual, id_cliente, nombre_razon_social, telefono,
                id_ot, fecha_servicio, tipo_aceite, intervalo_aceite_km, km_proximo, causa
         FROM v_proximos_cambios{}
         ORDER BY fecha_servicio ASC, id_ot ASC
         LIMIT ? OFFSET ?",
        where_clause
    );

    let mut params_select = params_vec;
    params_select.push(Box::new(limite_validado));
    params_select.push(Box::new(offset));

    let params_select_refs: Vec<&dyn rusqlite::ToSql> =
        params_select.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql_select)?;

    let es_mecanico = rol_ejecutor
        .map(|r| r.to_uppercase() == "MECANICO")
        .unwrap_or(false);

    let rows = stmt.query_map(params_select_refs.as_slice(), |row| {
        let mut telefono: Option<String> = row.get(5)?;
        if es_mecanico {
            telefono = None;
        }

        Ok(ProximoCambioAceiteDto {
            id_vehiculo: row.get(0)?,
            placa: row.get(1)?,
            kilometraje_actual: row.get(2)?,
            id_cliente: row.get(3)?,
            nombre_razon_social: row.get(4)?,
            telefono,
            id_ot: row.get(6)?,
            fecha_servicio: row.get(7)?,
            tipo_aceite: row.get(8)?,
            intervalo_aceite_km: row.get(9)?,
            km_proximo: row.get(10)?,
            causa: row.get(11)?,
        })
    })?;

    let mut items = Vec::new();
    for r in rows {
        items.push(r?);
    }

    Ok(ResultadoPaginadoDto {
        items,
        total_registros,
        pagina_actual: pagina_validada,
        total_paginas: total_paginas_final,
    })
}

/// Obtiene todos los próximos cambios de aceite (compatibilidad para llamadas sin paginación explícita)
pub fn obtener_proximos_cambios_aceite(
    conn: &Connection,
    rol_ejecutor: Option<&str>,
) -> Result<Vec<ProximoCambioAceiteDto>, AppError> {
    let res = obtener_proximos_cambios_aceite_paginado(conn, rol_ejecutor, 1, 10_000, None, None)?;
    Ok(res.items)
}

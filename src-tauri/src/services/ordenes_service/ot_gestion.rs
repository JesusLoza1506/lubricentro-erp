use super::helpers::{
    generar_codigo_ot, recalcular_kilometraje_vehiculo, validar_mecanico_activo,
    validar_transicion_estado, DatosVehiculoCliente,
};
use super::ot_consultas::obtener_ot_por_id;
use crate::dto::orden_trabajo_dto::{
    CambiarEstadoOtDto, CrearOrdenTrabajoDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto,
};
use crate::errors::AppError;
use rusqlite::{params, Connection};
use uuid::Uuid;

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
    if !tipo_aceite_upper.is_empty()
        && tipo_aceite_upper != "MINERAL"
        && tipo_aceite_upper != "SINTETICO"
    {
        return Err(AppError::Validation(format!(
            "Tipo de aceite '{}' no válido. Solo se permite 'MINERAL' o 'SINTETICO'.",
            req.tipo_aceite
        )));
    }

    let id_mecanico_final = if req.id_mecanico > 0 {
        validar_mecanico_activo(conn, req.id_mecanico)?;
        Some(req.id_mecanico)
    } else {
        None
    };

    let datos_vc = conn
        .query_row(
            "SELECT v.id_vehiculo, v.id_cliente, COALESCE(v.kilometraje_actual, 0),
                    c.nombre_razon_social, c.numero_documento, c.telefono, c.direccion, c.correo, c.tipo_documento
             FROM vehiculos v
             JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.placa = ?1 AND v.activo = 1",
            [&placa],
            |row| {
                Ok(DatosVehiculoCliente {
                    id_vehiculo: row.get(0)?,
                    id_cliente_atendido: row.get(1)?,
                    km_actual_ficha: row.get(2)?,
                    nombre_cliente: row.get(3)?,
                    doc_cliente: row.get(4)?,
                    tel_cliente: row.get(5)?,
                    dir_cliente: row.get(6)?,
                    mail_cliente: row.get(7)?,
                    tipo_doc_cliente: row.get(8)?,
                })
            },
        )
        .map_err(|_| {
            AppError::NotFound(format!(
                "El vehículo con placa {} no se encuentra registrado o está desactivado.",
                placa
            ))
        })?;

    if req.kilometraje_ingreso < datos_vc.km_actual_ficha {
        return Err(AppError::Validation(format!(
            "El kilometraje de ingreso ({} km) no puede ser inferior al kilometraje actual registrado en la Ficha Vehicular ({} km).",
            req.kilometraje_ingreso, datos_vc.km_actual_ficha
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

    let codigo_ot = generar_codigo_ot(conn)?;
    let id_operacion = Uuid::new_v4().to_string();

    let cliente_snapshot = serde_json::json!({
        "nombre": datos_vc.nombre_cliente,
        "documento": datos_vc.doc_cliente,
        "tipo_documento": datos_vc.tipo_doc_cliente,
        "telefono": datos_vc.tel_cliente,
        "direccion": datos_vc.dir_cliente,
        "correo": datos_vc.mail_cliente
    })
    .to_string();

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
            datos_vc.id_vehiculo,
            datos_vc.id_cliente_atendido,
            placa,
            cliente_snapshot,
            id_creador,
            id_mecanico_final,
            req.zanja,
            req.kilometraje_ingreso,
            req.observaciones,
            if tipo_aceite_upper.is_empty() { None } else { Some(tipo_aceite_upper) }
        ],
    )?;

    let id_ot = tx.last_insert_rowid();

    tx.execute(
        "UPDATE vehiculos SET kilometraje_actual = ?1 WHERE id_vehiculo = ?2",
        params![req.kilometraje_ingreso, datos_vc.id_vehiculo],
    )?;

    tx.commit()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    obtener_ot_por_id(conn, id_ot)
}

pub fn cambiar_estado_ot(
    conn: &mut Connection,
    req: CambiarEstadoOtDto,
    id_usuario: Option<i64>,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot_actual = obtener_ot_por_id(conn, req.id_ot)?;
    let nuevo_estado = req.nuevo_estado.trim().to_uppercase();

    if ot_actual.estado == nuevo_estado {
        return obtener_ot_por_id(conn, req.id_ot);
    }

    validar_transicion_estado(&ot_actual.estado, &nuevo_estado)?;

    let id_u = id_usuario.ok_or_else(|| {
        AppError::Validation("Se requiere un usuario válido para cambiar el estado.".to_string())
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

    if rol_usuario == "MECANICO" && ot_actual.id_mecanico != id_u && ot_actual.id_mecanico != 0 {
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

        recalcular_kilometraje_vehiculo(conn, &ot_actual.placa)?;
    } else if nuevo_estado == "FINALIZADO" {
        conn.execute(
            "UPDATE ordenes_trabajo 
             SET estado = 'FINALIZADO', id_zanja_ejecucion = NULL, fecha_fin = datetime('now', '-5 hours'), kilometraje_salida = kilometraje_ingreso, id_mecanico_terminal = id_mecanico, resumen_publico = COALESCE(pedido_cliente, 'Mantenimiento finalizado correctamente') 
             WHERE id_ot = ?1",
            params![req.id_ot],
        )?;
    }

    obtener_ot_por_id(conn, req.id_ot)
}

pub fn reasignar_mecanico_ot(
    conn: &Connection,
    req: ReasignarMecanicoOtDto,
) -> Result<OrdenTrabajoHistorialDto, AppError> {
    let ot = obtener_ot_por_id(conn, req.id_ot)?;

    if ot.estado == "FINALIZADO" || ot.estado == "CANCELADO" {
        return Err(AppError::Validation(
            "No se puede reasignar mecánico en una orden de trabajo finalizada o cancelada."
                .to_string(),
        ));
    }

    validar_mecanico_activo(conn, req.nuevo_id_mecanico)?;

    conn.execute(
        "UPDATE ordenes_trabajo SET id_mecanico = ?1 WHERE id_ot = ?2",
        params![req.nuevo_id_mecanico, req.id_ot],
    )?;

    obtener_ot_por_id(conn, req.id_ot)
}

use super::cliente_crud::registrar_cliente;
use super::validaciones::{
    sanitizar_placa, sanitizar_telefono, validar_motivo, validar_placa, validar_telefono,
};
use crate::dto::cliente_dto::CrearClienteDto;
use crate::dto::vehiculo_dto::{
    CambiarEstadoVehiculoPayloadDto, ClienteResumenDto, CrearVehiculoPayloadDto,
    EditarVehiculoPayloadDto, VehiculoCompletoDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection, OptionalExtension};

pub fn registrar_vehiculo(
    conn: &Connection,
    req: CrearVehiculoPayloadDto,
    id_usuario: Option<i64>,
    username: Option<&str>,
    rol: Option<&str>,
) -> Result<String, AppError> {
    let placa = sanitizar_placa(&req.placa);
    let marca = req.marca.trim().to_string();
    let modelo = req.modelo.trim().to_string();

    validar_placa(&placa)?;
    validar_telefono(req.telefono.as_ref())?;

    let anio_actual = chrono::Datelike::year(&chrono::Local::now()) + 1;
    if let Some(a) = req.anio {
        if a < 1900 || a > anio_actual {
            return Err(AppError::Validation(format!(
                "El año del vehículo debe estar entre 1900 y {}.",
                anio_actual
            )));
        }
    }

    if req.kilometraje_actual < 0 || req.kilometraje_actual > 9_999_999 {
        return Err(AppError::Validation(
            "El kilometraje actual debe estar entre 0 y 9,999,999 km.".to_string(),
        ));
    }

    if marca.len() < 2 || marca.len() > 60 || modelo.len() < 2 || modelo.len() > 60 {
        return Err(AppError::Validation(
            "La marca y el modelo deben tener entre 2 y 60 caracteres.".to_string(),
        ));
    }

    let mut id_cliente_final = req.id_cliente.unwrap_or(0);

    if id_cliente_final == 0 {
        if let Some(doc) = req.numero_documento.as_ref() {
            let doc_clean = doc.trim();
            let tipo_doc = req
                .tipo_documento
                .clone()
                .unwrap_or_else(|| "DNI".to_string());
            if !doc_clean.is_empty() {
                let cliente_existente: Option<i64> = conn
                    .query_row(
                        "SELECT id_cliente FROM clientes WHERE tipo_documento = ?1 AND numero_documento = ?2 AND es_varios = 0",
                        params![tipo_doc, doc_clean],
                        |row| row.get(0),
                    )
                    .optional()?;

                if let Some(id_c) = cliente_existente {
                    id_cliente_final = id_c;
                } else if let Some(nombre) = req.nombre_razon_social.as_ref() {
                    let cliente_req = CrearClienteDto {
                        tipo_documento: tipo_doc,
                        numero_documento: doc_clean.to_string(),
                        nombre_razon_social: nombre.clone(),
                        telefono: req.telefono.clone(),
                        correo: req.correo.clone(),
                        direccion: req.direccion.clone(),
                    };
                    id_cliente_final =
                        registrar_cliente(conn, cliente_req, id_usuario, username, rol)?;
                }
            }
        }
    }

    if id_cliente_final == 0 {
        return Err(AppError::Validation(
            "Un vehículo requiere un cliente identificado válido (no se permite 'Cliente Varios')."
                .to_string(),
        ));
    }

    conn.execute(
        "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)",
        params![
            placa,
            id_cliente_final,
            marca,
            modelo,
            req.anio,
            req.tipo_motor.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            req.kilometraje_actual,
        ],
    )?;

    let detalle_auditoria = format!(
        "Alta de vehículo placa: {}, cliente ID: {}",
        placa, id_cliente_final
    );

    let despues_json = serde_json::json!({
        "placa": placa,
        "id_cliente": id_cliente_final,
        "marca": marca,
        "modelo": modelo,
        "anio": req.anio,
        "kilometraje_actual": req.kilometraje_actual
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
        "REGISTRAR_VEHICULO",
        Some("vehiculos"),
        Some(&placa),
        None,
        Some(&despues_json),
        Some(&detalle_auditoria),
        "OK",
    )?;

    Ok(placa)
}

pub fn actualizar_vehiculo(
    conn: &Connection,
    req: EditarVehiculoPayloadDto,
    id_usuario: Option<i64>,
    username: Option<&str>,
    rol_usuario: &str,
) -> Result<(), AppError> {
    let placa_nueva = sanitizar_placa(&req.placa);
    let marca = req.marca.trim().to_string();
    let modelo = req.modelo.trim().to_string();

    validar_placa(&placa_nueva)?;
    validar_telefono(req.telefono.as_ref())?;

    let (id_vehiculo_target, placa_actual, id_cliente_actual, km_actual, activo_actual): (
        i64,
        String,
        i64,
        i64,
        i32,
    ) = if let Some(id_v) = req.id_vehiculo {
        conn.query_row(
            "SELECT id_vehiculo, placa, id_cliente, kilometraje_actual, activo FROM vehiculos WHERE id_vehiculo = ?1",
            [id_v],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .map_err(|_| AppError::NotFound(format!("Vehículo con ID {} no encontrado.", id_v)))?
    } else {
        conn.query_row(
            "SELECT id_vehiculo, placa, id_cliente, kilometraje_actual, activo FROM vehiculos WHERE placa = ?1",
            [&placa_nueva],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .map_err(|_| AppError::NotFound(format!("Vehículo con placa {} no encontrado.", placa_nueva)))?
    };

    if activo_actual == 0 {
        return Err(AppError::Validation(
            "El vehículo está desactivado. Un Administrador debe reactivarlo primero.".to_string(),
        ));
    }

    let cambio_placa = placa_nueva != placa_actual;
    if cambio_placa {
        if rol_usuario != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "Acceso denegado: Solo un usuario ADMINISTRADOR puede cambiar la placa de un vehículo."
                    .to_string(),
            ));
        }
        let len_motivo = req.motivo.as_ref().map(|m| m.trim().len()).unwrap_or(0);
        if !(5..=250).contains(&len_motivo) {
            return Err(AppError::Validation(
                "Se requiere un motivo obligatorio (entre 5 y 250 caracteres) para cambiar la placa del vehículo."
                    .to_string(),
            ));
        }

        let existe_placa: i32 = conn.query_row(
            "SELECT COUNT(*) FROM vehiculos WHERE placa = ?1 AND id_vehiculo <> ?2",
            params![placa_nueva, id_vehiculo_target],
            |row| row.get(0),
        )?;

        if existe_placa > 0 {
            return Err(AppError::Validation(format!(
                "Ya existe otro vehículo registrado con la placa {}.",
                placa_nueva
            )));
        }
    }

    let disminuye_km = req.kilometraje_actual < km_actual;
    if disminuye_km {
        if rol_usuario != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "Solo el ADMINISTRADOR puede disminuir manualmente el kilometraje.".to_string(),
            ));
        }
        let len_motivo = req.motivo.as_ref().map(|m| m.trim().len()).unwrap_or(0);
        if !(5..=250).contains(&len_motivo) {
            return Err(AppError::Validation(
                "Se requiere un motivo obligatorio (entre 5 y 250 caracteres) para disminuir el kilometraje."
                    .to_string(),
            ));
        }
    }

    let mut id_cliente_asignar = id_cliente_actual;

    if let (Some(tipo), Some(doc)) = (req.tipo_documento.as_ref(), req.numero_documento.as_ref()) {
        let doc_clean = doc.trim();
        if !doc_clean.is_empty() {
            let cliente_ex: Option<i64> = conn
                .query_row(
                    "SELECT id_cliente FROM clientes WHERE tipo_documento = ?1 AND numero_documento = ?2 AND es_varios = 0",
                    params![tipo.trim().to_uppercase(), doc_clean],
                    |row| row.get(0),
                )
                .optional()?;

            if let Some(id_c) = cliente_ex {
                id_cliente_asignar = id_c;
            } else if let Some(ref nombre) = req.nombre_razon_social {
                let nuevo_req = CrearClienteDto {
                    tipo_documento: tipo.clone(),
                    numero_documento: doc_clean.to_string(),
                    nombre_razon_social: nombre.clone(),
                    telefono: req.telefono.clone(),
                    correo: req.correo.clone(),
                    direccion: req.direccion.clone(),
                };
                id_cliente_asignar =
                    registrar_cliente(conn, nuevo_req, id_usuario, username, Some(rol_usuario))?;
            }
        }
    }

    let cambio_propietario = id_cliente_asignar != id_cliente_actual;
    if cambio_propietario {
        if rol_usuario != "ADMINISTRADOR" {
            return Err(AppError::Validation(
                "Solo el ADMINISTRADOR puede realizar el cambio de propietario del vehículo."
                    .to_string(),
            ));
        }
        let len_motivo = req.motivo.as_ref().map(|m| m.trim().len()).unwrap_or(0);
        if !(5..=250).contains(&len_motivo) {
            return Err(AppError::Validation(
                "Se requiere un motivo obligatorio (entre 5 y 250 caracteres) para el cambio de propietario."
                    .to_string(),
            ));
        }
    }

    let motivo_str = req.motivo.as_deref().unwrap_or("");

    conn.execute(
        "UPDATE vehiculos
         SET placa = ?1, marca = ?2, modelo = ?3, anio = ?4, tipo_motor = ?5, kilometraje_actual = ?6, id_cliente = ?7
         WHERE id_vehiculo = ?8",
        params![
            placa_nueva,
            marca,
            modelo,
            req.anio,
            req.tipo_motor.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            req.kilometraje_actual,
            id_cliente_asignar,
            id_vehiculo_target,
        ],
    )?;

    if let Some(ref nombre) = req.nombre_razon_social {
        let nombre_clean = nombre.trim();
        if !nombre_clean.is_empty() {
            let telefono_limpio = sanitizar_telefono(req.telefono.as_ref());

            conn.execute(
                "UPDATE clientes
                 SET nombre_razon_social = ?1,
                     telefono = ?2,
                     correo = ?3,
                     direccion = ?4
                 WHERE id_cliente = ?5 AND es_varios = 0",
                params![
                    nombre_clean,
                    telefono_limpio,
                    req.correo
                        .as_ref()
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty()),
                    req.direccion
                        .as_ref()
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty()),
                    id_cliente_asignar,
                ],
            )?;
        }
    }

    let antes_json = serde_json::json!({
        "placa": placa_actual,
        "kilometraje_actual": km_actual,
        "id_cliente": id_cliente_actual
    })
    .to_string();

    let despues_json = serde_json::json!({
        "placa": placa_nueva,
        "kilometraje_actual": req.kilometraje_actual,
        "id_cliente": id_cliente_asignar
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
        Some(rol_usuario),
        None,
        "FICHA_VEHICULAR",
        "ACTUALIZAR_VEHICULO",
        Some("vehiculos"),
        Some(&placa_nueva),
        Some(&antes_json),
        Some(&despues_json),
        Some(motivo_str),
        "OK",
    )?;

    Ok(())
}

pub fn cambiar_estado_vehiculo(
    conn: &Connection,
    payload: CambiarEstadoVehiculoPayloadDto,
    id_usuario: Option<i64>,
    username: Option<&str>,
    rol: Option<&str>,
) -> Result<(), AppError> {
    let placa = sanitizar_placa(&payload.placa);
    let motivo = validar_motivo(&payload.motivo)?;

    let (estado_anterior,): (i32,) = conn
        .query_row(
            "SELECT activo FROM vehiculos WHERE placa = ?1",
            [&placa],
            |row| Ok((row.get(0)?,)),
        )
        .map_err(|_| AppError::NotFound(format!("Vehículo {} no encontrado.", placa)))?;

    let rows = conn.execute(
        "UPDATE vehiculos SET activo = ?1 WHERE placa = ?2",
        params![payload.activo, placa],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound(format!(
            "Vehículo {} no encontrado.",
            placa
        )));
    }

    let accion = if payload.activo == 1 {
        "REACTIVAR_VEHICULO"
    } else {
        "DESACTIVAR_VEHICULO"
    };

    let antes_json = serde_json::json!({ "activo": estado_anterior }).to_string();
    let despues_json = serde_json::json!({ "activo": payload.activo }).to_string();

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
        accion,
        Some("vehiculos"),
        Some(&placa),
        Some(&antes_json),
        Some(&despues_json),
        Some(&motivo),
        "OK",
    )?;

    Ok(())
}

pub fn obtener_vehiculo_por_placa(
    conn: &Connection,
    placa: &str,
    rol_ejecutor: Option<&str>,
) -> Result<VehiculoCompletoDto, AppError> {
    let placa_clean = sanitizar_placa(placa);
    let mut stmt = conn.prepare(
        "SELECT
            v.id_vehiculo, v.placa, v.id_cliente, v.marca, v.modelo, v.anio, v.tipo_motor, v.kilometraje_actual, v.activo,
            c.tipo_documento, c.numero_documento, c.nombre_razon_social, c.telefono, c.correo, c.direccion
         FROM vehiculos v
         LEFT JOIN clientes c ON v.id_cliente = c.id_cliente
         WHERE v.placa = ?1",
    )?;

    let mut dto = stmt
        .query_row([&placa_clean], |row| {
            let id_cliente: i64 = row.get(2)?;
            let tipo_doc: Option<String> = row.get(9)?;
            let num_doc: Option<String> = row.get(10)?;
            let nombre: Option<String> = row.get(11)?;
            let telefono: Option<String> = row.get(12)?;
            let correo: Option<String> = row.get(13)?;
            let direccion: Option<String> = row.get(14)?;

            let cliente_resumen = nombre.as_ref().map(|nom| ClienteResumenDto {
                id_cliente,
                tipo_documento: tipo_doc.clone(),
                numero_documento: num_doc.clone(),
                nombre_razon_social: nom.clone(),
                telefono: telefono.clone(),
                correo: correo.clone(),
                direccion: direccion.clone(),
            });

            Ok(VehiculoCompletoDto {
                id_vehiculo: row.get(0)?,
                placa: row.get(1)?,
                id_cliente,
                marca: row.get(3)?,
                modelo: row.get(4)?,
                anio: row.get(5)?,
                tipo_motor: row.get(6)?,
                kilometraje_actual: row.get(7)?,
                activo: row.get(8)?,
                cliente: cliente_resumen,
                nombre_cliente: nombre,
                tipo_documento_cliente: tipo_doc,
                documento_cliente: num_doc,
                telefono_cliente: telefono,
                correo_cliente: correo,
                direccion_cliente: direccion,
            })
        })
        .map_err(|_| {
            AppError::NotFound(format!(
                "No se encontró vehículo registrado con la placa {}",
                placa_clean
            ))
        })?;

    if let Some(r) = rol_ejecutor {
        if r.to_uppercase() == "MECANICO" {
            dto.telefono_cliente = None;
            dto.correo_cliente = None;
            dto.direccion_cliente = None;
            if let Some(ref mut c) = dto.cliente {
                c.telefono = None;
                c.correo = None;
                c.direccion = None;
            }
        }
    }

    Ok(dto)
}

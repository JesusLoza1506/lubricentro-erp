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

#[derive(Default)]
struct ClienteSnapshot {
    nombre_razon_social: Option<String>,
    tipo_documento: Option<String>,
    numero_documento: Option<String>,
    telefono: Option<String>,
    correo: Option<String>,
    direccion: Option<String>,
}

struct VehiculoClienteSnapshot {
    id_vehiculo: i64,
    placa: String,
    marca: String,
    modelo: String,
    anio: Option<i32>,
    tipo_motor: Option<String>,
    kilometraje_actual: i64,
    id_cliente: i64,
    activo: i32,
    cliente: ClienteSnapshot,
}

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

    let tipo_motor_clean = req
        .tipo_motor
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    conn.execute(
        "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)",
        params![
            placa,
            id_cliente_final,
            marca,
            modelo,
            req.anio,
            tipo_motor_clean,
            req.kilometraje_actual,
        ],
    )?;

    let cliente_snap: ClienteSnapshot = conn
        .query_row(
            "SELECT nombre_razon_social, tipo_documento, numero_documento, telefono, correo, direccion FROM clientes WHERE id_cliente = ?1",
            [id_cliente_final],
            |row| {
                Ok(ClienteSnapshot {
                    nombre_razon_social: row.get(0)?,
                    tipo_documento: row.get(1)?,
                    numero_documento: row.get(2)?,
                    telefono: row.get(3)?,
                    correo: row.get(4)?,
                    direccion: row.get(5)?,
                })
            },
        )
        .unwrap_or_default();

    let detalle_auditoria = format!(
        "Alta de vehículo placa: {}, cliente ID: {}",
        placa, id_cliente_final
    );

    let despues_json = serde_json::json!({
        "vehiculo": {
            "placa": placa,
            "id_cliente": id_cliente_final,
            "marca": marca,
            "modelo": modelo,
            "anio": req.anio,
            "tipo_motor": tipo_motor_clean,
            "kilometraje_actual": req.kilometraje_actual,
            "activo": 1
        },
        "cliente": {
            "id_cliente": id_cliente_final,
            "nombre_razon_social": cliente_snap.nombre_razon_social,
            "tipo_documento": cliente_snap.tipo_documento,
            "numero_documento": cliente_snap.numero_documento,
            "telefono": cliente_snap.telefono,
            "correo": cliente_snap.correo,
            "direccion": cliente_snap.direccion
        }
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

    let snap_previo: VehiculoClienteSnapshot = if let Some(id_v) = req.id_vehiculo {
        conn.query_row(
            "SELECT 
                v.id_vehiculo, v.placa, v.marca, v.modelo, v.anio, v.tipo_motor, v.kilometraje_actual, v.id_cliente, v.activo,
                c.nombre_razon_social, c.tipo_documento, c.numero_documento, c.telefono, c.correo, c.direccion
             FROM vehiculos v
             LEFT JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.id_vehiculo = ?1",
            [id_v],
            |row| {
                Ok(VehiculoClienteSnapshot {
                    id_vehiculo: row.get(0)?,
                    placa: row.get(1)?,
                    marca: row.get(2)?,
                    modelo: row.get(3)?,
                    anio: row.get(4)?,
                    tipo_motor: row.get(5)?,
                    kilometraje_actual: row.get(6)?,
                    id_cliente: row.get(7)?,
                    activo: row.get(8)?,
                    cliente: ClienteSnapshot {
                        nombre_razon_social: row.get(9)?,
                        tipo_documento: row.get(10)?,
                        numero_documento: row.get(11)?,
                        telefono: row.get(12)?,
                        correo: row.get(13)?,
                        direccion: row.get(14)?,
                    },
                })
            },
        )
        .map_err(|_| AppError::NotFound(format!("Vehículo con ID {} no encontrado.", id_v)))?
    } else {
        conn.query_row(
            "SELECT 
                v.id_vehiculo, v.placa, v.marca, v.modelo, v.anio, v.tipo_motor, v.kilometraje_actual, v.id_cliente, v.activo,
                c.nombre_razon_social, c.tipo_documento, c.numero_documento, c.telefono, c.correo, c.direccion
             FROM vehiculos v
             LEFT JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.placa = ?1",
            [&placa_nueva],
            |row| {
                Ok(VehiculoClienteSnapshot {
                    id_vehiculo: row.get(0)?,
                    placa: row.get(1)?,
                    marca: row.get(2)?,
                    modelo: row.get(3)?,
                    anio: row.get(4)?,
                    tipo_motor: row.get(5)?,
                    kilometraje_actual: row.get(6)?,
                    id_cliente: row.get(7)?,
                    activo: row.get(8)?,
                    cliente: ClienteSnapshot {
                        nombre_razon_social: row.get(9)?,
                        tipo_documento: row.get(10)?,
                        numero_documento: row.get(11)?,
                        telefono: row.get(12)?,
                        correo: row.get(13)?,
                        direccion: row.get(14)?,
                    },
                })
            },
        )
        .map_err(|_| AppError::NotFound(format!("Vehículo con placa {} no encontrado.", placa_nueva)))?
    };

    if snap_previo.activo == 0 {
        return Err(AppError::Validation(
            "El vehículo está desactivado. Un Administrador debe reactivarlo primero.".to_string(),
        ));
    }

    let cambio_placa = placa_nueva != snap_previo.placa;
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
            params![placa_nueva, snap_previo.id_vehiculo],
            |row| row.get(0),
        )?;

        if existe_placa > 0 {
            return Err(AppError::Validation(format!(
                "Ya existe otro vehículo registrado con la placa {}.",
                placa_nueva
            )));
        }
    }

    let disminuye_km = req.kilometraje_actual < snap_previo.kilometraje_actual;
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

    let mut id_cliente_asignar = snap_previo.id_cliente;

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

    let cambio_propietario = id_cliente_asignar != snap_previo.id_cliente;
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

    let tipo_motor_clean = req
        .tipo_motor
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    conn.execute(
        "UPDATE vehiculos
         SET placa = ?1, marca = ?2, modelo = ?3, anio = ?4, tipo_motor = ?5, kilometraje_actual = ?6, id_cliente = ?7
         WHERE id_vehiculo = ?8",
        params![
            placa_nueva,
            marca,
            modelo,
            req.anio,
            tipo_motor_clean,
            req.kilometraje_actual,
            id_cliente_asignar,
            snap_previo.id_vehiculo,
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

    let cliente_nuevo_snap: ClienteSnapshot = conn
        .query_row(
            "SELECT nombre_razon_social, tipo_documento, numero_documento, telefono, correo, direccion FROM clientes WHERE id_cliente = ?1",
            [id_cliente_asignar],
            |row| {
                Ok(ClienteSnapshot {
                    nombre_razon_social: row.get(0)?,
                    tipo_documento: row.get(1)?,
                    numero_documento: row.get(2)?,
                    telefono: row.get(3)?,
                    correo: row.get(4)?,
                    direccion: row.get(5)?,
                })
            },
        )
        .unwrap_or_default();

    let antes_json = serde_json::json!({
        "vehiculo": {
            "id_vehiculo": snap_previo.id_vehiculo,
            "placa": snap_previo.placa,
            "marca": snap_previo.marca,
            "modelo": snap_previo.modelo,
            "anio": snap_previo.anio,
            "tipo_motor": snap_previo.tipo_motor,
            "kilometraje_actual": snap_previo.kilometraje_actual,
            "id_cliente": snap_previo.id_cliente,
            "activo": snap_previo.activo
        },
        "cliente": {
            "id_cliente": snap_previo.id_cliente,
            "nombre_razon_social": snap_previo.cliente.nombre_razon_social,
            "tipo_documento": snap_previo.cliente.tipo_documento,
            "numero_documento": snap_previo.cliente.numero_documento,
            "telefono": snap_previo.cliente.telefono,
            "correo": snap_previo.cliente.correo,
            "direccion": snap_previo.cliente.direccion
        }
    })
    .to_string();

    let despues_json = serde_json::json!({
        "vehiculo": {
            "id_vehiculo": snap_previo.id_vehiculo,
            "placa": placa_nueva,
            "marca": marca,
            "modelo": modelo,
            "anio": req.anio,
            "tipo_motor": tipo_motor_clean,
            "kilometraje_actual": req.kilometraje_actual,
            "id_cliente": id_cliente_asignar,
            "activo": snap_previo.activo
        },
        "cliente": {
            "id_cliente": id_cliente_asignar,
            "nombre_razon_social": cliente_nuevo_snap.nombre_razon_social,
            "tipo_documento": cliente_nuevo_snap.tipo_documento,
            "numero_documento": cliente_nuevo_snap.numero_documento,
            "telefono": cliente_nuevo_snap.telefono,
            "correo": cliente_nuevo_snap.correo,
            "direccion": cliente_nuevo_snap.direccion
        }
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

    let (
        id_vehiculo_target,
        marca,
        modelo,
        anio,
        tipo_motor,
        km_actual,
        id_cliente,
        estado_anterior,
    ): (
        i64,
        String,
        String,
        Option<i32>,
        Option<String>,
        i64,
        i64,
        i32,
    ) = conn
        .query_row(
            "SELECT id_vehiculo, marca, modelo, anio, tipo_motor, kilometraje_actual, id_cliente, activo FROM vehiculos WHERE placa = ?1",
            [&placa],
            |row| Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            )),
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

    let antes_json = serde_json::json!({
        "vehiculo": {
            "id_vehiculo": id_vehiculo_target,
            "placa": placa,
            "marca": marca,
            "modelo": modelo,
            "anio": anio,
            "tipo_motor": tipo_motor,
            "kilometraje_actual": km_actual,
            "id_cliente": id_cliente,
            "activo": estado_anterior
        }
    })
    .to_string();

    let despues_json = serde_json::json!({
        "vehiculo": {
            "id_vehiculo": id_vehiculo_target,
            "placa": placa,
            "marca": marca,
            "modelo": modelo,
            "anio": anio,
            "tipo_motor": tipo_motor,
            "kilometraje_actual": km_actual,
            "id_cliente": id_cliente,
            "activo": payload.activo
        }
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

    let opt_row = stmt
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
        .optional()?;

    let mut dto = match opt_row {
        Some(v) => v,
        None => {
            return Err(AppError::NotFound(format!(
                "No se encontró vehículo registrado con la placa {}.",
                placa_clean
            )))
        }
    };

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

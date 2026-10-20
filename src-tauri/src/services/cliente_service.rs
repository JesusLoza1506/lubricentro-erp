use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto, EditarClienteDto};
use crate::dto::vehiculo_dto::{
    CambiarEstadoVehiculoPayloadDto, ClienteResumenDto, CorregirHistorialOtPayloadDto,
    CrearVehiculoPayloadDto, EditarVehiculoPayloadDto, FichaVehicularCompletaDto,
    ProximoCambioAceiteDto, VehiculoCompletoDto,
};
use crate::entities::cliente::ClienteEntity;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::{params, Connection, OptionalExtension};

pub struct ClienteService;

impl ClienteService {
    pub fn sanitizar_placa(placa: &str) -> String {
        placa.replace(['-', ' '], "").trim().to_uppercase()
    }

    pub fn sanitizar_telefono(telefono: Option<&String>) -> Option<String> {
        telefono.and_then(|t| {
            let clean = t.trim().replace([' ', '-'], "");
            let sin_prefijo = clean.strip_prefix("+51").unwrap_or(&clean);
            if sin_prefijo.is_empty() {
                None
            } else {
                Some(sin_prefijo.to_string())
            }
        })
    }

    fn validar_placa(placa_clean: &str) -> Result<(), AppError> {
        if placa_clean.len() != 6 || !placa_clean.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(AppError::Validation(
                "La placa debe contener exactamente 6 caracteres alfanuméricos (ej: ABC123)."
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn validar_documento(tipo: &str, doc: &str) -> Result<(), AppError> {
        let t = tipo.trim().to_uppercase();
        let d = doc.trim();

        match t.as_str() {
            "DNI" => {
                if d.len() != 8 || !d.chars().all(|c| c.is_ascii_digit()) {
                    return Err(AppError::Validation(
                        "DNI inválido: debe tener exactamente 8 dígitos numéricos.".to_string(),
                    ));
                }
            }
            "RUC" => {
                if d.len() != 11 || !d.chars().all(|c| c.is_ascii_digit()) {
                    return Err(AppError::Validation(
                        "RUC inválido: debe tener exactamente 11 dígitos numéricos.".to_string(),
                    ));
                }
            }
            "CE" => {
                if d.len() < 6 || d.len() > 12 || !d.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(AppError::Validation(
                        "Carné de Extranjería (CE) inválido: debe tener entre 6 y 12 caracteres alfanuméricos.".to_string(),
                    ));
                }
            }
            _ => {
                return Err(AppError::Validation(
                    "Tipo de documento no soportado. Elija DNI, RUC o CE.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validar_telefono(telefono: Option<&String>) -> Result<(), AppError> {
        if let Some(t) = telefono {
            let clean = t.trim().replace([' ', '-'], "");
            if !clean.is_empty() {
                let sin_prefijo = clean.strip_prefix("+51").unwrap_or(&clean);
                if (sin_prefijo.len() != 7 && sin_prefijo.len() != 9)
                    || !sin_prefijo.chars().all(|c| c.is_ascii_digit())
                {
                    return Err(AppError::Validation(
                        "El teléfono debe contener 7 (fijo) o 9 (móvil) dígitos numéricos."
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn validar_motivo(motivo: &str) -> Result<String, AppError> {
        let m = motivo.trim();
        if m.len() < 5 || m.len() > 250 {
            return Err(AppError::Validation(
                "El motivo de la acción es obligatorio y debe contener entre 5 y 250 caracteres."
                    .to_string(),
            ));
        }
        Ok(m.to_string())
    }

    pub fn registrar_cliente(
        conn: &Connection,
        req: CrearClienteDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<i64, AppError> {
        let tipo = req.tipo_documento.trim().to_uppercase();
        let doc = req.numero_documento.trim().to_string();
        let nombre = req.nombre_razon_social.trim().to_string();

        Self::validar_documento(&tipo, &doc)?;
        Self::validar_telefono(req.telefono.as_ref())?;

        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "El nombre o razón social debe tener entre 3 y 150 caracteres.".to_string(),
            ));
        }

        let existe: i32 = conn.query_row(
            "SELECT COUNT(*) FROM clientes WHERE tipo_documento = ?1 AND numero_documento = ?2",
            params![tipo, doc],
            |row| row.get(0),
        )?;

        if existe > 0 {
            return Err(AppError::Validation(format!(
                "Ya existe un cliente registrado con el documento {} {}",
                tipo, doc
            )));
        }

        let telefono_limpio = Self::sanitizar_telefono(req.telefono.as_ref());

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, es_varios)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            params![
                tipo,
                doc,
                nombre,
                telefono_limpio,
                req.correo.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
                req.direccion.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            ],
        )?;

        let id_cliente_nuevo = conn.last_insert_rowid();

        let despues_json = serde_json::json!({
            "id_cliente": id_cliente_nuevo,
            "tipo_documento": tipo,
            "numero_documento": doc,
            "nombre_razon_social": nombre
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
            "REGISTRAR_CLIENTE",
            Some("clientes"),
            Some(&id_cliente_nuevo.to_string()),
            None,
            Some(&despues_json),
            Some("Alta de cliente de forma autónoma"),
            "OK",
        )?;

        Ok(id_cliente_nuevo)
    }

    pub fn actualizar_cliente(
        conn: &Connection,
        req: EditarClienteDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<(), AppError> {
        let tipo = req.tipo_documento.trim().to_uppercase();
        let doc = req.numero_documento.trim().to_string();
        let nombre = req.nombre_razon_social.trim().to_string();

        Self::validar_documento(&tipo, &doc)?;
        Self::validar_telefono(req.telefono.as_ref())?;

        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "El nombre o razón social debe tener entre 3 y 150 caracteres.".to_string(),
            ));
        }

        let (nombre_old, doc_old, tipo_old): (String, String, String) = conn
            .query_row(
                "SELECT nombre_razon_social, numero_documento, tipo_documento FROM clientes WHERE id_cliente = ?1",
                [req.id_cliente],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .map_err(|_| AppError::NotFound("Cliente no encontrado.".to_string()))?;

        let existe_otro: i32 = conn.query_row(
            "SELECT COUNT(*) FROM clientes WHERE tipo_documento = ?1 AND numero_documento = ?2 AND id_cliente <> ?3",
            params![tipo, doc, req.id_cliente],
            |row| row.get(0),
        )?;

        if existe_otro > 0 {
            return Err(AppError::Validation(format!(
                "Ya existe otro cliente registrado con el documento {} {}",
                tipo, doc
            )));
        }

        let telefono_limpio = Self::sanitizar_telefono(req.telefono.as_ref());

        let rows = conn.execute(
            "UPDATE clientes 
             SET tipo_documento = ?1, 
                 numero_documento = ?2, 
                 nombre_razon_social = ?3, 
                 telefono = ?4, 
                 correo = ?5, 
                 direccion = ?6 
             WHERE id_cliente = ?7 AND es_varios = 0",
            params![
                tipo,
                doc,
                nombre,
                telefono_limpio,
                req.correo
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty()),
                req.direccion
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty()),
                req.id_cliente,
            ],
        )?;

        if rows == 0 {
            return Err(AppError::NotFound(
                "Cliente no encontrado o es 'Cliente Varios' protegida.".to_string(),
            ));
        }

        let antes_json = serde_json::json!({
            "tipo_documento": tipo_old,
            "numero_documento": doc_old,
            "nombre_razon_social": nombre_old
        })
        .to_string();

        let despues_json = serde_json::json!({
            "tipo_documento": tipo,
            "numero_documento": doc,
            "nombre_razon_social": nombre
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
            "ACTUALIZAR_CLIENTE",
            Some("clientes"),
            Some(&req.id_cliente.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            req.motivo.as_deref(),
            "OK",
        )?;

        Ok(())
    }

    pub fn buscar_cliente_por_documento(
        conn: &Connection,
        tipo_documento: &str,
        numero_documento: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<ClienteDto, AppError> {
        let tipo_clean = tipo_documento.trim().to_uppercase();
        let doc_clean = numero_documento.trim();

        let mut stmt = conn.prepare(
            "SELECT id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, es_varios, fecha_creacion, version
             FROM clientes
             WHERE tipo_documento = ?1 AND numero_documento = ?2",
        )?;

        let entity = stmt
            .query_row(params![tipo_clean, doc_clean], |row| {
                Ok(ClienteEntity {
                    id_cliente: row.get(0)?,
                    tipo_documento: row.get(1)?,
                    numero_documento: row.get(2)?,
                    nombre_razon_social: row.get(3)?,
                    telefono: row.get(4)?,
                    correo: row.get(5)?,
                    direccion: row.get(6)?,
                    es_varios: row.get(7)?,
                    fecha_creacion: row.get(8)?,
                    version: row.get(9)?,
                })
            })
            .map_err(|_| {
                AppError::NotFound(format!(
                    "No se encontró cliente con el documento {} {}",
                    tipo_clean, doc_clean
                ))
            })?;

        let mut dto = ClienteDto::from(entity);

        if let Some(r) = rol_ejecutor {
            if r.to_uppercase() == "MECANICO" {
                dto.telefono = None;
                dto.correo = None;
                dto.direccion = None;
            }
        }

        Ok(dto)
    }

    pub fn registrar_vehiculo(
        conn: &Connection,
        req: CrearVehiculoPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<String, AppError> {
        let placa = Self::sanitizar_placa(&req.placa);
        let marca = req.marca.trim().to_string();
        let modelo = req.modelo.trim().to_string();

        Self::validar_placa(&placa)?;
        Self::validar_telefono(req.telefono.as_ref())?;

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
                            Self::registrar_cliente(conn, cliente_req, id_usuario, username, rol)?;
                    }
                }
            }
        }

        if id_cliente_final == 0 {
            return Err(AppError::Validation(
                "Un vehículo requiere un cliente identificado válido (no se permite 'Cliente Varios').".to_string(),
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
        let placa_nueva = Self::sanitizar_placa(&req.placa);
        let marca = req.marca.trim().to_string();
        let modelo = req.modelo.trim().to_string();

        Self::validar_placa(&placa_nueva)?;
        Self::validar_telefono(req.telefono.as_ref())?;

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
                "El vehículo está desactivado. Un Administrador debe reactivarlo primero."
                    .to_string(),
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
                    "Se requiere un motivo obligatorio (entre 5 y 250 caracteres) para disminuir el kilometraje.".to_string(),
                ));
            }
        }

        let mut id_cliente_asignar = id_cliente_actual;

        if let (Some(tipo), Some(doc)) =
            (req.tipo_documento.as_ref(), req.numero_documento.as_ref())
        {
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
                    id_cliente_asignar = Self::registrar_cliente(
                        conn,
                        nuevo_req,
                        id_usuario,
                        username,
                        Some(rol_usuario),
                    )?;
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
                let telefono_limpio = Self::sanitizar_telefono(req.telefono.as_ref());

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
        let placa = Self::sanitizar_placa(&payload.placa);
        let motivo = Self::validar_motivo(&payload.motivo)?;

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

    pub fn corregir_historial_ot(
        conn: &Connection,
        payload: CorregirHistorialOtPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<(), AppError> {
        let motivo = Self::validar_motivo(&payload.motivo)?;
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

    pub fn obtener_vehiculo_por_placa(
        conn: &Connection,
        placa: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<VehiculoCompletoDto, AppError> {
        let placa_clean = Self::sanitizar_placa(placa);
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

    pub fn obtener_ficha_vehicular_completa(
        conn: &Connection,
        placa: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<FichaVehicularCompletaDto, AppError> {
        let vehiculo = Self::obtener_vehiculo_por_placa(conn, placa, rol_ejecutor)?;
        let historial =
            OrdenesService::obtener_historial_por_id_vehiculo(conn, vehiculo.id_vehiculo)?;

        Ok(FichaVehicularCompletaDto {
            vehiculo,
            historial,
        })
    }

    pub fn obtener_proximos_cambios_aceite(
        conn: &Connection,
        rol_ejecutor: Option<&str>,
    ) -> Result<Vec<ProximoCambioAceiteDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_vehiculo, placa, kilometraje_actual, id_cliente, nombre_razon_social, telefono,
                    id_ot, fecha_servicio, tipo_aceite, intervalo_aceite_km, km_proximo, causa
             FROM v_proximos_cambios
             ORDER BY fecha_servicio DESC",
        )?;

        let es_mecanico = rol_ejecutor
            .map(|r| r.to_uppercase() == "MECANICO")
            .unwrap_or(false);

        let rows = stmt.query_map([], |row| {
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

        let mut lista = Vec::new();
        for r in rows {
            lista.push(r?);
        }

        Ok(lista)
    }
}

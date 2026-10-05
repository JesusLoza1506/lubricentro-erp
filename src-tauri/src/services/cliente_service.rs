use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto};
use crate::dto::vehiculo_dto::{
    ClienteResumenDto, CrearVehiculoPayloadDto, EditarVehiculoPayloadDto,
    FichaVehicularCompletaDto, VehiculoCompletoDto,
};
use crate::entities::cliente::ClienteEntity;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use crate::services::ordenes_service::OrdenesService;
use rusqlite::{params, Connection, OptionalExtension};

pub struct ClienteService;

impl ClienteService {
    fn validar_placa(placa: &str) -> Result<(), AppError> {
        let clean = placa.trim().to_uppercase();
        let es_formato_estandar = clean.len() == 7
            && clean.chars().take(3).all(|c| c.is_ascii_alphabetic())
            && clean.chars().nth(3) == Some('-')
            && clean.chars().skip(4).take(3).all(|c| c.is_ascii_digit());

        let es_formato_sin_guion =
            clean.len() == 6 && clean.chars().all(|c| c.is_ascii_alphanumeric());

        if !es_formato_estandar && !es_formato_sin_guion {
            return Err(AppError::Validation(
                "Formato de placa inválido. Use el formato ABC-123 o 6 caracteres alfanuméricos."
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
                        "Número de documento inválido para DNI (debe tener exactamente 8 dígitos)."
                            .to_string(),
                    ));
                }
            }
            "RUC" => {
                if d.len() != 11 || !d.chars().all(|c| c.is_ascii_digit()) {
                    return Err(AppError::Validation(
                        "Número de documento inválido para RUC (debe tener exactamente 11 dígitos)."
                            .to_string(),
                    ));
                }
            }
            "CE" | "PASAPORTE" => {
                if d.len() < 6 || d.len() > 12 || !d.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(AppError::Validation(
                        "Número de documento inválido para CE/PASAPORTE (debe tener entre 6 y 12 caracteres)."
                            .to_string(),
                    ));
                }
            }
            _ => {
                return Err(AppError::Validation(
                    "Tipo de documento no soportado. Elija DNI, RUC, CE o PASAPORTE.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validar_telefono(telefono: Option<&String>) -> Result<(), AppError> {
        if let Some(t) = telefono {
            let clean = t.trim();
            if !clean.is_empty()
                && (clean.len() < 6
                    || clean.len() > 9
                    || !clean.chars().all(|c| c.is_ascii_digit()))
            {
                return Err(AppError::Validation(
                    "El teléfono debe contener entre 6 y 9 dígitos numéricos.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validar_anio(anio: Option<i32>) -> Result<(), AppError> {
        if let Some(a) = anio {
            use chrono::Datelike;
            let anio_actual = chrono::Local::now().year();
            let anio_max = anio_actual + 1;

            if a < 1980 || a > anio_max {
                return Err(AppError::Validation(format!(
                    "Año de fabricación fuera de rango válido (debe estar entre 1980 y {}).",
                    anio_max
                )));
            }
        }
        Ok(())
    }

    pub fn registrar_cliente(conn: &Connection, req: CrearClienteDto) -> Result<i64, AppError> {
        let tipo = req.tipo_documento.trim().to_uppercase();
        let doc = req.numero_documento.trim();
        let nombre = req.nombre_razon_social.trim();

        Self::validar_documento(&tipo, doc)?;
        Self::validar_telefono(req.telefono.as_ref())?;

        if nombre.is_empty() {
            return Err(AppError::Validation(
                "El nombre o razón social no puede estar vacío.".to_string(),
            ));
        }

        let existe: i32 = conn.query_row(
            "SELECT COUNT(*) FROM clientes WHERE numero_documento = ?1",
            [doc],
            |row| row.get(0),
        )?;

        if existe > 0 {
            return Err(AppError::Validation(format!(
                "Ya existe un cliente registrado con este documento ({})",
                doc
            )));
        }

        conn.execute(
            "INSERT INTO clientes (tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo)
             VALUES (?1, ?2, ?3, ?4, ?5, 1)",
            params![
                tipo,
                doc,
                nombre,
                req.telefono.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
                req.direccion.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn buscar_cliente_por_documento(
        conn: &Connection,
        documento: &str,
    ) -> Result<ClienteDto, AppError> {
        let doc_clean = documento.trim();
        let mut stmt = conn.prepare(
            "SELECT id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, direccion, activo
             FROM clientes
             WHERE numero_documento = ?1 AND activo = 1",
        )?;

        let entity = stmt
            .query_row([doc_clean], |row| {
                Ok(ClienteEntity {
                    id_cliente: row.get(0)?,
                    tipo_documento: row.get(1)?,
                    numero_documento: row.get(2)?,
                    nombre_razon_social: row.get(3)?,
                    telefono: row.get(4)?,
                    direccion: row.get(5)?,
                    activo: row.get(6)?,
                })
            })
            .map_err(|_| {
                AppError::NotFound(format!(
                    "No se encontró cliente activo con el documento {}",
                    doc_clean
                ))
            })?;

        Ok(ClienteDto::from(entity))
    }

    pub fn registrar_vehiculo(
        conn: &Connection,
        req: CrearVehiculoPayloadDto,
        id_usuario: Option<i64>,
    ) -> Result<String, AppError> {
        let placa = req.placa.trim().to_uppercase();
        let marca = req.marca.trim();
        let modelo = req.modelo.trim();

        Self::validar_placa(&placa)?;
        Self::validar_anio(req.anio)?;
        Self::validar_telefono(req.telefono.as_ref())?;

        if req.kilometraje_actual < 0 {
            return Err(AppError::Validation(
                "El kilometraje actual no puede ser un valor negativo.".to_string(),
            ));
        }

        if marca.is_empty() || modelo.is_empty() {
            return Err(AppError::Validation(
                "La marca y modelo del vehículo son obligatorios.".to_string(),
            ));
        }

        let mut id_cliente_final = req.id_cliente.unwrap_or(0);

        if id_cliente_final == 0 {
            if let Some(doc) = req.numero_documento.as_ref() {
                let doc_clean = doc.trim();
                if !doc_clean.is_empty() {
                    let cliente_existente: Option<i64> = conn
                        .query_row(
                            "SELECT id_cliente FROM clientes WHERE numero_documento = ?1 AND activo = 1",
                            [doc_clean],
                            |row| row.get(0),
                        )
                        .optional()?;

                    if let Some(id_c) = cliente_existente {
                        id_cliente_final = id_c;
                    } else if let Some(nombre) = req.nombre_razon_social.as_ref() {
                        let tipo_doc = req.tipo_documento.unwrap_or_else(|| "DNI".to_string());
                        let cliente_req = CrearClienteDto {
                            tipo_documento: tipo_doc,
                            numero_documento: doc_clean.to_string(),
                            nombre_razon_social: nombre.clone(),
                            telefono: req.telefono.clone(),
                            direccion: req.direccion.clone(),
                        };
                        id_cliente_final = Self::registrar_cliente(conn, cliente_req)?;
                    }
                }
            }
        }

        if id_cliente_final == 0 {
            return Err(AppError::Validation(
                "Debe proporcionar un cliente válido o completar los datos del propietario."
                    .to_string(),
            ));
        }

        let placa_existe: i32 = conn.query_row(
            "SELECT COUNT(*) FROM vehiculos WHERE placa = ?1",
            [&placa],
            |row| row.get(0),
        )?;

        let accion_auditoria: &str;
        let detalle_auditoria: String;

        if placa_existe > 0 {
            // Reasignación / Venta de vehículo a un cliente nuevo o existente
            conn.execute(
                "UPDATE vehiculos 
                 SET id_cliente = ?1, marca = ?2, modelo = ?3, anio = ?4, tipo_motor = ?5, kilometraje_actual = ?6, activo = 1
                 WHERE placa = ?7",
                params![
                    id_cliente_final,
                    marca,
                    modelo,
                    req.anio,
                    req.tipo_motor.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
                    req.kilometraje_actual,
                    placa,
                ],
            )?;

            accion_auditoria = "UPDATE";
            detalle_auditoria = format!(
                "Reasignación/Venta de vehículo placa: {} a cliente ID: {}",
                placa, id_cliente_final
            );
        } else {
            // Registro de un vehículo nuevo
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

            accion_auditoria = "INSERT";
            detalle_auditoria = format!(
                "Registro de vehículo placa: {}, cliente ID: {}",
                placa, id_cliente_final
            );
        }

        let id_u32 = id_usuario.map(|id| id as i32);
        AuditoriaService::registrar(
            conn,
            id_u32,
            "vehiculos",
            accion_auditoria,
            &detalle_auditoria,
        )
        .map_err(|e| AppError::Validation(format!("Error de auditoría: {}", e)))?;

        Ok(placa)
    }

    pub fn actualizar_vehiculo(
        conn: &Connection,
        req: EditarVehiculoPayloadDto,
        id_usuario: Option<i64>,
    ) -> Result<(), AppError> {
        let placa = req.placa.trim().to_uppercase();
        let marca = req.marca.trim();
        let modelo = req.modelo.trim();

        Self::validar_placa(&placa)?;
        Self::validar_anio(req.anio)?;
        Self::validar_telefono(req.telefono.as_ref())?;

        if req.kilometraje_actual < 0 {
            return Err(AppError::Validation(
                "El kilometraje actual no puede ser un valor negativo.".to_string(),
            ));
        }

        if marca.is_empty() || modelo.is_empty() {
            return Err(AppError::Validation(
                "La marca y modelo del vehículo son obligatorios.".to_string(),
            ));
        }

        let max_km_historico: Option<i64> = conn
            .query_row(
                "SELECT MAX(kilometraje_ingreso) FROM ordenes_trabajo WHERE placa = ?1",
                [&placa],
                |row| row.get(0),
            )
            .optional()?
            .flatten();

        if let Some(max_km) = max_km_historico {
            if req.kilometraje_actual < max_km {
                return Err(AppError::Validation(format!(
                    "El kilometraje no puede ser menor al último registrado en el historial ({} km).",
                    max_km
                )));
            }
        }

        let mut id_cliente_asignar: Option<i64> = req.id_cliente.filter(|&id| id > 0);

        if let (Some(tipo), Some(doc)) =
            (req.tipo_documento.as_ref(), req.numero_documento.as_ref())
        {
            let doc_clean = doc.trim();
            if !doc_clean.is_empty() {
                Self::validar_documento(tipo, doc_clean)?;

                let cliente_existente: Option<i64> = conn
                    .query_row(
                        "SELECT id_cliente FROM clientes WHERE numero_documento = ?1 AND activo = 1",
                        [doc_clean],
                        |row| row.get(0),
                    )
                    .optional()?;

                if let Some(id_ex) = cliente_existente {
                    id_cliente_asignar = Some(id_ex);
                } else if let Some(ref nombre) = req.nombre_razon_social {
                    let nuevo_req = CrearClienteDto {
                        tipo_documento: tipo.clone(),
                        numero_documento: doc_clean.to_string(),
                        nombre_razon_social: nombre.clone(),
                        telefono: req.telefono.clone(),
                        direccion: req.direccion.clone(),
                    };
                    id_cliente_asignar = Some(Self::registrar_cliente(conn, nuevo_req)?);
                }
            }
        }

        let rows_v = if let Some(id_c) = id_cliente_asignar {
            conn.execute(
                "UPDATE vehiculos
                 SET marca = ?1, modelo = ?2, anio = ?3, tipo_motor = ?4, kilometraje_actual = ?5, id_cliente = ?6
                 WHERE placa = ?7",
                params![
                    marca,
                    modelo,
                    req.anio,
                    req.tipo_motor.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
                    req.kilometraje_actual,
                    id_c,
                    placa,
                ],
            )?
        } else {
            conn.execute(
                "UPDATE vehiculos
                 SET marca = ?1, modelo = ?2, anio = ?3, tipo_motor = ?4, kilometraje_actual = ?5
                 WHERE placa = ?6",
                params![
                    marca,
                    modelo,
                    req.anio,
                    req.tipo_motor
                        .as_ref()
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty()),
                    req.kilometraje_actual,
                    placa,
                ],
            )?
        };

        if rows_v == 0 {
            return Err(AppError::NotFound(format!(
                "No se encontró el vehículo registrado con la placa {}",
                placa
            )));
        }

        if let Some(id_c) = id_cliente_asignar {
            if let Some(ref nombre) = req.nombre_razon_social {
                let nombre_clean = nombre.trim();
                if !nombre_clean.is_empty() {
                    conn.execute(
                        "UPDATE clientes
                         SET nombre_razon_social = ?1,
                             telefono = COALESCE(?2, telefono),
                             direccion = COALESCE(?3, direccion)
                         WHERE id_cliente = ?4",
                        params![
                            nombre_clean,
                            req.telefono
                                .as_ref()
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty()),
                            req.direccion
                                .as_ref()
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty()),
                            id_c,
                        ],
                    )?;
                }
            }
        }

        let id_u32 = id_usuario.map(|id| id as i32);
        let detalle = format!("Actualización de vehículo placa: {}", placa);
        AuditoriaService::registrar(conn, id_u32, "vehiculos", "UPDATE", &detalle)
            .map_err(|e| AppError::Validation(format!("Error de auditoría: {}", e)))?;

        Ok(())
    }

    pub fn obtener_vehiculo_por_placa(
        conn: &Connection,
        placa: &str,
    ) -> Result<VehiculoCompletoDto, AppError> {
        let placa_clean = placa.trim().to_uppercase();
        let mut stmt = conn.prepare(
            "SELECT
                v.placa, v.id_cliente, v.marca, v.modelo, v.anio, v.tipo_motor, v.kilometraje_actual, v.activo,
                c.tipo_documento, c.numero_documento, c.nombre_razon_social, c.telefono, c.direccion
             FROM vehiculos v
             LEFT JOIN clientes c ON v.id_cliente = c.id_cliente
             WHERE v.placa = ?1 AND v.activo = 1",
        )?;

        let result = stmt
            .query_row([&placa_clean], |row| {
                let id_cliente: i64 = row.get(1)?;
                let tipo_doc: Option<String> = row.get(8)?;
                let num_doc: Option<String> = row.get(9)?;
                let nombre: Option<String> = row.get(10)?;
                let telefono: Option<String> = row.get(11)?;
                let direccion: Option<String> = row.get(12)?;

                let cliente_resumen = nombre.as_ref().map(|nom| ClienteResumenDto {
                    id_cliente,
                    tipo_documento: tipo_doc.clone().unwrap_or_else(|| "DNI".to_string()),
                    numero_documento: num_doc.clone().unwrap_or_default(),
                    nombre_razon_social: nom.clone(),
                    telefono: telefono.clone(),
                    direccion: direccion.clone(),
                });

                Ok(VehiculoCompletoDto {
                    placa: row.get(0)?,
                    id_cliente,
                    marca: row.get(2)?,
                    modelo: row.get(3)?,
                    anio: row.get(4)?,
                    tipo_motor: row.get(5)?,
                    kilometraje_actual: row.get(6)?,
                    activo: row.get(7)?,
                    cliente: cliente_resumen,
                    nombre_cliente: nombre,
                    tipo_documento_cliente: tipo_doc,
                    documento_cliente: num_doc,
                    telefono_cliente: telefono,
                    direccion_cliente: direccion,
                })
            })
            .map_err(|_| {
                AppError::NotFound(format!(
                    "No se encontró vehículo registrado con la placa {}",
                    placa_clean
                ))
            })?;

        Ok(result)
    }

    pub fn obtener_ficha_vehicular_completa(
        conn: &Connection,
        placa: &str,
    ) -> Result<FichaVehicularCompletaDto, AppError> {
        let vehiculo = Self::obtener_vehiculo_por_placa(conn, placa)?;
        let historial = OrdenesService::obtener_historial_por_placa(conn, placa)?;

        Ok(FichaVehicularCompletaDto {
            vehiculo,
            historial,
        })
    }

    pub fn eliminar_vehiculo(
        conn: &Connection,
        placa: &str,
        id_usuario: Option<i64>,
    ) -> Result<(), AppError> {
        let placa_clean = placa.trim().to_uppercase();

        if placa_clean.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo es obligatoria.".to_string(),
            ));
        }

        let total_ots: i32 = conn.query_row(
            "SELECT COUNT(*) FROM ordenes_trabajo WHERE placa = ?1",
            [&placa_clean],
            |row| row.get(0),
        )?;

        if total_ots > 0 {
            return Err(AppError::Validation(format!(
                "No se puede eliminar el vehículo {}: tiene {} orden(es) de trabajo asociada(s).",
                placa_clean, total_ots
            )));
        }

        let total_comprobantes: i32 = conn.query_row(
            "SELECT COUNT(*)
             FROM comprobantes c
             JOIN ordenes_trabajo ot ON ot.id_ot = c.id_ot
             WHERE ot.placa = ?1",
            [&placa_clean],
            |row| row.get(0),
        )?;

        if total_comprobantes > 0 {
            return Err(AppError::Validation(format!(
                "No se puede eliminar el vehículo {}: tiene {} comprobante(s) de venta asociado(s).",
                placa_clean, total_comprobantes
            )));
        }

        let rows = conn.execute(
            "UPDATE vehiculos SET activo = 0 WHERE placa = ?1",
            [&placa_clean],
        )?;

        if rows == 0 {
            return Err(AppError::NotFound(format!(
                "No se encontró el vehículo registrado con placa {}",
                placa_clean
            )));
        }

        let id_u32 = id_usuario.map(|id| id as i32);
        let detalle = format!("Eliminación lógica de vehículo placa: {}", placa_clean);
        AuditoriaService::registrar(conn, id_u32, "vehiculos", "DELETE", &detalle)
            .map_err(|e| AppError::Validation(format!("Error de auditoría: {}", e)))?;

        Ok(())
    }
}

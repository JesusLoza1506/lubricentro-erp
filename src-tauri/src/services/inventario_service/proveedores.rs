use super::InventarioService;
use crate::dto::proveedor_dto::{ActualizarProveedorDto, CrearProveedorDto, ProveedorDto};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};

fn validar_ruc_peruano(ruc: &str) -> bool {
    let ruc = ruc.trim();
    if ruc.len() != 11 || !ruc.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let factores = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let mut suma = 0;
    for (i, &f) in factores.iter().enumerate() {
        let dig = ruc.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        suma += dig * f;
    }
    let residuo = suma % 11;
    let digito_calculado = (11 - residuo) % 10;
    let digito_real = ruc.chars().nth(10).unwrap().to_digit(10).unwrap() as i32;
    digito_calculado == digito_real
}

fn validar_documento_proveedor(tipo: &str, numero: &str) -> Result<(), AppError> {
    let t = tipo.trim().to_uppercase();
    let n = numero.trim();
    match t.as_str() {
        "DNI" => {
            if n.len() != 8 || !n.chars().all(|c| c.is_ascii_digit()) {
                return Err(AppError::Validation(
                    "DNI debe tener exactamente 8 dígitos numéricos.".into(),
                ));
            }
        }
        "RUC" => {
            if !validar_ruc_peruano(n) {
                return Err(AppError::Validation(
                    "El número de RUC es inválido o no supera el dígito de chequeo.".into(),
                ));
            }
        }
        "CE" => {
            if n.len() < 6 || n.len() > 12 || !n.chars().all(|c| c.is_alphanumeric()) {
                return Err(AppError::Validation(
                    "Carné de Extranjería (CE) debe tener entre 6 y 12 caracteres alfanuméricos."
                        .into(),
                ));
            }
        }
        _ => {
            return Err(AppError::Validation(
                "Tipo de documento no soportado. Use DNI, RUC o CE.".into(),
            ))
        }
    }
    Ok(())
}

impl InventarioService {
    pub fn listar_proveedores(conn: &Connection) -> Result<Vec<ProveedorDto>, AppError> {
        let mut stmt = conn.prepare("SELECT id_proveedor, tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor FROM proveedores ORDER BY nombre_razon_social ASC")?;
        let iter = stmt.query_map([], |r| {
            Ok(ProveedorDto {
                id_proveedor: r.get(0)?,
                tipo_documento: r.get(1)?,
                numero_documento: r.get(2)?,
                nombre_razon_social: r.get(3)?,
                telefono: r.get(4)?,
                correo: r.get(5)?,
                direccion: r.get(6)?,
                contacto_asesor: r.get(7)?,
            })
        })?;

        let mut list = Vec::new();
        for p in iter {
            list.push(p?);
        }
        Ok(list)
    }

    pub fn crear_proveedor(
        conn: &mut Connection,
        req: CrearProveedorDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        validar_documento_proveedor(&req.tipo_documento, &req.numero_documento)?;

        let nombre = req.nombre_razon_social.trim();
        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "La razón social o nombre debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO proveedores (tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                req.tipo_documento.trim().to_uppercase(),
                req.numero_documento.trim(),
                nombre,
                req.telefono,
                req.correo,
                req.direccion,
                req.contacto_asesor
            ],
        )?;

        let id_proveedor = tx.last_insert_rowid();

        let despues_json = serde_json::json!({
            "id_proveedor": id_proveedor,
            "tipo_documento": req.tipo_documento.trim().to_uppercase(),
            "numero_documento": req.numero_documento.trim(),
            "nombre_razon_social": nombre,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "CREAR_PROVEEDOR",
            Some("proveedores"),
            Some(&id_proveedor.to_string()),
            None,
            Some(&despues_json),
            Some("Alta de nuevo proveedor"),
            "OK",
        )?;

        tx.commit()?;
        Ok(id_proveedor)
    }

    #[allow(clippy::type_complexity)]
    pub fn actualizar_proveedor(
        conn: &mut Connection,
        req: ActualizarProveedorDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let prov_ant: Option<(String, String, String, Option<String>, Option<String>, Option<String>, Option<String>)> = conn
            .query_row(
                "SELECT tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor FROM proveedores WHERE id_proveedor = ?1",
                [req.id_proveedor],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
            )
            .ok();

        let (tipo_ant, num_ant, nom_ant, tel_ant, cor_ant, dir_ant, con_ant) = match prov_ant {
            Some(p) => p,
            None => return Err(AppError::NotFound("Proveedor no encontrado.".to_string())),
        };

        validar_documento_proveedor(&req.tipo_documento, &req.numero_documento)?;

        let nombre = req.nombre_razon_social.trim();
        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "La razón social o nombre debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE proveedores 
             SET tipo_documento = ?1, numero_documento = ?2, nombre_razon_social = ?3, 
                 telefono = ?4, correo = ?5, direccion = ?6, contacto_asesor = ?7 
             WHERE id_proveedor = ?8",
            params![
                req.tipo_documento.trim().to_uppercase(),
                req.numero_documento.trim(),
                nombre,
                req.telefono,
                req.correo,
                req.direccion,
                req.contacto_asesor,
                req.id_proveedor
            ],
        )?;

        let antes_json = serde_json::json!({
            "tipo_documento": tipo_ant,
            "numero_documento": num_ant,
            "nombre_razon_social": nom_ant,
            "telefono": tel_ant,
            "correo": cor_ant,
            "direccion": dir_ant,
            "contacto_asesor": con_ant,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "tipo_documento": req.tipo_documento.trim().to_uppercase(),
            "numero_documento": req.numero_documento.trim(),
            "nombre_razon_social": nombre,
            "telefono": req.telefono,
            "correo": req.correo,
            "direccion": req.direccion,
            "contacto_asesor": req.contacto_asesor,
        })
        .to_string();

        AuditoriaService::registrar_transaccion_v5(
            &tx,
            Some(id_usuario),
            "USUARIO",
            None,
            None,
            None,
            "INVENTARIO",
            "ACTUALIZAR_PROVEEDOR",
            Some("proveedores"),
            Some(&req.id_proveedor.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some("Actualización de datos de proveedor"),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }
}

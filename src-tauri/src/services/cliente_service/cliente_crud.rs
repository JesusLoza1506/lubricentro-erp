use super::validaciones::{sanitizar_telefono, validar_documento, validar_telefono};
use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto, EditarClienteDto};
use crate::entities::cliente::ClienteEntity;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};

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

    validar_documento(&tipo, &doc)?;
    validar_telefono(req.telefono.as_ref())?;

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

    let telefono_limpio = sanitizar_telefono(req.telefono.as_ref());

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

    validar_documento(&tipo, &doc)?;
    validar_telefono(req.telefono.as_ref())?;

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

    let telefono_limpio = sanitizar_telefono(req.telefono.as_ref());

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

pub fn buscar_clientes_por_termino(
    conn: &Connection,
    termino: &str,
    rol_ejecutor: Option<&str>,
) -> Result<Vec<ClienteDto>, AppError> {
    let t = format!("%{}%", termino.trim().to_uppercase());
    let mut stmt = conn.prepare(
        "SELECT id_cliente, tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, es_varios, fecha_creacion, version
         FROM clientes
         WHERE es_varios = 0 AND (UPPER(nombre_razon_social) LIKE ?1 OR numero_documento LIKE ?1)
         ORDER BY nombre_razon_social ASC
         LIMIT 10",
    )?;

    let es_mecanico = rol_ejecutor
        .map(|r| r.to_uppercase() == "MECANICO")
        .unwrap_or(false);

    let cliente_iter = stmt.query_map([&t], |row| {
        let mut telefono: Option<String> = row.get(4)?;
        let mut correo: Option<String> = row.get(5)?;
        let mut direccion: Option<String> = row.get(6)?;

        if es_mecanico {
            telefono = None;
            correo = None;
            direccion = None;
        }

        Ok(ClienteEntity {
            id_cliente: row.get(0)?,
            tipo_documento: row.get(1)?,
            numero_documento: row.get(2)?,
            nombre_razon_social: row.get(3)?,
            telefono,
            correo,
            direccion,
            es_varios: row.get(7)?,
            fecha_creacion: row.get(8)?,
            version: row.get(9)?,
        })
    })?;

    let mut lista = Vec::new();
    for entity in cliente_iter {
        lista.push(ClienteDto::from(entity?));
    }

    Ok(lista)
}

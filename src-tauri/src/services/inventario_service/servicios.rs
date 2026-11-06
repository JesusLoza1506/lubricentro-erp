use super::InventarioService;
use crate::dto::proveedor_dto::{
    ActualizarServicioDto, CrearServicioDto, ServicioMantenimientoDto,
};
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;
use rusqlite::{params, Connection};

impl InventarioService {
    pub fn listar_servicios_admin(
        conn: &Connection,
    ) -> Result<Vec<ServicioMantenimientoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_servicio, codigo, nombre, descripcion, precio_venta_cent, es_cambio_aceite, activo 
             FROM servicios ORDER BY nombre ASC",
        )?;
        let iter = stmt.query_map([], |r| {
            Ok(ServicioMantenimientoDto {
                id_servicio: r.get(0)?,
                codigo: r.get(1)?,
                nombre: r.get(2)?,
                descripcion: r.get(3)?,
                precio_venta_cent: r.get(4)?,
                es_cambio_aceite: r.get::<_, i32>(5)? == 1,
                activo: r.get::<_, i32>(6)? == 1,
            })
        })?;

        let mut lista = Vec::new();
        for item in iter {
            lista.push(item?);
        }
        Ok(lista)
    }

    pub fn listar_servicios_publicos(
        conn: &Connection,
    ) -> Result<Vec<ServicioMantenimientoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id_servicio, codigo, nombre, descripcion, precio_venta_cent, es_cambio_aceite, activo 
             FROM servicios WHERE activo = 1 ORDER BY nombre ASC",
        )?;
        let iter = stmt.query_map([], |r| {
            Ok(ServicioMantenimientoDto {
                id_servicio: r.get(0)?,
                codigo: r.get(1)?,
                nombre: r.get(2)?,
                descripcion: r.get(3)?,
                precio_venta_cent: r.get(4)?,
                es_cambio_aceite: r.get::<_, i32>(5)? == 1,
                activo: r.get::<_, i32>(6)? == 1,
            })
        })?;

        let mut lista = Vec::new();
        for item in iter {
            lista.push(item?);
        }
        Ok(lista)
    }

    pub fn crear_servicio(
        conn: &mut Connection,
        req: CrearServicioDto,
        id_usuario: i64,
    ) -> Result<i64, AppError> {
        let codigo = req.codigo.trim().to_uppercase();
        let nombre = req.nombre.trim();

        if codigo.len() < 3 || codigo.len() > 30 {
            return Err(AppError::Validation(
                "El código del servicio debe tener entre 3 y 30 caracteres.".into(),
            ));
        }

        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "El nombre del servicio debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        if req.precio_venta_cent <= 0 {
            return Err(AppError::Validation(
                "El precio del servicio debe ser mayor a 0.".into(),
            ));
        }

        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO servicios (codigo, nombre, descripcion, precio_venta_cent, es_cambio_aceite, activo)
             VALUES (?1, ?2, ?3, ?4, ?5, 1)",
            params![
                codigo,
                nombre,
                req.descripcion.as_deref().map(|d| d.trim()),
                req.precio_venta_cent,
                if req.es_cambio_aceite { 1 } else { 0 }
            ],
        )?;

        let id_servicio = tx.last_insert_rowid();

        let despues_json = serde_json::json!({
            "id_servicio": id_servicio,
            "codigo": codigo,
            "nombre": nombre,
            "precio_venta_cent": req.precio_venta_cent,
            "es_cambio_aceite": req.es_cambio_aceite,
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
            "CREAR_SERVICIO",
            Some("servicios"),
            Some(&id_servicio.to_string()),
            None,
            Some(&despues_json),
            Some("Alta de nuevo servicio de mantenimiento"),
            "OK",
        )?;

        tx.commit()?;
        Ok(id_servicio)
    }

    pub fn actualizar_servicio(
        conn: &mut Connection,
        req: ActualizarServicioDto,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let servicio_ant: Option<(String, String, Option<String>, i64, bool)> = conn
            .query_row(
                "SELECT codigo, nombre, descripcion, precio_venta_cent, es_cambio_aceite FROM servicios WHERE id_servicio = ?1",
                [req.id_servicio],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get::<_, i32>(4)? == 1)),
            )
            .ok();

        let (cod_srv, nom_ant, desc_ant, precio_ant, cambio_aceite_ant) = match servicio_ant {
            Some(s) => s,
            None => return Err(AppError::NotFound("Servicio no encontrado.".to_string())),
        };

        let nombre = req.nombre.trim();
        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "El nombre del servicio debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        if req.precio_venta_cent <= 0 {
            return Err(AppError::Validation(
                "El precio del servicio debe ser mayor a 0.".into(),
            ));
        }

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE servicios 
             SET nombre = ?1, descripcion = ?2, precio_venta_cent = ?3, es_cambio_aceite = ?4 
             WHERE id_servicio = ?5",
            params![
                nombre,
                req.descripcion.as_deref().map(|d| d.trim()),
                req.precio_venta_cent,
                if req.es_cambio_aceite { 1 } else { 0 },
                req.id_servicio
            ],
        )?;

        let antes_json = serde_json::json!({
            "codigo": cod_srv,
            "nombre": nom_ant,
            "descripcion": desc_ant,
            "precio_venta_cent": precio_ant,
            "es_cambio_aceite": cambio_aceite_ant,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "codigo": cod_srv,
            "nombre": nombre,
            "descripcion": req.descripcion.as_deref().map(|d| d.trim()),
            "precio_venta_cent": req.precio_venta_cent,
            "es_cambio_aceite": req.es_cambio_aceite,
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
            "ACTUALIZAR_SERVICIO",
            Some("servicios"),
            Some(&req.id_servicio.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some("Actualización de datos del servicio"),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn cambiar_estado_servicio(
        conn: &mut Connection,
        id_servicio: i64,
        activo: bool,
        motivo: &str,
        id_usuario: i64,
    ) -> Result<(), AppError> {
        let motivo_trim = motivo.trim();
        if motivo_trim.len() < 5 || motivo_trim.len() > 250 {
            return Err(AppError::Validation(
                "El motivo del cambio de estado debe tener entre 5 y 250 caracteres.".into(),
            ));
        }

        let (cod_srv, nom_srv, activo_ant): (String, String, bool) = conn
            .query_row(
                "SELECT codigo, nombre, activo FROM servicios WHERE id_servicio = ?1",
                [id_servicio],
                |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i32>(2)? == 1)),
            )
            .map_err(|_| AppError::NotFound("Servicio no encontrado.".to_string()))?;

        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE servicios SET activo = ?1 WHERE id_servicio = ?2",
            params![if activo { 1 } else { 0 }, id_servicio],
        )?;

        let antes_json = serde_json::json!({
            "codigo": cod_srv,
            "nombre": nom_srv,
            "activo": activo_ant,
        })
        .to_string();

        let despues_json = serde_json::json!({
            "codigo": cod_srv,
            "nombre": nom_srv,
            "activo": activo,
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
            "CAMBIAR_ESTADO_SERVICIO",
            Some("servicios"),
            Some(&id_servicio.to_string()),
            Some(&antes_json),
            Some(&despues_json),
            Some(motivo_trim),
            "OK",
        )?;

        tx.commit()?;
        Ok(())
    }
}

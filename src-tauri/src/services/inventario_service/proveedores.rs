use super::InventarioService;
use crate::dto::proveedor_dto::{ActualizarProveedorDto, CrearProveedorDto, ProveedorDto};
use crate::errors::AppError;
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

    pub fn crear_proveedor(conn: &Connection, req: CrearProveedorDto) -> Result<i64, AppError> {
        validar_documento_proveedor(&req.tipo_documento, &req.numero_documento)?;

        let nombre = req.nombre_razon_social.trim();
        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "La razón social o nombre debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        conn.execute(
            "INSERT INTO proveedores (tipo_documento, numero_documento, nombre_razon_social, telefono, correo, direccion, contacto_asesor)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![req.tipo_documento.trim().to_uppercase(), req.numero_documento.trim(), nombre, req.telefono, req.correo, req.direccion, req.contacto_asesor],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_proveedor(
        conn: &Connection,
        req: ActualizarProveedorDto,
    ) -> Result<(), AppError> {
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM proveedores WHERE id_proveedor = ?1",
            [req.id_proveedor],
            |r| r.get(0),
        )?;

        if count == 0 {
            return Err(AppError::NotFound("Proveedor no encontrado.".to_string()));
        }

        validar_documento_proveedor(&req.tipo_documento, &req.numero_documento)?;

        let nombre = req.nombre_razon_social.trim();
        if nombre.len() < 3 || nombre.len() > 150 {
            return Err(AppError::Validation(
                "La razón social o nombre debe tener entre 3 y 150 caracteres.".into(),
            ));
        }

        conn.execute(
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

        Ok(())
    }
}

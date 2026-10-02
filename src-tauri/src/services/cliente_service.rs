use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto};
use crate::dto::vehiculo_dto::{CrearVehiculoDto, VehiculoDto};
use crate::entities::cliente::ClienteEntity;
use crate::entities::vehiculo::VehiculoEntity;
use crate::errors::AppError;
use rusqlite::{params, Connection};

pub struct ClienteService;

impl ClienteService {
    /// Registra un nuevo cliente validando DNI (8 dígitos) o RUC (11 dígitos) y duplicados
    pub fn registrar_cliente(conn: &Connection, req: CrearClienteDto) -> Result<i64, AppError> {
        let tipo = req.tipo_documento.trim().to_uppercase();
        let doc = req.numero_documento.trim();
        let nombre = req.nombre_razon_social.trim();

        if tipo != "DNI" && tipo != "RUC" {
            return Err(AppError::Validation(
                "El tipo de documento debe ser DNI o RUC.".to_string(),
            ));
        }

        if (tipo == "DNI" && doc.len() != 8) || (tipo == "RUC" && doc.len() != 11) {
            return Err(AppError::Validation(format!(
                "El documento {} de tipo {} no tiene una longitud válida.",
                doc, tipo
            )));
        }

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
                "Ya existe un cliente registrado con el documento {}",
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

    /// Busca un cliente por número de documento (DNI/RUC)
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
                    "No se encontró cliente con el documento {}",
                    doc_clean
                ))
            })?;

        Ok(ClienteDto::from(entity))
    }

    /// Registra un nuevo vehículo asociado a un cliente
    pub fn registrar_vehiculo(
        conn: &Connection,
        req: CrearVehiculoDto,
    ) -> Result<String, AppError> {
        let placa = req.placa.trim().to_uppercase();
        let marca = req.marca.trim();
        let modelo = req.modelo.trim();

        if placa.is_empty() {
            return Err(AppError::Validation(
                "La placa del vehículo no puede estar vacía.".to_string(),
            ));
        }

        if marca.is_empty() || modelo.is_empty() {
            return Err(AppError::Validation(
                "La marca y modelo del vehículo son obligatorios.".to_string(),
            ));
        }

        let cliente_existe: i32 = conn.query_row(
            "SELECT COUNT(*) FROM clientes WHERE id_cliente = ?1 AND activo = 1",
            [req.id_cliente],
            |row| row.get(0),
        )?;

        if cliente_existe == 0 {
            return Err(AppError::NotFound(format!(
                "El cliente asociado (ID {}) no existe o está inactivo.",
                req.id_cliente
            )));
        }

        let placa_existe: i32 = conn.query_row(
            "SELECT COUNT(*) FROM vehiculos WHERE placa = ?1",
            [&placa],
            |row| row.get(0),
        )?;

        if placa_existe > 0 {
            return Err(AppError::Validation(format!(
                "El vehículo con placa {} ya está registrado.",
                placa
            )));
        }

        conn.execute(
            "INSERT INTO vehiculos (placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)",
            params![
                placa,
                req.id_cliente,
                marca,
                modelo,
                req.anio,
                req.tipo_motor.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()),
                req.kilometraje_actual,
            ],
        )?;

        Ok(placa)
    }

    /// Obtiene la lista de vehículos pertenecientes a un cliente
    pub fn listar_vehiculos_por_cliente(
        conn: &Connection,
        id_cliente: i64,
    ) -> Result<Vec<VehiculoDto>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT placa, id_cliente, marca, modelo, anio, tipo_motor, kilometraje_actual, activo
             FROM vehiculos
             WHERE id_cliente = ?1 AND activo = 1",
        )?;

        let iter = stmt.query_map([id_cliente], |row| {
            let entity = VehiculoEntity {
                placa: row.get(0)?,
                id_cliente: row.get(1)?,
                marca: row.get(2)?,
                modelo: row.get(3)?,
                anio: row.get(4)?,
                tipo_motor: row.get(5)?,
                kilometraje_actual: row.get(6)?,
                activo: row.get(7)?,
            };
            Ok(VehiculoDto::from(entity))
        })?;

        let mut vehiculos = Vec::new();
        for item in iter {
            vehiculos.push(item?);
        }

        Ok(vehiculos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::tempdir;

    #[test]
    fn test_crud_cliente_y_vehiculo_service() {
        let dir = tempdir().unwrap();
        let conn = init_db(dir.path().to_path_buf()).unwrap();

        let cliente_req = CrearClienteDto {
            tipo_documento: "RUC".to_string(),
            numero_documento: "20600695771".to_string(),
            nombre_razon_social: "NUBEFACT SA".to_string(),
            telefono: Some("987654321".to_string()),
            direccion: Some("Calle Libertad 116".to_string()),
        };

        let id_cliente = ClienteService::registrar_cliente(&conn, cliente_req).unwrap();
        assert!(id_cliente > 0);

        let cliente_dto =
            ClienteService::buscar_cliente_por_documento(&conn, "20600695771").unwrap();
        assert_eq!(cliente_dto.nombre_razon_social, "NUBEFACT SA");

        let vehiculo_req = CrearVehiculoDto {
            placa: "ABC-123".to_string(),
            id_cliente,
            marca: "Toyota".to_string(),
            modelo: "Yaris".to_string(),
            anio: Some(2020),
            tipo_motor: Some("1.5L".to_string()),
            kilometraje_actual: 45000,
        };

        let placa = ClienteService::registrar_vehiculo(&conn, vehiculo_req).unwrap();
        assert_eq!(placa, "ABC-123");

        let vehiculos = ClienteService::listar_vehiculos_por_cliente(&conn, id_cliente).unwrap();
        assert_eq!(vehiculos.len(), 1);
        assert_eq!(vehiculos[0].marca, "Toyota");
    }
}

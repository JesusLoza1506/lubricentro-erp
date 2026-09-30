use crate::dto::cliente_dto::ClienteDTO;
use crate::entities::cliente::ClienteEntity;

pub struct ClienteService;

impl ClienteService {
    pub fn mapear_a_dto(cliente: ClienteEntity) -> ClienteDTO {
        ClienteDTO {
            id_cliente: cliente.id_cliente,
            tipo_documento: cliente.tipo_documento,
            numero_documento: cliente.numero_documento,
            nombre_razon_social: cliente.nombre_razon_social,
            telefono: cliente.telefono,
            direccion: cliente.direccion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapear_cliente_a_dto() {
        let entity = ClienteEntity {
            id_cliente: 1,
            tipo_documento: "RUC".to_string(),
            numero_documento: "20600695771".to_string(),
            nombre_razon_social: "NUBEFACT SA".to_string(),
            telefono: Some("987654321".to_string()),
            direccion: Some("Calle Libertad 116".to_string()),
        };

        let dto = ClienteService::mapear_a_dto(entity);
        assert_eq!(dto.id_cliente, 1);
        assert_eq!(dto.numero_documento, "20600695771");
    }
}

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

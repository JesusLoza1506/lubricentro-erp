use crate::dto::orden_trabajo_dto::OrdenTrabajoDTO;
use crate::entities::orden_trabajo::OrdenTrabajoEntity;

pub struct OrdenesService;

impl OrdenesService {
    pub fn mapear_a_dto(entity: OrdenTrabajoEntity) -> OrdenTrabajoDTO {
        OrdenTrabajoDTO {
            id_ot: entity.id_ot,
            codigo_ot: entity.codigo_ot,
            placa: entity.placa,
            id_mecanico: entity.id_mecanico,
            zanja: entity.zanja,
            estado: entity.estado,
            kilometraje_ingreso: entity.kilometraje_ingreso,
            proximo_kilometraje: entity.proximo_kilometraje,
            fecha_ingreso: entity.fecha_ingreso,
        }
    }
}

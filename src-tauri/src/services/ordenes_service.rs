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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapear_orden_a_dto() {
        let entity = OrdenTrabajoEntity {
            id_ot: 1,
            codigo_ot: "OT-001".to_string(),
            placa: "ABC-123".to_string(),
            id_mecanico: 2,
            zanja: Some(1),
            estado: "EN_PROCESO".to_string(),
            kilometraje_ingreso: 45000,
            proximo_kilometraje: 50000,
            fecha_ingreso: Some("2026-09-29".to_string()),
        };

        let dto = OrdenesService::mapear_a_dto(entity);
        assert_eq!(dto.codigo_ot, "OT-001");
    }
}

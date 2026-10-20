pub mod helpers;
pub mod ot_consultas;
pub mod ot_gestion;
pub mod ot_productos;
pub mod ot_servicios;

use crate::dto::orden_trabajo_dto::{
    AgregarProductoOtDto, AgregarServicioOtDto, CambiarEstadoOtDto, CrearOrdenTrabajoDto,
    DetalleItemOtDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto, ServicioResumenDto,
};
use crate::errors::AppError;
use rusqlite::Connection;

pub struct OrdenesService;

impl OrdenesService {
    pub fn crear_orden_trabajo(
        conn: &mut Connection,
        req: CrearOrdenTrabajoDto,
        id_creador: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::crear_orden_trabajo(conn, req, id_creador)
    }

    pub fn cambiar_estado_ot(
        conn: &mut Connection,
        req: CambiarEstadoOtDto,
        id_usuario: Option<i64>,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::cambiar_estado_ot(conn, req, id_usuario)
    }

    pub fn reasignar_mecanico_ot(
        conn: &Connection,
        req: ReasignarMecanicoOtDto,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::reasignar_mecanico_ot(conn, req)
    }

    pub fn listar_ordenes_trabajo(
        conn: &Connection,
        id_usuario: Option<i64>,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::listar_ordenes_trabajo(conn, id_usuario)
    }

    pub fn obtener_detalles_ot(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<Vec<DetalleItemOtDto>, AppError> {
        ot_consultas::obtener_detalles_ot(conn, id_ot)
    }

    pub fn obtener_ot_por_id(
        conn: &Connection,
        id_ot: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_consultas::obtener_ot_por_id(conn, id_ot)
    }

    pub fn obtener_historial_por_id_vehiculo(
        conn: &Connection,
        id_vehiculo: i64,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::obtener_historial_por_id_vehiculo(conn, id_vehiculo)
    }

    pub fn obtener_historial_por_placa(
        conn: &Connection,
        placa: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::obtener_historial_por_placa(conn, placa)
    }

    pub fn agregar_producto_ot(
        conn: &mut Connection,
        req: AgregarProductoOtDto,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_productos::agregar_producto_ot(conn, req, id_usuario_ejecutor)
    }

    pub fn eliminar_producto_ot(
        conn: &mut Connection,
        id_detalle: i64,
        id_ot: i64,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_productos::eliminar_producto_ot(conn, id_detalle, id_ot, id_usuario_ejecutor)
    }

    pub fn agregar_servicio_ot(
        conn: &Connection,
        req: AgregarServicioOtDto,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_servicios::agregar_servicio_ot(conn, req, id_usuario_ejecutor)
    }

    pub fn eliminar_servicio_ot(
        conn: &Connection,
        id_detalle: i64,
        id_ot: i64,
        id_usuario_ejecutor: i64,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_servicios::eliminar_servicio_ot(conn, id_detalle, id_ot, id_usuario_ejecutor)
    }

    pub fn listar_servicios(conn: &Connection) -> Result<Vec<ServicioResumenDto>, AppError> {
        ot_servicios::listar_servicios(conn)
    }
}

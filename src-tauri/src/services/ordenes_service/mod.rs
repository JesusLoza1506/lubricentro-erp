pub mod helpers;
pub mod ot_consultas;
pub mod ot_gestion;
pub mod ot_productos;
pub mod ot_servicios;

use crate::dto::orden_trabajo_dto::{
    AgregarServicioOtDto, CancelarOtDto, CrearOrdenTrabajoDto, DevolverInsumoOtDto, FinalizarOtDto,
    IniciarTrabajoOtDto, ModificarServicioOtDto, OrdenTrabajoHistorialDto, ReasignarMecanicoOtDto,
    RegistrarInsumoOtDto, ServicioResumenDto, TableroZanjasDto,
};
use crate::errors::AppError;
use rusqlite::Connection;

pub struct OrdenesService;

impl OrdenesService {
    pub fn obtener_tablero_zanjas(
        conn: &Connection,
        id_usuario: i64,
        rol: &str,
    ) -> Result<TableroZanjasDto, AppError> {
        ot_consultas::obtener_tablero_zanjas(conn, id_usuario, rol)
    }

    pub fn listar_ordenes_trabajo(
        conn: &Connection,
        id_usuario: i64,
        rol: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::listar_ordenes_trabajo(conn, id_usuario, rol)
    }

    pub fn crear_orden_trabajo(
        conn: &mut Connection,
        req: CrearOrdenTrabajoDto,
        id_creador: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::crear_orden_trabajo(conn, req, id_creador, rol)
    }

    pub fn iniciar_trabajo_ot(
        conn: &mut Connection,
        req: IniciarTrabajoOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::iniciar_trabajo_ot(conn, req, id_usuario, rol)
    }

    pub fn reasignar_mecanico_ot(
        conn: &mut Connection,
        req: ReasignarMecanicoOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::reasignar_mecanico_ot(conn, req, id_usuario, rol)
    }

    pub fn registrar_insumo_ot(
        conn: &mut Connection,
        req: RegistrarInsumoOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_productos::registrar_insumo_ot(conn, req, id_usuario, rol)
    }

    pub fn devolver_insumo_ot(
        conn: &mut Connection,
        req: DevolverInsumoOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_productos::devolver_insumo_ot(conn, req, id_usuario, rol)
    }

    pub fn agregar_servicio_ot(
        conn: &mut Connection,
        req: AgregarServicioOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_servicios::agregar_servicio_ot(conn, req, id_usuario, rol)
    }

    pub fn modificar_servicio_ot(
        conn: &mut Connection,
        req: ModificarServicioOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_servicios::modificar_servicio_ot(conn, req, id_usuario, rol)
    }

    pub fn eliminar_servicio_ot(
        conn: &mut Connection,
        id_detalle: i64,
        id_ot: i64,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_servicios::eliminar_servicio_ot(conn, id_detalle, id_ot, id_usuario, rol)
    }

    pub fn finalizar_ot(
        conn: &mut Connection,
        req: FinalizarOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::finalizar_ot(conn, req, id_usuario, rol)
    }

    pub fn cancelar_ot(
        conn: &mut Connection,
        req: CancelarOtDto,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_gestion::cancelar_ot(conn, req, id_usuario, rol)
    }

    pub fn obtener_ot_por_id(
        conn: &Connection,
        id_ot: i64,
        id_usuario: i64,
        rol: &str,
    ) -> Result<OrdenTrabajoHistorialDto, AppError> {
        ot_consultas::obtener_ot_por_id(conn, id_ot, id_usuario, rol)
    }

    pub fn obtener_historial_por_id_vehiculo(
        conn: &Connection,
        id_vehiculo: i64,
        id_usuario: i64,
        rol: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::obtener_historial_por_id_vehiculo(conn, id_vehiculo, id_usuario, rol)
    }

    pub fn obtener_historial_paginado_por_id_vehiculo(
        conn: &Connection,
        id_vehiculo: i64,
        id_usuario: i64,
        rol: &str,
        limite: u32,
        offset: u32,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::obtener_historial_paginado_por_id_vehiculo(
            conn,
            id_vehiculo,
            id_usuario,
            rol,
            limite,
            offset,
        )
    }

    pub fn obtener_historial_por_placa(
        conn: &Connection,
        placa: &str,
        id_usuario: i64,
        rol: &str,
    ) -> Result<Vec<OrdenTrabajoHistorialDto>, AppError> {
        ot_consultas::obtener_historial_por_placa(conn, placa, id_usuario, rol)
    }

    pub fn listar_servicios(conn: &Connection) -> Result<Vec<ServicioResumenDto>, AppError> {
        ot_servicios::listar_servicios(conn)
    }
}

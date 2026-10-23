pub mod cliente_crud;
pub mod ficha_vehicular;
pub mod validaciones;
pub mod vehiculo_crud;

use crate::dto::cliente_dto::{ClienteDto, CrearClienteDto, EditarClienteDto};
use crate::dto::vehiculo_dto::{
    CambiarEstadoVehiculoPayloadDto, CorregirHistorialOtPayloadDto, CrearVehiculoPayloadDto,
    EditarVehiculoPayloadDto, FichaVehicularCompletaDto, ProximoCambioAceiteDto,
    VehiculoCompletoDto,
};
use crate::errors::AppError;
use rusqlite::Connection;

pub struct ClienteService;

impl ClienteService {
    pub fn sanitizar_placa(placa: &str) -> String {
        validaciones::sanitizar_placa(placa)
    }

    pub fn sanitizar_telefono(telefono: Option<&String>) -> Option<String> {
        validaciones::sanitizar_telefono(telefono)
    }

    pub fn registrar_cliente(
        conn: &Connection,
        req: CrearClienteDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<i64, AppError> {
        cliente_crud::registrar_cliente(conn, req, id_usuario, username, rol)
    }

    pub fn actualizar_cliente(
        conn: &Connection,
        req: EditarClienteDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<(), AppError> {
        cliente_crud::actualizar_cliente(conn, req, id_usuario, username, rol)
    }

    pub fn buscar_cliente_por_documento(
        conn: &Connection,
        tipo_documento: &str,
        numero_documento: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<ClienteDto, AppError> {
        cliente_crud::buscar_cliente_por_documento(
            conn,
            tipo_documento,
            numero_documento,
            rol_ejecutor,
        )
    }

    pub fn buscar_clientes_por_termino(
        conn: &Connection,
        termino: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<Vec<ClienteDto>, AppError> {
        cliente_crud::buscar_clientes_por_termino(conn, termino, rol_ejecutor)
    }

    pub fn registrar_vehiculo(
        conn: &Connection,
        req: CrearVehiculoPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<String, AppError> {
        vehiculo_crud::registrar_vehiculo(conn, req, id_usuario, username, rol)
    }

    pub fn actualizar_vehiculo(
        conn: &Connection,
        req: EditarVehiculoPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol_usuario: &str,
    ) -> Result<(), AppError> {
        vehiculo_crud::actualizar_vehiculo(conn, req, id_usuario, username, rol_usuario)
    }

    pub fn cambiar_estado_vehiculo(
        conn: &Connection,
        payload: CambiarEstadoVehiculoPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<(), AppError> {
        vehiculo_crud::cambiar_estado_vehiculo(conn, payload, id_usuario, username, rol)
    }

    pub fn corregir_historial_ot(
        conn: &Connection,
        payload: CorregirHistorialOtPayloadDto,
        id_usuario: Option<i64>,
        username: Option<&str>,
        rol: Option<&str>,
    ) -> Result<(), AppError> {
        ficha_vehicular::corregir_historial_ot(conn, payload, id_usuario, username, rol)
    }

    pub fn obtener_vehiculo_por_placa(
        conn: &Connection,
        placa: &str,
        rol_ejecutor: Option<&str>,
    ) -> Result<VehiculoCompletoDto, AppError> {
        vehiculo_crud::obtener_vehiculo_por_placa(conn, placa, rol_ejecutor)
    }

    pub fn obtener_ficha_vehicular_completa(
        conn: &Connection,
        placa: &str,
        rol_ejecutor: Option<&str>,
        pagina: u32,
        limite: u32,
    ) -> Result<FichaVehicularCompletaDto, AppError> {
        ficha_vehicular::obtener_ficha_vehicular_completa(conn, placa, rol_ejecutor, pagina, limite)
    }

    pub fn obtener_proximos_cambios_aceite(
        conn: &Connection,
        rol_ejecutor: Option<&str>,
    ) -> Result<Vec<ProximoCambioAceiteDto>, AppError> {
        ficha_vehicular::obtener_proximos_cambios_aceite(conn, rol_ejecutor)
    }
}

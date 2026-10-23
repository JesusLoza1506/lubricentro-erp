import { invoke } from '@tauri-apps/api/core';
import {
  OrdenTrabajoHistorialDTO,
  TableroZanjasDTO,
  CrearOrdenTrabajoPayload,
  IniciarTrabajoOtPayload,
  ReasignarMecanicoOtPayload,
  FinalizarOtPayload,
  CancelarOtPayload,
  RegistrarInsumoOtPayload,
  DevolverInsumoOtPayload,
  AgregarServicioOtPayload,
  ModificarServicioOtPayload,
} from '../types/ordenTrabajo';

export interface MecanicoDTO {
  id_usuario: number;
  nombre_completo: string;
}

export interface ServicioResumenDTO {
  id_servicio: number;
  codigo: string;
  nombre: string;
  precio_venta_cent: number;
  precio_venta_soles: number;
  es_cambio_aceite: number;
}

export const ordenesService = {
  async obtenerTableroZanjas(idUsuario: number, rol: string): Promise<TableroZanjasDTO> {
    return await invoke<TableroZanjasDTO>('obtener_tablero_zanjas_cmd', { idUsuario, rol });
  },

  async listarOrdenesTrabajo(idUsuario: number, rol: string): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('listar_ordenes_trabajo_cmd', {
      idUsuario,
      rol,
    });
  },

  async obtenerOtPorId(
    idOt: number,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('obtener_ot_por_id_cmd', {
      idOt,
      idUsuario,
      rol,
    });
  },

  async crearOrdenTrabajo(
    payload: CrearOrdenTrabajoPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('crear_orden_trabajo_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async iniciarTrabajoOT(
    payload: IniciarTrabajoOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('iniciar_trabajo_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async reasignarMecanicoOT(
    payload: ReasignarMecanicoOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('reasignar_mecanico_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async finalizarOT(
    payload: FinalizarOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('finalizar_ot_cmd', { payload, idUsuario, rol });
  },

  async cancelarOT(
    payload: CancelarOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('cancelar_ot_cmd', { payload, idUsuario, rol });
  },

  async registrarInsumoOT(
    payload: RegistrarInsumoOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('registrar_insumo_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async devolverInsumoOT(
    payload: DevolverInsumoOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('devolver_insumo_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async agregarServicioOT(
    payload: AgregarServicioOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('agregar_servicio_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async modificarServicioOT(
    payload: ModificarServicioOtPayload,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('modificar_servicio_ot_cmd', {
      payload,
      idUsuario,
      rol,
    });
  },

  async eliminarServicioOT(
    idDetalle: number,
    idOt: number,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('eliminar_servicio_ot_cmd', {
      idDetalle,
      idOt,
      idUsuario,
      rol,
    });
  },

  async obtenerHistorialPorPlaca(
    placa: string,
    idUsuario: number,
    rol: string
  ): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('obtener_historial_por_placa_cmd', {
      placa,
      idUsuario,
      rol,
    });
  },

  async listarMecanicos(): Promise<MecanicoDTO[]> {
    return await invoke<MecanicoDTO[]>('listar_mecanicos_cmd');
  },

  async listarServicios(): Promise<ServicioResumenDTO[]> {
    return await invoke<ServicioResumenDTO[]>('listar_servicios_cmd');
  },
};

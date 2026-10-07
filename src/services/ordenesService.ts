import { invoke } from '@tauri-apps/api/core';
import {
  OrdenTrabajoHistorialDTO,
  CrearOrdenTrabajoPayload,
  CambiarEstadoOtPayload,
  ReasignarMecanicoOtPayload,
} from '../types/ordenTrabajo';

export interface MecanicoDTO {
  id_usuario: number;
  nombre_completo: string;
}

export interface ServicioResumenDTO {
  id_servicio: number;
  descripcion: string;
  precio_base: number;
}

export interface AgregarProductoOtPayload {
  id_ot: number;
  id_producto: number;
  cantidad: number;
}

export interface AgregarServicioOtPayload {
  id_ot: number;
  id_servicio: number;
}

export const ordenesService = {
  /**
   * Obtiene la lista global o filtrada de OTs para el tablero Kanban
   */
  async listarOrdenesTrabajo(idUsuario?: number): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('listar_ordenes_trabajo_cmd', {
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Registra una nueva Orden de Trabajo
   */
  async crearOrdenTrabajo(
    payload: CrearOrdenTrabajoPayload,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('crear_orden_trabajo_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Cambia el estado técnico de la OT (EN_ESPERA -> EN_PROCESO -> FINALIZADO/CANCELADO)
   */
  async cambiarEstadoOT(
    payload: CambiarEstadoOtPayload,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('cambiar_estado_ot_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Reasigna el mecánico de una OT (Exclusivo Administrador)
   */
  async reasignarMecanicoOT(
    payload: ReasignarMecanicoOtPayload,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('reasignar_mecanico_ot_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Obtiene el historial cronológico de OTs de un vehículo específico por su placa
   */
  async obtenerHistorialPorPlaca(
    placa: string,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('obtener_historial_por_placa_cmd', {
      placa,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Obtiene dinámicamente la lista de usuarios activos con rol MECANICO
   */
  async listarMecanicos(): Promise<MecanicoDTO[]> {
    return await invoke<MecanicoDTO[]>('listar_mecanicos_cmd');
  },

  /**
   * Agrega un producto del inventario a la OT
   */
  async agregarProductoOT(
    payload: AgregarProductoOtPayload,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('agregar_producto_ot_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Elimina un producto de la OT
   */
  async eliminarProductoOT(
    idDetalle: number,
    idOt: number,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('eliminar_producto_ot_cmd', {
      idDetalle,
      idOt,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Agrega un servicio a la OT
   */
  async agregarServicioOT(
    payload: AgregarServicioOtPayload,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('agregar_servicio_ot_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Elimina un servicio de la OT
   */
  async eliminarServicioOT(
    idDetalle: number,
    idOt: number,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('eliminar_servicio_ot_cmd', {
      idDetalle,
      idOt,
      idUsuario: idUsuario ?? null,
    });
  },

  /**
   * Obtiene la lista de servicios activos para seleccionar en la OT
   */
  async listarServicios(): Promise<ServicioResumenDTO[]> {
    return await invoke<ServicioResumenDTO[]>('listar_servicios_cmd');
  },
};

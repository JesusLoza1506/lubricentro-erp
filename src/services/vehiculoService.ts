import { invoke } from '@tauri-apps/api/core';
import {
  VehiculoCompletoDTO,
  OrdenTrabajoHistorialDTO,
  FichaVehicularCompletaDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
} from '../types/vehiculo';

export const vehiculoService = {
  // 1. Obtener Ficha Vehicular e Historial en 1 sola consulta atómica
  obtenerFichaVehicularCompleta: async (
    placa: string,
    idUsuario?: number
  ): Promise<FichaVehicularCompletaDTO> => {
    return await invoke<FichaVehicularCompletaDTO>('obtener_ficha_vehicular_completa_cmd', {
      placa,
      idUsuario: idUsuario ?? null,
    });
  },

  // 2. Obtener Ficha Vehicular Completa por Placa
  obtenerVehiculoPorPlaca: async (
    placa: string,
    idUsuario?: number
  ): Promise<VehiculoCompletoDTO> => {
    return await invoke<VehiculoCompletoDTO>('obtener_vehiculo_por_placa_cmd', {
      placa,
      idUsuario: idUsuario ?? null,
    });
  },

  // 3. Obtener Historial de Ordenes de Trabajo por Placa
  obtenerHistorialPorPlaca: async (
    placa: string,
    idUsuario?: number
  ): Promise<OrdenTrabajoHistorialDTO[]> => {
    return await invoke<OrdenTrabajoHistorialDTO[]>('obtener_historial_por_placa_cmd', {
      placa,
      idUsuario: idUsuario ?? null,
    });
  },

  // 4. Registrar Nuevo Vehículo / Cliente
  registrarVehiculo: async (payload: CrearVehiculoDTO, idUsuario?: number): Promise<void> => {
    return await invoke('registrar_vehiculo_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  // 5. Actualizar Vehículo Existente
  actualizarVehiculo: async (payload: EditarVehiculoDTO, idUsuario?: number): Promise<void> => {
    return await invoke('actualizar_vehiculo_cmd', {
      payload,
      idUsuario: idUsuario ?? null,
    });
  },

  // 6. Eliminar Vehículo (Exclusivo Administrador)
  eliminarVehiculo: async (placa: string, idUsuario?: number): Promise<void> => {
    return await invoke('eliminar_vehiculo_cmd', {
      placa,
      idUsuario: idUsuario ?? null,
    });
  },
};

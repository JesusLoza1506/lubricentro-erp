import { invoke } from '@tauri-apps/api/core';
import {
  VehiculoCompletoDTO,
  FichaVehicularCompletaDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
  CambiarEstadoVehiculoPayloadDTO,
  CorregirHistorialOtPayloadDTO,
  ProximoCambioAceiteDTO,
  ResultadoPaginadoDTO,
} from '../types/vehiculo';

export const vehiculoService = {
  // 1. Obtener Ficha Vehicular e Historial Paginado en BD
  obtenerFichaVehicularCompleta: async (
    placa: string,
    pagina = 1,
    limite = 20
  ): Promise<FichaVehicularCompletaDTO> => {
    return await invoke<FichaVehicularCompletaDTO>('obtener_ficha_vehicular_completa_cmd', {
      placa: placa.replace(/[^a-zA-Z0-9]/g, '').toUpperCase(),
      pagina,
      limite,
    });
  },

  // 2. Obtener Vehículo por Placa
  obtenerVehiculoPorPlaca: async (placa: string): Promise<VehiculoCompletoDTO> => {
    return await invoke<VehiculoCompletoDTO>('obtener_vehiculo_por_placa_cmd', {
      placa: placa.replace(/[^a-zA-Z0-9]/g, '').toUpperCase(),
    });
  },

  // 3. Listar Vehículos Candidatos (Compatibilidad Modal OT)
  listarVehiculos: async (): Promise<VehiculoCompletoDTO[]> => {
    const res = await invoke<ResultadoPaginadoDTO<ProximoCambioAceiteDTO>>(
      'obtener_proximos_cambios_aceite_cmd',
      {
        pagina: 1,
        limite: 100,
        busqueda: null,
        filtroCausa: null,
      }
    );
    return (res.items || []).map((p) => ({
      id_vehiculo: p.id_vehiculo,
      placa: p.placa,
      id_cliente: p.id_cliente,
      marca: '',
      modelo: '',
      kilometraje_actual: p.kilometraje_actual,
      activo: 1,
      nombre_cliente: p.nombre_razon_social,
      telefono_cliente: p.telefono,
    }));
  },

  // 4. Registrar Nuevo Vehículo
  registrarVehiculo: async (payload: CrearVehiculoDTO): Promise<string> => {
    return await invoke<string>('registrar_vehiculo_cmd', {
      payload,
    });
  },

  // 5. Actualizar Vehículo / Reasignar Propietario
  actualizarVehiculo: async (payload: EditarVehiculoDTO): Promise<void> => {
    return await invoke<void>('actualizar_vehiculo_cmd', {
      payload,
    });
  },

  // 6. Cambiar Estado del Vehículo (Desactivar/Reactivar - Admins)
  cambiarEstadoVehiculo: async (payload: CambiarEstadoVehiculoPayloadDTO): Promise<void> => {
    return await invoke<void>('cambiar_estado_vehiculo_cmd', {
      payload,
    });
  },

  // 7. Corregir Historial de OT Finalizada (Admins)
  corregirHistorialOt: async (payload: CorregirHistorialOtPayloadDTO): Promise<void> => {
    return await invoke<void>('corregir_historial_ot_cmd', {
      payload,
    });
  },

  // 8. Consultar Próximos Cambios de Aceite (Admins y Cajeros) - Paginado y Filtrado en BD
  obtenerProximosCambiosAceite: async (
    pagina = 1,
    limite = 20,
    busqueda?: string,
    filtroCausa?: string
  ): Promise<ResultadoPaginadoDTO<ProximoCambioAceiteDTO>> => {
    return await invoke<ResultadoPaginadoDTO<ProximoCambioAceiteDTO>>(
      'obtener_proximos_cambios_aceite_cmd',
      {
        pagina,
        limite,
        busqueda: busqueda && busqueda.trim().length > 0 ? busqueda.trim() : null,
        filtroCausa: filtroCausa && filtroCausa !== 'TODOS' ? filtroCausa : null,
      }
    );
  },
};

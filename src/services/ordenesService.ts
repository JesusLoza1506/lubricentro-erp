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
  FiltroPaginacionOtPayload,
  ResultadoPaginadoOtDTO,
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

export interface ProductoInsumoDTO {
  id_producto: number;
  codigo?: string;
  nombre: string;
  categoria?: string;
  unidad_medida: string;
  stock_actual: number;
  precio_venta?: number; // En céntimos
  precio_venta_soles: number; // Garantizado en Soles para la UI
}

export const ordenesService = {
  async obtenerTableroZanjas(_idUsuario?: number, _rol?: string): Promise<TableroZanjasDTO> {
    return await invoke<TableroZanjasDTO>('obtener_tablero_zanjas_cmd');
  },

  async listarOrdenesTrabajo(
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('listar_ordenes_trabajo_cmd');
  },

  async listarOrdenesTrabajoPaginado(
    filtros: FiltroPaginacionOtPayload
  ): Promise<ResultadoPaginadoOtDTO> {
    return await invoke<ResultadoPaginadoOtDTO>('listar_ordenes_trabajo_paginado_cmd', { filtros });
  },

  async obtenerOtPorId(
    idOt: number,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('obtener_ot_por_id_cmd', { idOt });
  },

  async crearOrdenTrabajo(
    payload: CrearOrdenTrabajoPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('crear_orden_trabajo_cmd', { payload });
  },

  async iniciarTrabajoOT(
    payload: IniciarTrabajoOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('iniciar_trabajo_ot_cmd', { payload });
  },

  async reasignarMecanicoOT(
    payload: ReasignarMecanicoOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('reasignar_mecanico_ot_cmd', { payload });
  },

  async modificarNotaInternaOT(
    payload: { id_ot: number; nota_interna: string },
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('modificar_nota_interna_ot_cmd', { payload });
  },

  async finalizarOT(
    payload: FinalizarOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('finalizar_ot_cmd', { payload });
  },

  async cancelarOT(
    payload: CancelarOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('cancelar_ot_cmd', { payload });
  },

  async registrarInsumoOT(
    payload: RegistrarInsumoOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('registrar_insumo_ot_cmd', { payload });
  },

  async devolverInsumoOT(
    payload: DevolverInsumoOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('devolver_insumo_ot_cmd', { payload });
  },

  async agregarServicioOT(
    payload: AgregarServicioOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('agregar_servicio_ot_cmd', { payload });
  },

  async modificarServicioOT(
    payload: ModificarServicioOtPayload,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('modificar_servicio_ot_cmd', { payload });
  },

  async eliminarServicioOT(
    idDetalle: number,
    idOt: number,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO> {
    return await invoke<OrdenTrabajoHistorialDTO>('eliminar_servicio_ot_cmd', {
      idDetalle,
      idOt,
    });
  },

  async obtenerHistorialPorPlaca(
    placa: string,
    _idUsuario?: number,
    _rol?: string
  ): Promise<OrdenTrabajoHistorialDTO[]> {
    return await invoke<OrdenTrabajoHistorialDTO[]>('obtener_historial_por_placa_cmd', { placa });
  },

  async listarMecanicos(): Promise<MecanicoDTO[]> {
    return await invoke<MecanicoDTO[]>('listar_mecanicos_cmd');
  },

  async listarServicios(): Promise<ServicioResumenDTO[]> {
    const servicios = await invoke<ServicioResumenDTO[]>('listar_servicios_cmd');
    return servicios.map((s) => ({
      ...s,
      precio_venta_soles:
        s.precio_venta_soles ??
        (typeof s.precio_venta_cent === 'number' ? s.precio_venta_cent / 100 : 0),
    }));
  },

  async listarProductosInsumo(): Promise<ProductoInsumoDTO[]> {
    const rawProds = await invoke<any[]>('listar_productos_cmd');
    return rawProds.map((p) => {
      let soles = 0;
      if (typeof p.precio_venta_soles === 'number') {
        soles = p.precio_venta_soles;
      } else if (typeof p.precio_venta === 'number') {
        soles = p.precio_venta / 100;
      } else if (typeof p.precio_venta_cent === 'number') {
        soles = p.precio_venta_cent / 100;
      }

      return {
        id_producto: p.id_producto,
        codigo: p.codigo || p.codigo_barras || '',
        nombre: p.nombre,
        categoria: p.categoria || p.nombre_categoria || '',
        unidad_medida: p.unidad_medida || 'UND',
        stock_actual: p.stock_actual ?? 0,
        precio_venta: p.precio_venta ?? p.precio_venta_cent ?? 0,
        precio_venta_soles: soles,
      };
    });
  },
};

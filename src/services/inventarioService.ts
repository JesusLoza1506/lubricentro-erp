import { invoke } from '@tauri-apps/api/core';
import {
  ProductoAdminDTO,
  ProductoPublicoDTO,
  ProductosPaginadosDTO,
  CrearProductoDTO,
  EditarProductoDTO,
  AjustarStockDTO,
  ConteoFisicoDTO,
  MovimientoKardexDTO,
  Categoria,
  ProveedorDTO,
  CrearProveedorDTO,
  ActualizarProveedorDTO,
  RegistrarIngresoDTO,
  AnularIngresoDTO,
  IngresoStockResponseDTO,
  ServicioDTO,
  CrearServicioDTO,
  ActualizarServicioDTO,
} from '../types/inventario';

export interface ResumenSaldosKardexDTO {
  saldo_inicial: number;
  saldo_final: number;
}

export const inventarioService = {
  listarProductos: async (
    incluirInactivos: boolean = false
  ): Promise<(ProductoAdminDTO | ProductoPublicoDTO)[]> => {
    return await invoke<(ProductoAdminDTO | ProductoPublicoDTO)[]>('listar_productos_cmd', {
      incluirInactivos,
    });
  },

  listarProductosAdminPaginado: async (
    busqueda?: string,
    categoria?: string,
    incluirInactivos?: boolean,
    limite: number = 20,
    offset: number = 0
  ): Promise<ProductosPaginadosDTO> => {
    return await invoke<ProductosPaginadosDTO>('listar_productos_admin_paginado_cmd', {
      busqueda,
      categoria,
      incluirInactivos,
      limite,
      offset,
    });
  },

  listarProductosPublicos: async (): Promise<ProductoPublicoDTO[]> => {
    return await invoke<ProductoPublicoDTO[]>('listar_productos_publicos_cmd');
  },

  listarProductosCriticos: async (): Promise<(ProductoAdminDTO | ProductoPublicoDTO)[]> => {
    return await invoke<(ProductoAdminDTO | ProductoPublicoDTO)[]>('listar_productos_criticos_cmd');
  },

  registrarProducto: async (payload: CrearProductoDTO): Promise<number> => {
    return await invoke<number>('registrar_producto_cmd', { payload });
  },

  actualizarProducto: async (payload: EditarProductoDTO): Promise<void> => {
    return await invoke('actualizar_producto_cmd', { payload });
  },

  eliminarProducto: async (idProducto: number): Promise<void> => {
    return await invoke('eliminar_producto_cmd', { idProducto });
  },

  reactivarProducto: async (idProducto: number): Promise<void> => {
    return await invoke('reactivar_producto_cmd', { idProducto });
  },

  despacharProductoGranel: async (
    idProducto: number,
    cantidad: number,
    motivo?: string
  ): Promise<void> => {
    return await invoke('despachar_producto_granel_cmd', { idProducto, cantidad, motivo });
  },

  ajustarStock: async (payload: AjustarStockDTO): Promise<void> => {
    return await invoke('ajustar_stock_cmd', { payload });
  },

  procesarConteoFisico: async (payload: ConteoFisicoDTO): Promise<void> => {
    return await invoke('procesar_conteo_fisico_cmd', { payload });
  },

  listarKardexProducto: async (
    idProducto: number,
    limite: number = 20,
    offset: number = 0,
    fechaInicio?: string,
    fechaFin?: string,
    tipoMovimiento?: string
  ): Promise<MovimientoKardexDTO[]> => {
    return await invoke<MovimientoKardexDTO[]>('listar_kardex_producto_cmd', {
      idProducto,
      limite,
      offset,
      fechaInicio,
      fechaFin,
      tipoMovimiento,
    });
  },

  obtenerResumenSaldosKardex: async (
    idProducto: number,
    fechaInicio?: string,
    fechaFin?: string,
    tipoMovimiento?: string
  ): Promise<ResumenSaldosKardexDTO> => {
    return await invoke<ResumenSaldosKardexDTO>('obtener_resumen_saldos_kardex_cmd', {
      idProducto,
      fechaInicio,
      fechaFin,
      tipoMovimiento,
    });
  },

  registrarIngresoCompra: async (payload: RegistrarIngresoDTO): Promise<number> => {
    return await invoke<number>('registrar_ingreso_compra_cmd', { payload });
  },

  listarIngresosStock: async (
    limite: number = 20,
    offset: number = 0,
    estado?: string,
    origen?: string
  ): Promise<IngresoStockResponseDTO[]> => {
    return await invoke<IngresoStockResponseDTO[]>('listar_ingresos_stock_cmd', {
      limite,
      offset,
      estado,
      origen,
    });
  },

  anularIngresoCompra: async (payload: AnularIngresoDTO): Promise<void> => {
    return await invoke('anular_ingreso_compra_cmd', { payload });
  },

  listarCategorias: async (): Promise<Categoria[]> => {
    return await invoke<Categoria[]>('listar_categorias_cmd');
  },

  listarProveedores: async (): Promise<ProveedorDTO[]> => {
    return await invoke<ProveedorDTO[]>('listar_proveedores_cmd');
  },

  crearProveedor: async (payload: CrearProveedorDTO): Promise<number> => {
    return await invoke<number>('crear_proveedor_cmd', { payload });
  },

  actualizarProveedor: async (payload: ActualizarProveedorDTO): Promise<void> => {
    return await invoke('actualizar_proveedor_cmd', { payload });
  },

  listarServiciosAdmin: async (): Promise<ServicioDTO[]> => {
    return await invoke<ServicioDTO[]>('listar_servicios_admin_cmd');
  },

  listarServiciosPublicos: async (): Promise<ServicioDTO[]> => {
    return await invoke<ServicioDTO[]>('listar_servicios_publicos_cmd');
  },

  crearServicio: async (payload: CrearServicioDTO): Promise<number> => {
    return await invoke<number>('crear_servicio_cmd', { payload });
  },

  actualizarServicio: async (payload: ActualizarServicioDTO): Promise<void> => {
    return await invoke('actualizar_servicio_cmd', { payload });
  },

  cambiarEstadoServicio: async (idServicio: number, activo: boolean): Promise<void> => {
    return await invoke('cambiar_estado_servicio_cmd', { idServicio, activo });
  },
};
